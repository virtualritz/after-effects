
use crate::*;

define_enum! {
    pr_sys::PrPPixBufferAccess,
    PPixBufferAccess {
        ReadOnly      = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ReadOnly,
        WriteOnly     = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_WriteOnly,
        ReadWrite     = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ReadWrite,
        ForceEnumSize = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ForceEnumSize,
    }
}
define_enum! {
    pr_sys::PrGPUDeviceFramework,
    GPUDeviceFramework {
        Cuda      = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_CUDA,
        OpenCl    = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_OpenCL,
        Metal     = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_Metal,
    }
}

define_enum! {
    pr_sys::PrRenderQuality,
    RenderQuality {
        Max           = pr_sys::PrRenderQuality_kPrRenderQuality_Max,
        High          = pr_sys::PrRenderQuality_kPrRenderQuality_High,
        Medium        = pr_sys::PrRenderQuality_kPrRenderQuality_Medium,
        Low           = pr_sys::PrRenderQuality_kPrRenderQuality_Low,
        Draft         = pr_sys::PrRenderQuality_kPrRenderQuality_Draft,
        Invalid       = pr_sys::PrRenderQuality_kPrRenderQuality_Invalid,
    }
}

define_enum! {
    pr_sys::pmFieldDisplay,
    FieldDisplay {
        ShowFirstField  = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowFirstField,
        ShowSecondField = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowSecondField,
        ShowBothFields  = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowBothFields,
        ForceSize       = pr_sys::pmFieldDisplay_pmFieldDisplay_ForceSize,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct YUV420PlanarBuffers {
    pub y_data: *mut std::ffi::c_char,
    pub y_row_bytes: u32,
    pub u_data: *mut std::ffi::c_char,
    pub u_row_bytes: u32,
    pub v_data: *mut std::ffi::c_char,
    pub v_row_bytes: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct VideoSegmentProperties {
    pub bounds: pr_sys::prRect,
    pub par_num: i32,
    pub par_den: i32,
    pub frame_rate: i64,
    pub field_type: pr_sys::prFieldType,
    pub color_space: Option<pr_sys::PrSDKColorSpaceID>,
}

define_suite!(GPUDeviceSuite, PrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuiteVersion);
impl GPUDeviceSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_device_count(&self) -> Result<usize, Error> {
        let mut count: u32 = 0;
        pr_call_suite_fn!(self.suite_ptr, GetDeviceCount, &mut count)?;
        Ok(count as usize)
    }
    pub fn get_device_info(&self, device_index: u32) -> Result<pr_sys::PrGPUDeviceInfo, Error> {
        let mut info: pr_sys::PrGPUDeviceInfo = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetDeviceInfo, pr_sys::kPrSDKGPUDeviceSuiteVersion, device_index, &mut info)?;
        Ok(info)
    }
    /// Acquire/release exclusive access to inDeviceIndex. All calls below this point generally require access be held.
    /// For full GPU plugins (those that use a separate entry point for GPU rendering) exclusive access is always held.
    /// These calls do not need to be made in that case.
    /// For CUDA calls cuCtxPushCurrent/cuCtxPopCurrent on the current thread to manage the devices context.
    pub fn acquire_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, AcquireExclusiveDeviceAccess, device_index)?;
        Ok(())
    }
    pub fn release_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReleaseExclusiveDeviceAccess, device_index)?;
        Ok(())
    }
    /// All device memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemAlloc or clCreateBuffer.
    pub fn allocate_device_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, AllocateDeviceMemory, device_index, size_in_bytes, &mut ptr)?;
        Ok(ptr)
    }
    pub fn free_device_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, FreeDeviceMemory, device_index, ptr)?;
        Ok(())
    }
    pub fn purge_device_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        let mut bytes_purged = 0;
        pr_call_suite_fn!(self.suite_ptr, PurgeDeviceMemory, device_index, requested_bytes_to_purge, &mut bytes_purged)?;
        Ok(bytes_purged)
    }
    /// All host (pinned) memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemHostAlloc or malloc.
    pub fn allocate_host_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, AllocateHostMemory, device_index, size_in_bytes, &mut ptr)?;
        Ok(ptr)
    }
    pub fn free_host_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, FreeHostMemory, device_index, ptr)?;
        Ok(())
    }
    pub fn purge_host_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        let mut bytes_purged = 0;
        pr_call_suite_fn!(self.suite_ptr, PurgeHostMemory, device_index, requested_bytes_to_purge, &mut bytes_purged)?;
        Ok(bytes_purged)
    }
    /// Information on a GPU ppix. The following ppix functions may also be used:
    /// - PPixSuite::Dispose
    /// - PPixSuite::GetBounds
    /// - PPixSuite::GetRowBytes
    /// - PPixSuite::GetPixelAspectRatio
    /// - PPixSuite::GetPixelFormat
    /// - PPix2Suite::GetFieldOrder
    pub fn create_gpu_ppix(&self, device_index: u32, pixel_format: PixelFormat, width: i32, height: i32, par_numerator: i32, par_denominator: i32, field_type: pr_sys::prFieldType) -> Result<pr_sys::PPixHand, Error> {
        let mut ptr: pr_sys::PPixHand = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, CreateGPUPPix, device_index, pixel_format.into(), width, height, par_numerator, par_denominator, field_type.into(), &mut ptr)?;
        Ok(ptr)
    }
    pub fn get_gpu_ppix_data(&self, ppix_handle: pr_sys::PPixHand) -> Result<*mut std::ffi::c_void, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixData, ppix_handle, &mut ptr)?;
        Ok(ptr)
    }
    pub fn get_gpu_ppix_device_index(&self, ppix_handle: pr_sys::PPixHand) -> Result<u32, Error> {
        let mut index = 0;
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixDeviceIndex, ppix_handle, &mut index)?;
        Ok(index)
    }
    pub fn get_gpu_ppix_size(&self, ppix_handle: pr_sys::PPixHand) -> Result<usize, Error> {
        let mut index = 0;
        pr_call_suite_fn!(self.suite_ptr, GetGPUPPixSize, ppix_handle, &mut index)?;
        Ok(index)
    }
}

