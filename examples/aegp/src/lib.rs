use after_effects::{sys::SPBasicSuite, Error};

#[no_mangle]
pub unsafe extern "C" fn EntryPointFunc(
    _pica_basic: *const SPBasicSuite,
    _major_version: i32,
    _minor_version: i32,
    _aegp_plugin_id: i32,
    _global_refcon: *mut *mut std::ffi::c_void,
) -> Error {
    Error::None
}
