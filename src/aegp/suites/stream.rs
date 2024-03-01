use crate::*;
use crate::aegp::*;
use ae_sys::{ AEGP_LayerH, AEGP_MaskRefH, AEGP_StreamRefH, AEGP_EffectRefH};

define_suite!(
    /// Access and manipulate the values of a layer's streams. For paint and text streams, use [`DynamicStreamSuite`] instead.
    StreamSuite,
    AEGP_StreamSuite6,
    kAEGPStreamSuite,
    kAEGPStreamSuiteVersion6
);

impl StreamSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Determines if the given stream is appropriate for the given layer.
    pub fn is_stream_legal(&self, layer_handle: impl AsPtr<AEGP_LayerH>, stream: LayerStream) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsStreamLegal -> ae_sys::A_Boolean, layer_handle.as_ptr(), stream as _)? != 0)
    }

    /// Given a stream, returns whether or not a stream is time-variant (and can be keyframed).
    pub fn can_vary_over_time(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_CanVaryOverTime -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Retrieves an [`KeyframeInterpolationMask`] indicating which interpolation types are valid for the [`StreamReferenceHandle`].
    pub fn valid_interpolations(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<KeyframeInterpolationMask, Error> {
        Ok(KeyframeInterpolationMask::from_bits_truncate(
            call_suite_fn_single!(self, AEGP_GetValidInterpolations -> ae_sys::AEGP_KeyInterpolationMask, stream_ref.as_ptr())?,
        ))
    }

    /// Get a layer's data stream.
    ///
    /// Note that this will not provide keyframe access; Use the [`KeyframeSuite`](aegp::suites::Keyframe) instead.
    pub fn new_layer_stream(&self, layer_handle: impl AsPtr<AEGP_LayerH>, plugin_id: PluginId, stream_name: LayerStream) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewLayerStream -> ae_sys::AEGP_StreamRefH, plugin_id, layer_handle.as_ptr(), stream_name as _)?,
            true, // is_owned
        ))
    }

    /// Get number of parameter streams associated with an effect.
    pub fn effect_num_param_streams(&self, effect_ref: impl AsPtr<AEGP_EffectRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetEffectNumParamStreams -> ae_sys::A_long, effect_ref.as_ptr())? as i32)
    }

    /// Get an effect's parameter stream.
    pub fn new_effect_stream_by_index(&self, effect_ref: impl AsPtr<AEGP_EffectRefH>, plugin_id: PluginId, index: i32) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewEffectStreamByIndex -> ae_sys::AEGP_StreamRefH, plugin_id, effect_ref.as_ptr(), index)?,
            true, // is_owned
        ))
    }

    /// Get a mask's stream.
    ///
    /// Also see the [`MaskSuite`](aegp::suites::Mask) and [`MaskOutlineSuite`](aegp::suites::MaskOutline) for additional Mask functions.
    pub fn new_mask_stream(&self, mask_ref: impl AsPtr<AEGP_MaskRefH>, plugin_id: PluginId, stream: MaskStream) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewMaskStream -> ae_sys::AEGP_StreamRefH, plugin_id, mask_ref.as_ptr(), stream.into())?,
            true, // is_owned
        ))
    }

    /// Dispose of a stream (do this with all streams passed to the plug-in by these functions).
    pub fn dispose_stream(&self, stream_ref: &mut StreamReferenceHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeStream, stream_ref.as_ptr())?;
        stream_ref.0 = std::ptr::null_mut();
        Ok(())
    }

    /// Get name of the stream (localized or forced English).
    ///
    /// NOTE: if `force_english` is `true`, the default name will override any stream renaming which has been done (either programatically, or by the user).
    pub fn stream_name(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, force_english: bool) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetStreamName -> ae_sys::AEGP_MemHandle, plugin_id, stream_ref.as_ptr(), force_english as _)?;
        // Create a mem handle each and lock it.
        // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Get stream units, formatted as text (localized or forced English).
    pub fn stream_units_text(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, force_english: bool) -> Result<String, Error> {
        let mut name = [0i8; ae_sys::AEGP_MAX_STREAM_NAME_SIZE as usize + 1];
        call_suite_fn!(self, AEGP_GetStreamUnitsText, stream_ref.as_ptr(), force_english as _, name.as_mut_ptr() as _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Get stream's flags, as well as minimum and maximum values (as floats), if the stream *has* mins and maxes.
    ///
    /// Returns a tuple containing ([`StreamFlags`], `Option<min>`, `Option<max>`).
    pub fn stream_properties(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<(StreamFlags, Option<f64>, Option<f64>), Error> {
        let mut flags = 0;
        let mut min = 0.0;
        let mut max = 0.0;
        call_suite_fn!(self, AEGP_GetStreamProperties, stream_ref.as_ptr(), &mut flags, &mut min, &mut max)?;
        let flags = StreamFlags::from_bits_truncate(flags);
        let min = if flags.contains(StreamFlags::HAS_MIN) { Some(min) } else { None };
        let max = if flags.contains(StreamFlags::HAS_MAX) { Some(max) } else { None };
        Ok((flags, min, max))
    }

    /// Returns whether or not the stream is affected by expressions.
    pub fn is_stream_timevarying(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsStreamTimevarying -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Get type (dimension) of a stream.
    ///
    /// NOTE: always returns [`StreamType::ThreeDSpatial`] for position, regardless of whether or not the layer is 3D.
    pub fn stream_type(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<StreamType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamType -> ae_sys::AEGP_StreamType, stream_ref.as_ptr())?.into())
    }

    // FIXME: should this handle memory owned by Ae properly?
    // Currently we just copy and dispose immedately. Should be fine
    // for what we're doing atm but for stream data like image buffers this
    // is wasteful and potentially slow.
    /// Get value, at a time you specify, of stream. `value` must be disposed by the plug-in.
    /// The `time_mode` indicates whether the time is in compositions or layer time.
    pub fn new_stream_value(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, time_mode: TimeMode, time: Time, sample_stream_pre_expression: bool) -> Result<StreamValue, Error> {
        let type_ = self.stream_type(stream_ref.as_ptr())?;

        let mut stream_value2 = call_suite_fn_single!(self,
            AEGP_GetNewStreamValue -> ae_sys::AEGP_StreamValue2,
            plugin_id,
            stream_ref.as_ptr(),
            time_mode.into(),
            &time.into() as *const _,
            sample_stream_pre_expression as u8
        )?;
        let value = StreamValue::from_sys(type_, stream_value2.val);

        self.dispose_stream_value(&mut stream_value2)?;

        Ok(value)
    }

    /// Dispose of stream value. Always deallocate values passed to the plug-in.
    pub fn dispose_stream_value(&self, stream_value: &mut ae_sys::AEGP_StreamValue2) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeStreamValue, stream_value)
    }

    /// NOTE: This convenience function is only valid for streams with primitive data types, and not for `StreamType::ArbBlock`, `StreamType::Marker` or `StreamType::MaskOutline`.
    /// For these and other complex types, use [`new_stream_value()`](Self::new_stream_value), described above.
    pub fn layer_stream_value(&self, layer_handle: impl AsPtr<AEGP_LayerH>, stream: LayerStream, time_mode: TimeMode, time: Time, pre_expression: bool) -> Result<StreamValue, Error> {
        let (stream_value, stream_type) = call_suite_fn_double!(self,
            AEGP_GetLayerStreamValue -> ae_sys::AEGP_StreamVal2, ae_sys::AEGP_StreamType,
            layer_handle.as_ptr(),
            stream as i32,
            time_mode.into(),
            &time.into() as *const _,
            pre_expression as u8
        )?;

        Ok(StreamValue::from_sys(stream_type, stream_value))
    }

    /// Determines whether expressions are enabled on the given [`StreamReferenceHandle`].
    pub fn expression_state(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetExpressionState -> ae_sys::A_Boolean, plugin_id, stream_ref.as_ptr())? != 0)
    }

    /// Set whether expressions are enabled on the given [`StreamReferenceHandle`].
    pub fn set_expression_state(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, enabled: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetExpressionState, plugin_id, stream_ref.as_ptr(), enabled as u8)
    }

    /// Get the expression string for the given [`StreamReferenceHandle`].
    pub fn expression_string(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetExpression -> ae_sys::AEGP_MemHandle, plugin_id, stream_ref.as_ptr())?;
        // Create a mem handle each and lock it.
        // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Set the expression string for the given [`StreamReferenceHandle`].
    pub fn set_expression_string(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, expression: &str) -> Result<(), Error> {
        let expression = U16CString::from_str(expression).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetExpression, plugin_id, stream_ref.as_ptr(), expression.as_ptr())
    }

    /// Duplicate a given [`StreamReferenceHandle`].
    pub fn duplicate_stream(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_DuplicateStreamRef -> ae_sys::AEGP_StreamRefH, plugin_id, stream_ref.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Get the unique ID of the given [`StreamReferenceHandle`].
    pub fn unique_stream_id(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetUniqueStreamID -> i32, stream_ref.as_ptr())
    }
}

