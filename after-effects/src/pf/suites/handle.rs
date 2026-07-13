use crate::*;
use ae_sys::*;

define_suite!(
    /// Use After Effects for any memory allocations of significant size.
    /// For small allocations, you can use new and delete, but this is the exception, not the rule.
    /// In low-memory conditions (such as during RAM preview), it's very important that plug-ins deal gracefully with out-of-memory conditions, and not compete with After Effects for OS memory.
    /// By using our memory allocation functions, After Effects can know when to free cached images, to avoid memory swapping.
    /// Failing to use our functions for sizable allocations can cause lock-ups, crashes, and tech support calls. Don't do that.
    ///
    /// Handles passed to you by After Effects are locked for you before you're called, and unlocked once you return.
    HandleSuite,
    PF_HandleSuite1,
    kPFHandleSuite,
    kPFHandleSuiteVersion1
);

impl HandleSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Create a new handle of the given size.
    pub fn new_handle(&self, size: A_HandleSize) -> PF_Handle {
        call_suite_fn_no_err!(self, host_new_handle, size)
    }

    /// Lock the handle and return a pointer to the data.
    ///
    /// `host_lock_handle` has been a no-op in After Effects since CS6, so we no
    /// longer call it. A `PF_Handle` is a pointer to the data pointer, so the
    /// data pointer is obtained by simply dereferencing the handle. Returns a
    /// null pointer if the handle itself is null.
    pub fn lock_handle(&self, pf_handle: PF_Handle) -> *mut std::ffi::c_void {
        if pf_handle.is_null() {
            return std::ptr::null_mut();
        }
        unsafe { *pf_handle }
    }

    /// Unlock the handle.
    ///
    /// `host_unlock_handle` has been a no-op in After Effects since CS6, so this
    /// does nothing. It is kept for source compatibility and pairs with
    /// [`Self::lock_handle`].
    pub fn unlock_handle(&self, _pf_handle: PF_Handle) {}

    /// Dispose of the handle and free the memory.
    pub fn dispose_handle(&self, pf_handle: PF_Handle) {
        call_suite_fn_no_err!(self, host_dispose_handle, pf_handle)
    }

    /// Returns the size, in bytes, of the reallocatable block whose handle is passed in.
    pub fn handle_size(&self, pf_handle: PF_Handle) -> A_HandleSize {
        call_suite_fn_no_err!(self, host_get_handle_size, pf_handle)
    }

    /// Resize the handle.
    pub fn resize_handle(&self, new_size: usize, handle: *mut PF_Handle) -> Result<(), Error> {
        call_suite_fn!(self, host_resize_handle, new_size as A_HandleSize, handle)
    }
}
