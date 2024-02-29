use crate::*;
use crate::drawbot::*;
use ae_sys::DRAWBOT_SupplierRef;

define_suite!(
    /// Calls to create and release drawing tools, get default settings, and query drawing capabilities.
    SupplierSuite,
    DRAWBOT_SupplierSuite1,
    kDRAWBOT_SupplierSuite,
    kDRAWBOT_SupplierSuite_Version1
);

impl SupplierSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Create a new pen.
    pub fn new_pen(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>, color: &ColorRgba, size: f32) -> Result<Pen, Error> {
        let pen_ref = call_suite_fn_single!(self, NewPen -> ae_sys::DRAWBOT_PenRef, supplier_ref.as_ptr(), color, size)?;
        Ok(Pen {
            handle: pen_ref,
            suite: crate::Suite::new()?,
            supplier_suite: self.clone(),
        })
    }

    /// Create a new brush.
    pub fn new_brush(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>, color: &ColorRgba) -> Result<Brush, Error> {
        let brush_ref = call_suite_fn_single!(self, NewBrush -> ae_sys::DRAWBOT_BrushRef, supplier_ref.as_ptr(), color)?;
        Ok(Brush {
            handle: brush_ref,
            supplier_suite: self.clone(),
        })
    }

    /// Check if current supplier supports text.
    pub fn supports_text(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, SupportsText -> ae_sys::DRAWBOT_Boolean, supplier_ref.as_ptr())? != 0)
    }

    /// Get the default font size.
    pub fn default_font_size(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<f32, Error> {
        Ok(call_suite_fn_single!(self, GetDefaultFontSize -> f32, supplier_ref.as_ptr())?)
    }

    /// Create a new font with default settings.
    pub fn new_default_font(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>, font_size: f32) -> Result<Font, Error> {
        let font_ref = call_suite_fn_single!(self, NewDefaultFont -> ae_sys::DRAWBOT_FontRef, supplier_ref.as_ptr(), font_size)?;
        Ok(Font {
            handle: font_ref,
            supplier_suite: self.clone(),
        })
    }

    /// Create a new image from buffer passed to `pixel_data`.
    pub fn new_image_from_buffer(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>, width: usize, height: usize, row_bytes: usize, pixel_layout: PixelLayout, pixel_data: Vec<u8>) -> Result<Image, Error> {
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
        Ok(Image {
            handle: image_ref,
            suite: crate::Suite::new()?,
            supplier_suite: self.clone(),
        })
    }

    /// Create a new path.
    pub fn new_path(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<Path, Error> {
        let path_ref = call_suite_fn_single!(self, NewPath -> ae_sys::DRAWBOT_PathRef, supplier_ref.as_ptr())?;
        Ok(Path {
            handle: path_ref,
            suite: crate::Suite::new()?,
            supplier_suite: self.clone(),
        })
    }

    /// Check if the supplier supports BGRA pixel layout.
    pub fn supports_pixel_layout_bgra(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, SupportsPixelLayoutBGRA -> ae_sys::DRAWBOT_Boolean, supplier_ref.as_ptr())? != 0)
    }

    /// Check if the supplier prefers BGRA pixel layout.
    pub fn prefers_pixel_layout_bgra(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PrefersPixelLayoutBGRA -> ae_sys::DRAWBOT_Boolean, supplier_ref.as_ptr())? != 0)
    }

    /// Check if the supplier supports ARGB pixel layout.
    pub fn supports_pixel_layout_argb(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, SupportsPixelLayoutARGB -> ae_sys::DRAWBOT_Boolean, supplier_ref.as_ptr())? != 0)
    }

    /// Check if the supplier prefers ARGB pixel layout.
    pub fn prefers_pixel_layout_argb(&self, supplier_ref: impl AsPtr<DRAWBOT_SupplierRef>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PrefersPixelLayoutARGB -> ae_sys::DRAWBOT_Boolean, supplier_ref.as_ptr())? != 0)
    }

    /// Retain (increase reference count on) any object (pen, brush, path, etc). For example, it should be used when any object is copied and the copied object should be retained.
    pub fn retain_object(&self, obj_ref: ae_sys::DRAWBOT_ObjectRef) -> Result<(), Error> {
        call_suite_fn!(self, RetainObject, obj_ref)
    }

    /// Release (decrease reference count on) any object (pen, brush, path, etc). This function MUST be called for any object created using ``NewXYZ()`` from this suite.
    ///
    /// Do not call this function on a ``DRAWBOT_SupplierRef`` and ``DRAWBOT_SupplierRef``, since these are not created by the plug-in.
    pub fn release_object(&self, obj_ref: ae_sys::DRAWBOT_ObjectRef) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseObject, obj_ref)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::DRAWBOT_PixelLayout,
    PixelLayout {
        Rgb24          = ae_sys::kDRAWBOT_PixelLayout_24RGB,
        Bgr24          = ae_sys::kDRAWBOT_PixelLayout_24BGR,
        Rgb32          = ae_sys::kDRAWBOT_PixelLayout_32RGB,
        Bgr32          = ae_sys::kDRAWBOT_PixelLayout_32BGR,
        Argb32Straight = ae_sys::kDRAWBOT_PixelLayout_32ARGB_Straight,
        Argb32Premul   = ae_sys::kDRAWBOT_PixelLayout_32ARGB_Premul,
        Bgra32Straight = ae_sys::kDRAWBOT_PixelLayout_32BGRA_Straight,
        Bgra32Premul   = ae_sys::kDRAWBOT_PixelLayout_32BGRA_Premul,
    }
}

define_suite_item_wrapper!(
    ae_sys::DRAWBOT_SupplierRef, SupplierRef,
    suite: SupplierSuite,
    /// Create and release drawing tools, get default settings, and query drawing capabilities.
    Supplier {
        dispose: ;

        /// Create a new pen.
        new_pen(color: &ColorRgba, size: f32) -> Pen => suite.new_pen,

        /// Create a new brush.
        new_brush(color: &ColorRgba)-> Brush => suite.new_brush,

        /// Check if current supplier supports text.
        supports_text() -> bool => suite.supports_text,

        /// Get the default font size.
        default_font_size() -> f32 => suite.default_font_size,

        /// Create a new font with default settings.
        new_default_font(font_size: f32) -> Font => suite.new_default_font,

        /// Create a new image from buffer passed to `pixel_data`.
        new_image_from_buffer(width: usize, height: usize, row_bytes: usize, pixel_layout: PixelLayout, pixel_data: Vec<u8>) -> Image => suite.new_image_from_buffer,

        /// Create a new path.
        new_path() -> Path => suite.new_path,

        /// Check if the supplier supports BGRA pixel layout.
        supports_pixel_layout_bgra() -> bool => suite.supports_pixel_layout_bgra,

        /// Check if the supplier prefers BGRA pixel layout.
        prefers_pixel_layout_bgra() -> bool => suite.prefers_pixel_layout_bgra,

        /// Check if the supplier supports ARGB pixel layout.
        supports_pixel_layout_argb() -> bool => suite.supports_pixel_layout_argb,

        /// Check if the supplier prefers ARGB pixel layout.
        prefers_pixel_layout_argb() -> bool => suite.prefers_pixel_layout_argb,
    }
);
