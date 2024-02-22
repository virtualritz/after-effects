use crate::*;

define_suite!(
    /// Calls to draw on the surface, and to query and set drawing settings.
    SurfaceSuite,
    DRAWBOT_SurfaceSuite1,
    kDRAWBOT_SurfaceSuite,
    kDRAWBOT_SurfaceSuite_Version1
);

impl SurfaceSuite {
    pub fn paint_rect(
        &self,
        surface_ref: &SurfaceRef,
        color: &ColorRGBA,
        rect: &RectF32,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            PaintRect,
            surface_ref.as_ptr(),
            color as _,
            rect as _,
        )
    }

    pub fn transform(&self, surface_ref: &SurfaceRef, matrix: &MatrixF32) -> Result<(), Error> {
        call_suite_fn!(self, Transform, surface_ref.as_ptr(), matrix as _,)
    }

    pub fn draw_image(
        &self,
        surface_ref: &SurfaceRef,
        image_ref: &ImageRef,
        origin: &PointF32,
        alpha: f32,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            DrawImage,
            surface_ref.as_ptr(),
            image_ref.as_ptr(),
            origin as _,
            alpha
        )
    }

    pub fn flush(&self, surface_ref: &SurfaceRef) -> Result<(), Error> {
        call_suite_fn!(self, Flush, surface_ref.as_ptr(),)
    }
}

// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// |        **Function**        |                                                             **Purpose**                                                              |
// +============================+======================================================================================================================================+
// | ``PushStateStack``         | Push the current surface state onto the stack. It should be popped to retrieve old state.                                            |
// |                            | It is required to restore state if you are going to clip or transform a surface or change the interpolation or anti-aliasing policy. |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``PopStateStack``          | Pop the last pushed surface state off the stack.                                                                                     |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``PaintRect``              | Paint a rectangle with a color on the surface.                                                                                       |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``FillPath``               | Fill a path using a brush and fill type.                                                                                             |
// |                            | ``DRAWBOT_FillType`` is one of the following:                                                                                        |
// |                            |                                                                                                                                      |
// |                            |   - ``kDRAWBOT_FillType_EvenOdd``,                                                                                                   |
// |                            |   - ``kDRAWBOT_FillType_Winding``                                                                                                    |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``StrokePath``             | Stroke a path using a pen.                                                                                                           |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``Clip``                   | Clip the surface.                                                                                                                    |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``GetClipBounds``          | Get clip bounds.                                                                                                                     |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``IsWithinClipBounds``     | Checks whether a rect is within the clip bounds.                                                                                     |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``Transform``              | Transform the last surface state.                                                                                                    |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``DrawString``             | Draw a string.                                                                                                                       |
// |                            |                                                                                                                                      |
// |                            | ``DRAWBOT_TextAlignment`` is one of the following:                                                                                   |
// |                            |                                                                                                                                      |
// |                            |   - ``kDRAWBOT_TextAlignment_Left``,                                                                                                 |
// |                            |   - ``kDRAWBOT_TextAlignment_Center``,                                                                                               |
// |                            |   - ``kDRAWBOT_TextAlignment_Right``                                                                                                 |
// |                            |                                                                                                                                      |
// |                            | ``DRAWBOT_TextTruncation`` is one of the following:                                                                                  |
// |                            |                                                                                                                                      |
// |                            |   - ``kDRAWBOT_TextTruncation_None``,                                                                                                |
// |                            |   - ``kDRAWBOT_TextTruncation_End``,                                                                                                 |
// |                            |   - ``kDRAWBOT_TextTruncation_EndEllipsis``,                                                                                         |
// |                            |   - ``kDRAWBOT_TextTruncation_PathEllipsis``                                                                                         |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``DrawImage``              | Draw an image created using ``NewImageFromBuffer()`` on the surface. Alpha = [0.0f, 1.0f ].                                          |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``SetInterpolationPolicy`` | ::                                                                                                                                   |
// |                            | ``DRAWBOT_InterpolationPolicy`` is one of the following:                                                                             |
// |                            |                                                                                                                                      |
// |                            |   - ``kDRAWBOT_InterpolationPolicy_None``,                                                                                           |
// |                            |   - ``kDRAWBOT_InterpolationPolicy_Med``,                                                                                            |
// |                            |   - ``kDRAWBOT_InterpolationPolicy_High``                                                                                            |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``GetInterpolationPolicy`` | ::                                                                                                                                   |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``SetAntiAliasPolicy``     | ::                                                                                                                                   |
// |                            | ``DRAWBOT_AntiAliasPolicy`` is one of the following:                                                                                 |
// |                            |                                                                                                                                      |
// |                            |   - ``kDRAWBOT_AntiAliasPolicy_None``,                                                                                               |
// |                            |   - ``kDRAWBOT_AntiAliasPolicy_Med``,                                                                                                |
// |                            |   - ``kDRAWBOT_AntiAliasPolicy_High``                                                                                                |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``GetAntiAliasPolicy``     | ::                                                                                                                                   |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// | ``Flush``                  | Flush drawing. This is not always needed, and if overused, may cause excessive redrawing and flashing.                               |
// +----------------------------+--------------------------------------------------------------------------------------------------------------------------------------+
// fn PushStateStack(in_surface_ref: DRAWBOT_SurfaceRef) -> SPErr>,
// fn PopStateStack(in_surface_ref: DRAWBOT_SurfaceRef) -> SPErr>,
// fn PaintRect(in_surface_ref: DRAWBOT_SurfaceRef,in_colorP: *const DRAWBOT_ColorRGBA,in_rectPR: *const DRAWBOT_RectF32) -> SPErr,
// fn FillPath(in_surface_ref: DRAWBOT_SurfaceRef,in_brush_ref: DRAWBOT_BrushRef,in_path_ref: DRAWBOT_PathRef,in_fill_type: DRAWBOT_FillType) -> SPErr,
// fn StrokePath(in_surface_ref: DRAWBOT_SurfaceRef,in_pen_ref: DRAWBOT_PenRef,in_path_ref: DRAWBOT_PathRef) -> SPErr,
// fn Clip(in_surface_ref: DRAWBOT_SurfaceRef,in_supplier_ref: DRAWBOT_SupplierRef,in_rectPR: *const DRAWBOT_Rect32) -> SPErr,
// fn GetClipBounds(in_surface_ref: DRAWBOT_SurfaceRef,out_rectPR: *mut DRAWBOT_Rect32) -> SPErr,
// fn IsWithinClipBounds(in_surface_ref: DRAWBOT_SurfaceRef,in_rectPR: *const DRAWBOT_Rect32,out_withinPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn Transform(in_surface_ref: DRAWBOT_SurfaceRef,in_matrixP: *const DRAWBOT_MatrixF32) -> SPErr,
// fn DrawString(in_surface_ref: DRAWBOT_SurfaceRef,in_brush_ref: DRAWBOT_BrushRef,in_font_ref: DRAWBOT_FontRef,in_stringP: *const DRAWBOT_UTF16Char,in_originP: *const DRAWBOT_PointF32,in_alignment_style: DRAWBOT_TextAlignment,in_truncation_style: DRAWBOT_TextTruncation,in_truncation_width: f32) -> SPErr,
// fn DrawImage(in_surface_ref: DRAWBOT_SurfaceRef,in_image_ref: DRAWBOT_ImageRef,in_originP: *const DRAWBOT_PointF32,in_alpha: f32) -> SPErr,
// fn SetInterpolationPolicy(in_surface_ref: DRAWBOT_SurfaceRef,in_interp: DRAWBOT_InterpolationPolicy) -> SPErr,
// fn GetInterpolationPolicy(in_surface_ref: DRAWBOT_SurfaceRef,out_interpP: *mut DRAWBOT_InterpolationPolicy) -> SPErr,
// fn SetAntiAliasPolicy(in_surface_ref: DRAWBOT_SurfaceRef,in_policy: DRAWBOT_AntiAliasPolicy) -> SPErr,
// fn GetAntiAliasPolicy(in_surface_ref: DRAWBOT_SurfaceRef,out_policyP: *mut DRAWBOT_AntiAliasPolicy) -> SPErr,
// fn Flush(in_surface_ref: DRAWBOT_SurfaceRef) -> SPErr>,
// fn GetTransformToScreenScale(in_surface_ref: DRAWBOT_SurfaceRef, out_scale: *mut f32) -> SPErr,
