use super::*;
use std::any::Any;

/*
pub const PF_RenderOutputFlag_RETURNS_EXTRA_PIXELS: _bindgen_ty_30 = 1;
pub const PF_RenderOutputFlag_GPU_RENDER_POSSIBLE: _bindgen_ty_30 = 2;
pub struct PF_RenderRequest {
    pub rect: PF_LRect,
    pub field: PF_Field,
    pub channel_mask: PF_ChannelMask,
    pub preserve_rgb_of_zero_alpha: PF_Boolean,
    pub unused: [::std::os::raw::c_char; 3usize],
    pub reserved: [A_long; 4usize],
}
pub struct PF_PreRenderInput {
    pub output_request: PF_RenderRequest,
    pub bitdepth: ::std::os::raw::c_short,
    pub gpu_data: *const ::std::os::raw::c_void,
    pub what_gpu: PF_GPU_Framework,
    pub device_index: A_u_long,
}
pub struct PF_PreRenderOutput {
    pub result_rect: PF_LRect,
    pub max_result_rect: PF_LRect,
    pub solid: PF_Boolean,
    pub reserved: PF_Boolean,
    pub flags: PF_RenderOutputFlags,
    pub pre_render_data: *mut ::std::os::raw::c_void,
    pub delete_pre_render_data_func: PF_DeletePreRenderDataFunc,
}
pub input: *mut PF_PreRenderInput,
pub output: *mut PF_PreRenderOutput,
pub cb: *mut PF_PreRenderCallbacks,
*/

