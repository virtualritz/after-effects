use crate::*;
use crate::aegp::*;
use ae_sys::{ A_long, A_short, A_Time, AEGP_StreamValue2, AEGP_KeyframeEase, AEGP_StreamRefH };

define_suite!(
    /// Keyframes make After Effects what it is. AEGPs (and...ssshh, don't tell anyone...effects) can use this suite to add, manipulate and remove keyframes from any keyframe-able stream.
    ///
    /// # Adding Multiple Keyframes
    /// Each time you call [`insert_keyframe`](Self::insert_keyframe), the entire stream is added to the undo stack.
    ///
    /// If you're adding one or two keyframes, this isn't a problem. However, if you're writing a keyframer, you'll want to do things the *right* way.
    ///
    /// Before you begin adding keyframes, call the (very-appropriately-named) [`start_add_keyframes`](Self::start_add_keyframes).
    ///
    /// For each keyframe to add, call [`AddKeyframesInfoHandle::add_keyframes`] to set the time to be used (and get the newly-added keyframe's index), then [`AddKeyframesInfoHandle::set_add_keyframe`] to specify the value to be used.
    ///
    /// Once you're finished, simply drop the [`AddKeyframesInfoHandle`] to let know After Effects know it's time to add the changed parameter stream to the undo stack.
    KeyframeSuite,
    AEGP_KeyframeSuite5,
    kAEGPKeyframeSuite,
    kAEGPKeyframeSuiteVersion5
);

