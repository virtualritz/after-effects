use crate::*;
use ae_sys::{ PF_TimeDisplay, prFieldType, PrTime, PrTimelineID, A_long };

register_handle!(PF_ProgPtr);
define_handle_wrapper!(EffectHandle, PF_ProgPtr);

define_suite_item_wrapper!(
    ae_sys::PF_ProgPtr, EffectHandle,
    pf_interface: aegp::suites::PFInterface,
    pf_utility: pf::suites::Utility,
    effect_sequence_data: pf::suites::EffectSequenceData,
    ///
    Effect {
        dispose: ;

        /// Returns the layer the effect is applied to
        layer() -> aegp::Layer => pf_interface.effect_layer,

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
        /// Retreives the clip name in use by the specified layer parameter.
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
    }
);
