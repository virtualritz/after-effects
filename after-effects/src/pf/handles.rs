use super::*;

#[derive(Debug)]
pub struct Handle<'a, T: 'a> {
    suite: pf::suites::Handle,
    handle: ae_sys::PF_Handle,
    owned: bool,
    _marker: PhantomData<&'a T>,
}

pub struct HandleLock<'a, T> {
    parent_handle: &'a Handle<'a, T>,
    ptr: *mut T,
}

impl<'a, T> HandleLock<'a, T> {
    pub fn as_ref(&self) -> Result<&'a T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            // SAFETY: The pointer is valid because:
            // 1. We've verified it's not null above
            // 2. The ptr was obtained from suite.lock_handle() which returns a valid pointer to allocated memory
            // 3. The HandleLock ensures the handle remains locked (and thus the memory valid) for its lifetime
            // 4. The 'a lifetime ensures the reference doesn't outlive the parent Handle
            // UB would occur if: the handle was unlocked while this reference exists, or if the handle was disposed
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn as_ref_mut(&mut self) -> Result<&mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            // SAFETY: The pointer is valid for mutable access because:
            // 1. We've verified it's not null above
            // 2. The ptr was obtained from suite.lock_handle() which provides exclusive access to the handle's memory
            // 3. The &mut self requirement ensures no other mutable references exist
            // 4. The HandleLock's lifetime ensures the handle remains locked during access
            // UB would occur if: multiple mutable references were created, handle was unlocked, or memory was freed
            Ok(unsafe { &mut *self.ptr })
        }
    }
}

impl<'a, T> Drop for HandleLock<'a, T> {
    fn drop(&mut self) {
        self.parent_handle.suite.unlock_handle(self.parent_handle.handle);
    }
}

pub struct BorrowedHandleLock<T> {
    suite: pf::suites::Handle,
    handle: ae_sys::PF_Handle,
    ptr: *mut T,
}

impl<T> BorrowedHandleLock<T> {
    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<Self, Error> {
        match pf::suites::Handle::new() {
            Ok(suite) => {
                let ptr = suite.lock_handle(handle) as *mut T;
                if ptr.is_null() {
                    return Err(Error::Generic);
                }
                Ok(BorrowedHandleLock {
                    suite,
                    handle,
                    ptr
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }
}
impl<T> std::ops::Deref for BorrowedHandleLock<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: The pointer dereference is safe because:
        // 1. self.ptr was obtained from suite.lock_handle() and verified non-null in from_raw()
        // 2. The handle remains locked for the entire lifetime of BorrowedHandleLock
        // 3. Drop implementation ensures unlock happens when this struct is dropped
        // 4. The returned reference's lifetime is tied to &self, preventing use-after-unlock
        // UB would occur if: the handle was unlocked externally, the memory was freed, or ptr was invalid
        unsafe { &*self.ptr }
    }
}
impl<T> std::ops::DerefMut for BorrowedHandleLock<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The mutable pointer dereference is safe because:
        // 1. self.ptr was obtained from suite.lock_handle() which provides exclusive access
        // 2. The &mut self requirement ensures no other references to the data exist
        // 3. The handle remains locked while BorrowedHandleLock exists
        // 4. The returned mutable reference's lifetime is tied to &mut self
        // UB would occur if: multiple mutable references existed, handle was unlocked, or memory was freed
        unsafe { &mut *self.ptr }
    }
}
impl<T> Drop for BorrowedHandleLock<T> {
    fn drop(&mut self) {
        self.suite.unlock_handle(self.handle);
    }
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(value: T) -> Result<Handle<'a, T>, Error> {
        assert!(std::mem::size_of::<T>() > 0);

