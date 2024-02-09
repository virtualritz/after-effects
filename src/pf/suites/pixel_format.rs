use crate::*;

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

define_suite!(
    PixelFormatSuite,
    PF_PixelFormatSuite1,
    kPFPixelFormatSuite,
    kPFPixelFormatSuiteVersion1
);

impl PixelFormatSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn clear_supported_pixel_formats(&self, effect_ref: ProgressInfo) -> Result<(), Error> {
        call_suite_fn!(self, ClearSupportedPixelFormats, effect_ref.as_ptr())
    }

    pub fn add_supported_pixel_format(&self, effect_ref: ProgressInfo, pixel_format: pf::PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedPixelFormat, effect_ref.as_ptr(), pixel_format.into())
    }

    pub fn add_pr_supported_pixel_format(&self, effect_ref: ProgressInfo, pixel_format: pr::PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedPixelFormat, effect_ref.as_ptr(), pixel_format.into())
    }
}
