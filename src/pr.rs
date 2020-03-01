use aftereffects_sys as ae_sys;

#[derive(Copy, Clone, Debug, Hash)]
pub struct InDataHandle {
    in_data_ptr: *const ae_sys::PR_InData,
}

impl InDataHandle {
    pub fn from_raw(
        in_data_ptr: *const ae_sys::PR_InData,
    ) -> InDataHandle {
        InDataHandle { in_data_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PR_InData {
        self.in_data_ptr
    }

    pub fn pica_basic_handle(&self) -> crate::PicaBasicSuiteHandle {
        crate::PicaBasicSuiteHandle::from_raw(unsafe {
            (*self.in_data_ptr).pica_basicP
        })
    }

    pub fn plugin_id(&self) -> i32 {
        unsafe { (*self.in_data_ptr).aegp_plug_id }
    }

    // Fixme: do we own this memory???!
    pub fn reference_context_ptr(&self) -> Box<std::os::raw::c_void> {
        unsafe {
            Box::<std::os::raw::c_void>::from_raw(
                (*self.in_data_ptr).aegp_refconPV,
            )
        }
    }
}

define_handle_wrapper!(
    RenderContextHandle,
    PR_RenderContextH,
    render_context_ptr
);

define_handle_wrapper!(
    InstanceData,
    PR_InstanceDataH,
    instance_data_pt
);

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
