use crate::*;

define_suite!(
    /// Calls to create and release drawing tools, get default settings, and query drawing capabilities.
    SupplierSuite,
    DRAWBOT_SupplierSuite1,
    kDRAWBOT_SupplierSuite,
    kDRAWBOT_SupplierSuite_Version1
);

impl SupplierSuite {
    pub fn new_image_from_buffer(
        &self,
        supplier_ref: &SupplierRef,
        width: usize,
        height: usize,
        row_bytes: usize,
        pixel_layout: PixelLayout,
        pixel_data: Vec<u8>,
    ) -> Result<ImageRef, Error> {
        assert!(row_bytes * height <= pixel_data.len());

        let image_ref = call_suite_fn_single!(
            self,
            NewImageFromBuffer -> ae_sys::DRAWBOT_ImageRef,
            supplier_ref.as_ptr(),
            width as _,
            height as _,
            row_bytes as _,
            pixel_layout as _,
            pixel_data.as_ptr() as _
        )?;
        Ok(ImageRef::from_raw(image_ref))
    }
}

// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// |        **Function**         |                                                                                 **Purpose**                                                                                 |
// +=============================+=============================================================================================================================================================================+
// | ``NewPen``                  | Create a new pen. Release this using ``ReleaseObject`` from :ref:`effect-ui-events/custom-ui-and-drawbot.Drawbot_SupplierSuite`.                                            |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``NewBrush``                | Create a new brush. Release this using ``ReleaseObject`` from :ref:`effect-ui-events/custom-ui-and-drawbot.Drawbot_SupplierSuite`.                                          |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``SupportsText``            | Check if current supplier supports text.                                                                                                                                    |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``GetDefaultFontSize``      | Get the default font size.                                                                                                                                                  |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``NewDefaultFont``          | Create a new font with default settings.                                                                                                                                    |
// |                             |                                                                                                                                                                             |
// |                             | You can pass the default font size from ``GetDefaultFontSize``.                                                                                                             |
// |                             |                                                                                                                                                                             |
// |                             | Release this using ``ReleaseObject`` from :ref:`effect-ui-events/custom-ui-and-drawbot.Drawbot_SupplierSuite`.                                                              |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``NewImageFromBuffer``      | Create a new image from buffer passed to in_dataP.                                                                                                                          |
// |                             |                                                                                                                                                                             |
// |                             | Release this using ``ReleaseObject`` from :ref:`effect-ui-events/custom-ui-and-drawbot.Drawbot_SupplierSuite`.                                                              |
// |                             | ``DRAWBOT_PixelLayout`` can be one of the following:                                                                                                                        |
// |                             |                                                                                                                                                                             |
// |                             |   - ``kDRAWBOT_PixelLayout_24RGB``,                                                                                                                                         |
// |                             |   - ``kDRAWBOT_PixelLayout_24BGR``,                                                                                                                                         |
// |                             |   - ``kDRAWBOT_PixelLayout_32RGB``,                                                                                                                                         |
// |                             |   - ``ARGB`` (A is ignored),                                                                                                                                                |
// |                             |   - ``kDRAWBOT_PixelLayout_32BGR``,                                                                                                                                         |
// |                             |   - ``BGRA`` (A is ignored),                                                                                                                                                |
// |                             |   - ``kDRAWBOT_PixelLayout_32ARGB_Straight``,                                                                                                                               |
// |                             |   - ``kDRAWBOT_PixelLayout_32ARGB_Premul``,                                                                                                                                 |
// |                             |   - ``kDRAWBOT_PixelLayout_32BGRA_Straight``,                                                                                                                               |
// |                             |   - ``kDRAWBOT_PixelLayout_32BGRA_Premul``                                                                                                                                  |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``NewPath``                 | Create a new path. Release this using ``ReleaseObject`` from :ref:`effect-ui-events/custom-ui-and-drawbot.Drawbot_SupplierSuite`.                                           |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``SupportsPixelLayoutBGRA`` | A given Drawbot implementation can support multiple channel orders, but will likely prefer one over the other.                                                              |
// |                             | Use the following four callbacks to get the preferred channel order for any API that takes a ``DRAWBOT_PixelLayout`` (e.g. ``NewImageFromBuffer``).                         |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``PrefersPixelLayoutBGRA``  | ::                                                                                                                                                                          |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``SupportsPixelLayoutARGB`` | ::                                                                                                                                                                          |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``PrefersPixelLayoutARGB``  | ::                                                                                                                                                                          |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``RetainObject``            | Retain (increase reference count on) any object (pen, brush, path, etc). For example, it should be used when any object is copied and the copied object should be retained. |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
// | ``ReleaseObject``           | Release (decrease reference count on) any object (pen, brush, path, etc). This function MUST be called for any object created using ``NewXYZ()`` from this suite.           |
// |                             | Do not call this function on a ``DRAWBOT_SupplierRef`` and ``DRAWBOT_SupplierRef``, since these are not created by the plug-in.                                             |
// +-----------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------+

// fn NewPen(in_supplier_ref: DRAWBOT_SupplierRef,in_colorP: *const DRAWBOT_ColorRGBA,in_size: f32,out_penP: *mut DRAWBOT_PenRef) -> SPErr,
// fn NewBrush(in_supplier_ref: DRAWBOT_SupplierRef,in_colorP: *const DRAWBOT_ColorRGBA,out_brushP: *mut DRAWBOT_BrushRef) -> SPErr,
// fn SupportsText(in_supplier_ref: DRAWBOT_SupplierRef,out_supports_textPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn GetDefaultFontSize(in_supplier_ref: DRAWBOT_SupplierRef,out_font_sizeF: *mut f32) -> SPErr,
// fn NewDefaultFont(in_supplier_ref: DRAWBOT_SupplierRef,in_font_sizeF: f32,out_fontP: *mut DRAWBOT_FontRef) -> SPErr,
// fn NewImageFromBuffer(in_supplier_ref: DRAWBOT_SupplierRef,in_width: ::std::os::raw::c_int,in_height: ::std::os::raw::c_int,in_row_bytes: ::std::os::raw::c_int,in_pl: DRAWBOT_PixelLayout,in_dataP: *const ::std::os::raw::c_void,out_imageP: *mut DRAWBOT_ImageRef) -> SPErr,
// fn NewPath(in_supplier_ref: DRAWBOT_SupplierRef,out_pathP: *mut DRAWBOT_PathRef) -> SPErr,
// fn SupportsPixelLayoutBGRA(in_supplier_ref: DRAWBOT_SupplierRef,out_supports_bgraPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn PrefersPixelLayoutBGRA(in_supplier_ref: DRAWBOT_SupplierRef,out_prefers_bgraPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn SupportsPixelLayoutARGB(in_supplier_ref: DRAWBOT_SupplierRef,out_supports_argbPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn PrefersPixelLayoutARGB(in_supplier_ref: DRAWBOT_SupplierRef,out_prefers_argbPB: *mut DRAWBOT_Boolean) -> SPErr,
// fn RetainObject(in_obj_ref: DRAWBOT_ObjectRef) -> SPErr>,
// fn ReleaseObject(in_obj_ref: DRAWBOT_ObjectRef) -> SPErr>,
