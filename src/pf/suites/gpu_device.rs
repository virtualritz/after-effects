use crate::*;
use std::ffi::c_void;

define_suite!(
    GPUDeviceSuite,
    PF_GPUDeviceSuite1,
    kPFGPUDeviceSuite,
    kPFGPUDeviceSuiteVersion1
);
impl GPUDeviceSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_device_info(&self, in_data_handle: InData, device_index: usize) -> Result<ae_sys::PF_GPUDeviceInfo, Error> {
        call_suite_fn_single!(self, GetDeviceInfo -> ae_sys::PF_GPUDeviceInfo, in_data_handle.effect_ref().as_ptr(), device_index as u32)
    }
    pub fn get_gpu_world_data(&self, in_data_handle: InData, mut world: EffectWorld) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, GetGPUWorldData -> *mut c_void, in_data_handle.effect_ref().as_ptr(), world.as_mut_ptr())
    }
}
