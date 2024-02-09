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
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments
    pub fn acquire_video_segments_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        call_suite_fn_single!(self, AcquireVideoSegmentsID -> i32, timeline_data)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_previews_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        call_suite_fn_single!(self, AcquireVideoSegmentsWithPreviewsID -> i32, timeline_data)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted, but only previews
    /// for sections that are opaque. This is appropriate for use when using previews for nested sequences. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_opaque_previews_id(&self, timeline_data: pr_sys::PrTimelineID) -> Result<i32, Error> {
        call_suite_fn_single!(self, AcquireVideoSegmentsWithOpaquePreviewsID -> i32, timeline_data)
    }

    /// Release a Video Segments ID
    /// * `video_segments_id` - The Video Segments ID to release
    pub fn release_video_segments_id(&self, video_segments_id: i32) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseVideoSegmentsID, video_segments_id)
    }
    /// Get the hash of a Video Segments object
    /// * `video_segments_id` - The Video Segments ID
    ///
    /// Returns the GUID hash of the segments
    pub fn get_hash(&self, video_segments_id: i32) -> Result<pr_sys::prPluginID, Error> {
        call_suite_fn_single!(self, GetHash -> pr_sys::prPluginID, video_segments_id)
    }

    /// Get the number of segments in the segments object
    /// * `video_segments_id` - The Video Segments ID
    ///
    /// Returns the number of segments
    pub fn get_segment_count(&self, video_segments_id: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetSegmentCount -> i32, video_segments_id)
    }

    /// Get the details of the Nth Node.
    /// * `video_segments_id` - The Video Segments ID
    /// * `inIndex` - Which segment?
    ///
    /// Returns a tuple containing:
    /// * `start_time` - The start time of the segment
    /// * `end_time` - The end time of the segment
    /// * `segment_offset` - The offset value for the segment
    /// * `hash` - The hash for the segment
    pub fn get_segment_info(&self, video_segments_id: i32, index: i32) -> Result<(i64, i64, i64, pr_sys::prPluginID), Error> {
        let mut start_time = 0;
        let mut end_time = 0;
        let mut segment_offset = 0;
        let mut hash: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetSegmentInfo, video_segments_id, index, &mut start_time, &mut end_time, &mut segment_offset, &mut hash)?;
        Ok((start_time, end_time, segment_offset, hash))
    }

    /// Get a segment node. This object is ref-counted and must be released.
    /// * `video_segments_id` - The Video Segments ID
    /// * `hash` - The hash for the segment
    ///
    /// Returns the video node ID.
    pub fn acquire_node_id(&self, video_segments_id: i32, hash: *mut pr_sys::prPluginID) -> Result<i32, Error> {
        call_suite_fn_single!(self, AcquireNodeID -> i32, video_segments_id, hash)
    }

    /// Release a Video Node ID
    /// * `video_node_id` - The Video Node ID to release
    pub fn release_video_node_id(&self, video_node_id: i32) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseVideoNodeID, video_node_id)
    }
    /// Get details about a node.
    /// * `video_node_id` - The Video Node ID
    ///
    /// Returns a tuple containing:
    /// * `node_type` - A string of size kMaxNodeTypeStringSize holding the node type
    /// * `hash` - The hash for the node (may be different than the hash used to get the node)
    /// * `info_flags` - The flags for this node (see enum above)
    pub fn get_node_info(&self, video_node_id: i32) -> Result<(String, pr_sys::prPluginID, i32), Error> {
        let mut node_type = [0; pr_sys::kMaxNodeTypeStringSize as usize];
        let mut hash: pr_sys::prPluginID = unsafe { std::mem::zeroed() };
        let mut flags = 0;
        call_suite_fn!(self, GetNodeInfo, video_node_id, node_type.as_mut_ptr() as *mut i8, &mut hash, &mut flags)?;
        Ok((String::from_utf8_lossy(&node_type).to_string(), hash, flags))
    }

    /// Get the number of inputs on the node object
    /// * `video_node_id` - The Video Node ID
    ///
    /// Returns the number of inputs
    pub fn get_node_input_count(&self, video_node_id: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetNodeInputCount -> i32, video_node_id)
    }

    /// Get a segment node that is an input to another node. This object is ref-counted and must be released.
    /// * `video_node_id` - The Video Node ID
    /// * `inIndex` - The index of the input
    ///
    /// Returns a tuple containing:
    /// * `input_video_node_id` - The video node ID of the input node.
    /// * `offset` - The time offset relative to it's parent node
    pub fn acquire_input_node_id(&self, video_node_id: i32, index: i32) -> Result<(i32, i64), Error> {
        let mut offset = 0;
        let mut input_video_node_id = 0;
        call_suite_fn!(self, AcquireInputNodeID, video_node_id, index, &mut offset, &mut input_video_node_id)?;
        Ok((input_video_node_id, offset))
    }

    /// Get the number of inputs on the node object
    /// * `video_node_id` - The Video Node ID
    ///
    /// Returns the number of operators
    pub fn get_node_operator_count(&self, video_node_id: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetNodeOperatorCount -> i32, video_node_id)
    }

    /// Get a segment node that is an operator on another node. This object is ref-counted and must be released.
    /// * `video_node_id` - The Video Node ID
    /// * `inIndex` - The index of the operator
    ///
    /// Returns the video node ID of the input node.
    pub fn acquire_operator_node_id(&self, video_node_id: i32, index: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, AcquireOperatorNodeID -> i32, video_node_id, index)
    }

    /// Iterate all of the properties on a node.
    /// * `video_node_id` - The Video Node ID
    /// * `callback` - The callback function to return the properties
    pub fn iterate_node_properties<F: Fn(Property, PropertyData) + Send + Sync + 'static>(&self, video_node_id: i32, callback: F) -> Result<(), Error> {
        use std::sync::OnceLock;
        use std::collections::HashMap;
        use parking_lot::RwLock;
        static MAP: OnceLock<RwLock<HashMap<i32, Box<dyn Fn(Property, PropertyData) + Send + Sync + 'static>>>> = OnceLock::new();

        let map = MAP.get_or_init(|| RwLock::new(HashMap::new()));

        unsafe extern "C" fn cb(plugin_object: pr_sys::csSDK_int32, in_key: *const std::ffi::c_char, in_value: *const pr_sys::prUTF8Char) -> pr_sys::prSuiteError {
            if let Some(callback) = MAP.get().unwrap().read().get(&plugin_object) {
                let key   = std::ffi::CStr::from_ptr(in_key   as *const _).to_str().unwrap();
                let value = std::ffi::CStr::from_ptr(in_value as *const _).to_str().unwrap();

                let key = Property::from_id(key.as_bytes());
                let value = key.parse_result(value);

                callback(key, value);
            }

            pr_sys::suiteError_NoError
        }

        let id = fastrand::i32(..).overflowing_add(video_node_id).0;
        map.write().insert(id, Box::new(callback));

        call_suite_fn!(self, IterateNodeProperties, video_node_id, Some(cb), id)?;

        map.write().remove(&id);
        Ok(())
    }

    /// Get the value of a single property on a node
    /// * `video_node_id` - The Video Node ID
    /// * `key` - The key of the property
    ///
    /// Returns the property value
    pub fn get_node_property(&self, video_node_id: i32, key: Property) -> Result<PropertyData, Error> {
        let mut ptr: pr_sys::PrMemoryPtr = std::ptr::null_mut();

        let key_bytes: &[u8] = key.as_id();

        call_suite_fn!(self, GetNodeProperty, video_node_id, key_bytes.as_ptr() as *const _, &mut ptr)?;
        let value = unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap() };

        let result = key.parse_result(value);

        match crate::MemoryManagerSuite::new() {
            Ok(mem) => mem.dispose_ptr(ptr),
            Err(e) => log::error!("Failed to dispose pointer in get_node_property. Failed to acquire memory suite: {e:?}")
        }

        Ok(result)
    }

    /// Get the number of params
    /// * `video_node_id` - The Video Node ID
    ///
    /// Returns the number of params
    pub fn get_param_count(&self, video_node_id: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetParamCount -> i32, video_node_id)
    }

    /// Get a specific param value at a specific time
    /// * `video_node_id` - The Video Node ID
    /// * `index` - The index of the param
    /// * `time` - The time requested (in Media time)
    ///
    /// Returns the param
    pub fn get_param(&self, video_node_id: i32, index: i32, time: i64) -> Result<crate::Param, Error> {
        Ok(call_suite_fn_single!(self, GetParam -> pr_sys::PrParam, video_node_id, index, time)?.into())
    }

    /// Get the next keyframe time after the specified time.
    /// Example: Keyframes at 0 and 10
    /// - `time` = -1, keyframe_time = 0
    /// - `time` = 0, keyframe_time = 10
    /// - `time` = 9, keyframe_time = 10
    /// - `time` = 10, returns [`Error::NoKeyframeAfterInTime`]
    ///
    /// Parameters:
    /// * `video_node_id` - The Video Node ID
    /// * `index` - The index of the param
    /// * `time` - The lower bound time
    ///
    /// Returns a tuple containing:
    /// * `keyframe_time` - The time of the next keyframe > inTime
    /// * `keyframe_interpolation_mode` - The temporal interpolation mode of the keyframe
    pub fn get_next_keyframe_time(&self, video_node_id: i32, index: i32, time: i64) -> Result<(i64, KeyframeInterpolationMode), Error> {
        let mut keyframe_time = 0;
        let mut keyframe_interpolation_mode: pr_sys::PrKeyframeInterpolationModeFlag = 0;
        call_suite_fn!(self, GetNextKeyframeTime, video_node_id, index, time, &mut keyframe_time, &mut keyframe_interpolation_mode)?;
        Ok((keyframe_time, keyframe_interpolation_mode.into()))
    }

    /// Transform a node local time into a time appropriate for node inputs and
    /// operators. When used on the clip node, for instance, this will take into
    /// account speed change, reverse, time remapping and return a time value
    /// which can be used in the Media and Effect nodes.
    /// If the node does not have a time transform, function will not fail but
    /// will return in input time in the output.
    pub fn transform_node_time(&self, video_node_id: i32, time: i64) -> Result<i64, Error> {
        call_suite_fn_single!(self, TransformNodeTime -> i64, video_node_id, time)
    }

    /// Retrieve general properties of a sequence (video segments). I.e. width/height, par and framerate.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns [`VideoSegmentProperties`] which contains:
    /// * `width` - Width of the sequence
    /// * `height` - Height of the sequence
    /// * `par_num` - Pixel aspect ratio numerator of the sequence
    /// * `par_den` - Pixel aspect ratio denominator of the sequence
    /// * `frame_rate` - Frame rate of the sequence
    /// * `field_type` - Field type of the sequence
    pub fn get_video_segments_properties(&self, timeline_data: pr_sys::PrTimelineID) -> Result<VideoSegmentProperties, Error> {
        let mut p: VideoSegmentProperties = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetVideoSegmentsProperties, timeline_data, &mut p.bounds, &mut p.par_num, &mut p.par_den, &mut p.frame_rate, &mut p.field_type)?;
        Ok(p)
    }
    /// From a sequence, get a segment node for a requested time. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `video_segments_id` - The Video Segments ID
    /// * `time` - Requested segment time
    ///
    /// Returns a tuple containing:
    /// * `video_node_id` - The video node ID
    /// * `segment_offset` - Offset of retrieved segment
    pub fn acquire_node_for_time(&self, video_segments_id: i32, time: i64) -> Result<(i32, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_offset = 0;
        call_suite_fn!(self, AcquireNodeForTime, video_segments_id, time, &mut video_node_id, &mut segment_offset)?;
        Ok((video_node_id, segment_offset))
    }

    /// From a sequence, get an ID to its video segments ID. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments
    pub fn acquire_video_segments_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        call_suite_fn!(self, AcquireVideoSegmentsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_previews_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        call_suite_fn!(self, AcquireVideoSegmentsWithPreviewsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get an ID to its video segments ID with preview files substituted, but only previews
    /// for sections that are opaque. This is appropriate for use when using previews for nested sequences. This is a ref-counted
    /// object, and must be released when no longer needed.
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns the ID for the Video Segments with Previews.
    pub fn acquire_video_segments_with_opaque_previews_id_with_stream_label(&self, timeline_data: pr_sys::PrTimelineID, stream_label: &str) -> Result<i32, Error> {
        let mut val = 0;
        let stream_label_c = std::ffi::CString::new(stream_label).unwrap();
        let stream_label_c = stream_label_c.as_bytes_with_nul();
        call_suite_fn!(self, AcquireVideoSegmentsWithOpaquePreviewsIDWithStreamLabel, timeline_data, stream_label_c.as_ptr() as *const _, &mut val)?;
        Ok(val)
    }

    /// From a sequence, get the first segment node that intersects with a range of times.
    /// This is a ref-counted object, and must be released when no longer needed.
    /// * `video_segments_id` - The Video Segments ID
    /// * `start_time` - The start of the requested segment time range
    /// * `end_time` - The end of the requested segment time range
    ///
    /// Returns a tuple containing:
    /// * `video_node_id` - The video node ID
    /// * `segment_offset` - Offset of retrieved segment
    pub fn acquire_first_node_in_time_range(&self, video_segments_id: i32, start_time: i64, end_time: i64) -> Result<(i32, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_offset = 0;
        call_suite_fn!(self, AcquireFirstNodeInTimeRange, video_segments_id, start_time, end_time, &mut video_node_id, &mut segment_offset)?;
        Ok((video_node_id, segment_offset))
    }

    /// Acquire the node owning an operator
    /// * `video_node_id` - The operator
    ///
    /// Returns the owner
    pub fn acquire_operator_owner_node_id(&self, video_node_id: i32) -> Result<i32, Error> {
        let mut val = 0;
        call_suite_fn!(self, AcquireOperatorOwnerNodeID, video_node_id, &mut val)?;
        Ok(val)
    }

    /// Get graphic layer params at a specific time
    /// * `video_node_id` - The Video Node ID
    /// * `time` - The time requested (in Media time)
    pub fn get_graphics_transformed_params(&self, video_node_id: i32, time: i64) -> Result<(pr_sys::prFPoint64, pr_sys::prFPoint64, pr_sys::prFPoint64, f32), Error> {
        let mut position = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut anchor = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut scale = pr_sys::prFPoint64 { x: 0.0, y: 0.0 };
        let mut rotation = 0.0;
        call_suite_fn!(self, GetGraphicsTransformedParams, video_node_id, time, &mut position, &mut anchor, &mut scale, &mut rotation)?;
        Ok((position, anchor, scale, rotation))
    }

    /// Get graphic layer group ID
    /// * `video_node_id` - The Video Node ID
    pub fn has_graphics_group(&self, video_node_id: i32) -> Result<bool, Error> {
        call_suite_fn_single!(self, HasGraphicsGroup -> bool, video_node_id)
    }

    /// Get graphic layer group ID
    /// * `video_node_id` - The Video Node ID
    pub fn get_graphics_group_id(&self, video_node_id: i32) -> Result<i32, Error> {
        call_suite_fn_single!(self, GetGraphicsGroupID -> i32, video_node_id)
    }

    /// Color managed version of GetVideoSegmentsProperties
    /// Retrieve general properties of a sequence (video segments). I.e. width/height, par and framerate and color space
    /// * `timeline_id` - The plugin timeline ID for the sequence
    ///
    /// Returns [`VideoSegmentProperties`]:
    /// * `width` - Width of the sequence
    /// * `height` - Height of the sequence
    /// * `par_num` - Pixel aspect ratio numerator of the sequence
    /// * `par_den` - Pixel aspect ratio denominator of the sequence
    /// * `frame_rate` - Frame rate of the sequence
    /// * `field_type` - Field type of the sequence
    /// * `color_space` - Opaque ID of the sequence's working color space
    pub fn get_video_segments_properties_ext(&self, timeline_data: pr_sys::PrTimelineID) -> Result<VideoSegmentProperties, Error> {
        let mut p: VideoSegmentProperties = unsafe { std::mem::zeroed() };
        let mut color_space: pr_sys::PrSDKColorSpaceID = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetVideoSegmentsPropertiesExt, timeline_data, &mut p.bounds, &mut p.par_num, &mut p.par_den, &mut p.frame_rate, &mut p.field_type, &mut color_space)?;
        p.color_space = Some(color_space);
        Ok(p)
    }

    /// From a sequence, get the first segment node that intersects with a range of times.
    /// This is a ref-counted object, and must be released when no longer needed.
    /// * `video_segments_id` - The Video Segments ID
    /// * `start_time` - The start of the requested segment time range
    /// * `end_time` - The end of the requested segment time range
    ///
    /// Returns a tuple containing:
    /// * `video_node_id` - The video node ID
    /// * `segment_start_time` - Start time of retrieved segment
    /// * `segment_end_time` - End time of retrieved segment
    /// * `segment_offset` - Offset of retrieved segment
    pub fn acquire_first_node_in_time_range_ext(&self, video_segments_id: i32, start_time: i64, end_time: i64) -> Result<(i32, i64, i64, i64), Error> {
        let mut video_node_id = 0;
        let mut segment_start_time = 0;
        let mut segment_end_time = 0;
        let mut segment_offset = 0;
        call_suite_fn!(self, AcquireFirstNodeInTimeRangeExt, video_segments_id, start_time, end_time, &mut video_node_id, &mut segment_start_time, &mut segment_end_time, &mut segment_offset)?;
        Ok((video_node_id, segment_start_time, segment_end_time, segment_offset))
    }

    /// Returns the relative time rate of a node at a given point in time.
    /// Node time rate varies with e.g. time remapping but not the playback speed of the sequence
    /// Can be thought of as the instantaneous rate of change of TransformNodeTime()
    /// * `video_node_id` - The Video Node ID
    /// * `time` - The time requested (in Media time - untransformed)
    ///
    /// Returns the node rate relative to the containing sequence
    pub fn get_node_time_scale(&self, video_node_id: i32, time: i64) -> Result<f64, Error> {
        call_suite_fn_single!(self, GetNodeTimeScale -> f64, video_node_id, time)
    }
}
