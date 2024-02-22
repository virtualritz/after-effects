use crate::*;
use ae_sys::*;

define_suite!(
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

    pub fn new_handle(&self, size: A_HandleSize) -> PF_Handle {
        call_suite_fn_no_err!(self, host_new_handle, size)
    }

    pub fn lock_handle(&self, pf_handle: PF_Handle) -> *mut std::ffi::c_void {
        call_suite_fn_no_err!(self, host_lock_handle, pf_handle)
    }

    pub fn unlock_handle(&self, pf_handle: PF_Handle) {
        call_suite_fn_no_err!(self, host_unlock_handle, pf_handle)
    }

    pub fn dispose_handle(&self, pf_handle: PF_Handle) {
        call_suite_fn_no_err!(self, host_dispose_handle, pf_handle)
    }

    pub fn get_handle_size(&self, pf_handle: PF_Handle) -> A_HandleSize {
        call_suite_fn_no_err!(self, host_get_handle_size, pf_handle)
    }

    pub fn resize_handle(&self, new_size: usize, handle: *mut PF_Handle) -> Result<(), Error> {
        call_suite_fn!(self, host_resize_handle, new_size as A_HandleSize, handle)
    }
}
