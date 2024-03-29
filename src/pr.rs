use crate::ae_sys;

#[derive(Copy, Clone, Debug, Hash)]
pub struct InDataHandle {
    in_data_ptr: *const ae_sys::PR_InData,
}

impl InDataHandle {
    #[inline]
    pub fn from_raw(in_data_ptr: *const ae_sys::PR_InData) -> InDataHandle {
        InDataHandle { in_data_ptr }
    }

    #[inline]
    pub fn as_ptr(self) -> *const ae_sys::PR_InData {
        self.in_data_ptr
    }

    #[inline]
    pub fn pica_basic_handle(self) -> crate::PicaBasicSuiteHandle {
        crate::PicaBasicSuiteHandle::from_raw(unsafe { (*self.in_data_ptr).pica_basicP })
    }

    #[inline]
    pub fn plugin_id(self) -> i32 {
        unsafe { (*self.in_data_ptr).aegp_plug_id }
    }

    // Fixme: do we own this memory???!
    #[inline]
    pub fn reference_context_ptr(self) -> Box<std::os::raw::c_void> {
        unsafe { Box::<std::os::raw::c_void>::from_raw((*self.in_data_ptr).aegp_refconPV) }
    }
}

define_handle_wrapper!(RenderContextHandle, PR_RenderContextH);
define_handle_wrapper!(InstanceDataHandle, PR_InstanceDataH);
define_handle_wrapper!(InstanceContextHandle, PR_InstanceContextH);
define_handle_wrapper!(GlobalContextHandle, PR_GlobalContextH);
define_handle_wrapper!(GlobalDataHandle, PR_GlobalDataH);
define_handle_wrapper!(RenderDataHandle, PR_RenderDataH);

//EffectWorld
/*
// FIXME: wrap this nicely
#[derive(Copy, Clone, Debug, Hash)]
pub struct RenderContextHandle {
    pub render_context_ptr: ae_sys::PR_RenderContextH,
}

impl RenderContextHandle {
    fn as_ptr(&self): ae_sys::PR_RenderContextH {
        render_context_ptr
    }
}*/