impl KeyframeSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the number of keyframes on the given stream.
    ///
    /// Returns `-1` if the stream is not keyframe-able.
    ///
    /// Also, note that a stream without keyframes isn't necessarily constant; it can be altered by expressions.
    pub fn stream_num_kfs(&self, stream: impl AsPtr<AEGP_StreamRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamNumKFs -> A_long, stream.as_ptr())? as i32)
    }

    /// Retrieves the time of the specified keyframe.
    pub fn keyframe_time(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, time_mode: TimeMode) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetKeyframeTime -> A_Time, stream.as_ptr(), key_index, time_mode.into())?.into())
    }

    /// Adds a keyframe to the specified stream (at the specified composition or layer time).
    ///
    /// Returns the new keyframe's index.
    ///
    /// All indexes greater than the new index are now invalid (but you knew that).
    ///
    /// If there is already a keyframe at that time, the values will be updated.
    pub fn insert_keyframe(&self, stream: impl AsPtr<AEGP_StreamRefH>, time_mode: TimeMode, time: Time) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_InsertKeyframe -> A_long, stream.as_ptr(), time_mode.into(), &time.into() as *const _)? as i32)
    }

    /// Deletes the specified keyframe.
    pub fn delete_keyframe(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteKeyframe, stream.as_ptr(), key_index)
    }

    /// Creates and populates an [`StreamValue`] for the stream's value at the time of the keyframe.
    pub fn new_keyframe_value(&self, plugin_id: PluginId, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<StreamValue, Error> {
        let stream_suite = aegp::suites::Stream::new()?;
        let type_ = stream_suite.stream_type(stream.as_ptr())?;

        let mut sys_stream_value2 = call_suite_fn_single!(self, AEGP_GetNewKeyframeValue -> AEGP_StreamValue2, plugin_id, stream.as_ptr(), key_index)?;

        let ret = StreamValue::from_sys(type_, sys_stream_value2.val);
        stream_suite.dispose_stream_value(&mut sys_stream_value2)?;
        Ok(ret)
    }

    /// Sets the stream's value at the time of the keyframe.
    pub fn set_keyframe_value(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, value: StreamValue) -> Result<(), Error> {
        let sys_stream_value2 = AEGP_StreamValue2 {
            streamH: stream.as_ptr(),
            val: value.to_sys()
        };
        call_suite_fn!(self, AEGP_SetKeyframeValue, stream.as_ptr(), key_index, &sys_stream_value2)
    }

    /// Retrieves the dimensionality of the stream's value.
    pub fn stream_value_dimensionality(&self, stream: impl AsPtr<AEGP_StreamRefH>) -> Result<i16, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamValueDimensionality -> A_short, stream.as_ptr())? as i16)
    }

    /// Retrieves the temporal dimensionality of the stream.
    pub fn stream_temporal_dimensionality(&self, stream: impl AsPtr<AEGP_StreamRefH>) -> Result<i16, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamTemporalDimensionality -> A_short, stream.as_ptr())? as i16)
    }

    /// Returns the [`StreamValue`]s representing the stream's tangential values at the time of the keyframe.
    ///
    /// Returns a tuple containing the in and out tangents.
    pub fn new_keyframe_spatial_tangents(&self, plugin_id: PluginId, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<(StreamValue, StreamValue), Error> {
        let stream_suite = aegp::suites::Stream::new()?;
        let type_ = stream_suite.stream_type(stream.as_ptr())?;

        let (mut sys_in_tan, mut sys_out_tan) =
            call_suite_fn_double!(self, AEGP_GetNewKeyframeSpatialTangents -> AEGP_StreamValue2, AEGP_StreamValue2, plugin_id, stream.as_ptr(), key_index)?;

        let in_tan = StreamValue::from_sys(type_, sys_in_tan.val);
        let out_tan = StreamValue::from_sys(type_, sys_out_tan.val);

        stream_suite.dispose_stream_value(&mut sys_in_tan)?;
        stream_suite.dispose_stream_value(&mut sys_out_tan)?;

        Ok((in_tan, out_tan))
    }

    /// Specifies the tangential [`StreamValue`]s to be used for the stream's value at the time of the keyframe.
    pub fn set_keyframe_spatial_tangents(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, in_tan: StreamValue, out_tan: StreamValue) -> Result<(), Error> {
        let sys_in_tan = AEGP_StreamValue2 {
            streamH: stream.as_ptr(),
            val: in_tan.to_sys()
        };
        let sys_out_tan = AEGP_StreamValue2 {
            streamH: stream.as_ptr(),
            val: out_tan.to_sys()
        };
        call_suite_fn!(self, AEGP_SetKeyframeSpatialTangents, stream.as_ptr(), key_index, &sys_in_tan, &sys_out_tan)
    }

    /// Retrieves the [`AEGP_KeyframeEase`](after_effects_sys::AEGP_KeyframeEase)s associated with the specified dimension of the stream's value at the time of the keyframe.
    pub fn keyframe_temporal_ease(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, dimension: i32) -> Result<(AEGP_KeyframeEase, AEGP_KeyframeEase), Error> {
        call_suite_fn_double!(self, AEGP_GetKeyframeTemporalEase -> ae_sys::AEGP_KeyframeEase, ae_sys::AEGP_KeyframeEase, stream.as_ptr(), key_index, dimension)
    }

    /// Specifies the [`AEGP_KeyframeEase`](after_effects_sys::AEGP_KeyframeEase)s to be used for the stream's value at the time of the keyframe.
    pub fn set_keyframe_temporal_ease(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, dimension: i32, in_ease: &AEGP_KeyframeEase, out_ease: &AEGP_KeyframeEase) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetKeyframeTemporalEase, stream.as_ptr(), key_index, dimension, in_ease, out_ease)
    }

    /// Retrieves the flags currently set for the keyframe.
    pub fn keyframe_flags(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<KeyframeFlags, Error> {
        Ok(KeyframeFlags::from_bits_truncate(
            call_suite_fn_single!(self, AEGP_GetKeyframeFlags -> ae_sys::AEGP_KeyframeFlags, stream.as_ptr(), key_index)?
        ))
    }

    /// Sets the specified flag for the keyframe. Flags must be set individually.
    pub fn set_keyframe_flag(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, flag: KeyframeFlags, value: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetKeyframeFlag, stream.as_ptr(), key_index, flag.bits(), value.into())
    }

    /// Retrieves the in and out [`KeyframeInterpolation`]s for the specified keyframe.
    pub fn keyframe_interpolation(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<(KeyframeInterpolation, KeyframeInterpolation), Error> {
        let (in_interp, out_interp) =
            call_suite_fn_double!(self,
                AEGP_GetKeyframeInterpolation -> ae_sys::AEGP_KeyframeInterpolationType, ae_sys::AEGP_KeyframeInterpolationType,
                stream.as_ptr(),
                key_index
            )?;
        Ok((in_interp.into(), out_interp.into()))
    }

    /// Specifies the in and out [`KeyframeInterpolation`]s to be used for the given keyframe.
    pub fn set_keyframe_interpolation(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, in_interp: KeyframeInterpolation, out_interp: KeyframeInterpolation) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetKeyframeInterpolation, stream.as_ptr(), key_index, in_interp.into(), out_interp.into())
    }

    /// Informs After Effects that you're going to be adding several keyframes to the specified stream.
    ///
    /// Returns an [`AddKeyframesInfoHandle`], which you can use to add keyframes.
    pub fn start_add_keyframes(&self, stream: impl AsPtr<AEGP_StreamRefH>) -> Result<AddKeyframesInfoHandle, Error> {
        Ok(AddKeyframesInfoHandle {
            suite: self.clone(),
            handle: call_suite_fn_single!(self, AEGP_StartAddKeyframes -> ae_sys::AEGP_AddKeyframesInfoH, stream.as_ptr())?,
            add: true
        })
    }

    pub fn keyframe_label_color_index(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetKeyframeLabelColorIndex -> A_long, stream.as_ptr(), key_index)? as i32)
    }

    pub fn set_keyframe_label_color_index(&self, stream: impl AsPtr<AEGP_StreamRefH>, key_index: i32, key_label: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetKeyframeLabelColorIndex, stream.as_ptr(), key_index, key_label)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