define_suite!(
    /// This suite accesses and manipulates paint and text streams.
    ///
    /// Use [`stream_grouping_type()`](Self::stream_grouping_type) and [`dynamic_stream_flags()`](Self::dynamic_stream_flags) to identify the stream before attempting to use functions which only work on certain stream types.
    ///
    /// Also note that, often, you can simply use [`StreamSuite`] calls to work with dynamic streams. On the other hand, only those functions specific to dynamic streams are in this suite.
    DynamicStreamSuite,
    AEGP_DynamicStreamSuite4,
    kAEGPDynamicStreamSuite,
    kAEGPDynamicStreamSuiteVersion4
);

impl DynamicStreamSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the [`StreamReferenceHandle`] corresponding to the layer. This function is used to initiate a recursive walk of the layer's streams.
    pub fn new_stream_ref_for_layer(&self, layer_handle: impl AsPtr<AEGP_LayerH>, plugin_id: PluginId) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewStreamRefForLayer -> ae_sys::AEGP_StreamRefH, plugin_id, layer_handle.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Retrieves the [`StreamReferenceHandle`] corresponding to the mask.
    pub fn new_stream_ref_for_mask(&self, mask_ref: impl AsPtr<AEGP_MaskRefH>, plugin_id: PluginId) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewStreamRefForMask -> ae_sys::AEGP_StreamRefH, plugin_id, mask_ref.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Retrieves the number of sub-streams associated with the given [`StreamReferenceHandle`].
    ///
    /// The initial layer has a depth of 0.
    pub fn stream_depth(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamDepth -> ae_sys::A_long, stream_ref.as_ptr())? as i32)
    }

    /// Retrieves the grouping type for the given [`StreamReferenceHandle`].
    pub fn stream_grouping_type(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<StreamGroupingType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamGroupingType -> ae_sys::AEGP_StreamGroupingType, stream_ref.as_ptr())?.into())
    }

    /// Retrieves the number of streams associated with the given [`StreamReferenceHandle`].
    ///
    /// This function will return an error if called with an [`StreamReferenceHandle`] with type [`StreamGroupingType::Leaf`].
    pub fn num_streams_in_group(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumStreamsInGroup -> ae_sys::A_long, stream_ref.as_ptr())? as i32)
    }

    /// Retrieves the flags for a given [`StreamReferenceHandle`].
    pub fn dynamic_stream_flags(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<DynamicStreamFlags, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetDynamicStreamFlags -> ae_sys::AEGP_DynStreamFlags, stream_ref.as_ptr())?.into())
    }

    /// Sets the specified flag for the [`StreamReferenceHandle`].
    ///
    /// Note: flags must be set individually. Undoable if `undoable` is ``true`.
    ///
    /// This call may be used to dynamically show or hide parameters, by setting and clearing [`DynamicStreamFlags::Hidden`].
    /// However, [`DynamicStreamFlags::Disabled`] may not be set.
    pub fn set_dynamic_stream_flag(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, flag: DynamicStreamFlags, undoable: bool, enabled: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetDynamicStreamFlag, stream_ref.as_ptr(), flag.into(), undoable as _, enabled as _)
    }

    /// Retrieves a sub-stream by index from a given [`StreamReferenceHandle`]. Cannot be used on streams of type [`StreamGroupingType::Leaf`].
    pub fn new_stream_ref_by_index(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, index: i32) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewStreamRefByIndex -> ae_sys::AEGP_StreamRefH, plugin_id, stream_ref.as_ptr(), index)?,
            true, // is_owned
        ))
    }

    /// Retrieves a sub-stream by match name from a given [`StreamReferenceHandle`]. Only legal for [`StreamGroupingType::NamedGroup`].
    ///
    /// Here are some handy stream names, for which references may be retrieved:
    /// * `"ADBE Mask Parade"`
    /// * `"ADBE Mask Atom"`
    /// * `"ADBE Mask Feather"`
    /// * `"ADBE Mask Opacity"`
    /// * `"ADBE Mask Offset"`
    /// * `"ADBE Effect Parade"`
    /// * `"ADBE Abstract Layer"`
    /// * `"ADBE AV Layer"`
    /// * `"ADBE Text Layer"`
    /// * `"ADBE Camera Layer"`
    /// * `"ADBE Light Layer"`
    /// * `"ADBE Audio Group"`
    /// * `"ADBE Material Options Group"`
    /// * `"ADBE Transform Group"`
    /// * `"ADBE Light Options Group"`
    /// * `"ADBE Camera Options Group"`
    pub fn new_stream_ref_by_match_name(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, match_name: &str) -> Result<StreamReferenceHandle, Error> {
        let match_name = CString::new(match_name).map_err(|_| Error::InvalidParms)?;
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewStreamRefByMatchname -> ae_sys::AEGP_StreamRefH, plugin_id, stream_ref.as_ptr(), match_name.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Deletes the specified stream from a stream grouping.
    /// Note that the caller must still dispose of any [`StreamReferenceHandle`] it's already acquired (allocated) via the API. Undoable.
    /// Only valid for children of type [`StreamGroupingType::IndexedGroup`].
    ///
    /// Note: as of 6.5, if a stream is deleted while it or any child stream is selected, the current composition selection will become `null`.
    pub fn delete_stream(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteStream, stream_ref.as_ptr())
    }

    /// Sets the new index of the specified [`StreamReferenceHandle`]. Undoable.
    /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
    /// The [`StreamReferenceHandle`] is updated to refer to the newly-ordered stream.
    pub fn reorder_stream(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, new_index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReorderStream, stream_ref.as_ptr(), new_index)
    }

    /// Duplicates the specified stream and appends it to the stream group. Undoable.
    /// Only valid for children of type [`StreamGroupingType::IndexedGroup`].
    pub fn duplicate_stream(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_DuplicateStream -> i32, plugin_id, stream_ref.as_ptr())
    }

    /// Sets the name of the given [`StreamReferenceHandle`]. Undoable.
    /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
    ///
    /// NOTE: If you retrieve the name with `force_english` set to `true`, you will get the canonical, unchanged name of the stream.
    ///
    /// Note: Use this on an effect stream's group to change the display name of an effect.
    pub fn set_stream_name(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, name: &str) -> Result<(), Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetStreamName, stream_ref.as_ptr(), name.as_ptr())
    }

    /// Returns whether or not it is currently possible to add a stream through the API.
    pub fn can_add_stream(&self, group_stream: impl AsPtr<AEGP_StreamRefH>, match_name: &str) -> Result<bool, Error> {
        let match_name = CString::new(match_name).map_err(|_| Error::InvalidParms)?;
        Ok(call_suite_fn_single!(self, AEGP_CanAddStream -> ae_sys::A_Boolean, group_stream.as_ptr(), match_name.as_ptr())? != 0)
    }

    /// Adds a stream to the specified stream group. Undoable. Only valid for [`StreamGroupingType::IndexedGroup`].
    pub fn add_stream(&self, group_stream: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId, match_name: &str) -> Result<StreamReferenceHandle, Error> {
        let match_name = CString::new(match_name).map_err(|_| Error::InvalidParms)?;
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_AddStream -> ae_sys::AEGP_StreamRefH, plugin_id, group_stream.as_ptr(), match_name.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Retrieves the match name for the specified [`StreamReferenceHandle`].
    ///
    /// Note that this may differ from the display name, which can be retrieves using [`StreamSuite::stream_name()`].
    pub fn match_name(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<String, Error> {
        let mut buffer = [0u8; ae_sys::AEGP_MAX_STREAM_MATCH_NAME_SIZE as usize];

        call_suite_fn!(self, AEGP_GetMatchName, stream_ref.as_ptr(), buffer.as_mut_ptr() as _)?;

        Ok(std::ffi::CStr::from_bytes_until_nul(&buffer).map_err(|_| Error::InvalidParms)?.to_string_lossy().into_owned())
    }

    /// Retrieves an [`StreamReferenceHandle`] for the parent of the specified [`StreamReferenceHandle`].
    pub fn new_parent_stream_ref(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, plugin_id: PluginId) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetNewParentStreamRef -> ae_sys::AEGP_StreamRefH, plugin_id, stream_ref.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Returns whether or not the specified [`StreamReferenceHandle`] has been modified.
    ///
    /// Note: the same result is available through the After Effect user interface by typing "UU" with the composition selected.
    pub fn stream_is_modified(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamIsModified -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Retrieves the index of a given stream, relative to its parent stream.
    ///
    /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
    ///
    /// NOTE: As mentioned *elsewhere*, [`StreamReferenceHandle`]s don't persist across function calls.
    /// If streams are re-ordered, added or removed, all [`StreamReferenceHandle`]s previously retrieved may be invalidated.
    pub fn stream_index_in_parent(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetStreamIndexInParent -> ae_sys::A_long, stream_ref.as_ptr())? as i32)
    }

    /// Valid on leaf streams only. Returns true if this stream is a multidimensional stream that can have its dimensions separated, though they may not be currently separated.
    ///
    /// Terminology: A Leader is the stream that can be separated, a Follower is one of N automatic streams that correspond to the N dimensions of the Leader.
    ///
    /// A Leader isn't always separated, call [`are_dimensions_separated()`](Self::are_dimensions_separated) to find out if it is.
    /// As of CS4, the only stream that is ever separarated is the layer's Position property.
    /// Please *do not* write code assuming that, we anticipate allowing separation of more streams in the future.
    pub fn is_separation_leader(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsSeparationLeader -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Methods such as [`new_keyframe_value()`](aegp::suites::Keyframe::new_keyframe_value) that work on keyframe indices will most definitely *not* work on the Leader property, you will need to retrieve and operate on the Followers explicitly.
    pub fn are_dimensions_separated(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_AreDimensionsSeparated -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Valid only if [`is_separation_leader()`](Self::is_separation_leader) is `true`.
    pub fn set_dimensions_separated(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, separated: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetDimensionsSeparated, stream_ref.as_ptr(), separated as u8)
    }

    /// Retrieve the Follower stream corresponding to a given dimension of the Leader stream.
    ///
    /// `dim` can range from `0` to `AEGP_GetStreamValueDimensionality(leader_streamH) - 1`.
    pub fn separation_follower(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>, dimension: i16) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetSeparationFollower -> ae_sys::AEGP_StreamRefH, stream_ref.as_ptr(), dimension)?,
            true, // is_owned
        ))
    }

    /// Valid on leaf streams only.
    /// Returns `true` if this stream is a one dimensional property that represents one of the dimensions of a Leader.
    /// You can retrieve stream from the Leader using [`separation_follower()`](Self::separation_follower).
    pub fn is_separation_follower(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsSeparationFollower -> ae_sys::A_Boolean, stream_ref.as_ptr())? != 0)
    }

    /// Valid on separation Followers only, returns the Leader it is part of.
    pub fn separation_leader(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle(
            call_suite_fn_single!(self, AEGP_GetSeparationLeader -> ae_sys::AEGP_StreamRefH, stream_ref.as_ptr())?,
            true, // is_owned
        ))
    }

    /// Valid on separation Followers only, returns which dimension of the Leader it corresponds to.
    pub fn separation_dimension(&self, stream_ref: impl AsPtr<AEGP_StreamRefH>) -> Result<i16, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetSeparationDimension -> i16, stream_ref.as_ptr())?)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_handle_wrapper!(TextDocumentHandle, AEGP_TextDocumentH);

