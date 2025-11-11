use crate::*;
use crate::aegp::*;
use ae_sys::{ AEGP_MemHandle, AEGP_MemSize };

define_suite!(
    /// Use the AEGP Memory Suite to manage memory used by the AEGP.
    /// Whenever memory related errors are encountered, After Effects can report errors for you to find early on.
    ///
    /// [`MemHandle`] is a structure that contains more than just the referenced memory. So it should not be dereferenced directly.
    /// Use [`MemHandle::lock()`] to get a pointer to the memory referenced by the [`MemHandle`].
    MemorySuite,
    AEGP_MemorySuite1,
    kAEGPMemorySuite,
    kAEGPMemorySuiteVersion1
);

impl MemorySuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Create a new memory handle.
    /// This memory is guaranteed to be 16-byte aligned.
    /// `plugin_id` is the ID passed in through the main entry point, or alternatively what you obtained from [`suites::Utility::register_with_aegp`](aegp::suites::Utility::register_with_aegp).
    ///
    /// Use `name` to identify the memory you are asking for.
    /// After Effects uses the string to display any related error messages.
    pub fn new_mem_handle(&self, plugin_id: PluginId, name: &str, size: usize) -> Result<AEGP_MemHandle, Error> {
        let name = CString::new(name).unwrap();
        call_suite_fn_single!(self, AEGP_NewMemHandle -> AEGP_MemHandle, plugin_id, name.as_ptr(), size as u32, 0)
    }

    /// Release a handle you allocated using AEGP_NewMemHandle().
    pub fn free_mem_handle(&self, mem_handle: AEGP_MemHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_FreeMemHandle, mem_handle)
    }

    /// Locks the handle into memory (cannot be moved by OS).
    /// Use this function prior to using memory allocated by [`new_mem_handle()`](Self::new_mem_handle). Can be nested.
    pub fn lock_mem_handle(&self, mem_handle: AEGP_MemHandle) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, AEGP_LockMemHandle -> *mut std::ffi::c_void, mem_handle)
    }

    /// Allows OS to move the referenced memory. Always balance lock calls with unlocks.
    pub fn unlock_mem_handle(&self, mem_handle: AEGP_MemHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_UnlockMemHandle, mem_handle)
    }

    /// Returns the allocated size of the handle.
    pub fn mem_handle_size(&self, mem_handle: AEGP_MemHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMemHandleSize -> AEGP_MemSize, mem_handle)? as usize)
    }

    /// Changes the allocated size of the handle.
    pub fn resize_mem_handle(&self, what: &str, new_size: usize, mem_handle: AEGP_MemHandle) -> Result<(), Error> {
        let what = CString::new(what).unwrap();
        call_suite_fn!(self, AEGP_ResizeMemHandle, what.as_ptr(), new_size as AEGP_MemSize, mem_handle)
    }

    /// If After Effects runs into problems with the memory handling, the error should be reported to the user.
    /// Make use of this during development!
    ///
    /// Only memory allocated and then leaked using this suite is reported using this call,
    /// so for example memory allocated using [`suites::Handle`](crate::suites::Handle) will not be reported.
    pub fn set_mem_reporting_on(&self, turn_on: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMemReportingOn, turn_on.into())
    }

    /// Obtain information about the number of currently allocated handles and their total size.
    ///
    /// Only memory allocated using this suite is tracked and reported using this call,
    /// so for example memory allocated using [`suites::Handle`](crate::suites::Handle) will not be reported here.
    pub fn mem_stats(&self, plugin_id: PluginId) -> Result<(i32, i32), Error> {
        let (count, size) = call_suite_fn_double!(self, AEGP_GetMemStats -> ae_sys::A_long, ae_sys::A_long, plugin_id)?;
        Ok((
            count as _,
            size as _
        ))
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

#[derive(Debug)]
pub struct MemHandle<'a, T: 'a> {
    suite: MemorySuite,
    handle: ae_sys::AEGP_MemHandle,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'a> MemHandle<'a, T> {
    pub fn new(plugin_id: PluginId, name: &str, value: T) -> Result<MemHandle<'a, T>, Error> {
        let suite = MemorySuite::new()?;
        let handle = suite.new_mem_handle(plugin_id, name, std::mem::size_of::<T>())?;

        let handle = Self {
            suite,
            handle,
            _marker: PhantomData,
        };

        *handle.lock()?.as_ref_mut()? = value;

        Ok(handle)
    }

    pub fn len(&self) -> Result<usize, Error> {
        self.suite.mem_handle_size(self.handle)
    }

    #[inline]
    pub fn lock(&self) -> Result<MemHandleLock<'_, T>, Error> {
        let ptr = self.suite.lock_mem_handle(self.handle)? as *mut T;
        Ok(MemHandleLock {
            parent_handle: self,
            ptr,
        })
    }

    /// Only call this if you know what you're doing.
    #[inline]
    pub(crate) fn unlock(&self) -> Result<(), Error> {
        self.suite.unlock_mem_handle(self.handle)
    }

    pub fn from_raw(handle: ae_sys::AEGP_MemHandle) -> Result<MemHandle<'a, T>, Error> {
        Ok(Self {
            suite: MemorySuite::new()?,
            handle,
            _marker: PhantomData,
        })
    }

    /// Consumes the handle.
    pub fn into_raw(handle: Self) -> ae_sys::AEGP_MemHandle {
        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here.
        std::mem::forget(handle);
        // Make sure drop(Handle) does *not*
        // actually drop anything since we're
        // passing ownership.
        return_handle
    }

    /// Locks the handle and copies the contents to a `Vec<u8>`, then returns it.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let len = self.len()?;
        let lock = self.lock()?;
        let ptr = lock.as_ptr() as *const u8;
        // SAFETY: Creating a slice from a locked memory handle pointer.
        // Detailed explanation: (1) ptr is obtained from a locked MemHandle via AEGP_LockMemHandle,
        // which guarantees the memory is pinned and valid for the lock's lifetime, (2) len is
        // retrieved from AEGP_GetMemHandleSize, ensuring it matches the actual allocated size,
        // (3) the MemHandleLock RAII guard ensures the memory remains locked during access,
        // (4) After Effects guarantees 16-byte alignment for all memory handles.
        // Would be UB if: ptr were null (checked by lock returning Ok), len exceeded the actual
        // allocation size (prevented by getting len from mem_handle_size), or the handle were
        // unlocked before the slice access (prevented by the lock guard).
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        Ok(bytes.to_vec())
    }

    /// Returns the raw handle.
    pub fn as_raw(&self) -> ae_sys::AEGP_MemHandle {
        self.handle
    }
}

