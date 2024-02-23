use crate::*;
use crate::aegp::*;

define_suite!(
    /// Provide information about the compositions in a project, and create cameras, lights, and solids.
    CompSuite,
    AEGP_CompSuite11,
    kAEGPCompSuite,
    kAEGPCompSuiteVersion11
);

impl CompSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the handle to the composition, given an item handle.
    ///
    /// Returns `None` if `item_handle` is not an `AEGP_CompH`.
    pub fn comp_from_item(&self, item_handle: ItemHandle) -> Result<Option<CompHandle>, Error> {
        let ptr = call_suite_fn_single!(self, AEGP_GetCompFromItem -> ae_sys::AEGP_CompH, item_handle.as_ptr())?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(CompHandle::from_raw(ptr))
        })
    }

    /// Used to get the item handle, given a composition handle.
    pub fn item_from_comp(&self, comp_handle: CompHandle) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetItemFromComp -> ae_sys::AEGP_ItemH, comp_handle.as_ptr())?
        ))
    }

    /// Returns current downsample factor. Measured in pixels X by Y.
    ///
    /// Users can choose a custom downsample factor with independent X and Y.
    pub fn comp_downsample_factor(&self, comp_handle: CompHandle) -> Result<ae_sys::AEGP_DownsampleFactor, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCompDownsampleFactor -> ae_sys::AEGP_DownsampleFactor, comp_handle.as_ptr())?.into())
    }

    /// Sets the composition's downsample factor.
    pub fn set_comp_downsample_factor(&self, comp_handle: CompHandle, downsample_factor: &ae_sys::AEGP_DownsampleFactor) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDownsampleFactor, comp_handle.as_ptr(), downsample_factor)
    }

    /// Returns the composition background color.
    pub fn comp_bg_color(&self, comp_handle: CompHandle) -> Result<ae_sys::AEGP_ColorVal, Error> {
        call_suite_fn_single!(self, AEGP_GetCompBGColor -> ae_sys::AEGP_ColorVal, comp_handle.as_ptr())
    }

    /// Sets a composition's background color.
    pub fn set_comp_bg_color(&self, comp_handle: CompHandle, color: ae_sys::AEGP_ColorVal) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompBGColor, comp_handle.as_ptr(), &color)
    }

    /// Returns composition flags, or'd together.
    pub fn comp_flags(&self, comp_handle: CompHandle) -> Result<CompFlags, Error> {
        CompFlags::from_bits(call_suite_fn_single!(self, AEGP_GetCompFlags -> ae_sys::A_long, comp_handle.as_ptr())?)
            .ok_or(Error::InvalidParms)
    }

    /// New in CC. Passes back true if the Comp's timeline shows layer names, false if source names.
    ///
    /// This will open the comp as a side effect.
    pub fn show_layer_name_or_source_name(&self, comp_handle: CompHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetShowLayerNameOrSourceName -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// New in CC. Pass in true to have the Comp's timeline show layer names, false for source names.
    ///
    /// This will open the comp as a side effect.
    pub fn set_show_layer_name_or_source_name(&self, comp_handle: CompHandle, show_layer_names: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetShowLayerNameOrSourceName, comp_handle.as_ptr(), if show_layer_names { 1 } else { 0 })
    }


    /// New in CC. Passes back true if the Comp's timeline shows blend modes column, false if hidden.
    ///
    /// This will open the comp as a side effect.
    pub fn show_blend_modes(&self, comp_handle: CompHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetShowBlendModes -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// New in CC. Pass in true to have the Comp's timeline show the blend modes column, false to hide it.
    ///
    /// This will open the comp as a side effect.
    pub fn set_show_blend_modes(&self, comp_handle: CompHandle, show_blend_modes: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetShowBlendModes, comp_handle.as_ptr(), if show_blend_modes { 1 } else { 0 })
    }

    /// Returns the composition's frames per second.
    pub fn comp_framerate(&self, comp_handle: CompHandle) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetCompFramerate -> f64, comp_handle.as_ptr())
    }

    /// Sets the composition's frames per second.
    pub fn set_comp_framerate(&self, comp_handle: CompHandle, framerate: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompFrameRate, comp_handle.as_ptr(), &framerate)
    }

    /// The composition shutter angle and phase.
    pub fn comp_shutter_angle_phase(&self, comp_handle: CompHandle) -> Result<(Ratio, Ratio), Error> {
        let (angle, phase) = call_suite_fn_double!(self, AEGP_GetCompShutterAnglePhase -> ae_sys::A_Ratio, ae_sys::A_Ratio, comp_handle.as_ptr())?;
        Ok((
            angle.into(),
            phase.into()
        ))
    }

    /// The duration of the shutter frame, in seconds.
    pub fn comp_shutter_frame_range(&self, comp_handle: CompHandle, comp_time: Time) -> Result<(Time, Time), Error> {
        let (start, duration) = call_suite_fn_double!(self, AEGP_GetCompShutterFrameRange -> ae_sys::A_Time, ae_sys::A_Time, comp_handle.as_ptr(), &comp_time.into() as *const _)?;
        Ok((
            start.into(),
            duration.into()
        ))
    }

    /// Retrieves the number of motion blur samples After Effects will perform in the given composition.
    pub fn comp_suggested_motion_blur_samples(&self, comp_handle: CompHandle) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetCompSuggestedMotionBlurSamples -> i32, comp_handle.as_ptr())
    }

    /// Specifies the number of motion blur samples After Effects will perform in the given composition. Undoable.
    pub fn set_comp_suggested_motion_blur_samples(&self, comp_handle: CompHandle, samples: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompSuggestedMotionBlurSamples, comp_handle.as_ptr(), samples)
    }

    /// New in CC. Retrieves the motion blur adaptive sample limit for the given composition.
    ///
    /// As of CC, a new comp defaults to 128.
    pub fn comp_motion_blur_adaptive_sample_limit(&self, comp_handle: CompHandle) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetCompMotionBlurAdaptiveSampleLimit -> i32, comp_handle.as_ptr())
    }

    /// New in CC. Specifies the motion blur adaptive sample limit for the given composition.
    ///
    /// As of CC, both the limit and the suggested values are clamped to \[2,256\] range and the limit value will not be allowed less than the suggested value.
    ///
    /// Undoable.
    pub fn set_comp_motion_blur_adaptive_sample_limit(&self, comp_handle: CompHandle, limit: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompMotionBlurAdaptiveSampleLimit, comp_handle.as_ptr(), limit)
    }

    /// Get the time where the current work area starts.
    pub fn comp_work_area_start(&self, comp_handle: CompHandle) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompWorkAreaStart -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Get the duration of a composition's current work area, in seconds.
    pub fn comp_work_area_duration(&self, comp_handle: CompHandle) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompWorkAreaDuration -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Set the work area start and duration, in seconds. Undo-able.
    ///
    /// One call to this function is sufficient to set the layer's in point and duration;
    /// it's not necessary to call it twice, once for each timespace.
    pub fn set_comp_work_area_start_and_duration(&self, comp_handle: CompHandle, start: Time, duration: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompWorkAreaStartAndDuration, comp_handle.as_ptr(), &start.into() as *const _ as *const ae_sys::A_Time, &duration.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Creates a new solid with a specified width, height, color, and duration in the composition. Undo-able.
    ///
    /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
    /// If you pass `None`, or an invalid time scale, duration is set to the length of the composition.
    pub fn create_solid_in_comp(&self, name: &str, width: i32, height: i32, color: ae_sys::AEGP_ColorVal, parent_comp_handle: CompHandle, duration: Option<Time>) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateSolidInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                width,
                height,
                &color,
                parent_comp_handle.as_ptr(),
                duration.map_or(std::ptr::null(), |t| &t.into() as *const _)
            )?
        ))
    }

    /// Creates and adds a camera to the specified composition.
    /// Once created, you can manipulate the camera's parameter streams using the [`suites::Stream`](aegp::suites::Stream).
    ///
    /// To specify a two-node camera, use [`suites::Layer::set_layer_flag()`](aegp::suites::Layer::set_layer_flag) to set [`LayerFlags::LOOK_AT_POI`].
    pub fn create_camera_in_comp(&self, name: &str, center_point: ae_sys::A_FloatPoint, parent_comp_handle: CompHandle) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateCameraInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                center_point,
                parent_comp_handle.as_ptr()
            )?
        ))
    }

    /// Creates and adds a light to the specified composition.
    /// Once created, you can manipulate the light's parameter streams using the AEGP [`suites::Stream`](aegp::suites::Stream).
    pub fn create_light_in_comp(&self, name: &str, center_point: ae_sys::A_FloatPoint, parent_comp_handle: CompHandle) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateLightInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                center_point,
                parent_comp_handle.as_ptr()
            )?
        ))
    }

    /// Creates a new composition for the project.
    /// If you don't provide a parent folder, the composition will be at the root level of the project.
    ///
    /// Undo-able.
    pub fn create_comp(&self, parent_folder: Option<ItemHandle>, name: &str, width: i32, height: i32, pixel_aspect_ratio: Ratio, duration: Time, frame_rate: Ratio) -> Result<CompHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateComp -> ae_sys::AEGP_CompH,
                parent_folder.map_or(std::ptr::null_mut(), |i| i.as_ptr()),
                name.as_ptr(),
                width,
                height,
                &pixel_aspect_ratio.into() as *const _,
                &duration.into() as *const _,
                &frame_rate.into() as *const _
            )?
        ))
    }

    /// Creates a new [`Collection2Handle`] from the items selected in the given composition.
    ///
    /// The plug-in is responsible for disposing of the [`Collection2Handle`].
    pub fn new_collection_from_comp_selection(&self, plugin_id: PluginID, comp_handle: CompHandle) -> Result<Collection2Handle, Error> {
        Ok(Collection2Handle::from_raw(
            call_suite_fn_single!(self, AEGP_GetNewCollectionFromCompSelection -> ae_sys::AEGP_Collection2H, plugin_id, comp_handle.as_ptr())?
        ))
    }

    /// Sets the selection within the given composition to the given [`Collection2Handle`].
    ///
    /// Will return an error if members of the [`Collection2Handle`] are not available.
    ///
    /// Don't assume that a composition hasn't changed between operations; always use a fresh [`Collection2Handle`].
    pub fn set_selection(&self, comp_handle: CompHandle, collection_handle: Collection2Handle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetSelection, comp_handle.as_ptr(), collection_handle.as_ptr())
    }

    pub fn comp_display_start_time(&self, comp_handle: CompHandle) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompDisplayStartTime -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Not undo-able. Sets the displayed start time of a composition (has no effect on the duration of the composition).
    pub fn set_comp_display_start_time(&self, comp_handle: CompHandle, time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDisplayStartTime, comp_handle.as_ptr(), &time.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Undoable. Sets the duration of the given composition.
    pub fn set_comp_duration(&self, comp_handle: CompHandle, duration: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDuration, comp_handle.as_ptr(), &duration.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Creates a "null object" in the composition (useful for translating projects from 3D applications into After Effects).
    ///
    /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
    /// If you pass 0, or an invalid time scale, duration is set to the length of the composition.
    pub fn create_null_in_comp(&self, name: &str, parent_comp_handle: CompHandle, duration: Option<Time>) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateNullInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                parent_comp_handle.as_ptr(),
                duration.map_or(std::ptr::null(), |t| &t.into() as *const _)
            )?
        ))
    }

    /// Sets the pixel aspect ratio of a composition.
    pub fn set_comp_pixel_aspect_ratio(&self, comp_handle: CompHandle, pixel_aspect_ratio: Ratio) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompPixelAspectRatio, comp_handle.as_ptr(), &pixel_aspect_ratio.into() as *const _)
    }

    /// Updated in CS6. Creates a text layer in the composition, and returns its [`LayerHandle`].
    pub fn create_text_layer_in_comp(&self, parent_comp_handle: CompHandle, select_new_layer: bool) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateTextLayerInComp -> ae_sys::AEGP_LayerH,
                parent_comp_handle.as_ptr(),
                if select_new_layer { 1 } else { 0 }
            )?
        ))
    }

    /// Updated in CS6. Creates a new box text layer, and returns its [`LayerHandle`].
    pub fn create_box_text_layer_in_comp(&self, parent_comp_handle: CompHandle, select_new_layer: bool, box_dimensions: FloatPoint) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateBoxTextLayerInComp -> ae_sys::AEGP_LayerH,
                parent_comp_handle.as_ptr(),
                if select_new_layer { 1 } else { 0 },
                box_dimensions.into()
            )?
        ))
    }

    /// Sets the dimensions of the composition. Undoable.
    pub fn set_comp_dimensions(&self, comp_handle: CompHandle, width: i32, height: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDimensions, comp_handle.as_ptr(), width, height)
    }

    /// Duplicates the composition. Undoable.
    pub fn duplicate_comp(&self, comp_handle: CompHandle) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_DuplicateComp -> ae_sys::AEGP_CompH, comp_handle.as_ptr())?
        ))
    }

    /// Retrieves the duration of a frame in a composition.
    pub fn comp_frame_duration(&self, comp_handle: CompHandle) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompFrameDuration -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Returns the most-recently-used composition.
    pub fn most_recently_used_comp(&self) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetMostRecentlyUsedComp -> ae_sys::AEGP_CompH)?
        ))
    }
    /// Creates and returns a handle to a new vector layer.
    pub fn create_vector_layer_in_comp(&self, parent_comp_handle: CompHandle) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_CreateVectorLayerInComp -> ae_sys::AEGP_LayerH, parent_comp_handle.as_ptr())?
        ))
    }

    /// Returns an [`StreamReferenceHandle`] to the composition's marker stream.
    ///
    /// Must be disposed by caller.
    pub fn new_comp_marker_stream(&self, plugin_id: PluginID, parent_comp_handle: CompHandle) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetNewCompMarkerStream -> ae_sys::AEGP_StreamRefH, plugin_id, parent_comp_handle.as_ptr())?
        ))
    }

    /// Passes back a boolean that indicates whether the specified comp uses drop-frame timecode or not.
    pub fn comp_display_drop_frame(&self, comp_handle: CompHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCompDisplayDropFrame -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// Sets the dropness of the timecode in the specified composition.
    pub fn set_comp_display_drop_frame(&self, comp_handle: CompHandle, drop_frame: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDisplayDropFrame, comp_handle.as_ptr(), if drop_frame { 1 } else { 0 })
    }

    /// Move the selection to a certain layer index. Use along with [`set_selection()`](Self::set_selection).
    pub fn reorder_comp_selection(&self, comp_handle: CompHandle, layer_index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReorderCompSelection, comp_handle.as_ptr(), layer_index)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_handle_wrapper!(CompHandle, AEGP_CompH);
