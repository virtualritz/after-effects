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
        call_suite_fn!(self, Dispose, ppix_handle)
    }
    /// This will return a pointer to the pixel buffer.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    /// * `requested_access` - Requested pixel access. Most PPixs do not support write access modes.
    ///
    /// Returns the output pixel buffer address. May be NULL if the requested pixel access is not supported.
    pub fn get_pixels(&self, ppix_handle: pr_sys::PPixHand, requested_access: PPixBufferAccess) -> Result<*mut std::ffi::c_char, Error> {
        call_suite_fn_single!(self, GetPixels -> *mut std::ffi::c_char, ppix_handle, requested_access.into())
    }
    /// This will return the bounding rect.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the bounding rect.
    pub fn get_bounds(&self, ppix_handle: pr_sys::PPixHand) -> Result<pr_sys::prRect, Error> {
        call_suite_fn_single!(self, GetBounds -> pr_sys::prRect, ppix_handle)
    }
    /// This will return the row bytes of the ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    /// Returns how many bytes must be added to the pixel buffer address to get to the next line.
    ///
    /// May be negative.
    pub fn get_row_bytes(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetRowBytes -> i32, ppix_handle)
    }
    /// This will return the pixel aspect ratio of this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the numerator and denominator of the pixel aspect ratio.
    pub fn get_pixel_aspect_ratio(&self, ppix_handle: pr_sys::PPixHand) -> Result<(u32, u32), Error> {
        let mut num = 0;
        let mut den = 0;
        call_suite_fn!(self, GetPixelAspectRatio, ppix_handle, &mut num, &mut den)?;
        Ok((num, den))
    }
    /// This will return the pixel format of this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the pixel format of this ppix.
    pub fn get_pixel_format(&self, ppix_handle: pr_sys::PPixHand) -> Result<PixelFormat, Error> {
        Ok(call_suite_fn_single!(self, GetPixelFormat -> pr_sys::PrPixelFormat, ppix_handle)?.into())
    }
    /// This will return the unique key for this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the pixel format of this ppix.
    ///
    /// Returns error if the key is not available.
    pub fn get_unique_key(&self, ppix_handle: pr_sys::PPixHand) -> Result<Vec<u8>, Error> {
        let mut size = 0;
        call_suite_fn!(self, GetUniqueKeySize, &mut size)?;
        let mut buffer = vec![0; size];
        call_suite_fn!(self, GetUniqueKey, ppix_handle, buffer.as_mut_ptr(), size)?;
        Ok(buffer)
    }
    /// This will return the render time for this ppix.
    /// * `ppix_handle` - The ppix handle you want to operate on.
    ///
    /// Returns the render time in milliseconds. If the frame was cached, this time will be 0.
    pub fn get_render_time(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetRenderTime -> i32, ppix_handle)
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
        call_suite_fn_single!(self, GetSize -> usize, ppix_handle)
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
        call_suite_fn!(self, GetYUV420PlanarBuffers, ppix_handle, requested_access.into(),
            &mut ret.y_data, &mut ret.y_row_bytes,
            &mut ret.u_data, &mut ret.u_row_bytes,
            &mut ret.v_data, &mut ret.v_row_bytes,
        )?;
        Ok(ret)
    }
    pub fn get_origin(&self, ppix_handle: pr_sys::PPixHand) -> Result<(i32, i32), Error> {
        let mut x = 0;
        let mut y = 0;
        call_suite_fn!(self, GetOrigin, ppix_handle, &mut x, &mut y)?;
        Ok((x, y))
    }
    pub fn get_field_order(&self, ppix_handle: pr_sys::PPixHand) -> Result<pr_sys::prFieldType, Error> {
        call_suite_fn_single!(self, GetFieldOrder -> pr_sys::prFieldType, ppix_handle)
    }
}
