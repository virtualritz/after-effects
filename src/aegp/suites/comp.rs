use crate::*;
use crate::aegp::*;
use ae_sys::{ AEGP_CompH, AEGP_ItemH };

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
    pub fn comp_from_item(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Option<CompHandle>, Error> {
        let ptr = call_suite_fn_single!(self, AEGP_GetCompFromItem -> ae_sys::AEGP_CompH, item_handle.as_ptr())?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(CompHandle::from_raw(ptr))
        })
    }

    /// Used to get the item handle, given a composition handle.
    pub fn item_from_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetItemFromComp -> ae_sys::AEGP_ItemH, comp_handle.as_ptr())?
        ))
    }

    /// Returns current downsample factor. Measured in pixels X by Y.
    ///
    /// Users can choose a custom downsample factor with independent X and Y.
    pub fn comp_downsample_factor(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<ae_sys::AEGP_DownsampleFactor, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCompDownsampleFactor -> ae_sys::AEGP_DownsampleFactor, comp_handle.as_ptr())?.into())
    }

    /// Sets the composition's downsample factor.
    pub fn set_comp_downsample_factor(&self, comp_handle: impl AsPtr<AEGP_CompH>, downsample_factor: &ae_sys::AEGP_DownsampleFactor) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDownsampleFactor, comp_handle.as_ptr(), downsample_factor)
    }

    /// Returns the composition background color.
    pub fn comp_bg_color(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<ae_sys::AEGP_ColorVal, Error> {
        call_suite_fn_single!(self, AEGP_GetCompBGColor -> ae_sys::AEGP_ColorVal, comp_handle.as_ptr())
    }

    /// Sets a composition's background color.
    pub fn set_comp_bg_color(&self, comp_handle: impl AsPtr<AEGP_CompH>, color: ae_sys::AEGP_ColorVal) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompBGColor, comp_handle.as_ptr(), &color)
    }

    /// Returns composition flags, or'd together.
    pub fn comp_flags(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<CompFlags, Error> {
        CompFlags::from_bits(call_suite_fn_single!(self, AEGP_GetCompFlags -> ae_sys::A_long, comp_handle.as_ptr())?)
            .ok_or(Error::InvalidParms)
    }

    /// New in CC. Passes back true if the Comp's timeline shows layer names, false if source names.
    ///
    /// This will open the comp as a side effect.
    pub fn show_layer_name_or_source_name(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetShowLayerNameOrSourceName -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// New in CC. Pass in true to have the Comp's timeline show layer names, false for source names.
    ///
    /// This will open the comp as a side effect.
    pub fn set_show_layer_name_or_source_name(&self, comp_handle: impl AsPtr<AEGP_CompH>, show_layer_names: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetShowLayerNameOrSourceName, comp_handle.as_ptr(), if show_layer_names { 1 } else { 0 })
    }


    /// New in CC. Passes back true if the Comp's timeline shows blend modes column, false if hidden.
    ///
    /// This will open the comp as a side effect.
    pub fn show_blend_modes(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetShowBlendModes -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// New in CC. Pass in true to have the Comp's timeline show the blend modes column, false to hide it.
    ///
    /// This will open the comp as a side effect.
    pub fn set_show_blend_modes(&self, comp_handle: impl AsPtr<AEGP_CompH>, show_blend_modes: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetShowBlendModes, comp_handle.as_ptr(), if show_blend_modes { 1 } else { 0 })
    }

    /// Returns the composition's frames per second.
    pub fn comp_framerate(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetCompFramerate -> f64, comp_handle.as_ptr())
    }

    /// Sets the composition's frames per second.
    pub fn set_comp_framerate(&self, comp_handle: impl AsPtr<AEGP_CompH>, framerate: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompFrameRate, comp_handle.as_ptr(), &framerate)
    }

    /// The composition shutter angle and phase.
    pub fn comp_shutter_angle_phase(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<(Ratio, Ratio), Error> {
        let (angle, phase) = call_suite_fn_double!(self, AEGP_GetCompShutterAnglePhase -> ae_sys::A_Ratio, ae_sys::A_Ratio, comp_handle.as_ptr())?;
        Ok((
            angle.into(),
            phase.into()
        ))
    }

    /// The duration of the shutter frame, in seconds.
    pub fn comp_shutter_frame_range(&self, comp_handle: impl AsPtr<AEGP_CompH>, comp_time: Time) -> Result<(Time, Time), Error> {
        let (start, duration) = call_suite_fn_double!(self, AEGP_GetCompShutterFrameRange -> ae_sys::A_Time, ae_sys::A_Time, comp_handle.as_ptr(), &comp_time.into() as *const _)?;
        Ok((
            start.into(),
            duration.into()
        ))
    }

    /// Retrieves the number of motion blur samples After Effects will perform in the given composition.
    pub fn comp_suggested_motion_blur_samples(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetCompSuggestedMotionBlurSamples -> i32, comp_handle.as_ptr())
    }

    /// Specifies the number of motion blur samples After Effects will perform in the given composition. Undoable.
    pub fn set_comp_suggested_motion_blur_samples(&self, comp_handle: impl AsPtr<AEGP_CompH>, samples: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompSuggestedMotionBlurSamples, comp_handle.as_ptr(), samples)
    }

    /// New in CC. Retrieves the motion blur adaptive sample limit for the given composition.
    ///
    /// As of CC, a new comp defaults to 128.
    pub fn comp_motion_blur_adaptive_sample_limit(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetCompMotionBlurAdaptiveSampleLimit -> i32, comp_handle.as_ptr())
    }

    /// New in CC. Specifies the motion blur adaptive sample limit for the given composition.
    ///
    /// As of CC, both the limit and the suggested values are clamped to \[2,256\] range and the limit value will not be allowed less than the suggested value.
    ///
    /// Undoable.
    pub fn set_comp_motion_blur_adaptive_sample_limit(&self, comp_handle: impl AsPtr<AEGP_CompH>, limit: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompMotionBlurAdaptiveSampleLimit, comp_handle.as_ptr(), limit)
    }

    /// Get the time where the current work area starts.
    pub fn comp_work_area_start(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompWorkAreaStart -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Get the duration of a composition's current work area, in seconds.
    pub fn comp_work_area_duration(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompWorkAreaDuration -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Set the work area start and duration, in seconds. Undo-able.
    ///
    /// One call to this function is sufficient to set the layer's in point and duration;
    /// it's not necessary to call it twice, once for each timespace.
    pub fn set_comp_work_area_start_and_duration(&self, comp_handle: impl AsPtr<AEGP_CompH>, start: Time, duration: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompWorkAreaStartAndDuration, comp_handle.as_ptr(), &start.into() as *const _ as *const ae_sys::A_Time, &duration.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Creates a new solid with a specified width, height, color, and duration in the composition. Undo-able.
    ///
    /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
    /// If you pass `None`, or an invalid time scale, duration is set to the length of the composition.
    pub fn create_solid_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, name: &str, width: i32, height: i32, color: ae_sys::AEGP_ColorVal, duration: Option<Time>) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateSolidInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                width,
                height,
                &color,
                comp_handle.as_ptr(),
                duration.map(Into::into).as_ref().map_or(std::ptr::null(), |t| t)
            )?
        ))
    }

    /// Creates and adds a camera to the specified composition.
    /// Once created, you can manipulate the camera's parameter streams using the [`suites::Stream`](aegp::suites::Stream).
    ///
    /// To specify a two-node camera, use [`suites::Layer::set_layer_flag()`](aegp::suites::Layer::set_layer_flag) to set [`LayerFlags::LOOK_AT_POI`].
    pub fn create_camera_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, name: &str, center_point: ae_sys::A_FloatPoint) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateCameraInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                center_point,
                comp_handle.as_ptr()
            )?
        ))
    }

    /// Creates and adds a light to the specified composition.
    /// Once created, you can manipulate the light's parameter streams using the AEGP [`suites::Stream`](aegp::suites::Stream).
    pub fn create_light_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, name: &str, center_point: ae_sys::A_FloatPoint) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateLightInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                center_point,
                comp_handle.as_ptr()
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
    pub fn new_collection_from_comp_selection(&self, comp_handle: impl AsPtr<AEGP_CompH>, plugin_id: PluginId) -> Result<Collection2Handle, Error> {
        Ok(Collection2Handle::from_raw(
            call_suite_fn_single!(self, AEGP_GetNewCollectionFromCompSelection -> ae_sys::AEGP_Collection2H, plugin_id, comp_handle.as_ptr())?
        ))
    }

    /// Sets the selection within the given composition to the given [`Collection2Handle`].
    ///
    /// Will return an error if members of the [`Collection2Handle`] are not available.
    ///
    /// Don't assume that a composition hasn't changed between operations; always use a fresh [`Collection2Handle`].
    pub fn set_selection(&self, comp_handle: impl AsPtr<AEGP_CompH>, collection_handle: Collection2Handle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetSelection, comp_handle.as_ptr(), collection_handle.as_ptr())
    }

    pub fn comp_display_start_time(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompDisplayStartTime -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Not undo-able. Sets the displayed start time of a composition (has no effect on the duration of the composition).
    pub fn set_comp_display_start_time(&self, comp_handle: impl AsPtr<AEGP_CompH>, time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDisplayStartTime, comp_handle.as_ptr(), &time.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Undoable. Sets the duration of the given composition.
    pub fn set_comp_duration(&self, comp_handle: impl AsPtr<AEGP_CompH>, duration: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDuration, comp_handle.as_ptr(), &duration.into() as *const _ as *const ae_sys::A_Time)
    }

    /// Creates a "null object" in the composition (useful for translating projects from 3D applications into After Effects).
    ///
    /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
    /// If you pass 0, or an invalid time scale, duration is set to the length of the composition.
    pub fn create_null_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, name: &str, duration: Option<Time>) -> Result<LayerHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateNullInComp -> ae_sys::AEGP_LayerH,
                name.as_ptr(),
                comp_handle.as_ptr(),
                duration.map(Into::into).as_ref().map_or(std::ptr::null(), |t| t)
            )?
        ))
    }

    /// Sets the pixel aspect ratio of a composition.
    pub fn set_comp_pixel_aspect_ratio(&self, comp_handle: impl AsPtr<AEGP_CompH>, pixel_aspect_ratio: Ratio) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompPixelAspectRatio, comp_handle.as_ptr(), &pixel_aspect_ratio.into() as *const _)
    }

    /// Updated in CS6. Creates a text layer in the composition, and returns its [`LayerHandle`].
    pub fn create_text_layer_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, select_new_layer: bool) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateTextLayerInComp -> ae_sys::AEGP_LayerH,
                comp_handle.as_ptr(),
                select_new_layer as _
            )?
        ))
    }

    /// Updated in CS6. Creates a new box text layer, and returns its [`LayerHandle`].
    pub fn create_box_text_layer_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>, select_new_layer: bool, box_dimensions: FloatPoint) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateBoxTextLayerInComp -> ae_sys::AEGP_LayerH,
                comp_handle.as_ptr(),
                select_new_layer as _,
                box_dimensions.into()
            )?
        ))
    }

    /// Sets the dimensions of the composition. Undoable.
    pub fn set_comp_dimensions(&self, comp_handle: impl AsPtr<AEGP_CompH>, width: i32, height: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDimensions, comp_handle.as_ptr(), width, height)
    }

    /// Duplicates the composition. Undoable.
    pub fn duplicate_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_DuplicateComp -> ae_sys::AEGP_CompH, comp_handle.as_ptr())?
        ))
    }

    /// Retrieves the duration of a frame in a composition.
    pub fn comp_frame_duration(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<Time, Error> {
        call_suite_fn_single!(self, AEGP_GetCompFrameDuration -> ae_sys::A_Time, comp_handle.as_ptr()).map(|t| t.into())
    }

    /// Returns the most-recently-used composition.
    pub fn most_recently_used_comp(&self) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetMostRecentlyUsedComp -> ae_sys::AEGP_CompH)?
        ))
    }

    /// Creates and returns a handle to a new vector layer.
    pub fn create_vector_layer_in_comp(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_CreateVectorLayerInComp -> ae_sys::AEGP_LayerH, comp_handle.as_ptr())?
        ))
    }

    /// Returns an [`StreamReferenceHandle`] to the composition's marker stream.
    ///
    /// Must be disposed by caller.
    pub fn new_comp_marker_stream(&self, comp_handle: impl AsPtr<AEGP_CompH>, plugin_id: PluginId) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetNewCompMarkerStream -> ae_sys::AEGP_StreamRefH, plugin_id, comp_handle.as_ptr())?
        ))
    }

    /// Passes back a boolean that indicates whether the specified comp uses drop-frame timecode or not.
    pub fn comp_display_drop_frame(&self, comp_handle: impl AsPtr<AEGP_CompH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCompDisplayDropFrame -> ae_sys::A_Boolean, comp_handle.as_ptr())? != 0)
    }

    /// Sets the dropness of the timecode in the specified composition.
    pub fn set_comp_display_drop_frame(&self, comp_handle: impl AsPtr<AEGP_CompH>, drop_frame: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCompDisplayDropFrame, comp_handle.as_ptr(), if drop_frame { 1 } else { 0 })
    }

    /// Move the selection to a certain layer index. Use along with [`set_selection()`](Self::set_selection).
    pub fn reorder_comp_selection(&self, comp_handle: impl AsPtr<AEGP_CompH>, layer_index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReorderCompSelection, comp_handle.as_ptr(), layer_index)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_CompH);
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

