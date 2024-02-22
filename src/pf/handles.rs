use super::*;

#[derive(Debug)]
pub struct Handle<'a, T: 'a> {
    suite: HandleSuite,
    handle: ae_sys::PF_Handle,
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
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn as_ref_mut(&self) -> Result<&'a mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &mut *self.ptr })
        }
    }
}

impl<'a, T> Drop for HandleLock<'a, T> {
    fn drop(&mut self) {
        self.parent_handle.suite.unlock_handle(self.parent_handle.handle);
    }
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(value: T) -> Result<Handle<'a, T>, Error> {
        match HandleSuite::new() {
            Ok(suite) => {
                let handle = suite.new_handle(std::mem::size_of::<T>() as u64);
                if handle.is_null() {
                    return Err(Error::OutOfMemory);
                }

                let ptr = suite.lock_handle(handle) as *mut T;
                if ptr.is_null() {
                    return Err(Error::InvalidIndex);
                }

                unsafe { ptr.write(value) };

                suite.unlock_handle(handle);

                Ok(Handle {
                    suite,
                    handle,
                    _marker: PhantomData,
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    pub fn set(&mut self, value: T) {
        let ptr = self.suite.lock_handle(self.handle) as *mut T;
        if !ptr.is_null() {
            unsafe {
                // Run destructors, if any.
                ptr.read()
            };
        }
        unsafe { ptr.write(value) };
        self.suite.unlock_handle(self.handle);
    }

    pub fn lock(&mut self) -> Result<HandleLock<T>, Error> {
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
        let ptr = unsafe { *(self.handle as *const *const T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &(*ptr) })
        }
    }

    pub fn as_mut(&self) -> Result<&'a mut T, Error> {
        let ptr = unsafe { *(self.handle as *mut *mut T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &mut (*ptr) })
        }
    }

    pub fn size(&self) -> usize {
        self.suite.get_handle_size(self.handle) as usize
    }

    /*
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        call_suite_fn!(self, host_resize_handle, size as u64, &mut self.handle)
    }*/

    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<Handle<'a, T>, Error> {
        match HandleSuite::new() {
            Ok(suite) => {
                Ok(Handle {
                    suite,
                    handle,
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
        let ptr = unsafe { *(self.handle as *const *const T) };
        if !ptr.is_null() {
            unsafe { ptr.read() };
        }

        self.suite.dispose_handle(self.handle);
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
    suite: HandleSuite,
    handle: ae_sys::PF_Handle,
    is_owned: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FlatHandle<'a> {
    pub fn new(slice: impl Into<Vec<u8>>) -> Result<FlatHandle<'a>, Error> {

        let suite = HandleSuite::new()?;
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
    pub fn lock<'b: 'a>(&'b self) -> Result<FlatHandleLock, Error> {
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
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*std::ptr::slice_from_raw_parts(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_slice_mut(&'a self) -> Option<&'a mut [u8]> {
        let ptr = unsafe { *(self.handle as *const *mut u8) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *std::ptr::slice_from_raw_parts_mut(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { *(self.handle as *const *const u8) }
    }

    #[inline]
    pub fn as_ptr_mut(&self) -> *mut u8 {
        unsafe { *(self.handle as *const *mut u8) }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe {
                &*std::ptr::slice_from_raw_parts(*(self.handle as *const *const u8), self.size())
            }
            .to_vec()
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.suite.get_handle_size(self.handle) as usize
    }

    #[inline]
    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Error> {
        if handle.is_null() {
            return Err(Error::Generic);
        }
        let suite = HandleSuite::new()?;
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
        let suite = HandleSuite::new()?;

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
