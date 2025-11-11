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
        // SAFETY: Dereferencing raw pointer to create immutable reference.
        // Detailed explanation: (1) self.ptr is validated as non-null in from_raw before storage, (2) the pointer originates from After Effects SDK and remains valid for the lifetime of this wrapper, (3) we only create immutable references preventing mutable aliasing.
        // Would be UB if: the pointer becomes invalid due to the underlying PF_GPUDeviceSetupExtra being freed by After Effects while this wrapper still exists, or if from_raw was bypassed allowing null pointers.
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
        // SAFETY: Dereferencing the input field pointer to access what_gpu enum.
        // Detailed explanation: (1) the input pointer is validated as non-null immediately before this unsafe block, (2) the input pointer is managed by After Effects SDK and valid during GPU device setup callback, (3) what_gpu is a plain C enum that can be safely converted to our GpuFramework enum.
        // Would be UB if: the input pointer became invalid between the null check and dereference (impossible in single-threaded callback context), or if After Effects passed an invalid pointer in the GPUDeviceSetupExtra structure.
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        // SAFETY: Dereferencing the input field pointer to access device_index field.
        // Detailed explanation: (1) the input pointer is validated as non-null immediately before this unsafe block, (2) the input pointer is managed by After Effects SDK and valid during GPU device setup callback, (3) device_index is a primitive integer type safe to read and cast.
        // Would be UB if: the input pointer became invalid between the null check and dereference, or if After Effects passed an invalid pointer in the GPUDeviceSetupExtra structure.
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn set_gpu_data<T: Any>(&mut self, val: T) {
        assert!(!self.as_ref().output.is_null());
        let boxed: Box<Box<dyn Any>> = Box::new(Box::new(val));
        // SAFETY: Dereferencing output pointer and converting Box to raw pointer for FFI storage.
        // Detailed explanation: (1) the output pointer is validated as non-null immediately before this unsafe block, (2) Box::into_raw transfers ownership to C side without dropping, preventing double-free, (3) the double-boxed structure ensures proper trait object layout for later downcasting, (4) the pointer will be reclaimed in gpu_data/gpu_data_mut/destroy_gpu_data methods.
        // Would be UB if: the output pointer became invalid, or if the gpu_data field is accessed after being freed, or if destroy_gpu_data is never called causing a memory leak (not UB but a resource leak).
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
        // SAFETY: Dereferencing raw pointer to create immutable reference.
        // Detailed explanation: (1) self.ptr is validated as non-null in from_raw before storage, (2) the pointer originates from After Effects SDK and remains valid for the lifetime of this wrapper, (3) we only create immutable references preventing mutable aliasing.
        // Would be UB if: the pointer becomes invalid due to the underlying PF_GPUDeviceSetdownExtra being freed by After Effects while this wrapper still exists, or if from_raw was bypassed allowing null pointers.
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
        // SAFETY: Dereferencing the input field pointer to access what_gpu enum.
        // Detailed explanation: (1) the input pointer is validated as non-null immediately before this unsafe block, (2) the input pointer is managed by After Effects SDK and valid during GPU device setdown callback, (3) what_gpu is a plain C enum that can be safely converted to our GpuFramework enum.
        // Would be UB if: the input pointer became invalid between the null check and dereference (impossible in single-threaded callback context), or if After Effects passed an invalid pointer in the GPUDeviceSetdownExtra structure.
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        // SAFETY: Dereferencing the input field pointer to access device_index field.
        // Detailed explanation: (1) the input pointer is validated as non-null immediately before this unsafe block, (2) the input pointer is managed by After Effects SDK and valid during GPU device setdown callback, (3) device_index is a primitive integer type safe to read and cast.
        // Would be UB if: the input pointer became invalid between the null check and dereference, or if After Effects passed an invalid pointer in the GPUDeviceSetdownExtra structure.
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn gpu_data_mut<'a, T: Any>(&'a mut self) -> &'a mut T {
        assert!(!self.as_ref().input.is_null());
        // SAFETY: Reconstructing Box from raw pointer for temporary access without taking ownership.
        // Detailed explanation: (1) the input pointer is validated as non-null before accessing gpu_data field, (2) gpu_data was originally created via Box::into_raw in set_gpu_data maintaining the same memory layout, (3) Box::from_raw temporarily reconstructs ownership for type checking, (4) Box::leak immediately releases ownership back to raw pointer preventing drop, (5) the returned reference lifetime is tied to self preventing use-after-free.
        // Would be UB if: gpu_data pointer is null/invalid/already freed, or if type T doesn't match the original type stored (caught by downcast panic), or if gpu_data was not created by set_gpu_data with matching double-Box structure.
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
        // SAFETY: Reconstructing Box from raw pointer for temporary access without taking ownership.
        // Detailed explanation: (1) the input pointer is validated as non-null before accessing gpu_data field, (2) gpu_data was originally created via Box::into_raw in set_gpu_data maintaining the same memory layout, (3) Box::from_raw temporarily reconstructs ownership for type checking, (4) Box::leak immediately releases ownership back to raw pointer preventing drop, (5) the returned reference lifetime is tied to self preventing use-after-free.
        // Would be UB if: gpu_data pointer is null/invalid/already freed, or if type T doesn't match the original type stored (caught by downcast panic), or if gpu_data was not created by set_gpu_data with matching double-Box structure.
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
        // SAFETY: Reconstructing Box from raw pointer to reclaim ownership and properly drop.
        // Detailed explanation: (1) the input pointer is validated as non-null before accessing gpu_data field, (2) gpu_data was originally created via Box::into_raw in set_gpu_data, (3) Box::from_raw takes ownership allowing proper cleanup, (4) downcast verifies type safety before dropping, (5) gpu_data pointer is nulled out after successful drop preventing double-free, (6) this method must only be called once per gpu_data lifecycle.
        // Would be UB if: gpu_data pointer is null/invalid/already freed, or if this method is called multiple times on the same gpu_data (double-free), or if gpu_data was not created by set_gpu_data with matching structure, or if type T doesn't match original type (caught by downcast panic).
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