define_suite!(GPUImageProcessingSuite, PrSDKGPUImageProcessingSuite, kPrSDKGPUImageProcessingSuite, kPrSDKGPUImageProcessingSuiteVersion);
impl GPUImageProcessingSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Convert between formats on the GPU
    /// One of inSrcPixelFormat or inDestPixelFormat must be a host format,
    /// the other must be either PrPixelFormat_GPU_BGRA_4444_16f or PrPixelFormat_GPU_BGRA_4444_32f
    pub fn pixel_format_convert(&self, device_index: u32, src: *const std::ffi::c_void, src_stride: i32, src_format: PixelFormat,
                                                          dst: *mut   std::ffi::c_void, dst_stride: i32, dst_format: PixelFormat,
                                                          width: u32, height: u32, quality: RenderQuality) -> Result<(), Error> {

        pr_call_suite_fn!(self.suite_ptr, PixelFormatConvert, device_index, src, src_stride, src_format.into(), dst, dst_stride, dst_format.into(), width, height, quality.into())?;
        Ok(())
    }

    /// Scale a frame on the GPU
    /// inPixelFormat must be PrPixelFormat_GPU_BGRA_4444_16f or PrPixelFormat_GPU_BGRA_4444_32f
    pub fn scale(&self, device_index: u32, src: *const std::ffi::c_void, src_stride: i32, src_width: u32, src_height: u32,
                                           dst: *mut   std::ffi::c_void, dst_stride: i32, dst_width: u32, dst_height: u32,
                                           format: PixelFormat, scale_x: f32, scale_y: f32, quality: RenderQuality) -> Result<(), Error> {

        pr_call_suite_fn!(self.suite_ptr, Scale, device_index, src, src_stride, src_width, src_height, dst, dst_stride, dst_width, dst_height, format.into(), scale_x, scale_y, quality.into())?;
        Ok(())
    }

    /// Gaussian blur on the GPU
    /// inPixelFormat must be PrPixelFormat_GPU_BGRA_4444_16f or PrPixelFormat_GPU_BGRA_4444_32f
    pub fn gaussian_blur(&self, device_index: u32, src: *const std::ffi::c_void, src_stride: i32, src_width: u32, src_height: u32,
                                                   dst: *mut   std::ffi::c_void, dst_stride: i32, dst_width: u32, dst_height: u32,
                                                   format: PixelFormat, sigma_x: f32, sigma_y: f32,
                                                   repeat_edge_pixels: bool,
                                                   blur_horizontally: bool,
                                                   blur_vertically: bool,
                                                   quality: RenderQuality) -> Result<(), Error> {

        pr_call_suite_fn!(self.suite_ptr, GaussianBlur, device_index, src, src_stride, src_width, src_height, dst, dst_stride, dst_width, dst_height, format.into(), sigma_x, sigma_y, if repeat_edge_pixels { 1 } else { 0 }, if blur_horizontally { 1 } else { 0 }, if blur_vertically { 1 } else { 0 }, quality.into())?;
        Ok(())
    }
}

define_suite!(MemoryManagerSuite, PrSDKMemoryManagerSuite, kPrSDKMemoryManagerSuite, kPrSDKMemoryManagerSuiteVersion);
impl MemoryManagerSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// Set the memory reserve size in bytes for the plugin with the specified ID.
    /// @param  inPluginID  The ID of the plugin.
    /// @param  inSize      The size in bytes to reserve.
    pub fn reserve_memory(&self, plugin_id: u32, size: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReserveMemory, plugin_id, size)?;
        Ok(())
    }
}

