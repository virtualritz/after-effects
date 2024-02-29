use crate::*;
use ae_sys::PF_ProgPtr;

define_suite!(
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

    pub fn add_supported_pixel_format(&self, effect_ref: impl AsPtr<PF_ProgPtr>, pixel_format: pr::PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedPixelFormat, effect_ref.as_ptr(), pixel_format.into())
    }

    pub fn clear_supported_pixel_formats(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<(), Error> {
        call_suite_fn!(self, ClearSupportedPixelFormats, effect_ref.as_ptr())
    }

// pub NewWorldOfPixelFormat(effect_ref: PF_ProgPtr, width: A_u_long, height: A_u_long, flags: PF_NewWorldFlags, pixelFormat: PrPixelFormat, world: *mut PF_EffectWorld) -> PF_Err,
// pub DisposeWorld(effect_ref: PF_ProgPtr, world: *mut PF_EffectWorld) -> PF_Err,
// pub GetPixelFormat(inWorld: *mut PF_EffectWorld, pixelFormat: *mut PrPixelFormat) -> PF_Err,
// pub GetBlackForPixelFormat(pixelFormat: PrPixelFormat, pixelData: *mut ::std::os::raw::c_void) -> PF_Err,
// pub GetWhiteForPixelFormat(pixelFormat: PrPixelFormat, pixelData: *mut ::std::os::raw::c_void) -> PF_Err,
// pub ConvertColorToPixelFormattedData(pixelFormat: PrPixelFormat, alpha: f32, red: f32, green: f32, blue: f32, pixelData: *mut ::std::os::raw::c_void) -> PF_Err,
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::PF_PixelFormat,
    PixelFormat {
        Argb32       = ae_sys::PF_PixelFormat_ARGB32,
        Argb64       = ae_sys::PF_PixelFormat_ARGB64,
        Argb128      = ae_sys::PF_PixelFormat_ARGB128,
        GpuBgra128   = ae_sys::PF_PixelFormat_GPU_BGRA128,
        Reserved     = ae_sys::PF_PixelFormat_RESERVED,
        Bgra32       = ae_sys::PF_PixelFormat_BGRA32,
        Vuya32       = ae_sys::PF_PixelFormat_VUYA32,
        NtscDv25     = ae_sys::PF_PixelFormat_NTSCDV25,
        PalDv25      = ae_sys::PF_PixelFormat_PALDV25,
        Invalid      = ae_sys::PF_PixelFormat_INVALID,
        ForceLongInt = ae_sys::PF_PixelFormat_FORCE_LONG_INT,
    }
}