        match pf::suites::Handle::new() {
            Ok(suite) => {
                let handle = suite.new_handle(std::mem::size_of::<T>() as u64);
                if handle.is_null() {
                    return Err(Error::OutOfMemory);
                }

                let ptr = suite.lock_handle(handle) as *mut T;
                if ptr.is_null() {
                    return Err(Error::InvalidIndex);
                }

                // SAFETY: Writing to the pointer is safe because:
                // 1. The handle was just allocated with size_of::<T>() bytes, providing sufficient space
                // 2. ptr is locked and verified non-null above
                // 3. ptr.write() initializes the memory without reading uninitialized data
                // 4. The memory is properly aligned (guaranteed by the suite's allocation)
                // 5. We unlock immediately after writing, ensuring no dangling locked state
                // UB would occur if: the allocation size was incorrect, ptr was misaligned, or handle was invalid
                unsafe { ptr.write(value) };

                suite.unlock_handle(handle);

                Ok(Handle {
                    suite,
                    handle,
                    owned: true,
                    _marker: PhantomData,
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    pub fn set(&mut self, value: T) -> Result<(), Error> {
        let ptr = self.suite.lock_handle(self.handle) as *mut T;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            // SAFETY: Reading and writing the pointer is safe because:
            // 1. The handle is locked, giving us exclusive access to the memory
            // 2. ptr is verified non-null above
            // 3. ptr.read() runs the destructor on the old value and moves it out (preventing double-drop)
            // 4. ptr.write(value) initializes the memory with the new value without reading old data
            // 5. The memory was previously initialized by Handle::new() or a prior set() call
            // 6. The size and alignment are correct (enforced at Handle creation)
            // UB would occur if: the memory was uninitialized, ptr was invalid, or multiple threads accessed concurrently
            unsafe {
                // Run destructors, if any.
                ptr.read();
                ptr.write(value);
            }
            self.suite.unlock_handle(self.handle);
            Ok(())
        }
    }

    pub fn lock(&mut self) -> Result<HandleLock<'_, T>, Error> {
        let ptr = self.suite.lock_handle(self.handle) as *mut T;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(HandleLock {
                parent_handle: self,
                ptr,
            })
        }
    }

    pub fn as_ref(&self) -> Result<&'a T, Error> {
        // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
        // 1. PF_Handle is defined as a handle type that acts as an indirect pointer (pointer to pointer)
        // 2. The handle was created by the suite and is valid for the lifetime 'a
        // 3. This follows After Effects' handle model where handles point to relocatable memory
        // UB would occur if: the handle was disposed, the handle type doesn't match the memory layout, or 'a is incorrect
        let ptr = unsafe { *(self.handle as *const *const T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            // SAFETY: Creating a reference from the dereferenced pointer is safe because:
            // 1. We've verified ptr is not null above
            // 2. The pointer points to valid, initialized memory of type T
            // 3. The lifetime 'a ensures this reference doesn't outlive the handle's validity
            // 4. The handle was initialized in Handle::new() with a valid T value
            // UB would occur if: ptr pointed to freed/invalid memory, the data was uninitialized, or lifetime 'a is too long
            Ok(unsafe { &(*ptr) })
        }
    }

    pub fn as_mut(&self) -> Result<&'a mut T, Error> {
        // SAFETY: Dereferencing the handle as a mutable pointer-to-pointer is safe because:
        // 1. PF_Handle is a handle type that can be dereferenced as an indirect pointer
        // 2. The handle is valid and owned by this Handle instance
        // 3. This access pattern matches After Effects' handle semantics
        // UB would occur if: the handle was disposed, concurrent access occurred, or the handle was invalid
        let ptr = unsafe { *(self.handle as *mut *mut T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            // SAFETY: Creating a mutable reference from the dereferenced pointer is safe because:
            // 1. We've verified ptr is not null above
            // 2. The pointer points to valid, initialized memory of type T
            // 3. The lifetime 'a ties the reference to the handle's validity period
            // 4. No other references to this data can exist (guaranteed by Rust's &mut rules at call site)
            // UB would occur if: multiple mutable references existed, memory was freed, or data was uninitialized
            Ok(unsafe { &mut (*ptr) })
        }
    }

    pub fn size(&self) -> usize {
        self.suite.handle_size(self.handle) as usize
    }

    /*
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        call_suite_fn!(self, host_resize_handle, size as u64, &mut self.handle)
    }*/

    pub fn from_raw(handle: ae_sys::PF_Handle, owned: bool) -> Result<Handle<'a, T>, Error> {
        assert!(!handle.is_null());
        match pf::suites::Handle::new() {
            Ok(suite) => {
                Ok(Handle {
                    suite,
                    handle,
                    owned,
                    _marker: PhantomData,
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: Handle<T>) -> ae_sys::PF_Handle {
        //let us = crate::aegp::UtilitySuite::new().unwrap();
        //us.write_to_os_console("Handle::into_raw()").unwrap();

        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here
        std::mem::forget(handle);
        // Make sure drop(Handle) does *not*
        // actually drop anything since we're
        // passing ownership.
        //handle.dispose = false;
        return_handle
        // drop(handle) gets called.
    }

    /// Returns the raw handle.
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
}

impl<'a, T: 'a> Drop for Handle<'a, T> {
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
            // 1. The handle is still valid (not yet disposed)
            // 2. This follows After Effects' handle indirection model
            // 3. We immediately check for null before dereferencing further
            // UB would occur if: the handle was already disposed or was never properly initialized
            let ptr = unsafe { *(self.handle as *const *const T) };
            if !ptr.is_null() {
                // SAFETY: Reading from ptr to run the destructor is safe because:
                // 1. We've verified ptr is not null above
                // 2. The memory was initialized in Handle::new() or Handle::set()
                // 3. ptr.read() moves the value out, running its destructor
                // 4. This prevents a memory leak of T's resources
                // 5. We immediately dispose the handle afterward, so no double-drop can occur
                // UB would occur if: the memory was already dropped, uninitialized, or ptr was invalid
                unsafe { ptr.read() };
            }

            self.suite.dispose_handle(self.handle);
        }
    }
}

pub struct FlatHandleLock<'a, 'b: 'a> {
    parent_handle: &'a FlatHandle<'b>,
}

