use crate::*;
use aftereffects_sys as ae_sys;

define_handle_wrapper!(DrawRef, DRAWBOT_DrawRef);
define_handle_wrapper!(SupplierRef, DRAWBOT_SupplierRef);
define_handle_wrapper!(SurfaceRef, DRAWBOT_SurfaceRef);
define_handle_wrapper!(ImageRef, DRAWBOT_ImageRef);

// FIXME: impl Drop for ImageRef – Needed or we'll leak the image every time

pub type PointF32 = ae_sys::DRAWBOT_PointF32;
pub type ColorRGBA = ae_sys::DRAWBOT_ColorRGBA;
pub type RectF32 = ae_sys::DRAWBOT_RectF32;

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

define_suite!(
    DrawbotSuite,
    DRAWBOT_DrawbotSuite1,
    kDRAWBOT_DrawSuite,
    kDRAWBOT_DrawSuite_Version1
);

impl DrawbotSuite {
    pub fn get_supplier(
        &self,
        draw_ref: &DrawRef,
    ) -> Result<SupplierRef, Error> {
        let mut supplier_ref =
            std::mem::MaybeUninit::<ae_sys::DRAWBOT_SupplierRef>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            GetSupplier,
            draw_ref.as_ptr(),
            supplier_ref.as_mut_ptr()
        ) {
            Ok(()) => Ok(SupplierRef(unsafe { supplier_ref.assume_init() })),
            Err(e) => Err(e),
        }
    }

    pub fn get_surface(
        &self,
        draw_ref: &DrawRef,
    ) -> Result<SurfaceRef, Error> {
        let mut surface_ref =
            std::mem::MaybeUninit::<ae_sys::DRAWBOT_SurfaceRef>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            GetSurface,
            draw_ref.as_ptr(),
            surface_ref.as_mut_ptr()
        ) {
            Ok(()) => Ok(SurfaceRef(unsafe { surface_ref.assume_init() })),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
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
        let mut image_ref =
            std::mem::MaybeUninit::<ae_sys::DRAWBOT_ImageRef>::uninit();

        assert!(row_bytes * height <= pixel_data.len());

        match ae_call_suite_fn!(
            self.suite_ptr,
            NewImageFromBuffer,
            supplier_ref.as_ptr(),
            width as _,
            height as _,
            row_bytes as _,
            pixel_layout as _,
            pixel_data.as_ptr() as _,
            image_ref.as_mut_ptr()
        ) {
            Ok(()) => {
                Ok(ImageRef::from_raw(unsafe { image_ref.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }
}

define_suite!(
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
        match ae_call_suite_fn!(
            self.suite_ptr,
            PaintRect,
            surface_ref.as_ptr(),
            color as _,
            rect as _,
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn draw_image(
        &self,
        surface_ref: &SurfaceRef,
        image_ref: &ImageRef,
        origin: &PointF32,
        alpha: f32,
    ) -> Result<(), Error> {
        match ae_call_suite_fn!(
            self.suite_ptr,
            DrawImage,
            surface_ref.as_ptr(),
            image_ref.as_ptr(),
            origin as _,
            alpha
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn flush(&self, surface_ref: &SurfaceRef) -> Result<(), Error> {
        match ae_call_suite_fn!(self.suite_ptr, Flush, surface_ref.as_ptr(),) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    PathSuite,
    DRAWBOT_PathSuite1,
    kDRAWBOT_PathSuite,
    kDRAWBOT_PathSuite_Version1
);