define_suite!(PPixSuite, PrSDKPPixSuite, kPrSDKPPixSuite, kPrSDKPPixSuiteVersion);
impl PPixSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// This will free this ppix. The ppix is no longer valid after this function is called.
    /// @param  inPPixHand    The ppix handle you want to dispose.
    pub fn dispose(&self, ppix_handle: pr_sys::PPixHand) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, Dispose, ppix_handle)?;
        Ok(())
    }
    /// This will return a pointer to the pixel buffer.
    /// @param  inPPixHand         The ppix handle you want to operate on.
    /// @param  inRequestedAccess  Requested pixel access. Most PPixs do not support write access modes.
    /// @param  outPixelAddress    The output pixel buffer address. May be NULL if the requested pixel access is not supported.
    pub fn get_pixels(&self, ppix_handle: pr_sys::PPixHand, requested_access: PPixBufferAccess) -> Result<*mut std::ffi::c_char, Error> {
        let mut ptr = std::ptr::null_mut();
        pr_call_suite_fn!(self.suite_ptr, GetPixels, ppix_handle, requested_access.into(), &mut ptr)?;
        Ok(ptr)
    }
    /// This will return the bounding rect.
    /// @param  inPPixHand       The ppix handle you want to operate on.
    /// @param  outBoundingRect  The address of a bounding rect to be filled in.
    pub fn get_bounds(&self, ppix_handle: pr_sys::PPixHand) -> Result<pr_sys::prRect, Error> {
        let mut rect: pr_sys::prRect = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetBounds, ppix_handle, &mut rect)?;
        Ok(rect)
    }
    /// This will return the row bytes of the ppix.
    /// @param  inPPixHand    The ppix handle you want to operate on.
    /// @param  outRowBytes   Returns how many bytes must be added to the pixel buffer address to get to the next line.
    /// May be negative.
    pub fn get_row_bytes(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetRowBytes, ppix_handle, &mut val)?;
        Ok(val)
    }
    /// This will return the pixel aspect ratio of this ppix.
    /// @param  inPPixHand                      The ppix handle you want to operate on.
    /// @param  outPixelAspectRatioNumerator    Returns the numerator of the pixel aspect ratio.
    /// @param  outPixelAspectRatioDenominator  Returns the denominator of the pixel aspect ratio.
    pub fn get_pixel_aspect_ratio(&self, ppix_handle: pr_sys::PPixHand) -> Result<(u32, u32), Error> {
        let mut num = 0;
        let mut den = 0;
        pr_call_suite_fn!(self.suite_ptr, GetPixelAspectRatio, ppix_handle, &mut num, &mut den)?;
        Ok((num, den))
    }
    /// This will return the pixel format of this ppix.
    /// @param  inPPixHand      The ppix handle you want to operate on.
    /// @param  outPixelFormat  Returns the pixel format of this ppix.
    pub fn get_pixel_format(&self, ppix_handle: pr_sys::PPixHand) -> Result<PixelFormat, Error> {
        let mut val: pr_sys::PrPixelFormat = 0;
        pr_call_suite_fn!(self.suite_ptr, GetPixelFormat, ppix_handle, &mut val)?;
        Ok(PixelFormat::from(val))
    }
    /// This will return the unique key for this ppix.
    /// @param  inPPixHand    The ppix handle you want to operate on.
    /// @param  outKeyBuffer  Returns the pixel format of this ppix.
    /// [TODO] Fill in returned error codes.
    /// @returns Error if the buffer size is too small (call GetUniqueKeySize() to get the correct size).
    /// @returns Error if the key is not available.
    /// @returns Success if the key data was filled in.
    pub fn get_unique_key(&self, ppix_handle: pr_sys::PPixHand) -> Result<Vec<u8>, Error> {
        let mut size = 0;
        pr_call_suite_fn!(self.suite_ptr, GetUniqueKeySize, &mut size)?;
        let mut buffer = vec![0; size];
        pr_call_suite_fn!(self.suite_ptr, GetUniqueKey, ppix_handle, buffer.as_mut_ptr(), size)?;
        Ok(buffer)
    }
    /// This will return the render time for this ppix.
    /// @param  inPPixHand             The ppix handle you want to operate on.
    /// @param  outRenderMilliseconds  Returns the render time in milliseconds. If the frame was cached, this time will be 0.
    pub fn get_render_time(&self, ppix_handle: pr_sys::PPixHand) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetRenderTime, ppix_handle, &mut val)?;
        Ok(val)
    }
}

define_suite!(PPix2Suite, PrSDKPPix2Suite, kPrSDKPPix2Suite, kPrSDKPPix2SuiteVersion);
impl PPix2Suite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// This will return the total size of the ppix in bytes.
    /// @param  inPPixHand    The ppix handle you want to operate on.
    /// @param  outSize       The size of the ppix in bytes.
    pub fn get_size(&self, ppix_handle: pr_sys::PPixHand) -> Result<usize, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetSize, ppix_handle, &mut val)?;
        Ok(val)
    }
    /// [Added in CS4]
    /// This will return the planar buffers and rowbytes for a PPixHand
    /// if the contained pixels are in a planar format, such as
    /// PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_601
    /// PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_601
    /// PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_709
    /// PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_709
    /// @param  inPPixHand            The ppix handle you want to operate on.
    /// @param  inRequestedAccess     Will return an error if the source is read-only and the request is for write or read/write.
    /// @param  out_xxx_PixelAddress  The output (Y, U, or V) pixel buffer address. May be NULL if the requested access is not supported.
    /// @param  out_xxx_RowBytes      Returns how many bytes must be added to the pixel buffer address to get to the next line.
    /// May be negative.
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

