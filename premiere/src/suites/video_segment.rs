use crate::*;

define_suite!(VideoSegmentSuite, PrSDKVideoSegmentSuite, kPrSDKVideoSegmentSuite, kPrSDKVideoSegmentSuiteVersion);

#[derive(Debug, Clone, Copy)]
pub struct VideoSegmentProperties {
    pub bounds: pr_sys::prRect,
    pub par_num: i32,
    pub par_den: i32,
    pub frame_rate: i64,
    pub field_type: pr_sys::prFieldType,
    pub color_space: Option<pr_sys::PrSDKColorSpaceID>,
}

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
