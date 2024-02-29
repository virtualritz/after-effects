use crate::*;
use crate::drawbot::*;
use ae_sys::DRAWBOT_SurfaceRef;

define_suite!(
    /// Calls to draw on the surface, and to query and set drawing settings.
    SurfaceSuite,
    DRAWBOT_SurfaceSuite1,
    kDRAWBOT_SurfaceSuite,
    kDRAWBOT_SurfaceSuite_Version1
);

impl SurfaceSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Push the current surface state onto the stack. It should be popped to retrieve old state.
    ///
    /// It is required to restore state if you are going to clip or transform a surface or change the interpolation or anti-aliasing policy.
    pub fn push_state_stack(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<(), Error> {
        call_suite_fn!(self, PushStateStack, surface_ref.as_ptr(),)
    }

    /// Pop the last pushed surface state off the stack.
    pub fn pop_state_stack(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<(), Error> {
        call_suite_fn!(self, PopStateStack, surface_ref.as_ptr(),)
    }

    /// Paint a rectangle with a color on the surface.
    pub fn paint_rect(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, color: &ColorRgba, rect: &RectF32) -> Result<(), Error> {
        call_suite_fn!(self, PaintRect, surface_ref.as_ptr(), color, rect)
    }

    /// Fill a path using a brush and fill type.
    pub fn fill_path(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, brush: &Brush, path: &Path, fill_type: FillType) -> Result<(), Error> {
        call_suite_fn!(self, FillPath, surface_ref.as_ptr(), brush.handle, path.handle, fill_type as _)
    }

    /// Stroke a path using a pen.
    pub fn stroke_path(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, pen: &Pen, path: &Path) -> Result<(), Error> {
        call_suite_fn!(self, StrokePath, surface_ref.as_ptr(), pen.handle, path.handle)
    }

    /// Clip the surface.
    pub fn clip(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, supplier: &Supplier, rect: &Rect32) -> Result<(), Error> {
        call_suite_fn!(self, Clip, surface_ref.as_ptr(), supplier.as_ptr(), rect)
    }

    /// Get clip bounds.
    pub fn clip_bounds(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<Rect32, Error> {
        call_suite_fn_single!(self, GetClipBounds -> ae_sys::DRAWBOT_Rect32, surface_ref.as_ptr())
    }

    /// Checks whether a rect is within the clip bounds.
    pub fn is_within_clip_bounds(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, rect: &Rect32) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, IsWithinClipBounds -> ae_sys::DRAWBOT_Boolean, surface_ref.as_ptr(), rect)? != 0)
    }

    /// Transform the last surface state.
    pub fn transform(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, matrix: &MatrixF32) -> Result<(), Error> {
        call_suite_fn!(self, Transform, surface_ref.as_ptr(), matrix as _,)
    }

    /// Draw a string.
    pub fn draw_string(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, brush: &Brush, font: &Font, string: &str, origin: &PointF32, alignment_style: TextAlignment, truncation_style: TextTruncation, truncation_width: f32) -> Result<(), Error> {
        let string = widestring::U16CString::from_str(string).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(self, DrawString, surface_ref.as_ptr(), brush.handle, font.handle, string.as_ptr(), origin, alignment_style.into(), truncation_style.into(), truncation_width)
    }

    /// Draw an image created using [`new_image_from_buffer()`](super::Supplier::new_image_from_buffer) on the surface. Alpha = [0.0, 1.0].
    pub fn draw_image(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, image: &Image, origin: &PointF32, alpha: f32) -> Result<(), Error> {
        call_suite_fn!(self, DrawImage, surface_ref.as_ptr(), image.handle, origin, alpha)
    }

    /// Set the interpolation policy.
    pub fn set_interpolation_policy(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, interp: InterpolationPolicy) -> Result<(), Error> {
        call_suite_fn!(self, SetInterpolationPolicy, surface_ref.as_ptr(), interp.into())
    }

    /// Get the interpolation policy.
    pub fn interpolation_policy(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<InterpolationPolicy, Error> {
        Ok(call_suite_fn_single!(self, GetInterpolationPolicy -> ae_sys::DRAWBOT_InterpolationPolicy, surface_ref.as_ptr())?.into())
    }

    /// Set the anti-alias policy.
    pub fn set_anti_alias_policy(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>, policy: AntiAliasPolicy) -> Result<(), Error> {
        call_suite_fn!(self, SetAntiAliasPolicy, surface_ref.as_ptr(), policy.into())
    }

    /// Get the anti-alias policy.
    pub fn anti_alias_policy(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<AntiAliasPolicy, Error> {
        Ok(call_suite_fn_single!(self, GetAntiAliasPolicy -> ae_sys::DRAWBOT_AntiAliasPolicy, surface_ref.as_ptr())?.into())
    }

    /// Flush drawing. This is not always needed, and if overused, may cause excessive redrawing and flashing.
    pub fn flush(&self, surface_ref: impl AsPtr<DRAWBOT_SurfaceRef>) -> Result<(), Error> {
        call_suite_fn!(self, Flush, surface_ref.as_ptr(),)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::DRAWBOT_FillType,
    FillType {
        EvenOdd = ae_sys::kDRAWBOT_FillType_EvenOdd,
        Winding = ae_sys::kDRAWBOT_FillType_Winding,
    }
}

define_enum! {
    ae_sys::DRAWBOT_TextAlignment,
    TextAlignment {
        Left   = ae_sys::kDRAWBOT_TextAlignment_Left,
        Center = ae_sys::kDRAWBOT_TextAlignment_Center,
        Right  = ae_sys::kDRAWBOT_TextAlignment_Right,
    }
}

define_enum! {
    ae_sys::DRAWBOT_TextTruncation,
    TextTruncation {
        None         = ae_sys::kDRAWBOT_TextTruncation_None,
        End          = ae_sys::kDRAWBOT_TextTruncation_End,
        EndEllipsis  = ae_sys::kDRAWBOT_TextTruncation_EndEllipsis,
        PathEllipsis = ae_sys::kDRAWBOT_TextTruncation_PathEllipsis,
    }
}

define_enum! {
    ae_sys::DRAWBOT_InterpolationPolicy,
    InterpolationPolicy {
        None    = ae_sys::kDRAWBOT_InterpolationPolicy_None,
        Med     = ae_sys::kDRAWBOT_InterpolationPolicy_Med,
        High    = ae_sys::kDRAWBOT_InterpolationPolicy_High,
    }
}

define_enum! {
    ae_sys::DRAWBOT_AntiAliasPolicy,
    AntiAliasPolicy {
        None    = ae_sys::kDRAWBOT_AntiAliasPolicy_None,
        Med     = ae_sys::kDRAWBOT_AntiAliasPolicy_Med,
        High    = ae_sys::kDRAWBOT_AntiAliasPolicy_High,
    }
}

define_suite_item_wrapper!(
    ae_sys::DRAWBOT_SurfaceRef, SurfaceRef,
    suite: SurfaceSuite,
    /// Calls to draw on the surface, and to query and set drawing settings.
    Surface {
        dispose: ;

        /// Push the current surface state onto the stack. It should be popped to retrieve old state.
        ///
        /// It is required to restore state if you are going to clip or transform a surface or change the interpolation or anti-aliasing policy.
        push_state_stack() -> () => suite.push_state_stack,

        /// Pop the last pushed surface state off the stack.
        pop_state_stack() -> () => suite.pop_state_stack,

        /// Paint a rectangle with a color on the surface.
        paint_rect(color: &ColorRgba, rect: &RectF32) -> () => suite.paint_rect,

        /// Fill a path using a brush and fill type.
        fill_path(brush: &Brush, path: &Path, fill_type: FillType) -> () => suite.fill_path,

        /// Stroke a path using a pen.
        stroke_path(pen: &Pen, path: &Path) -> () => suite.stroke_path,

        /// Clip the surface.
        clip(supplier: &Supplier, rect: &Rect32) -> () => suite.clip,

        /// Get clip bounds.
        clip_bounds() -> Rect32 => suite.clip_bounds,

        /// Checks whether a rect is within the clip bounds.
        is_within_clip_bounds(rect: &Rect32) -> bool => suite.is_within_clip_bounds,

        /// Transform the last surface state.
        transform(matrix: &MatrixF32) -> () => suite.transform,

        /// Draw a string.
        draw_string(brush: &Brush, font: &Font, string: &str, origin: &PointF32, alignment_style: TextAlignment, truncation_style: TextTruncation, truncation_width: f32) -> () => suite.draw_string,

        /// Draw an image created using [`new_image_from_buffer()`](super::Supplier::new_image_from_buffer) on the surface. Alpha = [0.0, 1.0].
        draw_image(image: &Image, origin: &PointF32, alpha: f32) -> () => suite.draw_image,

        /// Set the interpolation policy.
        set_interpolation_policy(interp: InterpolationPolicy) -> () => suite.set_interpolation_policy,

        /// Get the interpolation policy.
        interpolation_policy() -> InterpolationPolicy => suite.interpolation_policy,

        /// Set the anti-alias policy.
        set_anti_alias_policy(policy: AntiAliasPolicy) -> () => suite.set_anti_alias_policy,

        /// Get the anti-alias policy.
        anti_alias_policy() -> AntiAliasPolicy => suite.anti_alias_policy,

        /// Flush drawing. This is not always needed, and if overused, may cause excessive redrawing and flashing.
        flush() -> () => suite.flush,
    }
);
