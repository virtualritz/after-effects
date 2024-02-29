use crate::*;
use ae_sys::*;

define_suite!(
    /// Utility functions for use by AE style effect plugins, running in Premiere Pro.
    UtilitySuite,
    PF_UtilitySuite,
    kPFUtilitySuite,
    kPFUtilitySuiteVersion
);

impl UtilitySuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Gets the filter ID for the current effect reference.
    pub fn filter_instance_id(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetFilterInstanceID -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Retrieves formatted timecode, as well as the currently active video frame.
    ///
    /// Returns a tuple containing `(current_frame, time_display)`
    pub fn media_timecode(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<(i32, PF_TimeDisplay), Error> {
        call_suite_fn_double!(self, GetMediaTimecode -> i32, PF_TimeDisplay, effect_ref.as_ptr())
    }

    /// Retrieves the speed multiplier of the clip.
    pub fn clip_speed(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<f64, Error> {
        call_suite_fn_single!(self, GetClipSpeed -> f64, effect_ref.as_ptr())
    }

    /// Retrieves the duration of the clip.
    pub fn clip_duration(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetClipDuration -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Retrieves the start time of the clip.
    pub fn clip_start(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetClipStart -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Retrieves the duration of the clip, unaffected by any speed or retiming changes.
    pub fn unscaled_clip_duration(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetUnscaledClipDuration -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Retrives the start time of the clip, unaffected by any speed or retiming changes.
    pub fn unscaled_clip_start(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetUnscaledClipStart -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Gets the start time of the track item.
    pub fn track_item_start(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetTrackItemStart -> A_long, effect_ref.as_ptr())? as i32)
    }

    /// Retrieves the filed type in use with the media.
    pub fn media_field_type(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<prFieldType, Error> {
        call_suite_fn_single!(self, GetMediaFieldType -> prFieldType, effect_ref.as_ptr())
    }

    /// Gets the number of ticks per frame, for the media.
    pub fn media_frame_rate(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetMediaFrameRate -> PrTime, effect_ref.as_ptr())
    }

    /// Gets the ID of the timeline containing the clip to which the effect is applied.
    pub fn containing_timeline_id(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTimelineID, Error> {
        call_suite_fn_single!(self, GetContainingTimelineID -> PrTimelineID, effect_ref.as_ptr())
    }

    /// Gets the name of the clip to which the effect is applied (or the master clip).
    pub fn clip_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, get_master_clip_name: bool) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetClipName -> PrSDKString, effect_ref.as_ptr(), get_master_clip_name as _)?).into())
    }

    /// Indicates that the effect wants to received checked out frames, in the same format used for destination rendering.
    pub fn effect_wants_checked_out_frames_to_match_render_pixel_format(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<(), Error> {
        call_suite_fn!(self, EffectWantsCheckedOutFramesToMatchRenderPixelFormat, effect_ref.as_ptr())
    }

    /// Indicates whether the effect depends on the name of the clip to which it is applied.
    pub fn set_effect_depends_on_clip_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, depends_on_clip_name: bool) -> Result<(), Error> {
        call_suite_fn!(self, EffectDependsOnClipName, effect_ref.as_ptr(), depends_on_clip_name as _)
    }

    /// Sets the instance name of the effect.
    pub fn set_effect_instance_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, name: &str) -> Result<(), Error> {
        let pr_string = PrStringSuite::new()?.allocate_from_utf8(name)?;
        call_suite_fn!(self, SetEffectInstanceName, effect_ref.as_ptr(), &pr_string)
    }

    /// Retrieves the name of the media file to which the effect instance is applied.
    pub fn file_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetFileName -> PrSDKString, effect_ref.as_ptr())?).into())
    }

    /// Retrieves the original (non-interpreted, un-re-timed) frame rate, of the media to which the effect instance is applied.
    pub fn original_clip_frame_rate(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetOriginalClipFrameRate -> PrTime, effect_ref.as_ptr())
    }

    /// Retrieves the source media timecode for the specified frame within the specified layer, with or without transforms and start time offsets applied.
    pub fn source_track_media_timecode(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool) -> Result<A_long, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaTimecode -> A_long, effect_ref.as_ptr(), layer_param_index, apply_transform, add_start_time_offset)
    }

    /// Retrieves the name of the layer in use by the effect instance.
    pub fn source_track_clip_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, get_master_clip_name: bool) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSourceTrackClipName -> PrSDKString, effect_ref.as_ptr(), layer_param_index, get_master_clip_name as _)?).into())
    }

    /// Retrieves the file name of the source track item for the specified layer parameter.
    pub fn source_track_file_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSourceTrackFileName -> PrSDKString, effect_ref.as_ptr(), layer_param_index)?).into())
    }

    /// Specifies whether the effect instance depends on the specified layer parameter.
    pub fn set_effect_depends_on_clip_name2(&self, effect_ref: impl AsPtr<PF_ProgPtr>, depends_on_clip_name: bool, layer_param_index: u8) -> Result<(), Error> {
        call_suite_fn!(self, EffectDependsOnClipName2, effect_ref.as_ptr(), layer_param_index, depends_on_clip_name as _)
    }

    /// Retrieves formatted timecode and current frame number, with or without trims applied.
    ///
    /// Returns a tuple containing `(current_frame, time_display)`
    pub fn media_timecode2(&self, effect_ref: impl AsPtr<PF_ProgPtr>, apply_trim: bool) -> Result<(i32, PF_TimeDisplay), Error> {
        call_suite_fn_double!(self, GetMediaTimecode2 -> i32, PF_TimeDisplay, effect_ref.as_ptr(), apply_trim)
    }

    /// Given a specific sequence time, retrieves the source track media timecode for the specified layer parameter.
    pub fn source_track_media_timecode2(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool, sequence_time: PrTime) -> Result<A_long, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaTimecode2 -> A_long, effect_ref.as_ptr(), layer_param_index, apply_transform, add_start_time_offset, sequence_time)
    }

    /// Retrieves the clip name used by the specific layer parameter.
    pub fn source_track_clip_name2(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, get_master_clip_name: bool, sequence_time: PrTime) -> Result<String, Error> {
        let mut val: PrSDKString = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetSourceTrackClipName2, effect_ref.as_ptr(), layer_param_index, get_master_clip_name as _, &mut val, sequence_time)?;

        Ok(PrString(val).into())
    }

    /// Retreives the clip name in use by the specified layer parameter.
    pub fn source_track_file_name2(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, sequence_time: PrTime) -> Result<String, Error> {
        let mut val: PrSDKString = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetSourceTrackFileName2, effect_ref.as_ptr(), layer_param_index, &mut val, sequence_time)?;

        Ok(PrString(val).into())
    }

    /// Retrieves the comment string associated with the specified source track item, at the specified time.
    pub fn comment_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetCommentString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the log note associated with the source track, at the specified time.
    pub fn log_note_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetLogNoteString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the camera rolll info associated with the source track, at the specified time.
    pub fn camera_roll_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetCameraRollString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the metadata string associated with the source track, at the specified time.
    pub fn client_metadata_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetClientMetadataString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the daily roll string associated with the source track, at the specified time.
    pub fn daily_roll_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetDailyRollString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the description metadata string associated with the source track, at the specified time.
    pub fn description_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetDescriptionString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the lab roll string associated with the source track, at the specified time.
    pub fn lab_roll_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetLabRollString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the scene string associated with the source track, at the specified time.
    pub fn scene_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSceneString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the shot string associated with the source track item, at the specified time.
    pub fn shot_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetShotString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the tape name string associated with the source track item, at the specified time.
    pub fn tape_name_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetTapeNameString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves a string representing the video codec associated with the source track item, at the specified time.
    pub fn video_codec_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetVideoCodecString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves a string representing the "good" state of the source track item, at the specified time.
    pub fn good_metadata_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetGoodMetadataString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves a string representing the "sound roll" state of the source track item, at the specified time.
    pub fn sound_roll_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSoundRollString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    }

    /// Retrieves the timebase of the sequence in which the effect is applied.
    pub fn sequence_time(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetSequenceTime -> PrTime, effect_ref.as_ptr())
    }

    /// Retrieves the frame of the specified source time.
    pub fn sound_timecode(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetSoundTimecode -> A_long, effect_ref.as_ptr(), source_track, sequence_time)? as i32)
    }

    /// Retrieves the original "ticks per frame" for the specified source track.
    pub fn original_clip_frame_rate_for_source_track(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetOriginalClipFrameRateForSourceTrack -> PrTime, effect_ref.as_ptr(), source_track)
    }

    /// Retrieves the media frame rate for the specified source track.
    pub fn media_frame_rate_for_source_track(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetMediaFrameRateForSourceTrack -> PrTime, effect_ref.as_ptr(), source_track, sequence_time)
    }

    /// Retrieves the start time of the specified layer parameter.
    pub fn source_track_media_actual_start_time(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, sequence_time: PrTime) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaActualStartTime -> PrTime, effect_ref.as_ptr(), layer_param_index, sequence_time)
    }

    /// Retrieves whether the source track item has been trimmed.
    pub fn is_source_track_media_trimmed(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsSourceTrackMediaTrimmed -> bool, effect_ref.as_ptr(), layer_param_index, sequence_time)
    }

    /// Retrieves whether the track item has been trimmed.
    pub fn is_media_trimmed(&self, effect_ref: impl AsPtr<PF_ProgPtr>, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsMediaTrimmed -> bool, effect_ref.as_ptr(), sequence_time)
    }

    /// Retrieves whether, for the specified layer parameter, the track is empty.
    pub fn is_track_empty(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsTrackEmpty -> bool, effect_ref.as_ptr(), layer_param_index, sequence_time)
    }

    /// Retrieves whether the effect is applied to a track item backed by a synthetic importer.
    pub fn is_track_item_effect_applied_to_synthetic(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsTrackItemEffectAppliedToSynthetic -> bool, effect_ref.as_ptr())
    }

    // These are added in version 11, which is not in the AE SDK, but it is in the Pr SDK

    // /// Retrieves the current media time, including ticks per frame and a formatted string representing that time.
    // ///
    // /// Returns a tuple containing `(current_media_time, media_ticks_per_frame, media_time_display)`
    // pub fn get_source_track_current_media_time_info(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, use_sound_timecode_as_start_time: bool, sequence_time: PrTime) -> Result<(PrTime, PrTime, PF_TimeDisplay), Error> {
    //     let mut current_media_time = 0;
    //     let mut media_ticks_per_frame = 0;
    //     let mut media_time_display = 0;
    //     call_suite_fn!(self,
    //         GetSourceTrackCurrentMediaTimeInfo,
    //         effect_ref.as_ptr(),
    //         layer_param_index,
    //         use_sound_timecode_as_start_time,
    //         sequence_time,
    //         &mut current_media_time,
    //         &mut media_ticks_per_frame,
    //         &mut media_time_display
    //     )?;
    //     Ok((current_media_time, media_ticks_per_frame, media_time_display))
    // }

    // /// Retrieves the zero point (start time) of the sequence in which the effect is applied.
    // pub fn sequence_zero_point(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTime, Error> {
    //     call_suite_fn_single!(self, GetSequenceZeroPoint -> PrTime, effect_ref.as_ptr())
    // }

    // /// Retrieves the duration of the clip, at the specified layer index, at inSequenceTime.
    // pub fn source_track_current_clip_duration(&self, effect_ref: impl AsPtr<PF_ProgPtr>, layer_param_index: u32, sequence_time: PrTime) -> Result<PrTime, Error> {
    //     call_suite_fn_single!(self, GetSourceTrackCurrentClipDuration -> PrTime, effect_ref.as_ptr(), layer_param_index, sequence_time)
    // }

    // /// Retrieves the duration of the sequence in which the effect is applied.
    // pub fn sequence_duration(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<PrTime, Error> {
    //     call_suite_fn_single!(self, GetSequenceDuration -> PrTime, effect_ref.as_ptr())
    // }

    // /// Retrieve a string representing the dimensions of the track item to which the effect is applied.
    // /// It's formatted as a "width x height".
    // /// Set `source_track` to -1 to query the top-most clip at `sequence_time` (only if effect is on an adjustment layer)
    // pub fn video_resolution_string(&self, effect_ref: impl AsPtr<PF_ProgPtr>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
    //     Ok(PrString(call_suite_fn_single!(self, GetVideoResolutionString -> PrSDKString, effect_ref.as_ptr(), source_track, sequence_time)?).into())
    // }
}
