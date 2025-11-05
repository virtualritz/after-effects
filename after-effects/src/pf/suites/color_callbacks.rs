
use crate::*;
use ae_sys::*;

define_suite!(
    /// Plug-ins can draw on image processing algorithms written for nearly any color space by using the following callback functions.
    ColorCallbacksSuite,
    PF_ColorCallbacksSuite1,
    kPFColorCallbacksSuite,
    kPFColorCallbacksSuiteVersion1
);

impl ColorCallbacksSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given an RGB pixel, returns an HLS (hue, lightness, saturation) pixel. HLS values are scaled from 0 to 1 in fixed point.
    pub fn rgb_to_hls(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<HLSPixel, Error> {
        let mut hls = HLSPixel::default();
        call_suite_fn!(self, RGBtoHLS, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut hls as *mut _ as *mut _)?;
        Ok(hls)
    }

    /// Given an HLS pixel, returns an RGB pixel.
    pub fn hls_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, hls: &HLSPixel) -> Result<Pixel8, Error> {
        // SAFETY: Zero-initializing Pixel8 (PF_Pixel) FFI type for use as an out-parameter.
        // Detailed explanation: (1) Pixel8 is a repr(C) FFI struct from Adobe After Effects SDK where all-zero bytes represent a valid state, (2) the zeroed value is immediately passed as a mutable reference to the HLStoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: Pixel8 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, HLStoRGB, effect_ref.as_ptr(), hls as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns a YIQ (luminance, inphase chrominance, quadrature chrominance) pixel.
    /// Y is 0 to 1 in fixed point, I is -0.5959 to 0.5959 in fixed point, and Q is -0.5227 to 0.5227 in fixed point.
    pub fn rgb_to_yiq(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<YIQPixel, Error> {
        let mut yiq = YIQPixel::default();
        call_suite_fn!(self, RGBtoYIQ, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut yiq as *mut _ as *mut _)?;
        Ok(yiq)
    }

    /// Given a YIQ pixel, returns an RGB pixel.
    pub fn yiq_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, yiq: &YIQPixel) -> Result<Pixel8, Error> {
        // SAFETY: Zero-initializing Pixel8 (PF_Pixel) FFI type for use as an out-parameter.
        // Detailed explanation: (1) Pixel8 is a repr(C) FFI struct from Adobe After Effects SDK where all-zero bytes represent a valid state, (2) the zeroed value is immediately passed as a mutable reference to the YIQtoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: Pixel8 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, YIQtoRGB, effect_ref.as_ptr(), yiq as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns 100 times its luminance value (0 to 25500).
    pub fn luminance(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<i32, Error> {
        call_suite_fn_single!(self, Luminance -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its hue angle mapped from 0 to 255, where 0 is 0 degrees and 255 is 360 degrees.
    pub fn hue(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<i32, Error> {
        call_suite_fn_single!(self, Hue -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its lightness value (0 to 255).
    pub fn lightness(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<i32, Error> {
        call_suite_fn_single!(self, Lightness -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its saturation value (0 to 255).
    pub fn saturation(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<i32, Error> {
        call_suite_fn_single!(self, Saturation -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }
}

define_suite!(
    /// Plug-ins can draw on image processing algorithms written for nearly any color space by using the following callback functions.
    ColorCallbacks16Suite,
    PF_ColorCallbacks16Suite1,
    kPFColorCallbacks16Suite,
    kPFColorCallbacks16SuiteVersion1
);

impl ColorCallbacks16Suite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given an RGB pixel, returns an HLS (hue, lightness, saturation) pixel. HLS values are scaled from 0 to 1 in fixed point.
    pub fn rgb_to_hls(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel16) -> Result<HLSPixel, Error> {
        let mut hls = HLSPixel::default();
        call_suite_fn!(self, RGBtoHLS, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut hls as *mut _ as *mut _)?;
        Ok(hls)
    }

    /// Given an HLS pixel, returns an RGB pixel.
    pub fn hls_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, hls: &HLSPixel) -> Result<Pixel16, Error> {
        // SAFETY: Zero-initializing Pixel16 (PF_Pixel16) FFI type for use as an out-parameter.
        // Detailed explanation: (1) Pixel16 is a repr(C) FFI struct from Adobe After Effects SDK where all-zero bytes represent a valid state, (2) the zeroed value is immediately passed as a mutable reference to the HLStoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: Pixel16 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, HLStoRGB, effect_ref.as_ptr(), hls as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns a YIQ (luminance, inphase chrominance, quadrature chrominance) pixel.
    /// Y is 0 to 1 in fixed point, I is -0.5959 to 0.5959 in fixed point, and Q is -0.5227 to 0.5227 in fixed point.
    pub fn rgb_to_yiq(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<YIQPixel, Error> {
        let mut yiq = YIQPixel::default();
        call_suite_fn!(self, RGBtoYIQ, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut yiq as *mut _ as *mut _)?;
        Ok(yiq)
    }

    /// Given a YIQ pixel, returns an RGB pixel.
    pub fn yiq_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, yiq: &YIQPixel) -> Result<Pixel16, Error> {
        // SAFETY: Zero-initializing Pixel16 (PF_Pixel16) FFI type for use as an out-parameter.
        // Detailed explanation: (1) Pixel16 is a repr(C) FFI struct from Adobe After Effects SDK where all-zero bytes represent a valid state, (2) the zeroed value is immediately passed as a mutable reference to the YIQtoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: Pixel16 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, YIQtoRGB, effect_ref.as_ptr(), yiq as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns 100 times its luminance value (0 to 25500).
    pub fn luminance(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel16) -> Result<i32, Error> {
        call_suite_fn_single!(self, Luminance -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its hue angle mapped from 0 to 255, where 0 is 0 degrees and 255 is 360 degrees.
    pub fn hue(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel16) -> Result<i32, Error> {
        call_suite_fn_single!(self, Hue -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its lightness value (0 to 255).
    pub fn lightness(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel16) -> Result<i32, Error> {
        call_suite_fn_single!(self, Lightness -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its saturation value (0 to 255).
    pub fn saturation(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel16) -> Result<i32, Error> {
        call_suite_fn_single!(self, Saturation -> i32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }
}

define_suite!(
    /// Plug-ins can draw on image processing algorithms written for nearly any color space by using the following callback functions.
    ColorCallbacksFloatSuite,
    PF_ColorCallbacksFloatSuite1,
    kPFColorCallbacksFloatSuite,
    kPFColorCallbacksFloatSuiteVersion1
);

impl ColorCallbacksFloatSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given an RGB pixel, returns an HLS (hue, lightness, saturation) pixel. HLS values are scaled from 0 to 1 in fixed point.
    pub fn rgb_to_hls(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &PixelF32) -> Result<HLSPixel, Error> {
        let mut hls = HLSPixel::default();
        call_suite_fn!(self, RGBtoHLS, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut hls as *mut _ as *mut _)?;
        Ok(hls)
    }

    /// Given an HLS pixel, returns an RGB pixel.
    pub fn hls_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, hls: &HLSPixel) -> Result<PixelF32, Error> {
        // SAFETY: Zero-initializing PixelF32 (PF_Pixel32) FFI type for use as an out-parameter.
        // Detailed explanation: (1) PixelF32 is a repr(C) FFI struct from Adobe After Effects SDK containing f32 fields where all-zero bytes represent valid 0.0 float values, (2) the zeroed value is immediately passed as a mutable reference to the HLStoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: PixelF32 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, HLStoRGB, effect_ref.as_ptr(), hls as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns a YIQ (luminance, inphase chrominance, quadrature chrominance) pixel.
    /// Y is 0 to 1 in fixed point, I is -0.5959 to 0.5959 in fixed point, and Q is -0.5227 to 0.5227 in fixed point.
    pub fn rgb_to_yiq(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &Pixel8) -> Result<YIQPixel, Error> {
        let mut yiq = YIQPixel::default();
        call_suite_fn!(self, RGBtoYIQ, effect_ref.as_ptr(), rgb as *const _ as *mut _, &mut yiq as *mut _ as *mut _)?;
        Ok(yiq)
    }

    /// Given a YIQ pixel, returns an RGB pixel.
    pub fn yiq_to_rgb(&self, effect_ref: impl AsPtr<PF_ProgPtr>, yiq: &YIQPixel) -> Result<PixelF32, Error> {
        // SAFETY: Zero-initializing PixelF32 (PF_Pixel32) FFI type for use as an out-parameter.
        // Detailed explanation: (1) PixelF32 is a repr(C) FFI struct from Adobe After Effects SDK containing f32 fields where all-zero bytes represent valid 0.0 float values, (2) the zeroed value is immediately passed as a mutable reference to the YIQtoRGB FFI function which fully initializes it before we use it, (3) the value is only returned when the FFI call succeeds (no error), ensuring proper initialization.
        // Would be UB if: PixelF32 contained types with invalid zero-bit patterns (like non-nullable references or bool with values other than 0/1), or if we used the value before the FFI call initialized it, or if the FFI call failed but we used the value anyway.
        let mut rgb = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, YIQtoRGB, effect_ref.as_ptr(), yiq as *const _ as *mut _, &mut rgb)?;
        Ok(rgb)
    }

    /// Given an RGB pixel, returns 100 times its luminance value (0 to 25500).
    pub fn luminance(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &PixelF32) -> Result<f32, Error> {
        call_suite_fn_single!(self, Luminance -> f32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its hue angle mapped from 0 to 255, where 0 is 0 degrees and 255 is 360 degrees.
    pub fn hue(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &PixelF32) -> Result<f32, Error> {
        call_suite_fn_single!(self, Hue -> f32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its lightness value (0 to 255).
    pub fn lightness(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &PixelF32) -> Result<f32, Error> {
        call_suite_fn_single!(self, Lightness -> f32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }

    /// Given an RGB pixel, returns its saturation value (0 to 255).
    pub fn saturation(&self, effect_ref: impl AsPtr<PF_ProgPtr>, rgb: &PixelF32) -> Result<f32, Error> {
        call_suite_fn_single!(self, Saturation -> f32, effect_ref.as_ptr(), rgb as *const _ as *mut _)
    }
}
