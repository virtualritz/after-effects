use crate::*;

define_suite!(PPixSuite,  PrSDKPPixSuite,  kPrSDKPPixSuite,  kPrSDKPPixSuiteVersion);
define_suite!(PPix2Suite, PrSDKPPix2Suite, kPrSDKPPix2Suite, kPrSDKPPix2SuiteVersion);

#[derive(Clone, Copy, Debug)]
pub struct YUV420PlanarBuffers {
    pub y_data: *mut std::ffi::c_char,
    pub y_row_bytes: u32,
    pub u_data: *mut std::ffi::c_char,
    pub u_row_bytes: u32,
    pub v_data: *mut std::ffi::c_char,
    pub v_row_bytes: u32,
}

impl PPixSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// This will free this ppix. The ppix is no longer valid after this function is called.
    /// * `ppix_handle` - The ppix handle you want to dispose.
    pub fn dispose(&self, ppix_handle: pr_sys::PPixHand) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, Dispose, ppix_handle)?;
        Ok(())
    }
    /// This will return a pointer to the pixel buffer.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    /// * `requested_access` - Requested pixel access. Most PPixs do not support write access modes.
    ///
    /// Returns the output pixel buffer address. May be NULL if the requested pixel access is not supported.
    pub fn get_pixels(&self, ppix_handle: pr_sys::PPixHand, requested_access: PPixBufferAccess) -> Result<*mut std::ffi::c_char, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, GetPixels, ppix_handle, requested_access.into(), &mut ptr)?;
        Ok(ptr)
    }
    /// This will return the bounding rect.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the bounding rect.
    pub fn get_bounds(&self, ppix_handle: pr_sys::PPixHand) -> Result<pr_sys::prRect, Error> {
        let mut rect: pr_sys::prRect = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetBounds, ppix_handle, &mut rect)?;
        Ok(rect)
    }
    /// This will return the row bytes of the ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    /// Returns how many bytes must be added to the pixel buffer address to get to the next line.
    ///
    /// May be negative.
    pub fn get_row_bytes(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetRowBytes, ppix_handle, &mut val)?;
        Ok(val)
    }
    /// This will return the pixel aspect ratio of this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the numerator and denominator of the pixel aspect ratio.
    pub fn get_pixel_aspect_ratio(&self, ppix_handle: pr_sys::PPixHand) -> Result<(u32, u32), Error> {
        let mut num = 0;
        let mut den = 0;
        pr_call_suite_fn!(self.suite_ptr, GetPixelAspectRatio, ppix_handle, &mut num, &mut den)?;
        Ok((num, den))
    }
    /// This will return the pixel format of this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the pixel format of this ppix.
    pub fn get_pixel_format(&self, ppix_handle: pr_sys::PPixHand) -> Result<PixelFormat, Error> {
        let mut val: pr_sys::PrPixelFormat = 0;
        pr_call_suite_fn!(self.suite_ptr, GetPixelFormat, ppix_handle, &mut val)?;
        Ok(PixelFormat::from(val))
    }
    /// This will return the unique key for this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the pixel format of this ppix.
    ///
    /// Returns error if the key is not available.
    pub fn get_unique_key(&self, ppix_handle: pr_sys::PPixHand) -> Result<Vec<u8>, Error> {
        let mut size = 0;
        pr_call_suite_fn!(self.suite_ptr, GetUniqueKeySize, &mut size)?;
        let mut buffer = vec![0; size];
        pr_call_suite_fn!(self.suite_ptr, GetUniqueKey, ppix_handle, buffer.as_mut_ptr(), size)?;
        Ok(buffer)
    }
    /// This will return the render time for this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the render time in milliseconds. If the frame was cached, this time will be 0.
    pub fn get_render_time(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetRenderTime, ppix_handle, &mut val)?;
        Ok(val)
    }
}

impl PPix2Suite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// This will return the total size of the ppix in bytes.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the size of the ppix in bytes.
    pub fn get_size(&self, ppix_handle: pr_sys::PPixHand) -> Result<usize, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetSize, ppix_handle, &mut val)?;
        Ok(val)
    }
    /// [Added in CS4]
    /// This will return the planar buffers and rowbytes for a PPixHand if the contained pixels are in a planar format, such as
    /// - `PixelFormat::Yuv420Mpeg2FramePicturePlanar8u601`
    /// - `PixelFormat::Yuv420Mpeg2FieldPicturePlanar8u601`
    /// - `PixelFormat::Yuv420Mpeg2FramePicturePlanar8u709`
    /// - `PixelFormat::Yuv420Mpeg2FieldPicturePlanar8u709`
    /// * `ppix_handle` - The ppix handle you want to operate on.
    /// * `requested_access` - Will return an error if the source is read-only and the request is for write or read/write.
    ///
    /// Returns [`YUV420PlanarBuffers`] which contains:
    /// * `xxx_data` - The output (Y, U, or V) pixel buffer address. May be NULL if the requested access is not supported.
    /// * `xxx_row_bytes` - How many bytes must be added to the pixel buffer address to get to the next line. May be negative.
    pub fn get_yuv420_planar_buffers(&self, ppix_handle: pr_sys::PPixHand, requested_access: PPixBufferAccess) -> Result<YUV420PlanarBuffers, Error> {
        let mut ret: YUV420PlanarBuffers = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetYUV420PlanarBuffers, ppix_handle, requested_access.into(),
            &mut ret.y_data, &mut ret.y_row_bytes,
            &mut ret.u_data, &mut ret.u_row_bytes,
            &mut ret.v_data, &mut ret.v_row_bytes,
        )?;
        Ok(ret)
    }
    pub fn get_origin(&self, ppix_handle: pr_sys::PPixHand) -> Result<(i32, i32), Error> {
        let mut x = 0;
        let mut y = 0;
        pr_call_suite_fn!(self.suite_ptr, GetOrigin, ppix_handle, &mut x, &mut y)?;
        Ok((x, y))
    }
    pub fn get_field_order(&self, ppix_handle: pr_sys::PPixHand) -> Result<pr_sys::prFieldType, Error> {
        let mut val: pr_sys::prFieldType = 0;
        pr_call_suite_fn!(self.suite_ptr, GetFieldOrder, ppix_handle, &mut val)?;
        Ok(val)
    }
}