register_handle!(AEGP_StreamRefH);
define_owned_handle_wrapper!(StreamReferenceHandle, AEGP_StreamRefH);
impl Drop for StreamReferenceHandle {
    fn drop(&mut self) {
        if self.is_owned() {
            StreamSuite::new().unwrap().dispose_stream(self).unwrap();
        }
    }
}

define_enum! {
    ae_sys::AEGP_StreamType,
    StreamType {
        NoData        = ae_sys::AEGP_StreamType_NO_DATA,
        ThreeDSpatial = ae_sys::AEGP_StreamType_ThreeD_SPATIAL,
        ThreeD        = ae_sys::AEGP_StreamType_ThreeD,
        TwoDSpatial   = ae_sys::AEGP_StreamType_TwoD_SPATIAL,
        TwoD          = ae_sys::AEGP_StreamType_TwoD,
        OneD          = ae_sys::AEGP_StreamType_OneD,
        Color         = ae_sys::AEGP_StreamType_COLOR,
        // ArbBlock      = ae_sys::AEGP_StreamType_ARB,
        // Marker        = ae_sys::AEGP_StreamType_MARKER,
        LayerId       = ae_sys::AEGP_StreamType_LAYER_ID,
        MaskId        = ae_sys::AEGP_StreamType_MASK_ID,
        Mask          = ae_sys::AEGP_StreamType_MASK,
        TextDocument  = ae_sys::AEGP_StreamType_TEXT_DOCUMENT,
    }
}

