use crate::*;

define_suite!(
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

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_PixelFloat,
                out: *mut ae_sys::PF_PixelFloat,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}
define_suite!(
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

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_Pixel16,
                out: *mut ae_sys::PF_Pixel16,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}

define_suite!(
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

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_Pixel8,
                out: *mut ae_sys::PF_Pixel8,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}
