use crate::*;
use ae_sys::{ PF_TimeDisplay, prFieldType, PrTime, PrTimelineID, A_long };

register_handle!(PF_ProgPtr);
define_handle_wrapper!(EffectHandle, PF_ProgPtr);

define_suite_item_wrapper!(
    ae_sys::PF_ProgPtr, EffectHandle,
    pf_interface: aegp::suites::PFInterface,
    pf_utility: pf::suites::Utility,
    effect_sequence_data: pf::suites::EffectSequenceData,
    param_utils: pf::suites::ParamUtils,
    effect_ui: pf::suites::EffectUI,
    /// TODO: write docs for Effect
    Effect {
        dispose: ;

        /// Returns the layer the effect is applied to
        layer() -> aegp::Layer => pf_interface.effect_layer,

        /// Obtain the [`aegp::Effect`](aegp::Effect) corresponding to the effect.
        aegp_effect(plugin_id: aegp::PluginId) -> aegp::Effect => pf_interface.new_effect_for_effect,

        /// Retrieve the composition time corresponding to the effect's layer time.
        comp_time(time: i32, time_scale: u32) -> Time => pf_interface.convert_effect_to_comp_time,

        /// Obtain the transform used to move between the layer's coordinate space and that of the containing composition.
        ///
        /// NOTE: In cases where the effect's input layer has square pixels, but is in a non-square pixel composition,
        /// you must correct for the pixel aspect ratio by premultiplying the matrix by `(1/parF, 1, 1)`.
        ///
        /// The model view for the camera matrix is inverse of the matrix obtained from [`camera_matrix()`](Self::camera_matrix).
        ///
        /// Also note that our matrix is row-based; OpenGL's is column-based.
        ///
        /// Returns a tuple containing: (matrix, dist_to_image_plane, image_plane_width, image_plane_height)
        camera_matrix(time: Time) -> (Matrix4, f64, i16, i16) => pf_interface.effect_camera_matrix,

        // ―――――――――――――――――――――――――――― Utility suite functions ――――――――――――――――――――――――――――

        /// Gets the filter ID for the current effect reference.
        filter_instance_id()                                            ->  i32 => pf_utility.filter_instance_id,
        /// Retrieves formatted timecode, as well as the currently active video frame.
        ///
        /// Returns a tuple containing `(current_frame, time_display)`
        media_timecode()                                                ->  (i32, PF_TimeDisplay) => pf_utility.media_timecode,
        /// Retrieves the speed multiplier of the clip.
        clip_speed()                                                    ->  f64 => pf_utility.clip_speed,
        /// Retrieves the duration of the clip.
        clip_duration()                                                 ->  i32 => pf_utility.clip_duration,
        /// Retrieves the start time of the clip.
        clip_start()                                                    ->  i32 => pf_utility.clip_start,
        /// Retrieves the duration of the clip, unaffected by any speed or retiming changes.
        unscaled_clip_duration()                                        ->  i32 => pf_utility.unscaled_clip_duration,
        /// Retrives the start time of the clip, unaffected by any speed or retiming changes.
        unscaled_clip_start()                                           ->  i32 => pf_utility.unscaled_clip_start,
        /// Gets the start time of the track item.
        track_item_start()                                              ->  i32 => pf_utility.track_item_start,
        /// Retrieves the filed type in use with the media.
        media_field_type()                                              ->  prFieldType => pf_utility.media_field_type,
        /// Gets the number of ticks per frame, for the media.
        media_frame_rate()                                              ->  PrTime => pf_utility.media_frame_rate,
        /// Gets the ID of the timeline containing the clip to which the effect is applied.
        containing_timeline_id()                                        ->  PrTimelineID => pf_utility.containing_timeline_id,
        /// Gets the name of the clip to which the effect is applied (or the master clip).
        clip_name(get_master_clip_name: bool)                           ->  String => pf_utility.clip_name,
        /// Indicates that the effect wants to received checked out frames, in the same format used for destination rendering.
        effect_wants_checked_out_frames_to_match_render_pixel_format()  ->  () => pf_utility.effect_wants_checked_out_frames_to_match_render_pixel_format,
        /// Indicates whether the effect depends on the name of the clip to which it is applied.
        set_effect_depends_on_clip_name(depends_on_clip_name: bool)     ->  () => pf_utility.set_effect_depends_on_clip_name,
        /// Sets the instance name of the effect.
        set_effect_instance_name(name: &str)                            ->  () => pf_utility.set_effect_instance_name,
        /// Retrieves the name of the media file to which the effect instance is applied.
        file_name()                                                     ->  String => pf_utility.file_name,
        /// Retrieves the original (non-interpreted, un-re-timed) frame rate, of the media to which the effect instance is applied.
        original_clip_frame_rate()                                      ->  PrTime => pf_utility.original_clip_frame_rate,
        /// Retrieves the source media timecode for the specified frame within the specified layer, with or without transforms and start time offsets applied.
        source_track_media_timecode(layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool)  ->  A_long => pf_utility.source_track_media_timecode,
        /// Retrieves the name of the layer in use by the effect instance.
        source_track_clip_name(layer_param_index: u32, get_master_clip_name: bool)                               ->  String => pf_utility.source_track_clip_name,
        /// Retrieves the file name of the source track item for the specified layer parameter.
        source_track_file_name(layer_param_index: u32)                                                           ->  String => pf_utility.source_track_file_name,
        /// Specifies whether the effect instance depends on the specified layer parameter.
        set_effect_depends_on_clip_name2(depends_on_clip_name: bool, layer_param_index: u8)                          ->  () => pf_utility.set_effect_depends_on_clip_name2,
        /// Retrieves formatted timecode and current frame number, with or without trims applied.
        ///
        /// Returns a tuple containing `(current_frame, time_display)`
        media_timecode2(apply_trim: bool)                                                                        ->  (i32, PF_TimeDisplay) => pf_utility.media_timecode2,
        /// Given a specific sequence time, retrieves the source track media timecode for the specified layer parameter.
        source_track_media_timecode2(layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool, sequence_time: PrTime) -> A_long => pf_utility.source_track_media_timecode2,
        /// Retrieves the clip name used by the specific layer parameter.
        source_track_clip_name2(layer_param_index: u32, get_master_clip_name: bool, sequence_time: PrTime)       ->  String => pf_utility.source_track_clip_name2,
        /// Retrieves the clip name in use by the specified layer parameter.
        source_track_file_name2(layer_param_index: u32, sequence_time: PrTime)                                   ->  String => pf_utility.source_track_file_name2,
        /// Retrieves the comment string associated with the specified source track item, at the specified time.
        comment_string(source_track: i32, sequence_time: PrTime)         -> String => pf_utility.comment_string,
        /// Retrieves the log note associated with the source track, at the specified time.
        log_note_string(source_track: i32, sequence_time: PrTime)        -> String => pf_utility.log_note_string,
        /// Retrieves the camera rolll info associated with the source track, at the specified time.
        camera_roll_string(source_track: i32, sequence_time: PrTime)     -> String => pf_utility.camera_roll_string,
        /// Retrieves the metadata string associated with the source track, at the specified time.
        client_metadata_string(source_track: i32, sequence_time: PrTime) -> String => pf_utility.client_metadata_string,
        /// Retrieves the daily roll string associated with the source track, at the specified time.
        daily_roll_string(source_track: i32, sequence_time: PrTime)      -> String => pf_utility.daily_roll_string,
        /// Retrieves the description metadata string associated with the source track, at the specified time.
        description_string(source_track: i32, sequence_time: PrTime)     -> String => pf_utility.description_string,
        /// Retrieves the lab roll string associated with the source track, at the specified time.
        lab_roll_string(source_track: i32, sequence_time: PrTime)        -> String => pf_utility.lab_roll_string,
        /// Retrieves the scene string associated with the source track, at the specified time.
        scene_string(source_track: i32, sequence_time: PrTime)           -> String => pf_utility.scene_string,
        /// Retrieves the shot string associated with the source track item, at the specified time.
        shot_string(source_track: i32, sequence_time: PrTime)            -> String => pf_utility.shot_string,
        /// Retrieves the tape name string associated with the source track item, at the specified time.
        tape_name_string(source_track: i32, sequence_time: PrTime)       -> String => pf_utility.tape_name_string,
        /// Retrieves a string representing the video codec associated with the source track item, at the specified time.
        video_codec_string(source_track: i32, sequence_time: PrTime)     -> String => pf_utility.video_codec_string,
        /// Retrieves a string representing the "good" state of the source track item, at the specified time.
        good_metadata_string(source_track: i32, sequence_time: PrTime)   -> String => pf_utility.good_metadata_string,
        /// Retrieves a string representing the "sound roll" state of the source track item, at the specified time.
        sound_roll_string(source_track: i32, sequence_time: PrTime)      -> String => pf_utility.sound_roll_string,
        /// Retrieves the timebase of the sequence in which the effect is applied.
        sequence_time()                                                  -> PrTime => pf_utility.sequence_time,
        /// Retrieves the frame of the specified source time.
        sound_timecode(source_track: i32, sequence_time: PrTime)         -> i32    => pf_utility.sound_timecode,
        /// Retrieves the original "ticks per frame" for the specified source track.
        original_clip_frame_rate_for_source_track(source_track: i32)     -> PrTime => pf_utility.original_clip_frame_rate_for_source_track,
        /// Retrieves the media frame rate for the specified source track.
        media_frame_rate_for_source_track(source_track: i32, sequence_time: PrTime)         -> PrTime => pf_utility.media_frame_rate_for_source_track,
        /// Retrieves the start time of the specified layer parameter.
        source_track_media_actual_start_time(layer_param_index: u32, sequence_time: PrTime) -> PrTime => pf_utility.source_track_media_actual_start_time,
        /// Retrieves whether the source track item has been trimmed.
        is_source_track_media_trimmed(layer_param_index: u32, sequence_time: PrTime)        -> bool => pf_utility.is_source_track_media_trimmed,
        /// Retrieves whether the track item has been trimmed.
        is_media_trimmed(sequence_time: PrTime)                          -> bool => pf_utility.is_media_trimmed,
        /// Retrieves whether, for the specified layer parameter, the track is empty.
        is_track_empty(layer_param_index: u32, sequence_time: PrTime)    -> bool => pf_utility.is_track_empty,
        /// Retrieves whether the effect is applied to a track item backed by a synthetic importer.
        is_track_item_effect_applied_to_synthetic()                      -> bool => pf_utility.is_track_item_effect_applied_to_synthetic,

        // ―――――――――――――――――――――――――――― Effect Sequence Data suite functions ――――――――――――――――――――――――――――

        /// Retrieves the read-only const sequence_data object for a rendering thread when Multi-Frame Rendering is enabled for an effect.
        const_sequence_data() -> ae_sys::PF_ConstHandle => effect_sequence_data.const_sequence_data,

        // ―――――――――――――――――――――――――――― Param utils suite functions ――――――――――――――――――――――――――――

        /// Force After Effects to refresh the parameter's UI, in the effect controls palette.
        ///
        /// Starting in CC 2014, After Effects will now honor a change to a custom UI height. Simply change the ui_height of your custom UI PF_ParamDef and then call PF_UpdateParamUI.
        /// The effect's custom UI height will be updated in the Effect Control Window.
        ///
        /// Starting in CS6, when a plug-in disables a parameter, we now save that state in the UI flags so that the plug-in can check that flag in the future to see if it is disabled.
        ///
        /// NOTE: Never pass param\[0\] to this function.
        ///
        /// You can call this function for each param whose UI settings you want to change when handling a `Command::UserChangedParam` or `Command::UpdateParamsUi`.
        /// These changes are cosmetic only, and don't go into the undo buffer.
        ///
        /// The ONLY fields that can be changed in this way are:
        ///     PF_ParamDef
        ///         ui_flags: `PF_PUI_ECW_SEPARATOR`, `PF_PUI_DISABLED` only (and `PF_PUI_INVISIBLE` in Premiere).
        ///         ui_width
        ///         ui_height
        ///         name
        ///         flags: `PF_ParamFlag_COLLAPSE_TWIRLY` only
        ///     PF_ParamDefUnion:
        ///         slider_min, slider_max, precision, display_flags of any slider type
        /// For `PF_PUI_STD_CONTROL_ONLY` params, you can also change the value field by setting `PF_ChangeFlag_CHANGED_VALUE` before returning.
        /// But you are not allowed to change the value during `PF_Cmd_UPDATE_PARAMS_UI`.
        update_param_ui(param_index: i32, param_def: &ParamDef) -> () => param_utils.update_param_ui,

        /// This API, combined with [`are_states_identical()`](Self::are_states_identical) below, lets you determine if a set of inputs (either layers, other properties, or both)
        /// are different between when you first called [`current_state()`](Self::current_state) and a current call, so it can be used for caching.
        /// You can specify a range of time to consider or all of time.
        ///
        /// Updated in CS6 to add `param_index`, `start`, and `duration`. Pre-defined constants for `param_index` are as follows:
        ///
        ///   - [`PARAM_INDEX_CHECK_ALL`] - check every parameter, including every layer referred to by a layer parameter.
        ///   - [`PARAM_INDEX_CHECK_ALL_EXCEPT_LAYER_PARAMS`] - omit all layers. Pass a specific layer parameter index to include that as the only layer parameter tested.
        ///   - [`PARAM_INDEX_CHECK_ALL_HONOR_EXCLUDE`] - Similar to CHECK_ALL, but honor `PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED`.
        ///
        /// Passing in `None` for both start and duration indicates all time.
        /// For effects that do simulation across time and therefore set `PF_OutFlag2_AUTOMATIC_WIDE_TIME_INPUT`, when you ask about a time range,
        /// it will be expanded to include any times needed to produce that range.
        ///
        /// Populates a `PF_State`, an opaque data type used as a receipt for the current state of the effect's parameters (the `PF_State` is used in our internal frame caching database).
        current_param_state(param_index: i32, start: Option<Time>, duration: Option<Time>) -> ae_sys::PF_State => param_utils.current_state,

        /// New in CS6. Compare two different states, retrieved using `PF_GetCurrentState`, above.
        are_param_states_identical(state1: &ae_sys::PF_State, state2: &ae_sys::PF_State) -> bool => param_utils.are_states_identical,

        /// Returns `true` if a parameter's value is the same at the two passed times.
        ///
        /// Note: the times need not be contiguous; there could be different intervening values.
        is_param_identical_checkout(param_index: i32, what_time1: i32, time_step1: i32, time_scale1: u32, what_time2: i32, time_step2: i32, time_scale2: u32) -> bool => param_utils.is_identical_checkout,

        /// Searches (in the specified direction) for the next keyframe in the parameter's stream. The last three parameters are optional.
        ///
        /// Returns a tuple containing: (found, key_index, key_time, key_timescale)
        find_keyframe_time(param_index: i32, what_time: i32, time_scale: u32, time_dir: TimeDir) -> (bool, i32, i32, u32) => param_utils.find_keyframe_time,

        /// Returns the number of keyframes in the parameter's stream.
        keyframe_count(param_index: i32) -> i32 => param_utils.keyframe_count,

        /// Checks a keyframe for the specified parameter out of our keyframe database. `param_index` is zero-based. You can request time, timescale, or neither; useful if you're performing your own motion blur.
        checkout_keyframe(param_index: i32, key_index: i32) -> (i32, u32, ae_sys::PF_ParamDef) => param_utils.checkout_keyframe,

        /// All calls to `checkout_keyframe` must be balanced with this check-in, or pain will ensue.
        checkin_keyframe(param: ae_sys::PF_ParamDef) -> () => param_utils.checkin_keyframe,

        /// Returns the time (and timescale) of the specified keyframe.
        key_index_to_time(param_index: i32, key_index: i32) ->(i32, u32) => param_utils.key_index_to_time,

        // ―――――――――――――――――――――――――――― Effect UI suite functions ――――――――――――――――――――――――――――

        /// Changes the text on the options button in the effect controls palette.
        ///
        /// Button name can be up to 31 characters.
        ///
        /// NOTE: This must be called during [`Command::ParamSetup`].
        set_options_button_name(name: &str) -> () => effect_ui.set_options_button_name,
    }
);

impl Effect {
    /// Obtain the camera (if any) being used by After Effects to view the effect's layer.
    pub fn camera(&self, time: Time) -> Result<Option<aegp::Layer>, Error> {
        let Ok(ref suite) = *self.pf_interface else { return Err(Error::MissingSuite); };
        suite.effect_camera(self.handle.as_ptr(), time).map(|x| x.map(Into::into))
    }
}
