use crate::*;

define_handle_wrapper!(DrawRef, DRAWBOT_DrawRef);
define_handle_wrapper!(SupplierRef, DRAWBOT_SupplierRef);
define_handle_wrapper!(SurfaceRef, DRAWBOT_SurfaceRef);
define_handle_wrapper!(ImageRef, DRAWBOT_ImageRef);

// FIXME: impl Drop for ImageRef – Needed or we'll leak the image every time

pub mod suites {
    pub(crate) mod draw;     pub use draw::*;
    pub(crate) mod image;    pub use image::*;
    pub(crate) mod path;     pub use path::*;
    pub(crate) mod pen;      pub use pen::*;
    pub(crate) mod supplier; pub use supplier::*;
    pub(crate) mod surface;  pub use surface::*;
}

pub type PointF32 = ae_sys::DRAWBOT_PointF32;
pub type ColorRGBA = ae_sys::DRAWBOT_ColorRGBA;
pub type RectF32 = ae_sys::DRAWBOT_RectF32;
pub type MatrixF32 = ae_sys::DRAWBOT_MatrixF32;

#[repr(i32)]
pub enum PixelLayout {
    ARGB32Straight = ae_sys::kDRAWBOT_PixelLayout_32ARGB_Straight as _,
    ARGB32Premuliplied = ae_sys::kDRAWBOT_PixelLayout_32ARGB_Premul as _,
}

/*
impl PixelData {
    fn as_ptr(&self) -> *const u8 {
        match self {
            PixelData::ARGB32Straight(data) => data.as_ptr(),
            PixelData::ARGB32Premuliplied(data) => data.as_ptr(),
        }
    }
}*/
