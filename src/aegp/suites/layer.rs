use crate::*;
use crate::aegp::*;
use ae_sys::AEGP_LayerH;

define_suite!(
    /// This suite provides information about layers within a composition, and the relationship(s) between the source and layer times.
    ///
    /// As most After Effects usage boils down to layer manipulation, this is among the largest function suites in our API.
    ///
    /// ## Layer Creation Notes
    /// All layers created using AEGP calls will start at composition time 0, and have the duration of the composition.
    ///
    /// Use [`suites::Layer::set_layer_offset()`] and [`suites::Layer::set_layer_in_point_and_duration()`] to properly set the layer's time information.
    ///
    /// When the layer stretch factor (obtained using [`suites::Layer::layer_stretch()`], naturally) is not 100%, the following computation will be needed to yield the correct layer offset:
    /// ```
    /// offset = compIn - stretch * layerIn;
    /// ```
    LayerSuite,
    AEGP_LayerSuite9,
    kAEGPLayerSuite,
    kAEGPLayerSuiteVersion9
);

impl LayerSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Obtains the number of layers in a composition.
    pub fn comp_num_layers(&self, comp_handle: &CompHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCompNumLayers -> i32, comp_handle.as_ptr())? as usize)
    }

    /// Get a [`LayerHandle`] from a composition. Zero is the foremost layer.
    pub fn comp_layer_by_index(&self, comp_handle: &CompHandle, layer_index: usize) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetCompLayerByIndex -> ae_sys::AEGP_LayerH, comp_handle.as_ptr(), layer_index as _)?
        ))
    }

    /// Get the active layer. If a Layer or effect controls palette is active, the active layer is that associated with the front-most tab in the window.
    ///
    /// If a composition or timeline window is active, the active layer is the selected layer (if only one is selected; otherwise `None` is returned).
    pub fn active_layer(&self) -> Result<Option<LayerHandle>, Error> {
        let layer_handle = call_suite_fn_single!(self, AEGP_GetActiveLayer -> ae_sys::AEGP_LayerH)?;
        if layer_handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(LayerHandle::from_raw(layer_handle)))
        }
    }

    /// Get the index of the layer (0 is the topmost layer in the composition).
    pub fn layer_index(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerIndex -> i32, layer_handle.as_ptr())? as usize)
    }

    /// Get the [`ItemHandle`] of the layer's source item.
    pub fn layer_source_item(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetLayerSourceItem -> ae_sys::AEGP_ItemH, layer_handle.as_ptr())?
        ))
    }

    /// Retrieves the ID of the given [`LayerHandle`].
    ///
    /// This is useful when hunting for a specific layer's ID in an [`StreamValue`].
    pub fn layer_source_item_id(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerSourceItemID -> i32, layer_handle.as_ptr())?)
    }

    /// Get the AEGP_CompH of the composition containing the layer.
    pub fn layer_parent_comp(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetLayerParentComp -> ae_sys::AEGP_CompH, layer_handle.as_ptr())?
        ))
    }

    /// Get the name of a layer.
    pub fn layer_name(&self, layer_handle: impl AsPtr<AEGP_LayerH>, plugin_id: PluginId) -> Result<(String, String), Error> {
        let (layer_name, source_name) = call_suite_fn_double!(self, AEGP_GetLayerName ->ae_sys::AEGP_MemHandle, ae_sys::AEGP_MemHandle, plugin_id, layer_handle.as_ptr())?;
        unsafe {
            Ok((
                // Create a mem handle each and lock it.
                // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
                U16CString::from_ptr_str(MemHandle::<u16>::from_raw(layer_name)?.lock()?.as_ptr()).to_string_lossy(),
                U16CString::from_ptr_str(MemHandle::<u16>::from_raw(source_name)?.lock()?.as_ptr()).to_string_lossy()
            ))
        }
    }

    /// Get the quality of a layer.
    pub fn layer_quality(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerQuality, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerQuality -> ae_sys::AEGP_LayerQuality, layer_handle.as_ptr())?.into())
    }

    /// Sets the quality of a layer. Undoable.
    pub fn set_layer_quality(&self, layer_handle: impl AsPtr<AEGP_LayerH>, quality: LayerQuality) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerQuality, layer_handle.as_ptr(), quality.into())
    }

    /// Get flags for a layer.
    pub fn layer_flags(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerFlags, Error> {
        Ok(LayerFlags::from_bits_truncate(
            call_suite_fn_single!(self, AEGP_GetLayerFlags -> ae_sys::AEGP_LayerFlags, layer_handle.as_ptr())?
        ))
    }

    /// Sets one layer flag at a time. Undoable.
    pub fn set_layer_flag(&self, layer_handle: impl AsPtr<AEGP_LayerH>, single_flag: LayerFlags, value: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerFlag, layer_handle.as_ptr(), single_flag.bits(), value as _)
    }

    /// Determines whether the layer's video is visible.
    ///
    /// This is necessary to account for 'solo' status of other layers in the composition; non-solo'd layers are still on.
    pub fn is_layer_video_really_on(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsLayerVideoReallyOn -> ae_sys::A_Boolean, layer_handle.as_ptr())? != 0)
    }

    /// Accounts for solo status of other layers in the composition.
    pub fn is_layer_audio_really_on(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsLayerAudioReallyOn -> ae_sys::A_Boolean, layer_handle.as_ptr())? != 0)
    }

    /// Get current time, in layer or composition timespace. This value is not updated during rendering.
    ///
    /// NOTE: If a layer starts at other than time 0 or is time-stretched other than 100%, layer time and composition time are distinct.
    pub fn layer_current_time(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time_mode: TimeMode) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerCurrentTime -> ae_sys::A_Time, layer_handle.as_ptr(), time_mode.into())?.into())
    }

    /// Get time of first visible frame in composition or layer time.
    ///
    /// In layer time, the `in_point` is always 0.
    pub fn layer_in_point(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time_mode: TimeMode) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerInPoint -> ae_sys::A_Time, layer_handle.as_ptr(), time_mode.into())?.into())
    }

    /// Get duration of layer, in composition or layer time, in seconds.
    pub fn layer_duration(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time_mode: TimeMode) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerDuration -> ae_sys::A_Time, layer_handle.as_ptr(), time_mode.into())?.into())
    }

    /// Set duration and in point of layer in composition or layer time. Undo-able.
    pub fn set_layer_in_point_and_duration(&self, layer_handle: impl AsPtr<AEGP_LayerH>, in_point: Time, duration: Time, time_mode: TimeMode) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerInPointAndDuration, layer_handle.as_ptr(), time_mode.into(), &in_point.into() as *const _, &duration.into() as *const _)
    }

    /// Get the offset from the start of the composition to layer time 0, in composition time.
    pub fn layer_offset(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerOffset -> ae_sys::A_Time, layer_handle.as_ptr())?.into())
    }

    /// Set the offset from the start of the composition to the first frame of the layer, in composition time. Undoable.
    pub fn set_layer_offset(&self, layer_handle: impl AsPtr<AEGP_LayerH>, offset: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerOffset, layer_handle.as_ptr(), &offset.into() as *const _)
    }

    /// Get stretch factor of a layer.
    pub fn layer_stretch(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<Ratio, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerStretch -> ae_sys::A_Ratio, layer_handle.as_ptr())?.into())
    }

    /// Set stretch factor of a layer.
    pub fn set_layer_stretch(&self, layer_handle: impl AsPtr<AEGP_LayerH>, stretch: Ratio) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerStretch, layer_handle.as_ptr(), &stretch.into() as *const _)
    }

    /// Get transfer mode of a layer.
    pub fn layer_transfer_mode(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<ae_sys::AEGP_LayerTransferMode, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerTransferMode -> ae_sys::AEGP_LayerTransferMode, layer_handle.as_ptr())?.into())
    }

    /// Set transfer mode of a layer. Undoable.
    ///
    /// As of 23.0, when you make a layer a track matte, the layer being matted will be disabled,
    /// as when you do this via the interface.
    pub fn set_layer_transfer_mode(&self, layer_handle: impl AsPtr<AEGP_LayerH>, transfer_mode: &ae_sys::AEGP_LayerTransferMode) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerTransferMode, layer_handle.as_ptr(), transfer_mode)
    }

    /// Tests whether it's currently valid to add a given item to a composition.
    ///
    /// A composition cannot be added to itself, or to any compositions which it contains; other conditions can preclude successful adding too.
    ///
    /// Adding a layer without first using this function will produce undefined results.
    pub fn is_add_layer_valid(&self, item_handle: &ItemHandle, comp_handle: &CompHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsAddLayerValid -> ae_sys::A_Boolean, item_handle.as_ptr(), comp_handle.as_ptr())? != 0)
    }

    /// Add an item to the composition, above all other layers. Undo-able.
    ///
    /// Use [`Self::is_add_layer_valid()`] first, to confirm that it's possible.
    pub fn add_layer(&self, item_handle: &ItemHandle, comp_handle: &CompHandle) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_AddLayer -> ae_sys::AEGP_LayerH, item_handle.as_ptr(), comp_handle.as_ptr())?
        ))
    }

    /// Change the order of layers. Undoable.
    ///
    /// To add a layer to the end of the composition, to use `layer_index = -1`
    pub fn reorder_layer(&self, layer_handle: impl AsPtr<AEGP_LayerH>, layer_index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReorderLayer, layer_handle.as_ptr(), layer_index)
    }

    /// Given a layer's handle and a time, returns the bounds of area visible with masks applied.
    pub fn layer_masked_bounds(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time_mode: TimeMode, time: Time) -> Result<FloatRect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerMaskedBounds -> ae_sys::A_FloatRect, layer_handle.as_ptr(), time_mode.into(), &time.into() as *const _)?.into())
    }

    /// Returns a layer's object type.
    pub fn layer_object_type(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<ObjectType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerObjectType -> ae_sys::AEGP_ObjectType, layer_handle.as_ptr())?.into())
    }

    /// Is the footage item a 3D layer. All AV layers are either 2D or 3D.
    pub fn is_layer_3d(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsLayer3D -> ae_sys::A_Boolean, layer_handle.as_ptr())? != 0)
    }

    /// Is the footage item a 2D layer. All AV layers are either 2D or 3D.
    pub fn is_layer_2d(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsLayer2D -> ae_sys::A_Boolean, layer_handle.as_ptr())? != 0)
    }

    /// Given composition time and a layer, see if the layer will render.
    ///
    /// Time mode is either [`TimeMode::LayerTime`] or [`TimeMode::CompTime`].
    pub fn is_video_active(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time_mode: TimeMode, time: Time) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsVideoActive -> ae_sys::A_Boolean, layer_handle.as_ptr(), time_mode.into(), &time.into() as *const _)? != 0)
    }

    /// Is the layer used as a track matte?
    pub fn is_layer_used_as_track_matte(&self, layer_handle: impl AsPtr<AEGP_LayerH>, fill_must_be_active: bool) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsLayerUsedAsTrackMatte -> ae_sys::A_Boolean, layer_handle.as_ptr(), fill_must_be_active as _)? != 0)
    }

    /// Does this layer have a Track Matte?
    pub fn does_layer_have_track_matte(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_DoesLayerHaveTrackMatte -> ae_sys::A_Boolean, layer_handle.as_ptr())? != 0)
    }

    /// Given a time in composition space, returns the time relative to the layer source footage.
    pub fn convert_comp_to_layer_time(&self, layer_handle: impl AsPtr<AEGP_LayerH>, comp_time: Time) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ConvertCompToLayerTime -> ae_sys::A_Time, layer_handle.as_ptr(), &comp_time.into() as *const _)?.into())
    }

    /// Given a time in layer space, find the corresponding time in composition space.
    pub fn convert_layer_to_comp_time(&self, layer_handle: impl AsPtr<AEGP_LayerH>, layer_time: Time) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ConvertLayerToCompTime -> ae_sys::A_Time, layer_handle.as_ptr(), &layer_time.into() as *const _)?.into())
    }

    /// Used by the dancing dissolve transfer function.
    pub fn layer_dancing_rand_value(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time: Time) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerDancingRandValue -> ae_sys::A_long, layer_handle.as_ptr(), &time.into() as *const _)?.into())
    }

    /// Supplies the layer's unique ID. This ID never changes during the lifetime of the project.
    pub fn layer_id(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerId, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerID -> ae_sys::AEGP_LayerIDVal, layer_handle.as_ptr())? as LayerId)
    }

    /// Given a layer handle and time, returns the layer-to-world transformation matrix.
    pub fn layer_to_world_xform(&self, layer_handle: impl AsPtr<AEGP_LayerH>, time: Time) -> Result<Matrix4, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerToWorldXform -> ae_sys::A_Matrix4, layer_handle.as_ptr(), &time.into() as *const _)?.into())
    }

    /// Given a layer handle, the current (composition) time, and the requested view time, returns the translation between the user's view and the layer, corrected for the composition's current aspect ratio.
    pub fn layer_to_world_xform_from_view(&self, layer_handle: impl AsPtr<AEGP_LayerH>, comp_time: Time, view_time: Time) -> Result<Matrix4, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerToWorldXformFromView -> ae_sys::A_Matrix4, layer_handle.as_ptr(), &comp_time.into() as *const _, &view_time.into() as *const _)?.into())
    }

    /// Sets the name of a layer. Undo-able.
    pub fn set_layer_name(&self, layer_handle: impl AsPtr<AEGP_LayerH>, new_name: &str) -> Result<(), Error> {
        let new_name = U16CString::from_str(new_name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetLayerName, layer_handle.as_ptr(), new_name.as_ptr())
    }

    /// Retrieves the handle to a layer's parent (none if not parented).
    pub fn layer_parent(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<Option<LayerHandle>, Error> {
        let parent_handle = call_suite_fn_single!(self, AEGP_GetLayerParent -> ae_sys::AEGP_LayerH, layer_handle.as_ptr())?;
        if parent_handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(LayerHandle::from_raw(parent_handle)))
        }
    }

    /// Sets a layer's parent layer.
    pub fn set_layer_parent(&self, layer_handle: impl AsPtr<AEGP_LayerH>, parent_handle: LayerHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerParent, layer_handle.as_ptr(), parent_handle.as_ptr())
    }

    /// Deletes a layer. Can you believe it took us three suite versions to add a delete function? Neither can we.
    pub fn delete_layer(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteLayer, layer_handle.as_ptr())
    }

    /// Duplicates the layer. Undoable.
    pub fn duplicate_layer(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_DuplicateLayer -> ae_sys::AEGP_LayerH, layer_handle.as_ptr())?
        ))
    }

    /// Retrieves the [`LayerHandle`] associated with a given [`LayerId`] (which is what you get when accessing an effect's layer parameter stream).
    pub fn layer_from_layer_id(&self, parent: &CompHandle, layer_id: LayerId) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetLayerFromLayerID -> ae_sys::AEGP_LayerH, parent.as_ptr(), layer_id as _)?
        ))
    }

    /// Gets a layer's [`LabelId`].
    pub fn layer_label(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LabelId, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerLabel -> ae_sys::AEGP_LabelID, layer_handle.as_ptr())?.into())
    }

    /// Sets a layer's [`LabelId`]. Undoable.
    pub fn set_layer_label(&self, layer_handle: impl AsPtr<AEGP_LayerH>, label_id: LabelId) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerLabel, layer_handle.as_ptr(), label_id.into())
    }

    /// New in CC. Get the sampling quality of a layer.
    ///
    /// Layer sampling quality is one of the following flags:
    ///
    /// - [`LayerSamplingQuality::Bilinear`]
    /// - [`LayerSamplingQuality::Bicubic`]
    pub fn layer_sampling_quality(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerSamplingQuality, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerSamplingQuality -> ae_sys::AEGP_LayerSamplingQuality, layer_handle.as_ptr())?.into())
    }

    /// New in CC. Sets the sampling quality of a layer (see flag values above).
    ///
    /// Option is explicitly set on the layer independent of layer quality.
    ///
    /// If you want to force it on you must also set the layer quality to [`LayerQuality::Best`] with [`Self::set_layer_quality`].
    /// Otherwise it will only be using the specified layer sampling quality whenever the layer quality is set to [`LayerQuality::Best`].
    pub fn set_layer_sampling_quality(&self, layer_handle: impl AsPtr<AEGP_LayerH>, quality: LayerSamplingQuality) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerSamplingQuality, layer_handle.as_ptr(), quality.into())
    }

    /// New in 23.0. Returns the track matte layer of [`LayerHandle`]. Returns `None` if there is no track matte layer.
    pub fn track_matte_layer(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<Option<LayerHandle>, Error> {
        let track_matte_handle = call_suite_fn_single!(self, AEGP_GetTrackMatteLayer -> ae_sys::AEGP_LayerH, layer_handle.as_ptr())?;
        if track_matte_handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(LayerHandle::from_raw(track_matte_handle)))
        }
    }

    /// New in 23.0. Sets the track matte layer and track matte type of [`LayerHandle`].
    ///
    /// Setting the track matte type as [`TrackMatte::NoTrackMatte`] removes track matte.
    pub fn set_track_matte(&self, layer_handle: impl AsPtr<AEGP_LayerH>, track_matte_layer: Option<LayerHandle>, track_matte_type: TrackMatte) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetTrackMatte, layer_handle.as_ptr(), track_matte_layer.map_or(std::ptr::null_mut(), |h| h.as_ptr()), track_matte_type.into())
    }

    /// New in 23.0. Removes the track matte layer of [`LayerHandle`].
    pub fn remove_track_matte(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveTrackMatte, layer_handle.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_LayerH);