define_suite!(VideoSegmentSuite, PrSDKVideoSegmentSuite, kPrSDKVideoSegmentSuite, kPrSDKVideoSegmentSuiteVersion);
impl VideoSegmentSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// From a sequence, get an ID to its video segments ID. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID         The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID   Receives the ID for the Video Segments
    pub fn acquire_video_segments_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsID, timeline_data, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID         The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID   Receives the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_previews_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsWithPreviewsID, timeline_data, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted, but only previews
    /// for sections that are opaque. This is appropriate for use when using previews for nested sequences. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID        The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID  Recevies the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_opaque_previews_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsWithOpaquePreviewsID, timeline_data, &mut val)?;
        Ok(val)
    }

    /// Release a Video Segments ID
    /// @param  inVideoSegmentsID   The Video Segments ID to release
    pub fn release_video_segments_id(&self, video_segments_id: i32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReleaseVideoSegmentsID, video_segments_id)?;
        Ok(())
    }
    /// Get the hash of a Video Segments object
    /// @param  inVideoSegmentsID   The Video Segments ID
    /// @param  outHash             The GUID hash of the segments
    pub fn get_hash(&self, video_segments_id: i32) -> Result<pr_sys::prPluginID, Error> {
        let mut val: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetHash, video_segments_id, &mut val)?;
        Ok(val)
    }

    /// Get the number of segments in the segments object
    /// @param  inVideoSegmentsID  The Video Segments ID
    /// @param  outNumSegments     The number of segments
    pub fn get_segment_count(&self, video_segments_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetSegmentCount, video_segments_id, &mut val)?;
        Ok(val)
    }

    /// Get the details of the Nth Node.
    /// @param  inVideoSegmentsID  The Video Segments ID
    /// @param  inIndex            Which segment?
    /// @param  outStartTime       The start time of the segment
    /// @param  outEndTime         The end time of the segment
    /// @param  outSegmentOffset   The offset value for the segment
    /// @param  outHash            The hash for the segment
    pub fn get_segment_info(&self, video_segments_id: i32, index: i32) -> Result<(i64, i64, i64, pr_sys::prPluginID), Error> {
        let mut start_time = 0;
        let mut end_time = 0;
        let mut segment_offset = 0;
        let mut hash: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetSegmentInfo, video_segments_id, index, &mut start_time, &mut end_time, &mut segment_offset, &mut hash)?;
        Ok((start_time, end_time, segment_offset, hash))
    }

    /// Get a segment node. This object is ref-counted and must be released.
    /// @param  inVideoSegmentsID  The Video Segments ID
    /// @param  inHash             The hash for the segment
    /// @param  outVideoNodeID     The video node ID.
    pub fn acquire_node_id(&self, video_segments_id: i32, hash: *mut pr_sys::prPluginID) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireNodeID, video_segments_id, hash, &mut val)?;
        Ok(val)
    }

    /// Release a Video Node ID
    /// @param  inVideoNodeID  The Video Node ID to release
    pub fn release_video_node_id(&self, video_node_id: i32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReleaseVideoNodeID, video_node_id)?;
        Ok(())
    }
    /// Get details about a node.
    /// @param  inVideoNodeID   The Video Node ID
    /// @param  outNodeType     A string of size kMaxNodeTypeStringSize holding the node type
    /// @param  outHash         The hash for the node (may be different than the hash used to get the node)
    /// @param  outInfoFlags    The flags for this node (see enum above)
    pub fn get_node_info(&self, video_node_id: i32) -> Result<(String, pr_sys::prPluginID, i32), Error> {
        let mut node_type = [0; pr_sys::kMaxNodeTypeStringSize as usize];
        let mut hash: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        let mut flags = 0;
        pr_call_suite_fn!(self.suite_ptr, GetNodeInfo, video_node_id, node_type.as_mut_ptr() as *mut i8, &mut hash, &mut flags)?;
        Ok((String::from_utf8_lossy(&node_type).to_string(), hash, flags))
    }

    /// Get the number of inputs on the node object
    /// @param  inVideoNodeID  The Video Node ID
    /// @param  outNumInputs   The number of inputs
    pub fn get_node_input_count(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetNodeInputCount, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get a segment node that is an input to another node. This object is ref-counted and must be released.
    /// @param  inVideoNodeID         The Video Node ID
    /// @param  inIndex               The index of the input
    /// @param  outOffset             The time offset relative to it's parent node
    /// @param  outInputVideoNodeID   The video node ID of the input node.
    pub fn acquire_input_node_id(&self, video_node_id: i32, index: i32) -> Result<(i64, i32), Error> {
        let mut offset = 0;
        let mut input_video_node_id = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireInputNodeID, video_node_id, index, &mut offset, &mut input_video_node_id)?;
        Ok((offset, input_video_node_id))
    }

    /// Get the number of inputs on the node object
    /// @param  inVideoNodeID      The Video Node ID
    /// @param  outNumOperators    The number of operators
    pub fn get_node_operator_count(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetNodeOperatorCount, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get a segment node that is an operator on another node. This object is ref-counted and must be released.
    /// @param  inVideoNodeID            The Video Node ID
    /// @param  inIndex                  The index of the operator
    /// @param  outOperatorVideoNodeID   The video node ID of the input node.
    pub fn acquire_operator_node_id(&self, video_node_id: i32, index: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireOperatorNodeID, video_node_id, index, &mut val)?;
        Ok(val)
    }

    /// Iterate all of the properties on a node.
    /// @param  inVideoNodeID    The Video Node ID
    /// @param  inCallback       The callback function to return the properties
    /// @param  inPluginObject   The plugin object returned in the callback.
    pub fn iterate_node_properties<F: Fn(&str, &str) + Send + Sync + 'static>(&self, video_node_id: i32, callback: F) -> Result<(), Error> {
        use std::sync::OnceLock;
        use std::collections::HashMap;
        use parking_lot::RwLock;
        static MAP: OnceLock<RwLock<HashMap<i32, Box<dyn Fn(&str, &str) + Send + Sync + 'static>>>> = OnceLock::new();

        let map = MAP.get_or_init(|| RwLock::new(HashMap::new()));

        unsafe extern "C" fn cb(plugin_object: pr_sys::csSDK_int32, in_key: *const std::ffi::c_char, in_value: *const pr_sys::prUTF8Char) -> pr_sys::prSuiteError {
            let key = std::ffi::CStr::from_ptr(in_key as *const _).to_str().unwrap();
            let value = std::ffi::CStr::from_ptr(in_value as *const _).to_str().unwrap();

            if let Some(callback) = MAP.get().unwrap().read().get(&plugin_object) {
                callback(key, value);
            }

            pr_sys::suiteError_NoError
        }

        let id = fastrand::i32(..);
        map.write().insert(id, Box::new(callback));

        pr_call_suite_fn!(self.suite_ptr, IterateNodeProperties, video_node_id, Some(cb), id)?;

        map.write().remove(&id);
        Ok(())
    }

    /// Get the value of a single property on a node
    /// @param  inVideoNodeID   The Video Node ID
    /// @param  inKey           The key of the property
    /// @param  outValue        A string holding the value. This UTF8 string is allocated using PrNewPtr, and ownership is transferred to the plugin and must be disposed by the plugin.
    pub fn get_node_property(&self, video_node_id: i32, key: &str) -> Result<String, Error> {
        let mut ptr: pr_sys::PrMemoryPtr = std::ptr::null_mut();

        let key_c = std::ffi::CString::new(key).unwrap();
        let key_c = key_c.as_bytes_with_nul();

        pr_call_suite_fn!(self.suite_ptr, GetNodeProperty, video_node_id, key_c.as_ptr() as *const _, &mut ptr)?;
        let value = unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_string() };

        // TODO: dispose memory
        Ok(value)
    }

    /// Get the number of params
    /// @param  inVideoNodeID   The Video Node ID
    /// @param  outParamCount   The number of params
    pub fn get_param_count(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetParamCount, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get a specific param value at a specific time
    /// @param  inVideoNodeID  The Video Node ID
    /// @param  inIndex        The index of the param
    /// @param  inTime         The time requested (in Media time)
    /// @param  outParam       The param
    pub fn get_param(&self, video_node_id: i32, index: i32, time: i64) -> Result<pr_sys::PrParam, Error> {
        let mut val: pr_sys::PrParam = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetParam, video_node_id, index, time, &mut val)?;
        Ok(val)
    }

    /// Get the next keyframe time after the specified time.
    /// Example: Keyframes at 0 and 10
    /// inTime = -1, outKeyframeTime = 0
    /// inTime = 0, outKeyframeTime = 10
    /// inTime = 9, outKeyframeTime = 10
    /// inTime = 10, returns suiteError_NoKeyframeAfterInTime
    /// @param  inVideoNodeID                 The Video Node ID
    /// @param  inIndex                       The index of the param
    /// @param  inTime                        The lower bound time
    /// @param  outKeyframeTime               The time of the next keyframe > inTime
    /// @param  outKeyframeInterpolationMode  The temporal interpolation mode of the keyframe, see the enum for PrKeyframeInterpolationModeFlag above
    pub fn get_next_keyframe_time(&self, video_node_id: i32, index: i32, time: i64) -> Result<(i64, i32), Error> {
        let mut keyframe_time = 0;
        let mut keyframe_interpolation_mode = 0;
        pr_call_suite_fn!(self.suite_ptr, GetNextKeyframeTime, video_node_id, index, time, &mut keyframe_time, &mut keyframe_interpolation_mode)?;
        Ok((keyframe_time, keyframe_interpolation_mode))
    }

    /// Transform a node local time into a time appropriate for node inputs and
    /// operators. When used on the clip node, for instance, this will take into
    /// account speed change, reverse, time remapping and return a time value
    /// which can be used in the Media and Effect nodes.
    /// If the node does not have a time transform, function will not fail but
    /// will return in input time in the output.
    pub fn transform_node_time(&self, video_node_id: i32, time: i64) -> Result<i64, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, TransformNodeTime, video_node_id, time, &mut val)?;
        Ok(val)
    }

    /// Retrieve general properties of a sequence (video segments). I.e. width/height, par and framerate.
    /// @param  inTimelineID                    The plugin timeline ID for the sequence
    /// @param  outWidth                        Receives width of the sequence
    /// @param  outHeight                       Receives height of the sequence
    /// @param  outPixelAspectRatioNumerator    Receives the pixel aspect ratio numerator of the sequence
    /// @param  outPixelAspectRatioDenominator  Receives the pixel aspect ratio denominator of the sequence
    /// @param  outFrameRateNumerator           Receives the frame rate numerator of the sequence
    /// @param  outFrameRateDenominator         Receives the frame rate denominator of the sequence
    pub fn get_video_segments_properties(&self, timeline_data: pr_sys::PrTimelineID) -> Result<VideoSegmentProperties, Error> {
        let mut p: VideoSegmentProperties = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetVideoSegmentsProperties, timeline_data, &mut p.bounds, &mut p.par_num, &mut p.par_den, &mut p.frame_rate, &mut p.field_type)?;
        Ok(p)
    }
    /// From a sequence, get a segment node for a requested time. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inVideoSegmentsID   The Video Segments ID
    /// @param  inTime              Requested segment time
    /// @param  outVideoNodeID      The video node ID
    /// @param  outSegmentOffset    Offset of retrieved segment
    pub fn acquire_node_for_time(&self, video_segments_id: i32, time: i64) -> Result<(i32, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_offset = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireNodeForTime, video_segments_id, time, &mut video_node_id, &mut segment_offset)?;
        Ok((video_node_id, segment_offset))
    }

    /// From a sequence, get an ID to its video segments ID. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID         The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID   Receives the ID for the Video Segments
    pub fn acquire_video_segments_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID         The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID   Receives the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_previews_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsWithPreviewsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted, but only previews
    /// for sections that are opaque. This is appropriate for use when using previews for nested sequences. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// @param  inTimelineID         The plugin timeline ID for the sequence
    /// @param  outVideoSegmentsID   Recevies the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_opaque_previews_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        pr_call_suite_fn!(self.suite_ptr, AcquireVideoSegmentsWithOpaquePreviewsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get the first segment node that intersects with a range of times.\n  This is a ref-counted object, and must be released when no longer needed.
    /// @param  inVideoSegmentsID    The Video Segments ID
    /// @param  inStartTime          The start of the requested segment time range
    /// @param  inEndTime            The end of the requested segment time range
    /// @param  outVideoNodeID       The video node ID
    /// @param  outSegmentOffset     Offset of retrieved segment
    pub fn acquire_first_node_in_time_range(&self, video_segments_id: i32, start_time: i64, end_time: i64) -> Result<(i32, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_offset = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireFirstNodeInTimeRange, video_segments_id, start_time, end_time, &mut video_node_id, &mut segment_offset)?;
        Ok((video_node_id, segment_offset))
    }

    /// Acquire the node owning an operator
    /// @param  inVideoNodeID       The operator
    /// @param  outOwnerNodeID      The owner
    pub fn acquire_operator_owner_node_id(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireOperatorOwnerNodeID, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get graphic layer params at a specific time
    /// @param  inVideoNodeID      The Video Node ID
    /// @param  inTime             The time requested (in Media time)
    pub fn get_graphics_transformed_params(&self, video_node_id: i32, time: i64) -> Result<(pr_sys::prFPoint64, pr_sys::prFPoint64, pr_sys::prFPoint64, f32), Error> {
        let mut position = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut anchor = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut scale = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut rotation = 0.0;
        pr_call_suite_fn!(self.suite_ptr, GetGraphicsTransformedParams, video_node_id, time, &mut position, &mut anchor, &mut scale, &mut rotation)?;
        Ok((position, anchor, scale, rotation))
    }

    /// Get graphic layer group ID
    /// @param  inVideoNodeID   The Video Node ID
    pub fn has_graphics_group(&self, video_node_id: i32) -> Result<bool, Error> {
        let mut val: bool = false;
        pr_call_suite_fn!(self.suite_ptr, HasGraphicsGroup, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get graphic layer group ID
    /// @param  inVideoNodeID    The Video Node ID
    pub fn get_graphics_group_id(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        pr_call_suite_fn!(self.suite_ptr, GetGraphicsGroupID, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Color managed version of GetVideoSegmentsProperties
    /// Retrieve general properties of a sequence (video segments). I.e. width/height, par and framerate and color space
    /// @param  inTimelineID                     The plugin timeline ID for the sequence
    /// @param  outWidth                         Receives width of the sequence
    /// @param  outHeight                        Receives height of the sequence
    /// @param  outPixelAspectRatioNumerator     Receives the pixel aspect ratio numerator of the sequence
    /// @param  outPixelAspectRatioDenominator   Receives the pixel aspect ratio denominator of the sequence
    /// @param  outFrameRateNumerator            Receives the frame rate numerator of the sequence
    /// @param  outFrameRateDenominator          Receives the frame rate denominator of the sequence
    /// @param  outColorSpaceID                   Receives the opaque ID of the sequence's working color space
    pub fn get_video_segments_properties_ext(&self, timeline_data: pr_sys::PrTimelineID) -> Result<VideoSegmentProperties, Error> {
        let mut p: VideoSegmentProperties = unsafe { std::mem::zeroed() };
        let mut color_space: pr_sys::PrSDKColorSpaceID = unsafe { std::mem::zeroed() };
        pr_call_suite_fn!(self.suite_ptr, GetVideoSegmentsPropertiesExt, timeline_data, &mut p.bounds, &mut p.par_num, &mut p.par_den, &mut p.frame_rate, &mut p.field_type, &mut color_space)?;
        p.color_space = Some(color_space);
        Ok(p)
    }

    /// From a sequence, get the first segment node that intersects with a range of times.
    /// This is a ref-counted object, and must be released when no longer needed.
    /// @param  inVideoSegmentsID       The Video Segments ID
    /// @param  inStartTime             The start of the requested segment time range
    /// @param  inEndTime               The end of the requested segment time range
    /// @param  outVideoNodeID          The video node ID
    /// @param  outSegmentStartTime     Start time of retrieved segment
    /// @param  outSegmentEndTime       End time of retrieved segment
    /// @param  outSegmentOffset        Offset of retrieved segment
    pub fn acquire_first_node_in_time_range_ext(&self, video_segments_id: i32, start_time: i64, end_time: i64) -> Result<(i32, i64, i64, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_start_time = 0;
        let mut segment_end_time = 0;
        let mut segment_offset = 0;
        pr_call_suite_fn!(self.suite_ptr, AcquireFirstNodeInTimeRangeExt, video_segments_id, start_time, end_time, &mut video_node_id, &mut segment_start_time, &mut segment_end_time, &mut segment_offset)?;
        Ok((video_node_id, segment_start_time, segment_end_time, segment_offset))
    }

    /// Returns the relative time rate of a node at a given point in time.
    /// Node time rate varies with e.g. time remapping but not the playback speed of the sequence
    /// Can be thought of as the instantaneous rate of change of TransformNodeTime()
    /// @param  inVideoNodeID   The Video Node ID
    /// @param  inTime          The time requested (in Media time - untransformed)
    /// @param  outRate         The node rate relative to the containing sequence
    pub fn get_node_time_scale(&self, video_node_id: i32, time: i64) -> Result<f64, Error> {
        let mut val = 0.0;
        pr_call_suite_fn!(self.suite_ptr, GetNodeTimeScale, video_node_id, time, &mut val)?;
        Ok(val)
    }
}


#[derive(Clone)]
pub struct RenderParams {
    ptr: *const pr_sys::PrGPUFilterRenderParams,
}
impl RenderParams {
    pub fn from_raw(ptr: *const pr_sys::PrGPUFilterRenderParams) -> Self {
        Self {
            ptr
        }
    }
    pub fn clip_time(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inClipTime }
    }
    pub fn sequence_time(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inSequenceTime }
    }
    pub fn quality(&self) -> RenderQuality {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inQuality.into() }
    }
    pub fn downsample_factor(&self) -> (f32, f32) {
        assert!(!self.ptr.is_null());
        unsafe { ((*self.ptr).inDownsampleFactorX, (*self.ptr).inDownsampleFactorY) }
    }
    pub fn render_width(&self) -> u32 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderWidth }
    }
    pub fn render_height(&self) -> u32 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderHeight }
    }
    pub fn render_pixel_aspect_ratio(&self) -> (u32, u32)  {
        assert!(!self.ptr.is_null());
        unsafe { ((*self.ptr).inRenderPARNum, (*self.ptr).inRenderPARDen) }
    }
    pub fn render_field_type(&self) -> pr_sys::prFieldType {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderFieldType }
    }
    pub fn render_ticks_per_frame(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderTicksPerFrame }
    }
    pub fn render_field(&self) -> FieldDisplay {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderField.into() }
    }

}

