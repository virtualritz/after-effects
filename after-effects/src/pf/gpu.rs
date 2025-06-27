use super::*;
use std::any::Any;

define_enum! {
    ae_sys::PF_GPU_Framework,
    GpuFramework {
        None    = ae_sys::PF_GPU_Framework_NONE,
        OpenCl  = ae_sys::PF_GPU_Framework_OPENCL,
        Metal   = ae_sys::PF_GPU_Framework_METAL,
        Cuda    = ae_sys::PF_GPU_Framework_CUDA,
        DirectX = ae_sys::PF_GPU_Framework_DIRECTX,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GpuDeviceSetupExtra {
    pub(crate) ptr: *mut ae_sys::PF_GPUDeviceSetupExtra,
}
impl AsRef<ae_sys::PF_GPUDeviceSetupExtra> for GpuDeviceSetupExtra {
    fn as_ref(&self) -> &ae_sys::PF_GPUDeviceSetupExtra {
        unsafe { &*self.ptr }
    }
}
impl GpuDeviceSetupExtra {
    pub fn from_raw(ptr: *mut ae_sys::PF_GPUDeviceSetupExtra) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
    pub fn as_ptr(&self) -> *mut ae_sys::PF_GPUDeviceSetupExtra {
        self.ptr
    }
    pub fn what_gpu(&self) -> GpuFramework {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn set_gpu_data<T: Any>(&mut self, val: T) {
        assert!(!self.as_ref().output.is_null());
        let boxed: Box<Box<dyn Any>> = Box::new(Box::new(val));
        unsafe {
            (*self.as_ref().output).gpu_data = Box::<Box<dyn Any>>::into_raw(boxed) as *mut _;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GpuDeviceSetdownExtra {
    pub(crate) ptr: *mut ae_sys::PF_GPUDeviceSetdownExtra,
}
impl AsRef<ae_sys::PF_GPUDeviceSetdownExtra> for GpuDeviceSetdownExtra {
    fn as_ref(&self) -> &ae_sys::PF_GPUDeviceSetdownExtra {
        unsafe { &*self.ptr }
    }
}
impl GpuDeviceSetdownExtra {
    pub fn from_raw(ptr: *mut ae_sys::PF_GPUDeviceSetdownExtra) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }
    pub fn as_ptr(&self) -> *mut ae_sys::PF_GPUDeviceSetdownExtra {
        self.ptr
    }
    pub fn what_gpu(&self) -> GpuFramework {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn gpu_data_mut<'a, T: Any>(&'a mut self) -> &'a mut T {
        assert!(!self.as_ref().input.is_null());
        let data =
            unsafe { Box::<Box<dyn Any>>::from_raw((*(*self.ptr).input).gpu_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_mut::<T>() {
            Some(data) => data,
            None => panic!("Invalid type for gpu_data"),
        }
    }
    pub fn gpu_data<'a, T: Any>(&'a self) -> &'a T {
        assert!(!self.as_ref().input.is_null());
        let data =
            unsafe { Box::<Box<dyn Any>>::from_raw((*(*self.ptr).input).gpu_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_ref::<T>() {
            Some(data) => data,
            None => panic!("Invalid type for gpu_data"),
        }
    }
    pub fn destroy_gpu_data<T: Any>(&mut self) {
        assert!(!self.as_ref().input.is_null());
        unsafe {
            let data = Box::<Box<dyn Any>>::from_raw((*(*self.ptr).input).gpu_data as *mut _);
            match data.downcast::<T>() {
                Ok(_) => {
                    (*(*self.ptr).input).gpu_data = std::ptr::null_mut();
                    // data will be dropped here
                }
                Err(e) => panic!("Invalid type for gpu_data: {e:?}"),
            }
        }
    }
}