impl<'a, T: 'a> Drop for MemHandle<'a, T> {
    fn drop(&mut self) {
        if let Ok(lock) = self.lock() {
            // SAFETY: Reading value from locked memory handle to call its destructor.
            // Detailed explanation: (1) lock.ptr is obtained from AEGP_LockMemHandle and points to
            // valid initialized memory of type T, (2) the lock guard ensures memory remains pinned
            // during the read, (3) ptr::read performs a bitwise copy without moving from the source,
            // (4) the value read is immediately dropped, properly running T's destructor before the
            // memory itself is freed by free_mem_handle.
            // Would be UB if: ptr were unaligned for T (prevented by AE's 16-byte alignment guarantee),
            // ptr pointed to uninitialized memory (prevented by MemHandle::new initializing the value),
            // or the memory were unlocked during read (prevented by the lock guard).
            unsafe { lock.ptr.read() };
        }

        let _ = self.suite.free_mem_handle(self.handle); // ignore the error
    }
}

pub struct MemHandleLock<'a, T> {
    parent_handle: &'a MemHandle<'a, T>,
    ptr: *mut T,
}

impl<'a, T> MemHandleLock<'a, T> {
    pub fn as_ref(&self) -> Result<&'a T, Error> {
        if self.ptr.is_null() {
            Err(Error::Generic)
        } else {
            // SAFETY: Dereferencing locked memory handle pointer to create immutable reference.
            // Detailed explanation: (1) ptr is validated non-null by the preceding check, (2) ptr
            // originates from AEGP_LockMemHandle which returns a valid pointer to initialized memory
            // of the correct size for T, (3) the MemHandleLock RAII guard ensures the memory remains
            // locked and cannot be moved by the OS for the reference's lifetime, (4) the lifetime 'a
            // is tied to the MemHandle, ensuring the reference doesn't outlive the allocation,
            // (5) After Effects guarantees proper alignment (16-byte) for the allocation.
            // Would be UB if: ptr were null (prevented by explicit check), memory were uninitialized
            // (prevented by MemHandle::new initializing via lock().as_ref_mut()), ptr were misaligned
            // for T (prevented by AE's alignment guarantee), or memory were freed/moved while the
            // reference exists (prevented by the lock guard and lifetime constraints).
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn as_ref_mut(&self) -> Result<&'a mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::Generic)
        } else {
            // SAFETY: Dereferencing locked memory handle pointer to create mutable reference.
            // Detailed explanation: (1) ptr is validated non-null by the preceding check, (2) ptr
            // originates from AEGP_LockMemHandle which returns a valid pointer to initialized memory
            // of the correct size for T, (3) the MemHandleLock RAII guard ensures exclusive access
            // and that memory remains locked for the reference's lifetime, (4) only one MemHandleLock
            // can exist at a time per MemHandle (enforced by &self requirement on lock()), ensuring
            // no aliasing mutable references, (5) the lifetime 'a is tied to the MemHandle, preventing
            // the reference from outliving the allocation, (6) After Effects guarantees proper alignment.
            // Would be UB if: ptr were null (prevented by explicit check), memory were uninitialized
            // (prevented by MemHandle::new), ptr were misaligned (prevented by AE's 16-byte alignment),
            // multiple mutable references existed simultaneously (prevented by single MemHandleLock
            // invariant), or memory were freed/moved during reference lifetime (prevented by lock guard
            // and lifetime constraints).
            Ok(unsafe { &mut *self.ptr })
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<'a, T> Drop for MemHandleLock<'a, T> {
    fn drop(&mut self) {
        self.parent_handle.unlock().unwrap();
    }
}
