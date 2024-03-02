use crate::*;

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

    /// Obtain [`Drawbot`](drawbot::Drawbot) for the provided context handle.
    pub fn drawing_reference(&self, context_handle: impl AsPtr<ae_sys::PF_ContextH>) -> Result<drawbot::Drawbot, Error> {
        Ok(drawbot::Drawbot {
            suite: crate::Suite::new()?,
            theme_suite: crate::Suite::new(),
            handle: call_suite_fn_single!(self, PF_GetDrawingReference -> ae_sys::DRAWBOT_DrawRef, context_handle.as_ptr())?
        })
    }

    // fn PF_GetContextAsyncManager(in_data: *mut PF_InData, extra: *mut PF_EventExtra, managerPP0: *mut PF_AsyncManagerP) -> PF_Err,
}

define_suite!(
    /// This suite should be used for stroking and filling paths and vertices on the Composition and Layer Windows.
    ///
    /// After Effects is using this suite internally, and we have made it available to make custom UI look consistent across effects.
    ///
    /// The foreground/shadow colors are computed based on the app brightness level so that custom UI is always visible regardless of the application's Brightness setting in the Preferences.
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

    /// Get the preferred foreground color.
    pub fn preferred_foreground_color(&self) -> Result<drawbot::ColorRgba, Error> {
        call_suite_fn_single!(self, PF_GetPreferredForegroundColor -> drawbot::ColorRgba)
    }

    /// Get the preferred shadow color.
    pub fn preferred_shadow_color(&self) -> Result<drawbot::ColorRgba, Error> {
        call_suite_fn_single!(self, PF_GetPreferredShadowColor -> drawbot::ColorRgba)
    }

    /// Get the preferred foreground & shadow stroke width.
    pub fn preferred_stroke_width(&self) -> Result<f32, Error> {
        call_suite_fn_single!(self, PF_GetPreferredStrokeWidth -> f32)
    }

    /// Get the preferred vertex size.
    pub fn preferred_vertex_size(&self) -> Result<f32, Error> {
        call_suite_fn_single!(self, PF_GetPreferredVertexSize -> f32)
    }

    /// Get the preferred shadow offset.
    pub fn preferred_shadow_offset(&self) -> Result<ae_sys::A_LPoint, Error> {
        Ok(call_suite_fn_single!(self, PF_GetPreferredShadowOffset -> ae_sys::A_LPoint)?.into())
    }

    /// Stoke the path with the overlay theme foreground color.
    ///
    /// Optionally draw the shadow using the overlay theme shadow color.
    ///
    /// Uses overlay theme stroke width for stroking foreground and shadow strokes.
    pub fn stroke_path(&self, drawbot: impl AsPtr<ae_sys::DRAWBOT_DrawRef>, path: impl AsPtr<ae_sys::DRAWBOT_PathRef>, draw_shadow: bool) -> Result<(), Error> {
        call_suite_fn!(self, PF_StrokePath, drawbot.as_ptr(), path.as_ptr(), draw_shadow as _)
    }

    /// Fills the path with overlay theme foreground color.
    ///
    /// Optionally draw the shadow using the overlay theme shadow color.
    pub fn fill_path(&self, drawbot: impl AsPtr<ae_sys::DRAWBOT_DrawRef>, path: impl AsPtr<ae_sys::DRAWBOT_PathRef>, draw_shadow: bool) -> Result<(), Error> {
        call_suite_fn!(self, PF_FillPath, drawbot.as_ptr(), path.as_ptr(), draw_shadow as _)
    }

    /// Fills a square vertex around the center point using the overlay theme foreground color and vertex size.
    pub fn fill_vertex(&self, drawbot: impl AsPtr<ae_sys::DRAWBOT_DrawRef>, center_point: FloatPoint, draw_shadow: bool) -> Result<(), Error> {
        call_suite_fn!(self, PF_FillVertex, drawbot.as_ptr(), &center_point.into(), draw_shadow as _)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(PF_ContextH);
define_handle_wrapper!(ContextHandle, PF_ContextH);

impl ContextHandle {
    pub fn drawing_reference(&self) -> Result<drawbot::Drawbot, Error> {
        let suite = EffectCustomUISuite::new()?;
        suite.drawing_reference(self.0)
    }

    pub fn window_type(&self) -> WindowType {
        assert!(!self.as_ptr().is_null());
        let inner = unsafe { *self.as_ptr() };
        assert!(!inner.is_null());
        unsafe { (*inner).w_type }.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CustomUIInfo(ae_sys::PF_CustomUIInfo);

impl CustomUIInfo {
    pub fn new() -> Self {
        Self(unsafe { std::mem::zeroed() })
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
