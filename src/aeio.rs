pub use crate::*;

define_handle_wrapper!(InSpecHandle, AEIO_InSpecH, in_spec_ptr);

define_handle_wrapper!(Handle, AEIO_Handle, handle_ptr);

impl From<aeio::Handle> for ae_sys::AEGP_MemHandle {
    fn from(handle: aeio::Handle) -> Self {
        let ae_sys_handle: ae_sys::AEGP_MemHandle = handle.handle_ptr;
        ae_sys_handle
    }
}
