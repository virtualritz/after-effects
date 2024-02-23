use crate::*;

define_handle_wrapper!(ContextHandle, PF_ContextH);

define_suite!(
    EffectCustomUISuite,
    PF_EffectCustomUISuite2,
    kPFEffectCustomUISuite,
    kPFEffectCustomUISuiteVersion2
);

impl EffectCustomUISuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn drawing_reference(&self, context_handle: &ContextHandle) -> Result<drawbot::Drawbot, Error> {
        Ok(drawbot::Drawbot {
            suite: crate::Suite::new()?,
            handle: call_suite_fn_single!(self, PF_GetDrawingReference -> ae_sys::DRAWBOT_DrawRef, context_handle.as_ptr())?
        })
    }

    // fn PF_GetContextAsyncManager(in_data: *mut PF_InData, extra: *mut PF_EventExtra, managerPP0: *mut PF_AsyncManagerP) -> PF_Err,
}

define_suite!(
    EffectCustomUIOverlayThemeSuite,
    PF_EffectCustomUIOverlayThemeSuite1,
    kPFEffectCustomUIOverlayThemeSuite,
    kPFEffectCustomUIOverlayThemeSuiteVersion1
);

impl EffectCustomUIOverlayThemeSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn preferred_foreground_color(&self) -> Result<drawbot::ColorRgba, Error> {
        call_suite_fn_single!(self, PF_GetPreferredForegroundColor -> drawbot::ColorRgba)
    }

    pub fn preferred_shadow_color(&self) -> Result<drawbot::ColorRgba, Error> {
        call_suite_fn_single!(self, PF_GetPreferredShadowColor -> drawbot::ColorRgba)
    }

    pub fn preferred_stroke_width(&self) -> Result<f32, Error> {
        call_suite_fn_single!(self, PF_GetPreferredStrokeWidth -> f32)
    }

    pub fn preferred_vertex_size(&self) -> Result<f32, Error> {
        call_suite_fn_single!(self, PF_GetPreferredVertexSize -> f32)
    }

    pub fn preferred_shadow_offset(&self) -> Result<ae_sys::A_LPoint, Error> {
        Ok(call_suite_fn_single!(self, PF_GetPreferredShadowOffset -> ae_sys::A_LPoint)?.into())
    }

    // fn PF_GetPreferredForegroundColor(foreground_colorP: *mut DRAWBOT_ColorRGBA) -> PF_Err,
    // fn PF_GetPreferredShadowColor(shadow_colorP: *mut DRAWBOT_ColorRGBA) -> PF_Err,
    // fn PF_GetPreferredStrokeWidth(stroke_widthPF: *mut f32) -> PF_Err>,
    // fn PF_GetPreferredVertexSize(vertex_sizePF: *mut f32) -> PF_Err>,
    // fn PF_GetPreferredShadowOffset(shadow_offsetP: *mut A_LPoint) -> PF_Err>,

    // fn PF_StrokePath(drawbot_ref: DRAWBOT_DrawRef, path_ref: DRAWBOT_PathRef, draw_shadowB: PF_Boolean) -> PF_Err,
    // fn PF_FillPath(drawbot_ref: DRAWBOT_DrawRef, path_ref: DRAWBOT_PathRef, draw_shadowB: PF_Boolean) -> PF_Err,
    // fn PF_FillVertex(drawbot_ref: DRAWBOT_DrawRef, center_pointP: *const A_FloatPoint, draw_shadowB: PF_Boolean) -> PF_Err,
}

#[derive(Copy, Clone, Debug)]
pub struct CustomUIInfo(ae_sys::PF_CustomUIInfo);

impl CustomUIInfo {
    pub fn new() -> Self {
        Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_CustomUIInfo {
        &self.0
    }

    pub fn events(&mut self, events: CustomEventFlags) -> &mut Self {
        self.0.events = events.bits() as _;
        self
    }

    pub fn comp_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.comp_ui_width = width as _;
        self
    }

    pub fn comp_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.comp_ui_height = height as _;
        self
    }

    pub fn layer_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.layer_ui_width = width as _;
        self
    }

    pub fn layer_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.layer_ui_height = height as _;
        self
    }

    pub fn preview_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.preview_ui_width = width as _;
        self
    }

    pub fn preview_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.preview_ui_height = height as _;
        self
    }

    pub fn finalize(self) -> Self {
        self
    }
}

pub struct InteractCallbacks(InData);

impl InteractCallbacks {
    pub fn new(in_data: InData) -> Self {
        Self(in_data)
    }

    pub fn register_ui(&self, custom_ui_info: CustomUIInfo) -> Result<(), Error> {
        match unsafe {
            (*self.0.as_ptr()).inter.register_ui.unwrap()(
                (*self.0.as_ptr()).effect_ref,
                custom_ui_info.as_ptr() as _,
            )
        } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }
}
