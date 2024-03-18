use crate::*;
use pr_sys::{ PF_ProgPtr, PF_InData };

define_suite!(
    /// Premiere pixel format suite. Not available in After Effects.
    PixelFormatSuite,
    PF_PixelFormatSuite1,
    kPFPixelFormatSuite,
    kPFPixelFormatSuiteVersion1
);

impl PixelFormatSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn add_supported_pixel_format(&self, effect_ref: impl AsPtr<PF_ProgPtr>, pixel_format: PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedPixelFormat, effect_ref.as_ptr(), pixel_format.into())
    }

    pub fn clear_supported_pixel_formats(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<(), Error> {
        call_suite_fn!(self, ClearSupportedPixelFormats, effect_ref.as_ptr())
    }

    pub fn new_world_of_pixel_format(&self, in_data: impl AsPtr<*mut PF_InData>, width: u32, height: u32, flags: NewWorldFlags, pixel_format: PixelFormat) -> Result<pr_sys::PF_EffectWorld, Error> {
        call_suite_fn_single!(self, NewWorldOfPixelFormat -> pr_sys::PF_EffectWorld, (*in_data.as_ptr()).effect_ref, width, height, flags.bits(), pixel_format.into())
    }

    pub fn dispose_world(&self, effect_ref: impl AsPtr<PF_ProgPtr>, world: *mut pr_sys::PF_EffectWorld) -> Result<(), Error> {
        call_suite_fn!(self, DisposeWorld, effect_ref.as_ptr(), world)
    }

    pub fn pixel_format(&self, world: impl AsPtr<*mut pr_sys::PF_EffectWorld>) -> Result<PixelFormat, Error> {
        Ok(call_suite_fn_single!(self, GetPixelFormat -> pr_sys::PrPixelFormat, world.as_ptr())?.into())
    }

    /// Retrieves the minimum i.e. "black" value for a give pixel type.
    ///
    /// NOTE: pixel types like YUY2, YUYV actually contain a group of two pixels to specify a color completely, so the data size returned in this case will be 4 bytes (rather than 2)
    ///
    /// * `pixel_format` - the Premiere pixel format whose black level you want
    /// * `pixel_data` - a void pointer to data large enough to hold the pixel value (see note above)
    pub fn black_for_pixel_format(&self, pixel_format: PixelFormat) -> Result<Vec<u8>, Error> {
        let mut pixel_data = vec![0u8; pixel_size(pixel_format)];
        call_suite_fn!(self, GetBlackForPixelFormat, pixel_format.into(), pixel_data.as_mut_ptr() as *mut _)?;
        Ok(pixel_data)
    }

    /// Retrieves the maximum i.e. "white" value for a give pixel type.
    ///
    /// NOTE: pixel types like YUY2, YUYV actually contain a group of two pixels to specify a color completely, so the data size returned in this case will be 4 bytes (rather than 2)
    ///
    /// * `pixel_format` - the Premiere pixel format whose white level you want
    /// * `pixel_data` - a void pointer to data large enough to hold the pixel value (see note above)
    pub fn white_for_pixel_format(&self, pixel_format: PixelFormat) -> Result<Vec<u8>, Error> {
        let mut pixel_data = vec![0u8; pixel_size(pixel_format)];
        call_suite_fn!(self, GetWhiteForPixelFormat, pixel_format.into(), pixel_data.as_mut_ptr() as *mut _)?;
        Ok(pixel_data)
    }

    /// Converts an alpha, red, green, blue specification into a pixel value for a give pixel type.
    ///
    /// NOTE: pixel types like YUY2, YUYV actually contain a group of two pixels to specify a color completely, so the data size returned in this case will be 4 bytes (rather than 2)
    ///
    /// * `pixel_format` - the Premiere pixel format whose white level you want
    /// * `alpha`        - alpha value (0.0 - 1.0)
    /// * `red`          - red value (0.0 - 1.0)
    /// * `green`        - green value (0.0 - 1.0)
    /// * `blue`         - blue value (0.0 - 1.0)
    /// * `pixel_data`   - a void pointer to data large enough to hold the pixel value (see note above)
    pub fn convert_color_to_pixel_formatted_data(&self, pixel_format: PixelFormat, alpha: f32, red: f32, green: f32, blue: f32) -> Result<Vec<u8>, Error> {
        let mut pixel_data = vec![0u8; pixel_size(pixel_format)];
        call_suite_fn!(self, ConvertColorToPixelFormattedData, pixel_format.into(), alpha, red, green, blue, pixel_data.as_mut_ptr() as *mut _)?;
        Ok(pixel_data)
    }
}

fn pixel_size(pixel_format: PixelFormat) -> usize {
    use PixelFormat::*;
    match pixel_format {
        Bgra4444_8u | Vuya4444_8u | Vuya4444_8u709 | Argb4444_8u | Bgrx4444_8u | Vuyx4444_8u | Vuyx4444_8u709 | Xrgb4444_8u | Bgrp4444_8u | Vuyp4444_8u |
        Vuyp4444_8u709 | Prgb4444_8u | Vuya4444_16u | Rgb444_10u | Yuyv422_8u601 | Yuyv422_8u709 | Uyvy422_8u601 | Uyvy422_8u709 | Xrgb4444_32fLinear => 4,

        Bgra4444_16u | Argb4444_16u | Bgrx4444_16u | Xrgb4444_16u | Bgrp4444_16u | Prgb4444_16u => 8,

        Bgra4444_32f | Vuya4444_32f | Vuya4444_32f709 | Argb4444_32f | Bgrx4444_32f | Vuyx4444_32f | Vuyx4444_32f709 | Xrgb4444_32f | Bgrp4444_32f | Vuyp4444_32f | Vuyp4444_32f709 | Prgb4444_32f |
        V210422_10u601 | V210422_10u709 | Uyvy422_32f601 | Uyvy422_32f709 | Bgra4444_32fLinear | Bgrp4444_32fLinear | Bgrx4444_32fLinear | Argb4444_32fLinear | Prgb4444_32fLinear => 16,

        Yuv420Mpeg2FramePicturePlanar8u601 | Yuv420Mpeg2FieldPicturePlanar8u601 | Yuv420Mpeg2FramePicturePlanar8u601FullRange | Yuv420Mpeg2FieldPicturePlanar8u601FullRange |
        Yuv420Mpeg2FramePicturePlanar8u709 | Yuv420Mpeg2FieldPicturePlanar8u709 | Yuv420Mpeg2FramePicturePlanar8u709FullRange | Yuv420Mpeg2FieldPicturePlanar8u709FullRange |
        Yuv420Mpeg4FramePicturePlanar8u601 | Yuv420Mpeg4FieldPicturePlanar8u601 | Yuv420Mpeg4FramePicturePlanar8u601FullRange | Yuv420Mpeg4FieldPicturePlanar8u601FullRange |
        Yuv420Mpeg4FramePicturePlanar8u709 | Yuv420Mpeg4FieldPicturePlanar8u709 | Yuv420Mpeg4FramePicturePlanar8u709FullRange | Yuv420Mpeg4FieldPicturePlanar8u709FullRange => 1,

        _ => 32 // just to be safe
    }
}

bitflags::bitflags! {
    pub struct NewWorldFlags: pr_sys::A_long {
        const NONE         = pr_sys::PF_NewWorldFlag_NONE         as pr_sys::A_long;
        const CLEAR_PIXELS = pr_sys::PF_NewWorldFlag_CLEAR_PIXELS as pr_sys::A_long;
        const DEEP_PIXELS  = pr_sys::PF_NewWorldFlag_DEEP_PIXELS  as pr_sys::A_long;
    }
}