define_enum! {
    ae_sys::AEGP_StreamGroupingType,
    StreamGroupingType {
        None         = ae_sys::AEGP_StreamGroupingType_NONE,
        Leaf         = ae_sys::AEGP_StreamGroupingType_LEAF,
        NamedGroup   = ae_sys::AEGP_StreamGroupingType_NAMED_GROUP,
        IndexedGroup = ae_sys::AEGP_StreamGroupingType_INDEXED_GROUP,
    }
}

define_enum! {
    ae_sys::AEGP_DynStreamFlags,
    DynamicStreamFlags {
        /// Stream is available for reading and writing
        ActiveEyeball               = ae_sys::AEGP_DynStreamFlag_ACTIVE_EYEBALL,
        /// Stream is still readable/writable, but it may not currently be visible in the UI
        Hidden                      = ae_sys::AEGP_DynStreamFlag_HIDDEN,
        /// A read-only flag. Indicates whether the [`StreamReferenceHandle`] is grayed out in the UI.
        ///
        /// Note that as of CS5, this flag will not be returned if a parameter is disabled.
        /// Instead, check `PF_PUI_DISABLED` in `ParamDef`.
        Disabled                    = ae_sys::AEGP_DynStreamFlag_DISABLED,
        /// A read-only flag. Indicates that the [`StreamReferenceHandle`] is read-only, the user never sees it.
        ///
        /// However, the children are still seen and not indented in the Timeline panel.
        Elided                      = ae_sys::AEGP_DynStreamFlag_ELIDED,
        /// New in CS6. A read-only flag. Indicates that this stream group should be shown when empty.
        ShownWhenEmpty              = ae_sys::AEGP_DynStreamFlag_SHOWN_WHEN_EMPTY,
        /// New in CS6. A read-only flag. Indicates that this stream property will not be automatically revealed when un-hidden.
        SkipRevealWhenUnhidden      = ae_sys::AEGP_DynStreamFlag_SKIP_REVEAL_WHEN_UNHIDDEN,
    }
}