pub struct GpuFilterData {
    pub instance_ptr: *mut pr_sys::PrGPUFilterInstance,
    pub gpu_device_suite: GPUDeviceSuite,
    pub gpu_image_processing_suite: GPUImageProcessingSuite,
    pub memory_manager_suite: MemoryManagerSuite,
    pub ppix_suite: PPixSuite,
    pub ppix2_suite: PPix2Suite,
    pub video_segment_suite: VideoSegmentSuite,
    pub gpu_info: pr_sys::PrGPUDeviceInfo,
}
impl GpuFilterData {
    pub fn timeline_id(&self) -> pr_sys::PrTimelineID {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inTimelineID }
    }
    pub fn node_id(&self) -> i32 {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inNodeID as i32 }
    }
    pub fn device_index(&self) -> u32 {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inDeviceIndex as u32 }
    }
}

pub trait GpuFilter : Default {
	/// Return dependency information about a render, or nothing if
    /// only the current frame is required.
    fn get_frame_dependencies(&self, filter: &GpuFilterData, render_params: RenderParams, query_index: &mut i32) -> Result<pr_sys::PrGPUFilterFrameDependency, Error>;

	/// Precompute a result into preallocated uninitialized host (pinned) memory.
	/// Will only be called if PrGPUDependency_Precompute was returned from GetFrameDependencies.
	/// Precomputation may be called ahead of render time. Results will be
	/// uploaded to the GPU by the host. If outPrecomputePixelFormat is not custom,
	/// frames will be converted to the GPU pixel format.
    fn precompute(&self, filter: &GpuFilterData, render_params: RenderParams, index: i32, frame: pr_sys::PPixHand) -> Result<(), Error>;