bitflags::bitflags! {
    pub struct KeyframeInterpolationMask: ae_sys::A_long {
        const NONE       = ae_sys::AEGP_KeyInterpMask_NONE       as ae_sys::A_long;
        const LINEAR     = ae_sys::AEGP_KeyInterpMask_LINEAR     as ae_sys::A_long;
        const BEZIER     = ae_sys::AEGP_KeyInterpMask_BEZIER     as ae_sys::A_long;
        const HOLD       = ae_sys::AEGP_KeyInterpMask_HOLD       as ae_sys::A_long;
        const CUSTOM     = ae_sys::AEGP_KeyInterpMask_CUSTOM     as ae_sys::A_long;
        const ANY        = ae_sys::AEGP_KeyInterpMask_ANY        as ae_sys::A_long;
    }
}

bitflags::bitflags! {
    pub struct KeyframeFlags: ae_sys::A_long {
        const NONE                = ae_sys::AEGP_KeyframeFlag_NONE                as ae_sys::A_long;
        const TEMPORAL_CONTINUOUS = ae_sys::AEGP_KeyframeFlag_TEMPORAL_CONTINUOUS as ae_sys::A_long;
        const TEMPORAL_AUTOBEZIER = ae_sys::AEGP_KeyframeFlag_TEMPORAL_AUTOBEZIER as ae_sys::A_long;
        const SPATIAL_CONTINUOUS  = ae_sys::AEGP_KeyframeFlag_SPATIAL_CONTINUOUS  as ae_sys::A_long;
        const SPATIAL_AUTOBEZIER  = ae_sys::AEGP_KeyframeFlag_SPATIAL_AUTOBEZIER  as ae_sys::A_long;
        const ROVING              = ae_sys::AEGP_KeyframeFlag_ROVING              as ae_sys::A_long;
    }
}

define_enum! {
    ae_sys::AEGP_KeyframeInterpolationType,
    KeyframeInterpolation {
        None   = ae_sys::AEGP_KeyInterp_NONE,
        Linear = ae_sys::AEGP_KeyInterp_LINEAR,
        Bezier = ae_sys::AEGP_KeyInterp_BEZIER,
        Hold   = ae_sys::AEGP_KeyInterp_HOLD,
    }
}

/// Temporary struct to hold the state of adding keyframe batches, returned from [`KeyframeSuite::start_add_keyframes()`].
///
/// Keyframes will be commited when this struct goes out of scope.
pub struct AddKeyframesInfoHandle {
    handle: ae_sys::AEGP_AddKeyframesInfoH,
    suite: KeyframeSuite,
    add: bool,
}
impl AddKeyframesInfoHandle {
    /// Adds a keyframe to the specified stream at the specified (layer or composition) time.
    ///
    /// Note: this doesn't actually do anything to the stream's value.
    pub fn add_keyframes(&mut self, time_mode: TimeMode, time: Time) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self.suite, AEGP_AddKeyframes -> A_long, self.handle, time_mode.into(), &time.into() as *const _)? as i32)
    }

    /// Sets the value of the specified keyframe.
    pub fn set_add_keyframe(&mut self, key_index: i32, stream: impl AsPtr<AEGP_StreamRefH>, value: StreamValue) -> Result<(), Error> {
        let sys_stream_value2 = AEGP_StreamValue2 {
            streamH: stream.as_ptr(),
            val: value.to_sys()
        };
        call_suite_fn!(self.suite, AEGP_SetAddKeyframe, self.handle, key_index, &sys_stream_value2)
    }

    /// Sets the `add` flag, used in a call to `AEGP_EndAddKeyframes`. Defaults to true
    pub fn set_add(&mut self, add: bool) {
        self.add = add;
    }
}
impl Drop for AddKeyframesInfoHandle {
    fn drop(&mut self) {
        let _ = call_suite_fn!(self.suite, AEGP_EndAddKeyframes, self.add as _, self.handle);
    }
}