define_enum! {
    ae_sys::AEGP_MaskStream,
    MaskStream {
        Outline   = ae_sys::AEGP_MaskStream_OUTLINE,
        Opacity   = ae_sys::AEGP_MaskStream_OPACITY,
        Feather   = ae_sys::AEGP_MaskStream_FEATHER,
        Expansion = ae_sys::AEGP_MaskStream_EXPANSION,
        // Begin     = ae_sys::AEGP_MaskStream_BEGIN, // this is the same as Outline
        End       = ae_sys::AEGP_MaskStream_END,
    }
}

define_enum! {
    ae_sys::AEGP_LayerStream,
    LayerStream {
        None        = ae_sys::AEGP_LayerStream_NONE,
        AnchorPoint = ae_sys::AEGP_LayerStream_ANCHORPOINT,
        Position    = ae_sys::AEGP_LayerStream_POSITION,
        Scale       = ae_sys::AEGP_LayerStream_SCALE,
        // This is the layer's rotation for a 2D layer
        RotateZ     = ae_sys::AEGP_LayerStream_ROTATION,
        Opcaity     = ae_sys::AEGP_LayerStream_OPACITY,
        Audio       = ae_sys::AEGP_LayerStream_AUDIO,
        Marker      = ae_sys::AEGP_LayerStream_MARKER,
        TimeRemap   = ae_sys::AEGP_LayerStream_TIME_REMAP,
        RotateX     = ae_sys::AEGP_LayerStream_ROTATE_X,
        RotateY     = ae_sys::AEGP_LayerStream_ROTATE_Y,
        Orientation = ae_sys::AEGP_LayerStream_ORIENTATION,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectType_CAMERA
        Zoom          = ae_sys::AEGP_LayerStream_ZOOM,
        DepthOfField  = ae_sys::AEGP_LayerStream_DEPTH_OF_FIELD,
        FocusDistance = ae_sys::AEGP_LayerStream_FOCUS_DISTANCE,
        Aperture      = ae_sys::AEGP_LayerStream_APERTURE,
        BlurLevel     = ae_sys::AEGP_LayerStream_BLUR_LEVEL,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectType_LIGHT
        Intensity       = ae_sys::AEGP_LayerStream_INTENSITY,
        Color           = ae_sys::AEGP_LayerStream_COLOR,
        ConeAngle       = ae_sys::AEGP_LayerStream_CONE_ANGLE,
        ConeFeather     = ae_sys::AEGP_LayerStream_CONE_FEATHER,
        ShadowDarkness  = ae_sys::AEGP_LayerStream_SHADOW_DARKNESS,
        ShadowDiffusion = ae_sys::AEGP_LayerStream_SHADOW_DIFFUSION,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectType_AV
        AcceptsShadows    = ae_sys::AEGP_LayerStream_ACCEPTS_SHADOWS,
        AcceptsLights     = ae_sys::AEGP_LayerStream_ACCEPTS_LIGHTS,
        AmbientCoeff      = ae_sys::AEGP_LayerStream_AMBIENT_COEFF,
        DiffuseCoeff      = ae_sys::AEGP_LayerStream_DIFFUSE_COEFF,
        SpecularIntensity = ae_sys::AEGP_LayerStream_SPECULAR_INTENSITY,
        SpecularShininess = ae_sys::AEGP_LayerStream_SPECULAR_SHININESS,

        CastsShadows      = ae_sys::AEGP_LayerStream_CASTS_SHADOWS, /* LIGHT and AV only, no CAMERA */
        LightTransmission = ae_sys::AEGP_LayerStream_LIGHT_TRANSMISSION, /* AV Layer only */
        Metal             = ae_sys::AEGP_LayerStream_METAL,                // AV layer only

        SourceText = ae_sys::AEGP_LayerStream_SOURCE_TEXT,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectType_CAMERA
        IrisShape               = ae_sys::AEGP_LayerStream_IRIS_SHAPE,
        IrisRotation            = ae_sys::AEGP_LayerStream_IRIS_ROTATION,
        IrisRoundness           = ae_sys::AEGP_LayerStream_IRIS_ROUNDNESS,
        IrisAspectRatio         = ae_sys::AEGP_LayerStream_IRIS_ASPECT_RATIO,
        IrisDiffractionFringe   = ae_sys::AEGP_LayerStream_IRIS_DIFFRACTION_FRINGE,
        IrisHighlightGain       = ae_sys::AEGP_LayerStream_IRIS_HIGHLIGHT_GAIN,
        IrisHighlightThreshold  = ae_sys::AEGP_LayerStream_IRIS_HIGHLIGHT_THRESHOLD,
        IrisHighlightSaturation = ae_sys::AEGP_LayerStream_IRIS_HIGHLIGHT_SATURATION,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectTyp_LIGHT
        LightFalloffType     = ae_sys::AEGP_LayerStream_LIGHT_FALLOFF_TYPE,
        LightFalloffStart    = ae_sys::AEGP_LayerStream_LIGHT_FALLOFF_START,
        LightFalloffDistance = ae_sys::AEGP_LayerStream_LIGHT_FALLOFF_DISTANCE,

        // only valid for AEGP_ObjectType == ae_sys::AEGP_ObjectType_AV
        ReflactionIntensity = ae_sys::AEGP_LayerStream_REFLECTION_INTENSITY,
        ReflactionSharpness = ae_sys::AEGP_LayerStream_REFLECTION_SHARPNESS,
        ReflactionRolloff   = ae_sys::AEGP_LayerStream_REFLECTION_ROLLOFF,
        TransparencyCoeff   = ae_sys::AEGP_LayerStream_TRANSPARENCY_COEFF,
        TransparencyRolloff = ae_sys::AEGP_LayerStream_TRANSPARENCY_ROLLOFF,
        IndexOfRefraction   = ae_sys::AEGP_LayerStream_INDEX_OF_REFRACTION,

        BevelStyle             = ae_sys::AEGP_LayerStream_EXTRUSION_BEVEL_STYLE,
        BevelDirection         = ae_sys::AEGP_LayerStream_EXTRUSION_BEVEL_DIRECTION,
        BevelDepth             = ae_sys::AEGP_LayerStream_EXTRUSION_BEVEL_DEPTH,
        ExtrusionHoleBeveDepth = ae_sys::AEGP_LayerStream_EXTRUSION_HOLE_BEVEL_DEPTH,
        ExtrusionDepth         = ae_sys::AEGP_LayerStream_EXTRUSION_DEPTH,
        PlaneCurvature         = ae_sys::AEGP_LayerStream_PLANE_CURVATURE,
        PlaneSubdivision       = ae_sys::AEGP_LayerStream_PLANE_SUBDIVISION,
    }
}

