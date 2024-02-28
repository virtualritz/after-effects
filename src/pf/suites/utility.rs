use crate::*;
use ae_sys::*;

define_suite!(
    UtilitySuite,
    PF_UtilitySuite,
    kPFUtilitySuite,
    kPFUtilitySuiteVersion
);

impl UtilitySuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn filter_instance_id(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetFilterInstanceID -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn media_timecode(&self, in_data: impl AsRef<PF_InData>) -> Result<(i32, PF_TimeDisplay), Error> {
        let mut current_frame = 0;
        let mut time_display = 0;
        call_suite_fn!(self, GetMediaTimecode, in_data.as_ref().effect_ref, &mut current_frame, &mut time_display)?;
        Ok((current_frame, time_display))
    }

    pub fn clip_speed(&self, in_data: impl AsRef<PF_InData>) -> Result<f64, Error> {
        call_suite_fn_single!(self, GetClipSpeed -> f64, in_data.as_ref().effect_ref)
    }

    pub fn clip_duration(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetClipDuration -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn clip_start(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetClipStart -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn unscaled_clip_duration(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetUnscaledClipDuration -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn unscaled_clip_start(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetUnscaledClipStart -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn track_item_start(&self, in_data: impl AsRef<PF_InData>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetTrackItemStart -> A_long, in_data.as_ref().effect_ref)? as i32)
    }

    pub fn media_field_type(&self, in_data: impl AsRef<PF_InData>) -> Result<prFieldType, Error> {
        call_suite_fn_single!(self, GetMediaFieldType -> prFieldType, in_data.as_ref().effect_ref)
    }

    pub fn media_frame_rate(&self, in_data: impl AsRef<PF_InData>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetMediaFrameRate -> PrTime, in_data.as_ref().effect_ref)
    }

    pub fn containing_timeline_id(&self, in_data: impl AsRef<PF_InData>) -> Result<PrTimelineID, Error> {
        call_suite_fn_single!(self, GetContainingTimelineID -> PrTimelineID, in_data.as_ref().effect_ref)
    }

    pub fn clip_name(&self, in_data: impl AsRef<PF_InData>, get_master_clip_name: bool) -> Result<String, Error> {
        let get_master_clip_name = if get_master_clip_name { 1 } else { 0 };
        Ok(PrString(call_suite_fn_single!(self, GetClipName -> PrSDKString, in_data.as_ref().effect_ref, get_master_clip_name)?).into())
    }

    pub fn effect_wants_checked_out_frames_to_match_render_pixel_format(&self, in_data: impl AsRef<PF_InData>) -> Result<(), Error> {
        call_suite_fn!(self, EffectWantsCheckedOutFramesToMatchRenderPixelFormat, in_data.as_ref().effect_ref)
    }

    pub fn effect_depends_on_clip_name(&self, in_data: impl AsRef<PF_InData>, depends_on_clip_name: bool) -> Result<(), Error> {
        let depends_on_clip_name = if depends_on_clip_name { 1 } else { 0 };
        call_suite_fn!(self, EffectDependsOnClipName, in_data.as_ref().effect_ref, depends_on_clip_name)
    }

    pub fn set_effect_instance_name(&self, in_data: impl AsRef<PF_InData>, name: &str) -> Result<(), Error> {
        let pr_string = PrStringSuite::new()?.allocate_from_utf8(name)?;
        call_suite_fn!(self, SetEffectInstanceName, in_data.as_ref().effect_ref, &pr_string)
    }

    pub fn file_name(&self, in_data: impl AsRef<PF_InData>) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetFileName -> PrSDKString, in_data.as_ref().effect_ref)?).into())
    }

    pub fn original_clip_frame_rate(&self, in_data: impl AsRef<PF_InData>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetOriginalClipFrameRate -> PrTime, in_data.as_ref().effect_ref)
    }

    pub fn source_track_media_timecode(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool) -> Result<A_long, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaTimecode -> A_long, in_data.as_ref().effect_ref, layer_param_index, apply_transform, add_start_time_offset)
    }

    pub fn source_track_clip_name(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, get_master_clip_name: bool) -> Result<String, Error> {
        let get_master_clip_name = if get_master_clip_name { 1 } else { 0 };
        Ok(PrString(call_suite_fn_single!(self, GetSourceTrackClipName -> PrSDKString, in_data.as_ref().effect_ref, layer_param_index, get_master_clip_name)?).into())
    }

    pub fn source_track_file_name(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSourceTrackFileName -> PrSDKString, in_data.as_ref().effect_ref, layer_param_index)?).into())
    }

    pub fn effect_depends_on_clip_name2(&self, in_data: impl AsRef<PF_InData>, depends_on_clip_name: bool, layer_param_index: u8) -> Result<(), Error> {
        let depends_on_clip_name = if depends_on_clip_name { 1 } else { 0 };
        call_suite_fn!(self, EffectDependsOnClipName2, in_data.as_ref().effect_ref, layer_param_index, depends_on_clip_name)
    }

    pub fn media_timecode2(&self, in_data: impl AsRef<PF_InData>, apply_trim: bool) -> Result<(i32, PF_TimeDisplay), Error> {
        let mut current_frame = 0;
        let mut time_display = 0;
        call_suite_fn!(self, GetMediaTimecode2, in_data.as_ref().effect_ref, apply_trim, &mut current_frame, &mut time_display)?;
        Ok((current_frame, time_display))
    }

    pub fn source_track_media_timecode2(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, apply_transform: bool, add_start_time_offset: bool, sequence_time: PrTime) -> Result<A_long, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaTimecode2 -> A_long, in_data.as_ref().effect_ref, layer_param_index, apply_transform, add_start_time_offset, sequence_time)
    }

    pub fn source_track_clip_name2(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, get_master_clip_name: bool, sequence_time: PrTime) -> Result<String, Error> {
        let get_master_clip_name = if get_master_clip_name { 1 } else { 0 };

        let mut val: PrSDKString = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetSourceTrackClipName2, in_data.as_ref().effect_ref, layer_param_index, get_master_clip_name, &mut val, sequence_time)?;

        Ok(PrString(val).into())
    }

    pub fn source_track_file_name2(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, sequence_time: PrTime) -> Result<String, Error> {
        let mut val: PrSDKString = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, GetSourceTrackFileName2, in_data.as_ref().effect_ref, layer_param_index, &mut val, sequence_time)?;

        Ok(PrString(val).into())
    }

    pub fn comment_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetCommentString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn log_note_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetLogNoteString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn camera_roll_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetCameraRollString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn client_metadata_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetClientMetadataString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn daily_roll_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetDailyRollString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn description_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetDescriptionString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn lab_roll_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetLabRollString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn scene_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSceneString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn shot_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetShotString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn tape_name_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetTapeNameString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn video_codec_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetVideoCodecString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn good_metadata_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetGoodMetadataString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn sound_roll_string(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<String, Error> {
        Ok(PrString(call_suite_fn_single!(self, GetSoundRollString -> PrSDKString, in_data.as_ref().effect_ref, source_track, sequence_time)?).into())
    }

    pub fn sequence_time(&self, in_data: impl AsRef<PF_InData>) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetSequenceTime -> PrTime, in_data.as_ref().effect_ref)
    }

    pub fn sound_timecode(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, GetSoundTimecode -> A_long, in_data.as_ref().effect_ref, source_track, sequence_time)? as i32)
    }

    pub fn original_clip_frame_rate_for_source_track(&self, in_data: impl AsRef<PF_InData>, source_track: i32) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetOriginalClipFrameRateForSourceTrack -> PrTime, in_data.as_ref().effect_ref, source_track)
    }

    pub fn media_frame_rate_for_source_track(&self, in_data: impl AsRef<PF_InData>, source_track: i32, sequence_time: PrTime) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetMediaFrameRateForSourceTrack -> PrTime, in_data.as_ref().effect_ref, source_track, sequence_time)
    }

    pub fn source_track_media_actual_start_time(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, sequence_time: PrTime) -> Result<PrTime, Error> {
        call_suite_fn_single!(self, GetSourceTrackMediaActualStartTime -> PrTime, in_data.as_ref().effect_ref, layer_param_index, sequence_time)
    }

    pub fn is_source_track_media_trimmed(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsSourceTrackMediaTrimmed -> bool, in_data.as_ref().effect_ref, layer_param_index, sequence_time)
    }

    pub fn is_media_trimmed(&self, in_data: impl AsRef<PF_InData>, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsMediaTrimmed -> bool, in_data.as_ref().effect_ref, sequence_time)
    }

    pub fn is_track_empty(&self, in_data: impl AsRef<PF_InData>, layer_param_index: u32, sequence_time: PrTime) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsTrackEmpty -> bool, in_data.as_ref().effect_ref, layer_param_index, sequence_time)
    }

    pub fn is_track_item_effect_applied_to_synthetic(&self, in_data: impl AsRef<PF_InData>) -> Result<bool, Error> {
        call_suite_fn_single!(self, IsTrackItemEffectAppliedToSynthetic -> bool, in_data.as_ref().effect_ref)
    }
}
