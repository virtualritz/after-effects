use crate::*;
use std::ffi::c_void;
use ae_sys:: { A_u_long, PF_EffectWorld, PF_ProgPtr };

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

    /// This will return the number of gpu devices the host supports.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// Returns the number of available devices.
    pub fn device_count(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, GetDeviceCount -> A_u_long, effect_ref.as_ptr())? as usize)
    }

    /// This will return the device info with given device index, which includes necessary context/queue information
    /// needed to dispatch task to the device. Refer PF_GPUDeviceInfo for details.
    /// * `effect_ref`   - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// Returns the device info will to be filled.
    pub fn device_info(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize) -> Result<ae_sys::PF_GPUDeviceInfo, Error> {
        call_suite_fn_single!(self, GetDeviceInfo -> ae_sys::PF_GPUDeviceInfo, effect_ref.as_ptr(), device_index as _)
    }

    /// Acquire/release exclusive access to `device_index`. All calls below this point generally require access be held.
    /// For full GPU plugins (those that use a separate entry point for GPU rendering) exclusive access is always held.
    /// These calls do not need to be made in that case.
    /// * `effect_ref`   - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    pub fn acquire_exclusive_device_access(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize) -> Result<(), Error> {
        call_suite_fn!(self, AcquireExclusiveDeviceAccess, effect_ref.as_ptr(), device_index as _)
    }

    /// Acquire/release exclusive access to `device_index`. All calls below this point generally require access be held.
    /// For full GPU plugins (those that use a separate entry point for GPU rendering) exclusive access is always held.
    /// These calls do not need to be made in that case.
    /// * `effect_ref`   - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    pub fn release_exclusive_device_access(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseExclusiveDeviceAccess, effect_ref.as_ptr(), device_index as _)
    }

    /// All device memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemAlloc or clCreateBuffer.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `size_bytes` - The size of the memory to allocate.
    /// Returns the pointer to the allocated memory.
    pub fn allocate_device_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, size_bytes: usize) -> Result<*mut c_void, Error> {
        call_suite_fn_single!(self, AllocateDeviceMemory -> *mut c_void, effect_ref.as_ptr(), device_index as _, size_bytes)
    }

    /// Free the device memory.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `memory` - The pointer to the memory to free.
    pub fn free_device_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, memory: *mut c_void) -> Result<(), Error> {
        call_suite_fn!(self, FreeDeviceMemory, effect_ref.as_ptr(), device_index as _, memory)
    }

    /// Purge the device memory.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `size_bytes` - The size of the memory to purge.
    /// Returns the number of bytes purged.
    pub fn purge_device_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, size_bytes: usize) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, PurgeDeviceMemory -> usize, effect_ref.as_ptr(), device_index as _, size_bytes)?)
    }

    /// All host (pinned) memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemHostAlloc or malloc.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `size_bytes` - The size of the memory to allocate.
    /// Returns the pointer to the allocated memory.
    pub fn allocate_host_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, size_bytes: usize) -> Result<*mut c_void, Error> {
        call_suite_fn_single!(self, AllocateHostMemory -> *mut c_void, effect_ref.as_ptr(), device_index as _, size_bytes)
    }

    /// Free the host memory.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `memory` - The pointer to the memory to free.
    pub fn free_host_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, memory: *mut c_void) -> Result<(), Error> {
        call_suite_fn!(self, FreeHostMemory, effect_ref.as_ptr(), device_index as _, memory)
    }

    /// Purge the host memory.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device index for the requested device.
    /// * `bytes_to_purge` - The size of the memory to purge.
    /// Returns the number of bytes purged.
    pub fn purge_host_memory(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, bytes_to_purge: usize) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, PurgeHostMemory -> usize, effect_ref.as_ptr(), device_index as _, bytes_to_purge)?)
    }

    /// This will allocate a gpu effect world. Caller is responsible for deallocating the buffer with [`dispose_gpu_world()`](Self::dispose_gpu_world).
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `device_index` - The device you want your gpu effect world allocated with.
    /// * `width` - Width of the effect world.
    /// * `height` - Height of the effect world.
    /// * `pixel_aspect_ratio` - Pixel Aspect Ratio of the effect world.
    /// * `field_type` - The field of the effect world.
    /// * `pixel_format` - The pixel format of the effect world, only gpu formats are accepted.
    /// * `clear_pix` - Pass in 'true' for a transparent black frame.
    /// Returns the handle to the effect world to be created.
    pub fn create_gpu_world(&self, effect_ref: impl AsPtr<PF_ProgPtr>, device_index: usize, width: i32, height: i32, pixel_aspect_ratio: RationalScale, field_type: Field, pixel_format: pf::PixelFormat, clear_pix: bool) -> Result<EffectWorld, Error> {
        EffectWorld::from_raw(
            call_suite_fn_single!(self, CreateGPUWorld -> *mut PF_EffectWorld, effect_ref.as_ptr(), device_index as _, width, height, pixel_aspect_ratio.into(), field_type.into(), pixel_format.into(), clear_pix as _)?
        )
    }

    /// This will free this effect world. The effect world is no longer valid after this function is called.
    /// Plugin module is only allowed to dispose of gpu effect worlds they create.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `world` - The effect world you want to dispose.
    pub fn dispose_gpu_world(&self, effect_ref: impl AsPtr<PF_ProgPtr>, world: EffectWorld) -> Result<(), Error> {
        call_suite_fn!(self, DisposeGPUWorld, effect_ref.as_ptr(), world.as_ptr() as *mut _)
    }

    /// This will return the gpu buffer address of the given effect world.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `world` - The effect world you want to operate on, has to be a gpu effect world.
    /// Returns the gpu buffer address.
    pub fn gpu_world_data(&self, effect_ref: impl AsPtr<PF_ProgPtr>, world: &EffectWorld) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, GetGPUWorldData -> *mut c_void, effect_ref.as_ptr(), world.as_ptr() as *mut _)
    }

    /// This will return the size of the total data in the effect world.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `world` - The effect world you want to operate on, has to be a gpu effect world.
    /// Returns the size of the total data in the effect world.
    pub fn gpu_world_size(&self, effect_ref: impl AsPtr<PF_ProgPtr>, world: &EffectWorld) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, GetGPUWorldSize -> usize, effect_ref.as_ptr(), world.as_ptr() as *mut _)?)
    }

    /// This will return device index the gpu effect world is associated with.
    /// * `effect_ref` - Effect reference from [`InData`](crate::InData::effect_ref).
    /// * `world` - The effect world you want to operate on, has to be a gpu effect world.
    /// Returns the device index of the given effect world.
    pub fn gpu_world_device_index(&self, effect_ref: impl AsPtr<PF_ProgPtr>, world: &EffectWorld) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, GetGPUWorldDeviceIndex -> A_u_long, effect_ref.as_ptr(), world.as_ptr() as *mut _)? as usize)
    }
}