define_suite_item_wrapper!(
    ae_sys::AEGP_CompH, CompHandle,
    suite: CompSuite,
    layers: aegp::suites::Layer,
    /// Provide information about the compositions in a project, and create cameras, lights, and solids.
    Composition {
        dispose: ;

        /// Used to get the item handle.
        item() -> Item => suite.item_from_comp,

        /// Returns current downsample factor. Measured in pixels X by Y.
        ///
        /// Users can choose a custom downsample factor with independent X and Y.
        downsample_factor() -> ae_sys::AEGP_DownsampleFactor => suite.comp_downsample_factor,

        /// Sets the composition's downsample factor.
        set_downsample_factor(downsample_factor: &ae_sys::AEGP_DownsampleFactor) -> () => suite.set_comp_downsample_factor,

        /// Returns the composition background color.
        bg_color() -> ae_sys::AEGP_ColorVal => suite.comp_bg_color,

        /// Sets a composition's background color.
        set_bg_color(color: ae_sys::AEGP_ColorVal) -> () => suite.set_comp_bg_color,

        /// Returns composition flags, or'd together.
        flags() -> CompFlags => suite.comp_flags,

        /// New in CC. Passes back true if the Comp's timeline shows layer names, false if source names.
        ///
        /// This will open the comp as a side effect.
        show_layer_name_or_source_name() -> bool => suite.show_layer_name_or_source_name,

        /// New in CC. Pass in true to have the Comp's timeline show layer names, false for source names.
        ///
        /// This will open the comp as a side effect.
        set_show_layer_name_or_source_name(show_layer_names: bool) -> () => suite.set_show_layer_name_or_source_name,

        /// New in CC. Passes back true if the Comp's timeline shows blend modes column, false if hidden.
        ///
        /// This will open the comp as a side effect.
        show_blend_modes() -> bool => suite.show_blend_modes,

        /// New in CC. Pass in true to have the Comp's timeline show the blend modes column, false to hide it.
        ///
        /// This will open the comp as a side effect.
        set_show_blend_modes(show_blend_modes: bool) -> () => suite.set_show_blend_modes,

        /// Returns the composition's frames per second.
        framerate() -> f64 => suite.comp_framerate,

        /// Sets the composition's frames per second.
        set_framerate(framerate: f64) -> () => suite.set_comp_framerate,

        /// The composition shutter angle and phase.
        shutter_angle_phase() -> (Ratio, Ratio) => suite.comp_shutter_angle_phase,

        /// The duration of the shutter frame, in seconds.
        shutter_frame_range(comp_time: Time) -> (Time, Time) => suite.comp_shutter_frame_range,

        /// Retrieves the number of motion blur samples After Effects will perform in the given composition.
        suggested_motion_blur_samples() -> i32 => suite.comp_suggested_motion_blur_samples,

        /// Specifies the number of motion blur samples After Effects will perform in the given composition. Undoable.
        set_suggested_motion_blur_samples(samples: i32) -> () => suite.set_comp_suggested_motion_blur_samples,

        /// New in CC. Retrieves the motion blur adaptive sample limit for the given composition.
        ///
        /// As of CC, a new comp defaults to 128.
        motion_blur_adaptive_sample_limit() -> i32 => suite.comp_motion_blur_adaptive_sample_limit,

        /// New in CC. Specifies the motion blur adaptive sample limit for the given composition.
        ///
        /// As of CC, both the limit and the suggested values are clamped to \[2,256\] range and the limit value will not be allowed less than the suggested value.
        ///
        /// Undoable.
        set_motion_blur_adaptive_sample_limit(limit: i32) -> () => suite.set_comp_motion_blur_adaptive_sample_limit,

        /// Get the time where the current work area starts.
        work_area_start() -> Time => suite.comp_work_area_start,

        /// Get the duration of a composition's current work area, in seconds.
        work_area_duration() -> Time => suite.comp_work_area_duration,

        /// Set the work area start and duration, in seconds. Undo-able.
        ///
        /// One call to this function is sufficient to set the layer's in point and duration;
        /// it's not necessary to call it twice, once for each timespace.
        set_work_area_start_and_duration(start: Time, duration: Time) -> () => suite.set_comp_work_area_start_and_duration,

        /// Creates a new solid with a specified width, height, color, and duration in the composition. Undo-able.
        ///
        /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
        /// If you pass `None`, or an invalid time scale, duration is set to the length of the composition.
        create_solid(name: &str, width: i32, height: i32, color: ae_sys::AEGP_ColorVal, duration: Option<Time>) -> aegp::Layer => suite.create_solid_in_comp,

        /// Creates and adds a camera to the specified composition.
        /// Once created, you can manipulate the camera's parameter streams using the [`suites::Stream`](aegp::suites::Stream).
        ///
        /// To specify a two-node camera, use [`suites::Layer::set_layer_flag()`](aegp::suites::Layer::set_layer_flag) to set [`LayerFlags::LOOK_AT_POI`].
        create_camera(name: &str, center_point: ae_sys::A_FloatPoint) -> aegp::Layer => suite.create_camera_in_comp,

        /// Creates and adds a light to the specified composition.
        /// Once created, you can manipulate the light's parameter streams using the AEGP [`suites::Stream`](aegp::suites::Stream).
        create_light(name: &str, center_point: ae_sys::A_FloatPoint) -> aegp::Layer => suite.create_light_in_comp,

        /// Creates a new [`Collection2Handle`] from the items selected in the given composition.
        ///
        /// The plug-in is responsible for disposing of the [`Collection2Handle`].
        new_collection_from_comp_selection(plugin_id: PluginId) -> Collection2Handle => suite.new_collection_from_comp_selection,

        /// Sets the selection within the given composition to the given [`Collection2Handle`].
        ///
        /// Will return an error if members of the [`Collection2Handle`] are not available.
        ///
        /// Don't assume that a composition hasn't changed between operations; always use a fresh [`Collection2Handle`].
        set_selection(collection_handle: Collection2Handle) -> () => suite.set_selection,

        display_start_time() -> Time => suite.comp_display_start_time,

        /// Not undo-able. Sets the displayed start time of a composition (has no effect on the duration of the composition).
        set_display_start_time(time: Time) -> () => suite.set_comp_display_start_time,

        /// Undoable. Sets the duration of the given composition.
        set_duration(duration: Time) -> () => suite.set_comp_duration,

        /// Creates a "null object" in the composition (useful for translating projects from 3D applications into After Effects).
        ///
        /// If you pass `None` for the duration, After Effects uses its preference for the duration of a new still.
        /// If you pass 0, or an invalid time scale, duration is set to the length of the composition.
        create_null(name: &str, duration: Option<Time>) -> aegp::Layer => suite.create_null_in_comp,

        /// Sets the pixel aspect ratio of a composition.
        set_pixel_aspect_ratio(pixel_aspect_ratio: Ratio) -> () => suite.set_comp_pixel_aspect_ratio,

        /// Updated in CS6. Creates a text layer in the composition, and returns its [`LayerHandle`].
        create_text_layer(select_new_layer: bool) -> aegp::Layer => suite.create_text_layer_in_comp,

        /// Updated in CS6. Creates a new box text layer, and returns its [`LayerHandle`].
        create_box_text_layer(select_new_layer: bool, box_dimensions: FloatPoint) -> aegp::Layer => suite.create_box_text_layer_in_comp,

        /// Sets the dimensions of the composition. Undoable.
        set_dimensions(width: i32, height: i32) -> () => suite.set_comp_dimensions,

        /// Duplicates the composition. Undoable.
        duplicate_comp() -> Composition => suite.duplicate_comp,

        /// Retrieves the duration of a frame in a composition.
        frame_duration() -> Time => suite.comp_frame_duration,

        /// Creates and returns a handle to a new vector layer.
        create_vector_layer() -> aegp::Layer => suite.create_vector_layer_in_comp,

        /// Returns an [`StreamReferenceHandle`] to the composition's marker stream.
        ///
        /// Must be disposed by caller.
        new_marker_stream(plugin_id: PluginId) -> StreamReferenceHandle => suite.new_comp_marker_stream,

        /// Passes back a boolean that indicates whether the specified comp uses drop-frame timecode or not.
        display_drop_frame() -> bool => suite.comp_display_drop_frame,

        /// Sets the dropness of the timecode in the specified composition.
        set_display_drop_frame(drop_frame: bool) -> () => suite.set_comp_display_drop_frame,

        /// Move the selection to a certain layer index. Use along with [`set_selection()`](Self::set_selection).
        reorder_comp_selection(layer_index: i32) -> () => suite.reorder_comp_selection,

        // ―――――――――――――――――――――――――――― Layer suite functions ――――――――――――――――――――――――――――

        /// Obtains the number of layers in a composition.
        num_layers() -> usize => layers.comp_num_layers,

        /// Get a [`Layer`] from a composition. Zero is the foremost layer.
        layer_by_index(index: usize) -> aegp::Layer => layers.comp_layer_by_index,
    }
);

impl Composition {
    /// Retrieves the handle to the composition, given an item handle.
    ///
    /// Returns `Err` if `item_handle` is not an `AEGP_CompH`.
    pub fn from_item(item: impl AsPtr<AEGP_ItemH>) -> Result<Option<Composition>, Error> {
        Ok(CompSuite::new()?.comp_from_item(item.as_ptr())?.map(Into::into))
    }

    /// Creates a new composition for the project.
    /// If you don't provide a parent folder, the composition will be at the root level of the project.
    ///
    /// Undo-able.
    pub fn create(parent_folder: Option<ItemHandle>, name: &str, width: i32, height: i32, pixel_aspect_ratio: Ratio, duration: Time, frame_rate: Ratio) -> Result<Composition, Error> {
        CompSuite::new()?.create_comp(parent_folder, name, width, height, pixel_aspect_ratio, duration, frame_rate).map(Into::into)
    }

    /// Returns the most-recently-used composition.
    pub fn most_recently_used() -> Result<Composition, Error> {
        CompSuite::new()?.most_recently_used_comp().map(Into::into)
    }
}