    /// Render into an allocated outFrame allocated with PrSDKGPUDeviceSuite or operate
    /// in place. Result must be in the same pixel format as the input. For effects, frame 0
    /// will always be the frame at the current time, other input frames will be in the same order as
    /// returned from GetFrameDependencies. For transitions frame 0 will be the incoming frame and
    /// frame 1 the outgoing frame. Transitions may not have other frame dependencies.
    fn render(&self, filter: &GpuFilterData, render_params: RenderParams, frames: *const pr_sys::PPixHand, frame_count: usize, out_frame: *mut pr_sys::PPixHand) -> Result<(), Error>;
}

pub struct GpuFilterInstance<T: GpuFilter> {
    pub data: GpuFilterData,
    pub instance: T,
}

#[macro_export]
macro_rules! define_gpu_filter {
    ($struct_name:ty) => {
        use $crate::GpuFilter;

        unsafe extern "C" fn gpu_filter_create_instance(instance_data: *mut pr_sys::PrGPUFilterInstance) -> pr_sys::prSuiteError {
            assert!(!instance_data.is_null());

            let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
            log::set_max_level(log::LevelFilter::Debug);
            log_panics::init();

            log::info!("xGPUFilterEntry: gpu_filter_create_instance");

            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let sp_basic_suite = (util_funcs.getSPBasicSuite.unwrap())();

            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw(sp_basic_suite);

            let result = (|| -> Result<Box<$crate::GpuFilterInstance<$struct_name>>, $crate::Error> {
                let gpu_suite = $crate::GPUDeviceSuite::new()?;
                let gpu_info = gpu_suite.get_device_info((*instance_data).inDeviceIndex)?;
                Ok(Box::new($crate::GpuFilterInstance {
                    data: $crate::GpuFilterData {
                        instance_ptr: instance_data,
                        gpu_device_suite:           gpu_suite,
                        gpu_image_processing_suite: $crate::GPUImageProcessingSuite::new()?,
                        memory_manager_suite:       $crate::MemoryManagerSuite::new()?,
                        ppix_suite:                 $crate::PPixSuite::new()?,
                        ppix2_suite:                $crate::PPix2Suite::new()?,
                        video_segment_suite:        $crate::VideoSegmentSuite::new()?,
                        gpu_info
                    },
                    instance: <$struct_name>::default(),
                }))
            })();

            match result {
                Ok(instance) => {
                    (*instance_data).ioPrivatePluginData = Box::into_raw(instance) as *mut _;
                    pr_sys::suiteError_NoError
                }
                Err(e) => {
                    log::error!("Failed to create GPU filter instance: {e:?}");
                    e as pr_sys::prSuiteError
                }
            }
        }

        unsafe extern "C" fn gpu_filter_dispose_instance(instance_data: *mut pr_sys::PrGPUFilterInstance) -> pr_sys::prSuiteError {
            let _ = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            log::info!("xGPUFilterEntry: gpu_filter_dispose_instance");

            (*instance_data).ioPrivatePluginData = std::ptr::null_mut();

            pr_sys::suiteError_NoError
        }

        unsafe extern "C" fn gpu_filter_get_frame_dependencies(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            io_query_index: *mut pr_sys::csSDK_int32,
            out_frame_dependencies: *mut pr_sys::PrGPUFilterFrameDependency,
        ) -> pr_sys::prSuiteError {
            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.get_frame_dependencies(&instance.data, render_params, &mut *io_query_index);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(dep) => {
                    *out_frame_dependencies = dep;
                    pr_sys::suiteError_NoError
                },
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_precompute(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            index: pr_sys::csSDK_int32,
            frame: pr_sys::PPixHand,
        ) -> pr_sys::prSuiteError {
            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.precompute(&instance.data, render_params, index, frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => pr_sys::suiteError_NoError,
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_render(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            frames: *const pr_sys::PPixHand,
            frame_count: pr_sys::csSDK_size_t,
            out_frame: *mut pr_sys::PPixHand,
        ) -> pr_sys::prSuiteError {
            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.render(&instance.data, render_params, frames, frame_count as usize, out_frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => pr_sys::suiteError_NoError,
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn xGPUFilterEntry(
            host_interface_version: pr_sys::csSDK_uint32,
            io_index: *mut pr_sys::csSDK_int32,
            is_startup: pr_sys::prBool,
            pi_suites: pr_sys::piSuitesPtr,
            out_filter: *mut pr_sys::PrGPUFilter,
            out_filter_info: *mut pr_sys::PrGPUFilterInfo,
        ) -> pr_sys::prSuiteError {

            let util_funcs = (*(*pi_suites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
            log::set_max_level(log::LevelFilter::Debug);
            log_panics::init();

            log::info!("xGPUFilterEntry: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", host_interface_version, io_index, is_startup, pi_suites, out_filter, out_filter_info);

            if is_startup == 1 {
                (*out_filter).CreateInstance       = Some(gpu_filter_create_instance);
                (*out_filter).DisposeInstance      = Some(gpu_filter_dispose_instance);
                (*out_filter).GetFrameDependencies = Some(gpu_filter_get_frame_dependencies);
                (*out_filter).Precompute           = Some(gpu_filter_precompute);
                (*out_filter).Render               = Some(gpu_filter_render);

                let plugin_count = 1;

                let index = *io_index;
                if index + 1 > plugin_count {
                    return pr_sys::suiteError_InvalidParms;
                }
                if index + 1 < plugin_count {
                    *io_index += 1;
                }

                // let match_name = pr_sys::PrSDKString::default();
                (*out_filter_info).outMatchName = unsafe { std::mem::zeroed() };
                (*out_filter_info).outInterfaceVersion = pr_sys::PrSDKGPUFilterInterfaceVersion;
            } else {

            }
            pr_sys::suiteError_NoError
        }
    };
}