define_handle_wrapper!(Collection2Handle, AEGP_Collection2H);

bitflags::bitflags! {
    pub struct CompFlags: ae_sys::A_long {
        const SHOW_ALL_SHY       = ae_sys::AEGP_CompFlag_SHOW_ALL_SHY       as ae_sys::A_long;
        const RESERVED_1         = ae_sys::AEGP_CompFlag_RESERVED_1         as ae_sys::A_long;
        const RESERVED_2         = ae_sys::AEGP_CompFlag_RESERVED_2         as ae_sys::A_long;
        const ENABLE_MOTION_BLUR = ae_sys::AEGP_CompFlag_ENABLE_MOTION_BLUR as ae_sys::A_long;
        const ENABLE_TIME_FILTER = ae_sys::AEGP_CompFlag_ENABLE_TIME_FILTER as ae_sys::A_long;
        const GRID_TO_FRAMES     = ae_sys::AEGP_CompFlag_GRID_TO_FRAMES     as ae_sys::A_long;
        const GRID_TO_FIELDS     = ae_sys::AEGP_CompFlag_GRID_TO_FIELDS     as ae_sys::A_long;
        const USE_LOCAL_DSF      = ae_sys::AEGP_CompFlag_USE_LOCAL_DSF      as ae_sys::A_long;
        const DRAFT_3D           = ae_sys::AEGP_CompFlag_DRAFT_3D           as ae_sys::A_long;
        const SHOW_GRAPH         = ae_sys::AEGP_CompFlag_SHOW_GRAPH         as ae_sys::A_long;
        const RESERVED_3         = ae_sys::AEGP_CompFlag_RESERVED_3         as ae_sys::A_long;
    }
}

/*pub struct Comp {
    comp_suite: CompSuite,
    comp_handle: CompHandle,
}

impl Comp {
    pub fn from_item(item_handle: ItemHandle) -> Result<Self, Error> {
        let comp_suite = CompSuite::new()?;
        let comp_handle = comp_suite.comp_from_item(item_handle)?;
        if comp_handle.is_none() {
            return Err(Error::InvalidParms);
        }

        Ok(Self {
            comp_suite,
            comp_handle: comp_handle.unwrap()
        })
    }
}
*/