define_handle_wrapper!(LayerHandle, AEGP_LayerH);

define_enum! {
    ae_sys::AEGP_ObjectType,
    ObjectType {
        None       = ae_sys::AEGP_ObjectType_NONE,
        AudioVideo = ae_sys::AEGP_ObjectType_AV,
        Light      = ae_sys::AEGP_ObjectType_LIGHT,
        Camera     = ae_sys::AEGP_ObjectType_CAMERA,
        Text       = ae_sys::AEGP_ObjectType_TEXT,
        Vector     = ae_sys::AEGP_ObjectType_VECTOR,
    }
}

define_enum! {
    ae_sys::AEGP_LayerQuality,
    LayerQuality {
        None      = ae_sys::AEGP_LayerQual_NONE,
        Wireframe = ae_sys::AEGP_LayerQual_WIREFRAME,
        Draft     = ae_sys::AEGP_LayerQual_DRAFT,
        Best      = ae_sys::AEGP_LayerQual_BEST,
    }
}

define_enum! {
    ae_sys::AEGP_LayerSamplingQuality,
    LayerSamplingQuality {
        Bilinear = ae_sys::AEGP_LayerSamplingQual_BILINEAR,
        Bicubic  = ae_sys::AEGP_LayerSamplingQual_BICUBIC,
    }
}