#[derive(Clone, Copy, Debug)]
pub struct PreRenderExtra {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
    pub(crate) ptr: *mut ae_sys::PF_PreRenderExtra,
}
impl AsRef<ae_sys::PF_PreRenderExtra> for PreRenderExtra {
    fn as_ref(&self) -> &ae_sys::PF_PreRenderExtra {
        unsafe { &*self.ptr }
    }
}
impl AsMut<ae_sys::PF_PreRenderExtra> for PreRenderExtra {
    fn as_mut(&mut self) -> &mut ae_sys::PF_PreRenderExtra {
        unsafe { &mut *self.ptr }
    }
}
impl PreRenderExtra {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData, ptr: *mut ae_sys::PF_PreRenderExtra) -> Self {
        assert!(!in_data_ptr.is_null());
        assert!(!ptr.is_null());
        Self { in_data_ptr, ptr }
    }
    pub fn as_ptr(&self) -> *mut ae_sys::PF_PreRenderExtra {
        self.ptr
    }
    pub fn what_gpu(&self) -> GpuFramework {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn bit_depth(&self) -> i16 {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).bitdepth as i16 }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn set_pre_render_data<T: Any>(&mut self, val: T) {
        let boxed: Box<Box<dyn Any>> = Box::new(Box::new(val));
        unsafe {
            (*self.as_mut().output).pre_render_data =
                Box::<Box<dyn Any>>::into_raw(boxed) as *mut _;
        }
        unsafe {
            (*self.as_mut().output).delete_pre_render_data_func = Some(delete_pre_render_data);
        }
    }
    pub fn callbacks(&self) -> PreRenderCallbacks {
        unsafe { PreRenderCallbacks::from_raw(self.in_data_ptr, (*self.ptr).cb) }
    }

    pub fn output_request(&self) -> ae_sys::PF_RenderRequest {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).output_request }
    }
    pub fn set_gpu_render_possible(&mut self, val: bool) {
        assert!(!self.as_mut().output.is_null());
        unsafe {
            if val {
                (*self.as_mut().output).flags |= ae_sys::PF_RenderOutputFlag_GPU_RENDER_POSSIBLE as i16;
            } else {
                (*self.as_mut().output).flags &= !ae_sys::PF_RenderOutputFlag_GPU_RENDER_POSSIBLE as i16;
            }
        }
    }
    pub fn set_returns_extra_pixels(&mut self, val: bool) {
        assert!(!self.as_mut().output.is_null());
        unsafe {
            if val {
                (*self.as_mut().output).flags |=
                    ae_sys::PF_RenderOutputFlag_RETURNS_EXTRA_PIXELS as i16;
            } else {
                (*self.as_mut().output).flags &=
                    !ae_sys::PF_RenderOutputFlag_RETURNS_EXTRA_PIXELS as i16;
            }
        }
    }

    pub fn result_rect(&self) -> Rect {
        assert!(!self.as_ref().output.is_null());
        unsafe { (*self.as_ref().output).result_rect.into() }
    }
    pub fn max_result_rect(&self) -> Rect {
        assert!(!self.as_ref().output.is_null());
        unsafe { (*self.as_ref().output).max_result_rect.into() }
    }
    pub fn set_result_rect(&mut self, rect: Rect) {
        assert!(!self.as_mut().output.is_null());
        unsafe {
            (*self.as_mut().output).result_rect = rect.into();
        }
    }
    pub fn set_max_result_rect(&mut self, rect: Rect) {
        assert!(!self.as_mut().output.is_null());
        unsafe {
            (*self.as_mut().output).max_result_rect = rect.into();
        }
    }
    pub fn union_result_rect(&mut self, rect: Rect) -> Rect {
        let rect = *self.result_rect().union(&rect);
        self.set_result_rect(rect);
        rect
    }
    pub fn union_max_result_rect(&mut self, rect: Rect) -> Rect {
        let rect = *self.max_result_rect().union(&rect);
        self.set_max_result_rect(rect);
        rect
    }
}
unsafe extern "C" fn delete_pre_render_data(data: *mut std::ffi::c_void) {
    if !data.is_null() {
        let _ = Box::<Box<dyn Any>>::from_raw(data as *mut _);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SmartRenderExtra {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
    pub(crate) ptr: *mut ae_sys::PF_SmartRenderExtra,
}
impl AsRef<ae_sys::PF_SmartRenderExtra> for SmartRenderExtra {
    fn as_ref(&self) -> &ae_sys::PF_SmartRenderExtra {
        unsafe { &*self.ptr }
    }
}
impl SmartRenderExtra {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData, ptr: *mut ae_sys::PF_SmartRenderExtra) -> Self {
        assert!(!ptr.is_null());
        Self { in_data_ptr, ptr }
    }
    pub fn as_ptr(&self) -> *mut ae_sys::PF_SmartRenderExtra {
        self.ptr
    }
    pub fn callbacks(&self) -> SmartRenderCallbacks {
        unsafe { SmartRenderCallbacks::from_raw(self.in_data_ptr, (*self.ptr).cb) }
    }
    pub fn what_gpu(&self) -> GpuFramework {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).what_gpu.into() }
    }
    pub fn device_index(&self) -> usize {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).device_index as usize }
    }
    pub fn bit_depth(&self) -> i16 {
        assert!(!self.as_ref().input.is_null());
        unsafe { (*self.as_ref().input).bitdepth }
    }
    pub fn gpu_data<T: Any>(&self) -> Option<&T> {
        assert!(!self.as_ref().input.is_null());
        if unsafe { (*(*self.ptr).input).gpu_data.is_null() } {
            return None;
        }
        let data =
            unsafe { Box::<Box<dyn Any>>::from_raw((*(*self.ptr).input).gpu_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_ref::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for gpu_data"),
        }
    }
    pub fn pre_render_data< T: Any>(&self) -> Option<&T> {
        assert!(!self.as_ref().input.is_null());
        if unsafe { (*(*self.ptr).input).pre_render_data.is_null() } {
            return None;
        }
        let data = unsafe {
            Box::<Box<dyn Any>>::from_raw((*(*self.ptr).input).pre_render_data as *mut _)
        };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_ref::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for pre_render_data"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PreRenderCallbacks {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
    pub(crate) rc_ptr: *const ae_sys::PF_PreRenderCallbacks,
}

impl PreRenderCallbacks {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData, rc_ptr: *const ae_sys::PF_PreRenderCallbacks) -> Self {
        Self { in_data_ptr, rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_PreRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer(
        &self,
        index: i32,
        checkout_id: i32,
        // FIXME: warp this struct
        req: &ae_sys::PF_RenderRequest,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> Result<ae_sys::PF_CheckoutResult, Error> {
        if let Some(checkout_layer) = unsafe { *self.rc_ptr }.checkout_layer {
            let mut checkout_result = std::mem::MaybeUninit::<ae_sys::PF_CheckoutResult>::uninit();

            match unsafe {
                checkout_layer(
                    (*self.in_data_ptr).effect_ref,
                    index,
                    checkout_id,
                    req,
                    what_time,
                    time_step,
                    time_scale,
                    checkout_result.as_mut_ptr(),
                )
            } {
                0 => Ok(unsafe { checkout_result.assume_init() }),
                e => Err(Error::from(e)),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SmartRenderCallbacks {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
    pub(crate) rc_ptr: *const ae_sys::PF_SmartRenderCallbacks,
}

impl SmartRenderCallbacks {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData, rc_ptr: *const ae_sys::PF_SmartRenderCallbacks) -> Self {
        Self { in_data_ptr, rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_SmartRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer_pixels(&self, checkout_id: u32) -> Result<Layer, Error> {
        if let Some(checkout_layer_pixels) = unsafe { *self.rc_ptr }.checkout_layer_pixels {
            let mut effect_world_ptr = std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe {
                checkout_layer_pixels(
                    (*self.in_data_ptr).effect_ref,
                    checkout_id as i32,
                    effect_world_ptr.as_mut_ptr(),
                )
            } {
                0 => Ok(Layer::from_raw(unsafe { effect_world_ptr.assume_init() }, self.in_data_ptr, None)),
                e => Err(Error::from(e)),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }

    pub fn checkin_layer_pixels(&self, checkout_id: u32) -> Result<(), Error> {
        if let Some(checkin_layer_pixels) = unsafe { *self.rc_ptr }.checkin_layer_pixels {
            match unsafe { checkin_layer_pixels((*self.in_data_ptr).effect_ref, checkout_id as i32) } {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }

    pub fn checkout_output(&self) -> Result<Layer, Error> {
        if let Some(checkout_output) = unsafe { *self.rc_ptr }.checkout_output {
            let mut effect_world_ptr = std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe { checkout_output((*self.in_data_ptr).effect_ref, effect_world_ptr.as_mut_ptr()) } {
                0 => Ok(Layer::from_raw(unsafe { effect_world_ptr.assume_init() }, self.in_data_ptr, None)),
                e => Err(Error::from(e)),
            }
        } else {
            Err(Error::InvalidCallback)
        }
    }
}
