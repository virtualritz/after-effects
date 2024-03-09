use crate::*;
use crate::{ define_iterate, define_iterate_lut_and_generic };
use ae_sys::{ PF_EffectWorld, PF_Pixel, PF_Pixel16, PF_PixelFloat };
use std::ffi::c_void;

define_suite!(
    /// Effects often iterate over all pixels in an image, filtering each one.
    /// By taking advantage of After Effects' iteration suites, you make it possible for After Effects to sub-allocate your task
    /// to as many processors are present, taking advantage of hardware-specific acceleration.
    ///
    /// After Effects will also manage progress reporting and user cancellation automatically.
    ///
    /// Use these suites! Make sure the pixel processing functions you pass to these iterator callbacks are re-entrant.
    ///
    /// ```
    ///   The October 2021 SDK update increases the number of concurrent iterate threads up to the available system CPU cores instead of the previous hard-coded limit of 32.
    /// ```
    Iterate8Suite,
    PF_Iterate8Suite2,
    kPFIterate8Suite,
    kPFIterate8SuiteVersion2
);

impl Iterate8Suite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    // Helpers for the define_iterate macros
    #[inline(always)] fn get_in_data(&self) -> *const ae_sys::PF_InData { std::ptr::null() }
    #[inline(always)] fn get_funcs_ptr(&self) -> *const ae_sys::PF_Iterate8Suite2 { self.suite_ptr }

    define_iterate!(+ in_data: &InData, iterate,                       Pixel8,  PF_Pixel);
    define_iterate!(+ in_data: &InData, iterate_origin,                Pixel8,  PF_Pixel,   origin: Option<Point>);
    define_iterate!(+ in_data: &InData, iterate_origin_non_clip_src,   Pixel8,  PF_Pixel,   origin: Option<Point>);
    define_iterate_lut_and_generic!(+ in_data: &InData,);
}

define_suite!(
    /// Effects often iterate over all pixels in an image, filtering each one.
    /// By taking advantage of After Effects' iteration suites, you make it possible for After Effects to sub-allocate your task
    /// to as many processors are present, taking advantage of hardware-specific acceleration.
    ///
    /// After Effects will also manage progress reporting and user cancellation automatically.
    ///
    /// Use these suites! Make sure the pixel processing functions you pass to these iterator callbacks are re-entrant.
    ///
    /// ```
    ///   The October 2021 SDK update increases the number of concurrent iterate threads up to the available system CPU cores instead of the previous hard-coded limit of 32.
    /// ```
    Iterate16Suite,
    PF_Iterate16Suite2,
    kPFIterate16Suite,
    kPFIterate16SuiteVersion2
);

impl Iterate16Suite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    // Helpers for the define_iterate macros
    #[inline(always)] fn get_in_data(&self) -> *const ae_sys::PF_InData { std::ptr::null() }
    #[inline(always)] fn get_funcs_ptr(&self) -> *const ae_sys::PF_Iterate16Suite2 { self.suite_ptr }

    define_iterate!(+ in_data: &InData, iterate,                       Pixel16,  PF_Pixel16);
    define_iterate!(+ in_data: &InData, iterate_origin,                Pixel16,  PF_Pixel16,   origin: Option<Point>);
    define_iterate!(+ in_data: &InData, iterate_origin_non_clip_src,   Pixel16,  PF_Pixel16,   origin: Option<Point>);
}

define_suite!(
    /// Effects often iterate over all pixels in an image, filtering each one.
    /// By taking advantage of After Effects' iteration suites, you make it possible for After Effects to sub-allocate your task
    /// to as many processors are present, taking advantage of hardware-specific acceleration.
    ///
    /// After Effects will also manage progress reporting and user cancellation automatically.
    ///
    /// Use these suites! Make sure the pixel processing functions you pass to these iterator callbacks are re-entrant.
    ///
    /// ```
    ///   The October 2021 SDK update increases the number of concurrent iterate threads up to the available system CPU cores instead of the previous hard-coded limit of 32.
    /// ```
    IterateFloatSuite,
    PF_iterateFloatSuite2,
    kPFIterateFloatSuite,
    kPFIterateFloatSuiteVersion2
);

impl IterateFloatSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    // Helpers for the define_iterate macros
    #[inline(always)] fn get_in_data(&self) -> *const ae_sys::PF_InData { std::ptr::null() }
    #[inline(always)] fn get_funcs_ptr(&self) -> *const ae_sys::PF_iterateFloatSuite2 { self.suite_ptr }

    define_iterate!(+ in_data: impl AsPtr<*const ae_sys::PF_InData>, iterate,                       PixelF32,  PF_PixelFloat);
    define_iterate!(+ in_data: impl AsPtr<*const ae_sys::PF_InData>, iterate_origin,                PixelF32,  PF_PixelFloat,   origin: Option<Point>);
    define_iterate!(+ in_data: impl AsPtr<*const ae_sys::PF_InData>, iterate_origin_non_clip_src,   PixelF32,  PF_PixelFloat,   origin: Option<Point>);
}