impl<'a, 'b> Drop for FlatHandleLock<'a, 'b> {
    fn drop(&mut self) {
        self.parent_handle.suite.unlock_handle(self.parent_handle.handle);
    }
}

/// A flat handle takes a [`Vec<u8>``] as data. This is useful when data it passed
/// to Ae permanently or between runs of your plug-in.
/// You can use something like [`bincode::serialize()``] to serialize your data
/// structure into a flat [`Vec<u8>``].
#[derive(Debug)]
pub struct FlatHandle<'a> {
    suite: pf::suites::Handle,
    handle: ae_sys::PF_Handle,
    is_owned: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FlatHandle<'a> {
    pub fn new(slice: impl Into<Vec<u8>>) -> Result<FlatHandle<'a>, Error> {

        let suite = pf::suites::Handle::new()?;
        let vector = slice.into();

        let handle = suite.new_handle(vector.len() as u64);
        if handle.is_null() {
            return Err(Error::OutOfMemory);
        }

        let ptr = suite.lock_handle(handle) as *mut u8;
        if ptr.is_null() {
            return Err(Error::OutOfMemory);
        }

        let dest = std::ptr::slice_from_raw_parts_mut(ptr, vector.len());

        // SAFETY: Copying data to the dereferenced slice pointer is safe because:
        // 1. ptr was obtained from suite.lock_handle() and verified non-null above
        // 2. The handle was allocated with exactly vector.len() bytes
        // 3. dest is constructed with the same length as the source vector
        // 4. copy_from_slice() performs a proper memory copy for the exact length
        // 5. The handle is locked, ensuring the memory won't be relocated during the copy
        // 6. We unlock immediately after the copy completes
        // UB would occur if: the allocation size didn't match, ptr was invalid, or the handle was unlocked during copy
        unsafe {
            (*dest).copy_from_slice(vector.as_slice());
        }

        suite.unlock_handle(handle);
        Ok(Self {
            suite,
            handle,
            is_owned: true,
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        self.suite.resize_handle(size, &mut self.handle)
    }

    #[inline]
    pub fn lock<'b: 'a>(&'b self) -> Result<FlatHandleLock<'b, 'a>, Error> {
        let ptr = self.suite.lock_handle(self.handle) as *mut u8;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(FlatHandleLock {
                parent_handle: self,
            })
        }
    }

    #[inline]
    pub fn as_slice(&'a self) -> Option<&'a [u8]> {
        // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
        // 1. PF_Handle is an indirect pointer (handle) to relocatable memory
        // 2. The handle is valid for the lifetime 'a
        // 3. We check for null before further dereferencing
        // UB would occur if: the handle was disposed or invalid
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Creating a slice from the raw pointer is safe because:
            // 1. ptr is verified non-null above
            // 2. self.size() returns the actual allocated size from the suite
            // 3. The memory is valid for reads for the lifetime 'a
            // 4. The u8 type has no alignment requirements beyond 1-byte
            // 5. The slice's lifetime 'a ensures it doesn't outlive the handle
            // UB would occur if: self.size() returned incorrect length, memory was freed, or concurrent mutation occurred
            Some(unsafe { &*std::ptr::slice_from_raw_parts(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_slice_mut(&'a self) -> Option<&'a mut [u8]> {
        // SAFETY: Dereferencing the handle as a pointer-to-mutable-pointer is safe because:
        // 1. PF_Handle supports mutable access through its indirection
        // 2. The handle is valid and owned by this FlatHandle instance
        // 3. We check for null before creating a mutable reference
        // UB would occur if: the handle was disposed, invalid, or accessed concurrently
        let ptr = unsafe { *(self.handle as *const *mut u8) };
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Creating a mutable slice from the raw pointer is safe because:
            // 1. ptr is verified non-null above
            // 2. self.size() returns the exact allocated size
            // 3. The returned mutable reference has lifetime 'a, tied to the handle's validity
            // 4. Rust's borrowing rules at the call site ensure no other references exist
            // 5. u8 has minimal alignment requirements
            // UB would occur if: multiple mutable references existed, size was wrong, or memory was freed
            Some(unsafe { &mut *std::ptr::slice_from_raw_parts_mut(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        // SAFETY: Dereferencing the handle as a pointer-to-const-pointer is safe because:
        // 1. PF_Handle is designed as an indirect pointer to memory
        // 2. The handle is valid (not disposed) while self exists
        // 3. Returning a raw pointer (not a reference) doesn't create immediate safety issues
        // 4. The caller is responsible for ensuring safe usage of the returned raw pointer
        // Note: The returned pointer could be null if the handle is invalid - caller must check
        // UB would occur if: the handle was disposed, though returning the pointer itself is safe
        unsafe { *(self.handle as *const *const u8) }
    }

    #[inline]
    pub fn as_ptr_mut(&self) -> *mut u8 {
        // SAFETY: Dereferencing the handle as a pointer-to-mutable-pointer is safe because:
        // 1. PF_Handle supports both const and mutable access through indirection
        // 2. The handle is valid while self exists
        // 3. Returning a raw *mut pointer doesn't violate safety (the caller must use it safely)
        // 4. The caller is responsible for ensuring exclusive access when dereferencing
        // Note: The returned pointer could be null - caller must check before dereferencing
        // UB would occur if: the handle was disposed, though returning the pointer itself is safe
        unsafe { *(self.handle as *const *mut u8) }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
        // 1. The handle is valid (not disposed) while self exists
        // 2. We check for null before using the pointer
        // UB would occur if: the handle was disposed or invalid
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            Vec::new()
        } else {
            // SAFETY: Creating a temporary slice reference and copying to Vec is safe because:
            // 1. The handle is dereferenced to get the actual data pointer
            // 2. self.size() provides the correct length of allocated data
            // 3. The slice is immediately copied to a Vec, not stored as a reference
            // 4. The memory is valid for the duration of the to_vec() call
            // 5. u8 has no special alignment requirements
            // UB would occur if: self.size() was incorrect, handle was disposed, or concurrent modification occurred
            unsafe {
                &*std::ptr::slice_from_raw_parts(*(self.handle as *const *const u8), self.size())
            }
            .to_vec()
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.suite.handle_size(self.handle) as usize
    }

    #[inline]
    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Error> {
        if handle.is_null() {
            return Err(Error::Generic);
        }
        let suite = pf::suites::Handle::new()?;
        // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
        // 1. We've verified the handle is not null above
        // 2. The handle comes from After Effects and should follow the PF_Handle contract
        // 3. We check if the dereferenced pointer is null to validate the handle's internal state
        // 4. This is a borrowed handle (is_owned: false), so we assume the caller maintains validity
        // UB would occur if: the handle was not a valid PF_Handle, was already disposed, or doesn't point to a valid pointer
        let ptr = unsafe { *(handle as *const *const u8) };
        if ptr.is_null() {
            Err(Error::InternalStructDamaged)
        } else {
            Ok(Self {
                suite,
                handle,
                is_owned: false,
                _marker: PhantomData,
            })
        }
    }

    #[inline]
    pub fn from_raw_owned(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Error> {
        if handle.is_null() {
            return Err(Error::Generic);
        }
        let suite = pf::suites::Handle::new()?;

        // SAFETY: Dereferencing the handle as a pointer-to-pointer is safe because:
        // 1. We've verified the handle is not null above
        // 2. The handle follows the PF_Handle indirection pattern
        // 3. We validate the internal pointer is non-null before accepting ownership
        // 4. This creates an owned handle (is_owned: true), so we'll properly dispose it in Drop
        // UB would occur if: the handle was invalid, already disposed, or not a proper PF_Handle
        let ptr = unsafe { *(handle as *const *const u8) };
        if ptr.is_null() {
            Err(Error::InternalStructDamaged)
        } else {
            Ok(Self {
                suite,
                handle,
                is_owned: true,
                _marker: PhantomData,
            })
        }
    }

    /// Turns the handle into and owned one
    #[inline]
    pub fn into_owned(mut handle: Self) -> Self {
        handle.is_owned = true;
        handle
    }

    /// Consumes the handle.
    #[inline]
    pub fn into_raw(handle: Self) -> ae_sys::PF_Handle {
        let return_handle = handle.handle;
        // We need to call forget() or else
        // drop() will be called on handle
        // which will dispose the memory.
        // Handle is just on the stack so
        // we're not leaking anything here.
        std::mem::forget(handle);

        return_handle
    }

    #[inline]
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
}
/*
impl<'a> Clone for FlatHandle<'a> {
    fn clone(&self) -> FlatHandle<'a> {
        Self::new(self.as_slice()).unwrap()
    }
}*/

impl<'a> Drop for FlatHandle<'a> {
    #[inline]
    fn drop(&mut self) {
        if self.is_owned {
            self.suite.dispose_handle(self.handle);
        }
    }
}
