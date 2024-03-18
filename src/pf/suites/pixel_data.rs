
use crate::*;
use ae_sys::*;

define_suite!(
    /// Get raw pixel data from a layer.
    PixelDataSuite,
    PF_PixelDataSuite2,
    kPFPixelDataSuite,
    kPFPixelDataSuiteVersion2
);

impl PixelDataSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Obtain a pointer to a 8-bpc pixel within the specified world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 8-bpc.
    ///
    /// The second parameter is optional; if it is `Some`, the returned pixel will be an interpretation of the values in the passed-in pixel, as if it were in the specified PF_EffectWorld.
    pub fn get_pixel_data8(self, world: impl AsPtr<*mut PF_EffectWorld>, pixels: Option<*mut Pixel8>) -> Result<*mut Pixel8, Error> {
        let mut ptr = std::ptr::null_mut();
        call_suite_fn!(self, get_pixel_data8, world.as_ptr(), pixels.unwrap_or(std::ptr::null_mut()), &mut ptr)?;
        if ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        Ok(ptr)
    }

    /// Obtain a pointer to a 16-bpc pixel within the specified world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 16-bpc.
    ///
    /// The second parameter is optional; if it is `Some`, the returned pixel will be an interpretation of the values in the passed-in pixel, as if it were in the specified PF_EffectWorld.
    pub fn get_pixel_data16(self, world: impl AsPtr<*mut PF_EffectWorld>, pixels: Option<*mut Pixel8>) -> Result<*mut Pixel16, Error> {
        let mut ptr = std::ptr::null_mut();
        call_suite_fn!(self, get_pixel_data16, world.as_ptr(), pixels.unwrap_or(std::ptr::null_mut()), &mut ptr)?;
        if ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        Ok(ptr)
    }

    /// Obtain a pointer to a 32-bpc float pixel within the specified world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 32-bpc.
    ///
    /// The second parameter is optional; if it is `Some`, the returned pixel will be an interpretation of the values in the passed-in pixel, as if it were in the specified PF_EffectWorld.
    pub fn get_pixel_data_float(self, world: impl AsPtr<*mut PF_EffectWorld>, pixels: Option<*mut Pixel8>) -> Result<*mut PixelF32, Error> {
        let mut ptr = std::ptr::null_mut();
        call_suite_fn!(self, get_pixel_data_float, world.as_ptr(), pixels.unwrap_or(std::ptr::null_mut()), &mut ptr)?;
        if ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        Ok(ptr)
    }

    /// Obtain a pointer to a 32-bpc float pixel within the specified GPU world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 32-bpc float.
    pub fn get_pixel_data_float_gpu(self, world: impl AsPtr<*mut PF_EffectWorld>) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        call_suite_fn!(self, get_pixel_data_float_gpu, world.as_ptr(), &mut ptr)?;
        if ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        Ok(ptr)
    }
}