define_enum! {
    ae_sys::AEGP_TrackMatte,
    TrackMatte {
        NoTrackMatte = ae_sys::AEGP_TrackMatte_NO_TRACK_MATTE,
        Alpha        = ae_sys::AEGP_TrackMatte_ALPHA,
        NotAlpha     = ae_sys::AEGP_TrackMatte_NOT_ALPHA,
        Luma         = ae_sys::AEGP_TrackMatte_LUMA,
        NotLuma      = ae_sys::AEGP_TrackMatte_NOT_LUMA,
    }
}

define_enum! {
    ae_sys::AEGP_LTimeMode,
    TimeMode {
        LayerTime = ae_sys::AEGP_LTimeMode_LayerTime,
        CompTime  = ae_sys::AEGP_LTimeMode_CompTime,
    }
}

bitflags::bitflags! {
    pub struct LayerFlags: ae_sys::A_long {
        const NONE                        = ae_sys::AEGP_LayerFlag_NONE                        as ae_sys::A_long;
        const VIDEO_ACTIVE                = ae_sys::AEGP_LayerFlag_VIDEO_ACTIVE                as ae_sys::A_long;
        const AUDIO_ACTIVE                = ae_sys::AEGP_LayerFlag_AUDIO_ACTIVE                as ae_sys::A_long;
        const EFFECTS_ACTIVE              = ae_sys::AEGP_LayerFlag_EFFECTS_ACTIVE              as ae_sys::A_long;
        const MOTION_BLUR                 = ae_sys::AEGP_LayerFlag_MOTION_BLUR                 as ae_sys::A_long;
        const FRAME_BLENDING              = ae_sys::AEGP_LayerFlag_FRAME_BLENDING              as ae_sys::A_long;
        const LOCKED                      = ae_sys::AEGP_LayerFlag_LOCKED                      as ae_sys::A_long;
        const SHY                         = ae_sys::AEGP_LayerFlag_SHY                         as ae_sys::A_long;
        const COLLAPSE                    = ae_sys::AEGP_LayerFlag_COLLAPSE                    as ae_sys::A_long;
        const AUTO_ORIENT_ROTATION        = ae_sys::AEGP_LayerFlag_AUTO_ORIENT_ROTATION        as ae_sys::A_long;
        const ADJUSTMENT_LAYER            = ae_sys::AEGP_LayerFlag_ADJUSTMENT_LAYER            as ae_sys::A_long;
        const TIME_REMAPPING              = ae_sys::AEGP_LayerFlag_TIME_REMAPPING              as ae_sys::A_long;
        const LAYER_IS_3D                 = ae_sys::AEGP_LayerFlag_LAYER_IS_3D                 as ae_sys::A_long;
        const LOOK_AT_CAMERA              = ae_sys::AEGP_LayerFlag_LOOK_AT_CAMERA              as ae_sys::A_long;
        const LOOK_AT_POI                 = ae_sys::AEGP_LayerFlag_LOOK_AT_POI                 as ae_sys::A_long;
        const SOLO                        = ae_sys::AEGP_LayerFlag_SOLO                        as ae_sys::A_long;
        const MARKERS_LOCKED              = ae_sys::AEGP_LayerFlag_MARKERS_LOCKED              as ae_sys::A_long;
        const NULL_LAYER                  = ae_sys::AEGP_LayerFlag_NULL_LAYER                  as ae_sys::A_long;
        const HIDE_LOCKED_MASKS           = ae_sys::AEGP_LayerFlag_HIDE_LOCKED_MASKS           as ae_sys::A_long;
        const GUIDE_LAYER                 = ae_sys::AEGP_LayerFlag_GUIDE_LAYER                 as ae_sys::A_long;
        /// `true` only if pixel motion frame blending is on for the layer.
        const ADVANCED_FRAME_BLENDING     = ae_sys::AEGP_LayerFlag_ADVANCED_FRAME_BLENDING     as ae_sys::A_long;
        /// Used to get/set the state of per-character 3D enablement on a text layer.
        const SUBLAYERS_RENDER_SEPARATELY = ae_sys::AEGP_LayerFlag_SUBLAYERS_RENDER_SEPARATELY as ae_sys::A_long;
        const ENVIRONMENT_LAYER           = ae_sys::AEGP_LayerFlag_ENVIRONMENT_LAYER           as ae_sys::A_long;
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_LayerH, LayerHandle,
    suite: LayerSuite,
    /// A layer
    Layer {
        dispose: ;

        /// Get the index of the layer (0 is the topmost layer in the composition).
        index() -> usize => suite.layer_index,

        /// Get the [`ItemHandle`] of the layer's source item.
        source_item() -> Item => suite.layer_source_item,

        /// Retrieves the ID of the given [`LayerHandle`].
        ///
        /// This is useful when hunting for a specific layer's ID in an [`StreamValue`].
        source_item_id() -> i32 => suite.layer_source_item_id,

        /// Get the AEGP_CompH of the composition containing the layer.
        parent_comp() -> CompHandle => suite.layer_parent_comp,

        /// Get the name of a layer.
        name(plugin_id: PluginId) -> (String, String) => suite.layer_name,

        /// Get the quality of a layer.
        quality() -> LayerQuality => suite.layer_quality,

        /// Sets the quality of a layer. Undoable.
        set_quality(quality: LayerQuality) -> () => suite.set_layer_quality,

        /// Get flags for a layer.
        flags() -> LayerFlags => suite.layer_flags,

        /// Sets one layer flag at a time. Undoable.
        set_flag(single_flag: LayerFlags, value: bool) -> () => suite.set_layer_flag,

        /// Determines whether the layer's video is visible.
        ///
        /// This is necessary to account for 'solo' status of other layers in the composition; non-solo'd layers are still on.
        is_video_really_on() -> bool => suite.is_layer_video_really_on,

        /// Accounts for solo status of other layers in the composition.
        is_audio_really_on() -> bool => suite.is_layer_audio_really_on,

        /// Get current time, in layer or composition timespace. This value is not updated during rendering.
        ///
        /// NOTE: If a layer starts at other than time 0 or is time-stretched other than 100%, layer time and composition time are distinct.
        current_time(time_mode: TimeMode) -> Time => suite.layer_current_time,

        /// Get time of first visible frame in composition or layer time.
        ///
        /// In layer time, the `in_point` is always 0.
        in_point(time_mode: TimeMode) -> Time => suite.layer_in_point,

        /// Get duration of layer, in composition or layer time, in seconds.
        duration(time_mode: TimeMode) -> Time => suite.layer_duration,

        /// Set duration and in point of layer in composition or layer time. Undo-able.
        set_in_point_and_duration(in_point: Time, duration: Time, time_mode: TimeMode) -> () => suite.set_layer_in_point_and_duration,

        /// Get the offset from the start of the composition to layer time 0, in composition time.
        offset() -> Time => suite.layer_offset,

        /// Set the offset from the start of the composition to the first frame of the layer, in composition time. Undoable.
        set_offset(offset: Time) -> () => suite.set_layer_offset,

        /// Get stretch factor of a layer.
        stretch() -> Ratio => suite.layer_stretch,

        /// Set stretch factor of a layer.
        set_stretch(stretch: Ratio) -> () => suite.set_layer_stretch,

        /// Get transfer mode of a layer.
        transfer_mode() -> ae_sys::AEGP_LayerTransferMode => suite.layer_transfer_mode,

        /// Set transfer mode of a layer. Undoable.
        ///
        /// As of 23.0, when you make a layer a track matte, the layer being matted will be disabled,
        /// as when you do this via the interface.
        set_transfer_mode(transfer_mode: &ae_sys::AEGP_LayerTransferMode) -> () => suite.set_layer_transfer_mode,

        /// Change the order of layers. Undoable.
        ///
        /// To add a layer to the end of the composition, to use `layer_index = -1`
        reorder(layer_index: i32) -> () => suite.reorder_layer,

        /// Given a layer's handle and a time, returns the bounds of area visible with masks applied.
        masked_bounds(time_mode: TimeMode, time: Time) -> FloatRect => suite.layer_masked_bounds,

        /// Returns a layer's object type.
        object_type() -> ObjectType => suite.layer_object_type,

        /// Is the footage item a 3D layer. All AV layers are either 2D or 3D.
        is_3d() -> bool => suite.is_layer_3d,

        /// Is the footage item a 2D layer. All AV layers are either 2D or 3D.
        is_2d() -> bool => suite.is_layer_2d,

        /// Given composition time and a layer, see if the layer will render.
        ///
        /// Time mode is either [`TimeMode::LayerTime`] or [`TimeMode::CompTime`].
        is_video_active(time_mode: TimeMode, time: Time) -> bool => suite.is_video_active,

        /// Is the layer used as a track matte?
        is_used_as_track_matte(fill_must_be_active: bool) -> bool => suite.is_layer_used_as_track_matte,

        /// Does this layer have a Track Matte?
        does_layer_have_track_matte() -> bool => suite.does_layer_have_track_matte,

        /// Given a time in composition space, returns the time relative to the layer source footage.
        convert_comp_to_layer_time(comp_time: Time) -> Time => suite.convert_comp_to_layer_time,

        /// Given a time in layer space, find the corresponding time in composition space.
        convert_layer_to_comp_time(layer_time: Time) -> Time => suite.convert_layer_to_comp_time,

        /// Used by the dancing dissolve transfer function.
        dancing_rand_value(time: Time) -> i32 => suite.layer_dancing_rand_value,

        /// Supplies the layer's unique ID. This ID never changes during the lifetime of the project.
        id() -> LayerId => suite.layer_id,

        /// Given a layer handle and time, returns the layer-to-world transformation matrix.
        to_world_xform(time: Time) -> Matrix4 => suite.layer_to_world_xform,

        /// Given a layer handle, the current (composition) time, and the requested view time, returns the translation between the user's view and the layer, corrected for the composition's current aspect ratio.
        to_world_xform_from_view(comp_time: Time, view_time: Time) -> Matrix4 => suite.layer_to_world_xform_from_view,

        /// Sets the name of a layer. Undo-able.
        set_name(new_name: &str) -> () => suite.set_layer_name,

        /// Retrieves the handle to a layer's parent (none if not parented).
        parent() -> Option<LayerHandle> => suite.layer_parent,

        /// Sets a layer's parent layer.
        set_parent(parent_handle: LayerHandle) -> () => suite.set_layer_parent,

        /// Deletes a layer. Can you believe it took us three suite versions to add a delete function? Neither can we.
        delete() -> () => suite.delete_layer,

        /// Duplicates the layer. Undoable.
        duplicate() -> Layer => suite.duplicate_layer,

        /// Gets a layer's [`LabelId`].
        label() -> LabelId => suite.layer_label,

        /// Sets a layer's [`LabelId`]. Undoable.
        set_label(label_id: LabelId) -> () => suite.set_layer_label,

        /// New in CC. Get the sampling quality of a layer.
        ///
        /// Layer sampling quality is one of the following flags:
        ///
        /// - [`LayerSamplingQuality::Bilinear`]
        /// - [`LayerSamplingQuality::Bicubic`]
        sampling_quality() -> LayerSamplingQuality => suite.layer_sampling_quality,

        /// New in CC. Sets the sampling quality of a layer (see flag values above).
        ///
        /// Option is explicitly set on the layer independent of layer quality.
        ///
        /// If you want to force it on you must also set the layer quality to [`LayerQuality::Best`] with [`Self::set_layer_quality`].
        /// Otherwise it will only be using the specified layer sampling quality whenever the layer quality is set to [`LayerQuality::Best`].
        set_sampling_quality(quality: LayerSamplingQuality) -> () => suite.set_layer_sampling_quality,

        /// New in 23.0. Returns the track matte layer of [`LayerHandle`]. Returns `None` if there is no track matte layer.
        track_matte() -> Option<LayerHandle> => suite.track_matte_layer,

        /// New in 23.0. Sets the track matte layer and track matte type of [`LayerHandle`].
        ///
        /// Setting the track matte type as [`TrackMatte::NoTrackMatte`] removes track matte.
        set_track_matte(track_matte_layer: Option<LayerHandle>, track_matte_type: TrackMatte) -> () => suite.set_track_matte,

        /// New in 23.0. Removes the track matte layer of [`LayerHandle`].
        remove_track_matte() -> () => suite.remove_track_matte,
    }
);

impl Layer {
    /// Get the active layer. If a Layer or effect controls palette is active, the active layer is that associated with the front-most tab in the window.
    ///
    /// If a composition or timeline window is active, the active layer is the selected layer (if only one is selected; otherwise `None` is returned).
    pub fn active() -> Result<Option<Layer>, Error> {
        LayerSuite::new()?.active_layer().map(|h| h.map(|x| Layer::from_handle(x, false)))
    }
}
