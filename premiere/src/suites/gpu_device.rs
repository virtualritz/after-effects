use crate::*;

define_suite!(GPUDeviceSuite, PrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuiteVersion);

impl GPUDeviceSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_device_count(&self) -> Result<usize, Error> {
        let mut count: u32 = 0;
        pr_call_suite_fn!(self.suite_ptr, GetDeviceCount, &mut count)?;
        Ok(count as usize)
    }
    pub fn get_device_info(&self, device_index: u32) -> Result<pr_sys::PrGPUDeviceInfo, Error> {
        let mut info: pr_sys::PrGPUDeviceInfo = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetDeviceInfo, pr_sys::kPrSDKGPUDeviceSuiteVersion, device_index, &mut info)?;
        Ok(info)
    }
    /// Acquire/release exclusive access to inDeviceIndex. All calls below this point generally require access be held.
    /// For full GPU plugins (those that use a separate entry point for GPU rendering) exclusive access is always held.
    /// These calls do not need to be made in that case.
    /// For CUDA calls cuCtxPushCurrent/cuCtxPopCurrent on the current thread to manage the devices context.
    pub fn acquire_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, AcquireExclusiveDeviceAccess, device_index)?;
        Ok(())
    }
    pub fn release_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReleaseExclusiveDeviceAccess, device_index)?;
        Ok(())
    }
    /// All device memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemAlloc or clCreateBuffer.
    pub fn allocate_device_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, AllocateDeviceMemory, device_index, size_in_bytes, &mut ptr)?;
        Ok(ptr)
    }
    pub fn free_device_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, FreeDeviceMemory, device_index, ptr)?;
        Ok(())
    }
    pub fn purge_device_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        let mut bytes_purged = 0;
        pr_call_suite_fn!(self.suite_ptr, PurgeDeviceMemory, device_index, requested_bytes_to_purge, &mut bytes_purged)?;
        Ok(bytes_purged)
    }
    /// All host (pinned) memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemHostAlloc or malloc.
    pub fn allocate_host_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, AllocateHostMemory, device_index, size_in_bytes, &mut ptr)?;
        Ok(ptr)
    }
    pub fn free_host_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, FreeHostMemory, device_index, ptr)?;
        Ok(())
    }
    pub fn purge_host_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        let mut bytes_purged = 0;
        pr_call_suite_fn!(self.suite_ptr, PurgeHostMemory, device_index, requested_bytes_to_purge, &mut bytes_purged)?;
        Ok(bytes_purged)
    }
    /// Information on a GPU ppix. The following ppix functions may also be used:
    /// - PPixSuite::Dispose
    /// - PPixSuite::GetBounds
    /// - PPixSuite::GetRowBytes
    /// - PPixSuite::GetPixelAspectRatio
    /// - PPixSuite::GetPixelFormat
    /// - PPix2Suite::GetFieldOrder
    pub fn create_gpu_ppix(&self, device_index: u32, pixel_format: PixelFormat, width: i32, height: i32, par_numerator: i32, par_denominator: i32, field_type: pr_sys::prFieldType) -> Result<pr_sys::PPixHand, Error> {
        let mut ptr: pr_sys::PPixHand = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, CreateGPUPPix, device_index, pixel_format.into(), width, height, par_numerator, par_denominator, field_type.into(), &mut ptr)?;
        Ok(ptr)
    }
    pub fn get_gpu_ppix_data(&self, ppix_handle: pr_sys::PPixHand) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixData, ppix_handle, &mut ptr)?;
        Ok(ptr)
    }
    pub fn get_gpu_ppix_device_index(&self, ppix_handle: pr_sys::PPixHand) -> Result<u32, Error> {
        let mut index = 0;
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixDeviceIndex, ppix_handle, &mut index)?;
        Ok(index)
    }
    pub fn get_gpu_ppix_size(&self, ppix_handle: pr_sys::PPixHand) -> Result<usize, Error> {
        let mut index = 0;
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixSize, ppix_handle, &mut index)?;
        Ok(index)
    }
}