bitflags::bitflags! {
    pub struct StreamFlags: ae_sys::A_long {
        const NONE       = ae_sys::AEGP_StreamFlag_NONE       as ae_sys::A_long;
        const HAS_MIN    = ae_sys::AEGP_StreamFlag_HAS_MIN    as ae_sys::A_long;
        const HAS_MAX    = ae_sys::AEGP_StreamFlag_HAS_MAX    as ae_sys::A_long;
        const IS_SPATIAL = ae_sys::AEGP_StreamFlag_IS_SPATIAL as ae_sys::A_long;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum StreamValue {
    None,
    FourD(
        ae_sys::A_FpLong,
        ae_sys::A_FpLong,
        ae_sys::A_FpLong,
        ae_sys::A_FpLong,
    ),
    ThreeD {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
        z: ae_sys::A_FpLong,
    },
    ThreeDSpatial {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
        z: ae_sys::A_FpLong,
    },
    TwoD {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
    },
    TwoDSpatial {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
    },
    OneD(ae_sys::A_FpLong),
    Color {
        alpha: ae_sys::A_FpLong,
        red: ae_sys::A_FpLong,
        green: ae_sys::A_FpLong,
        blue: ae_sys::A_FpLong,
    },
    // ArbBlock,     // FIXME
    // Marker,       // FIXME
    LayerId(ae_sys::AEGP_LayerIDVal),
    MaskId(ae_sys::AEGP_MaskIDVal),
    Mask(MaskOutlineHandle),
    TextDocument(TextDocumentHandle),
}

impl StreamValue {
    /// Convert the `ae_sys::AEGP_StreamVal2` to a [`StreamValue`].
    pub fn from_sys(type_: impl Into<StreamType>, val: ae_sys::AEGP_StreamVal2) -> Self {
        match type_.into() {
            StreamType::NoData => {
                Self::None
            },
            StreamType::ThreeDSpatial => unsafe {
                Self::ThreeDSpatial {
                    x: val.three_d.x,
                    y: val.three_d.y,
                    z: val.three_d.z,
                }
            },
            StreamType::ThreeD => unsafe {
                Self::ThreeD {
                    x: val.three_d.x,
                    y: val.three_d.y,
                    z: val.three_d.z,
                }
            },
            StreamType::TwoDSpatial => unsafe {
                Self::TwoDSpatial {
                    x: val.two_d.x,
                    y: val.two_d.y,
                }
            },
            StreamType::TwoD => unsafe {
                Self::TwoD {
                    x: val.two_d.x,
                    y: val.two_d.y,
                }
            },
            StreamType::OneD => unsafe {
                Self::OneD(val.one_d)
            },
            StreamType::Color => unsafe {
                Self::Color {
                    alpha: val.color.alphaF,
                    red:   val.color.redF,
                    green: val.color.greenF,
                    blue:  val.color.blueF,
                }
            },
            // StreamType::ArbBlock => unsafe {},
            // StreamType::Marker => unsafe {},
            StreamType::LayerId => unsafe {
                Self::LayerId(val.layer_id)
            },
            StreamType::MaskId => unsafe {
                Self::MaskId(val.mask_id)
            },
            StreamType::Mask => unsafe {
                Self::Mask(MaskOutlineHandle::from_raw(val.mask))
            },
            StreamType::TextDocument => unsafe {
                Self::TextDocument(TextDocumentHandle::from_raw(val.text_documentH))
            },
        }
    }

    /// Convert this [`StreamValue`] to a `ae_sys::AEGP_StreamVal2`.
    pub fn to_sys(&self) -> ae_sys::AEGP_StreamVal2 {
        use ae_sys::AEGP_StreamVal2;
        match self {
            Self::None => unsafe { std::mem::zeroed() },
            Self::FourD(x, y, z, w) => AEGP_StreamVal2 {
                four_d:  [*x, *y, *z, *w]
            },
            Self::ThreeD        { x, y, z } |
            Self::ThreeDSpatial { x, y, z } => AEGP_StreamVal2 {
                three_d: ae_sys::AEGP_ThreeDVal {
                    x: *x,
                    y: *y,
                    z: *z,
                }
            },
            Self::TwoD        { x, y } |
            Self::TwoDSpatial { x, y } => AEGP_StreamVal2 {
                two_d: ae_sys::AEGP_TwoDVal {
                    x: *x,
                    y: *y,
                }
            },
            Self::OneD(x) => AEGP_StreamVal2 {
                one_d: *x
            },
            Self::Color { alpha, red, green, blue } => AEGP_StreamVal2 {
                color: ae_sys::AEGP_ColorVal {
                    alphaF: *alpha,
                    redF:   *red,
                    greenF: *green,
                    blueF:  *blue,
                }
            },
            // Self::ArbBlock => {},
            // Self::Marker => {},
            Self::LayerId(x) => AEGP_StreamVal2 {
                layer_id: *x
            },
            Self::MaskId(x) => AEGP_StreamVal2 {
                mask_id: *x
            },
            Self::Mask(x) => AEGP_StreamVal2 {
                mask: x.as_ptr()
            },
            Self::TextDocument(x) => AEGP_StreamVal2 {
                text_documentH: x.as_ptr()
            },
        }
    }
}

impl TryFrom<StreamValue> for f32 {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as f32),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for f64 {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for usize {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as usize),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for u32 {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as u32),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for bool {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v != 0.0f64),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 2] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::TwoD { x, y } | StreamValue::TwoDSpatial { x, y } => {
                Ok([x as f32, y as f32])
            }
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 2] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::TwoD { x, y } | StreamValue::TwoDSpatial { x, y } => Ok([x, y]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 3] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::ThreeD { x, y, z } | StreamValue::ThreeDSpatial { x, y, z } => {
                Ok([x as f32, y as f32, z as f32])
            }
            StreamValue::Color {
                alpha: _,
                red,
                green,
                blue,
            } => Ok([red as f32, green as f32, blue as f32]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 3] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::ThreeD { x, y, z } | StreamValue::ThreeDSpatial { x, y, z } => {
                Ok([x, y, z])
            }
            StreamValue::Color {
                alpha: _,
                red,
                green,
                blue,
            } => Ok([red, green, blue]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 4] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::FourD(a, b, c, d) => Ok([a as f32, b as f32, c as f32, d as f32]),
            StreamValue::Color {
                alpha,
                red,
                green,
                blue,
            } => Ok([alpha as f32, red as f32, green as f32, blue as f32]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 4] {
    type Error = Error;

    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::FourD(a, b, c, d) => Ok([a, b, c, d]),
            StreamValue::Color {
                alpha,
                red,
                green,
                blue,
            } => Ok([alpha, red, green, blue]),
            _ => Err(Error::Parameter),
        }
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_StreamRefH, StreamReferenceHandle,
    suite: StreamSuite,
    dynamic: DynamicStreamSuite,
    /// Access and manipulate the values of a layer's streams. This also covers the functions for dynamic streams instead.
    Stream {
        dispose: ;

        /// Given a stream, returns whether or not a stream is time-variant (and can be keyframed).
        can_vary_over_time() -> bool => suite.can_vary_over_time,

        /// Retrieves an [`KeyframeInterpolationMask`] indicating which interpolation types are valid for the [`StreamReferenceHandle`].
        valid_interpolations() -> KeyframeInterpolationMask => suite.valid_interpolations,

        /// Get name of the stream (localized or forced English).
        ///
        /// NOTE: if `force_english` is `true`, the default name will override any stream renaming which has been done (either programatically, or by the user).
        name(plugin_id: PluginId, force_english: bool) -> String => suite.stream_name,

        /// Get stream units, formatted as text (localized or forced English).
        units_text(force_english: bool) -> String => suite.stream_units_text,

        /// Get stream's flags, as well as minimum and maximum values (as floats), if the stream *has* mins and maxes.
        ///
        /// Returns a tuple containing ([`StreamFlags`], `Option<min>`, `Option<max>`).
        properties() -> (StreamFlags, Option<f64>, Option<f64>) => suite.stream_properties,

        /// Returns whether or not the stream is affected by expressions.
        is_timevarying() -> bool => suite.is_stream_timevarying,

        /// Get type (dimension) of a stream.
        ///
        /// NOTE: always returns [`StreamType::ThreeDSpatial`] for position, regardless of whether or not the layer is 3D.
        stream_type() -> StreamType => suite.stream_type,

        /// Get value, at a time you specify, of stream. `value` must be disposed by the plug-in.
        /// The `time_mode` indicates whether the time is in compositions or layer time.
        new_value(plugin_id: PluginId, time_mode: TimeMode, time: Time, sample_stream_pre_expression: bool) -> StreamValue => suite.new_stream_value,

        /// Determines whether expressions are enabled on this stream.
        expression_state(plugin_id: PluginId) -> bool => suite.expression_state,

        /// Set whether expressions are enabled on this stream.
        set_expression_state(plugin_id: PluginId, enabled: bool) -> () => suite.set_expression_state,

        /// Get the expression string for this stream.
        expression_string(plugin_id: PluginId) -> String => suite.expression_string,

        /// Set the expression string for this stream.
        set_expression_string(plugin_id: PluginId, expression: &str) -> () => suite.set_expression_string,

        /// Duplicate a given [`Stream`].
        duplicate(plugin_id: PluginId) -> Stream => suite.duplicate_stream,

        /// Get the unique ID of this stream.
        unique_id() -> i32 => suite.unique_stream_id,

        // ―――――――――――――――――――――――――――― Dynamic stream suite functions ――――――――――――――――――――――――――――

        /// Retrieves the number of sub-streams associated with the given [`Stream`].
        ///
        /// The initial layer has a depth of 0.
        stream_depth() -> i32 => dynamic.stream_depth,

        /// Retrieves the grouping type for the given [`Stream`].
        stream_grouping_type() -> StreamGroupingType => dynamic.stream_grouping_type,

        /// Retrieves the number of streams associated with the given [`Stream`].
        ///
        /// This function will return an error if called with an [`Stream`] with type [`StreamGroupingType::Leaf`].
        num_streams_in_group() -> i32 => dynamic.num_streams_in_group,

        /// Retrieves the flags for a given [`Stream`].
        dynamic_stream_flags() -> DynamicStreamFlags => dynamic.dynamic_stream_flags,

        /// Sets the specified flag for the [`Stream`].
        ///
        /// Note: flags must be set individually. Undoable if `undoable` is ``true`.
        ///
        /// This call may be used to dynamically show or hide parameters, by setting and clearing [`DynamicStreamFlags::Hidden`].
        /// However, [`DynamicStreamFlags::Disabled`] may not be set.
        set_dynamic_stream_flag(flag: DynamicStreamFlags, undoable: bool, enabled: bool) -> () => dynamic.set_dynamic_stream_flag,

        /// Retrieves a sub-stream by index from a given [`Stream`]. Cannot be used on streams of type [`StreamGroupingType::Leaf`].
        new_stream_by_index(plugin_id: PluginId, index: i32) -> StreamReferenceHandle => dynamic.new_stream_ref_by_index,

        /// Retrieves a sub-stream by match name from a given [`Stream`]. Only legal for [`StreamGroupingType::NamedGroup`].
        ///
        /// Here are some handy stream names, for which references may be retrieved:
        /// * `"ADBE Mask Parade"`
        /// * `"ADBE Mask Atom"`
        /// * `"ADBE Mask Feather"`
        /// * `"ADBE Mask Opacity"`
        /// * `"ADBE Mask Offset"`
        /// * `"ADBE Effect Parade"`
        /// * `"ADBE Abstract Layer"`
        /// * `"ADBE AV Layer"`
        /// * `"ADBE Text Layer"`
        /// * `"ADBE Camera Layer"`
        /// * `"ADBE Light Layer"`
        /// * `"ADBE Audio Group"`
        /// * `"ADBE Material Options Group"`
        /// * `"ADBE Transform Group"`
        /// * `"ADBE Light Options Group"`
        /// * `"ADBE Camera Options Group"`
        new_stream_by_match_name(plugin_id: PluginId, match_name: &str) -> Stream => dynamic.new_stream_ref_by_match_name,

        /// Deletes the specified stream from a stream grouping.
        /// Note that the caller must still dispose of any [`Stream`] it's already acquired (allocated) via the API. Undoable.
        /// Only valid for children of type [`StreamGroupingType::IndexedGroup`].
        ///
        /// Note: as of 6.5, if a stream is deleted while it or any child stream is selected, the current composition selection will become `null`.
        delete() -> () => dynamic.delete_stream,

        /// Sets the new index of the specified [`Stream`]. Undoable.
        /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
        /// The [`Stream`] is updated to refer to the newly-ordered stream.
        reorder(new_index: i32) -> () => dynamic.reorder_stream,

        /// Duplicates the specified stream and appends it to the stream group. Undoable.
        /// Only valid for children of type [`StreamGroupingType::IndexedGroup`].
        duplicate_dynamic_stream(plugin_id: PluginId) -> i32 => dynamic.duplicate_stream,

        /// Sets the name of the given [`Stream`]. Undoable.
        /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
        ///
        /// NOTE: If you retrieve the name with `force_english` set to `true`, you will get the canonical, unchanged name of the stream.
        ///
        /// Note: Use this on an effect stream's group to change the display name of an effect.
        set_name(name: &str) -> () => dynamic.set_stream_name,

        /// Returns whether or not it is currently possible to add a stream through the API.
        can_add_stream(match_name: &str) -> bool => dynamic.can_add_stream,

        /// Adds a stream to the specified stream group. Undoable. Only valid for [`StreamGroupingType::IndexedGroup`].
        add_stream(plugin_id: PluginId, match_name: &str) -> StreamReferenceHandle => dynamic.add_stream,

        /// Retrieves the match name for this stream.
        ///
        /// Note that this may differ from the display name, which can be retrieves using [`Stream::name()`].
        match_name() -> String => dynamic.match_name,

        /// Retrieves an [`Stream`] for the parent of this stream.
        new_parent_stream(plugin_id: PluginId) -> Stream => dynamic.new_parent_stream_ref,

        /// Returns whether or not the specified [`Stream`] has been modified.
        ///
        /// Note: the same result is available through the After Effect user interface by typing "UU" with the composition selected.
        is_modified() -> bool => dynamic.stream_is_modified,

        /// Retrieves the index of a given stream, relative to its parent stream.
        ///
        /// Only valid for children of [`StreamGroupingType::IndexedGroup`].
        ///
        /// NOTE: As mentioned *elsewhere*, [`Stream`]s don't persist across function calls.
        /// If streams are re-ordered, added or removed, all [`Stream`]s previously retrieved may be invalidated.
        stream_index_in_parent() -> i32 => dynamic.stream_index_in_parent,

        /// Valid on leaf streams only. Returns true if this stream is a multidimensional stream that can have its dimensions separated, though they may not be currently separated.
        ///
        /// Terminology: A Leader is the stream that can be separated, a Follower is one of N automatic streams that correspond to the N dimensions of the Leader.
        ///
        /// A Leader isn't always separated, call [`are_dimensions_separated()`](Self::are_dimensions_separated) to find out if it is.
        /// As of CS4, the only stream that is ever separarated is the layer's Position property.
        /// Please *do not* write code assuming that, we anticipate allowing separation of more streams in the future.
        is_separation_leader() -> bool => dynamic.is_separation_leader,

        /// Methods such as [`new_keyframe_value()`](aegp::suites::Keyframe::new_keyframe_value) that work on keyframe indices will most definitely *not* work on the Leader property, you will need to retrieve and operate on the Followers explicitly.
        are_dimensions_separated() -> bool => dynamic.are_dimensions_separated,

        /// Valid only if [`is_separation_leader()`](Self::is_separation_leader) is `true`.
        set_dimensions_separated(separated: bool) -> () => dynamic.set_dimensions_separated,

        /// Retrieve the Follower stream corresponding to a given dimension of the Leader stream.
        ///
        /// `dim` can range from `0` to `AEGP_GetStreamValueDimensionality(leader_streamH) - 1`.
        separation_follower(dimension: i16) -> Stream => dynamic.separation_follower,

        /// Valid on leaf streams only.
        /// Returns `true` if this stream is a one dimensional property that represents one of the dimensions of a Leader.
        /// You can retrieve stream from the Leader using [`separation_follower()`](Self::separation_follower).
        is_separation_follower() -> bool => dynamic.is_separation_follower,

        /// Valid on separation Followers only, returns the Leader it is part of.
        separation_leader() -> Stream => dynamic.separation_leader,

        /// Valid on separation Followers only, returns which dimension of the Leader it corresponds to.
        separation_dimension() -> i16 => dynamic.separation_dimension,
    }
);

impl Stream {
    /// Returns the [`Keyframes`] struct to manipulate keyframes on this stream.
    pub fn keyframes(&self) -> Result<Keyframes, Error> {
        Ok(Keyframes::from_handle(StreamReferenceHandle::from_raw(self.handle.as_ptr()), false))
    }
}
