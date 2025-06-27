#![allow(non_upper_case_globals)]

use crate::*;
use pr_sys::*;
use std::{ops::{Deref, DerefMut}, str::FromStr};

#[derive(Debug, Clone)]
pub enum PropertyData {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    USize(usize),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    String(String),
    Binary(Binary),
    Point32(Point32),
    Time(PrTime),
    Keyframes(Keyframes),
    Unknown(String),
}

// ------------------- Float point -------------------
#[derive(Debug, Clone, Copy)]
pub struct Point32 { pub x: f32, pub y: f32 }

impl FromStr for Point32 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let x = parts.next().ok_or(Error::InvalidParms)?.parse::<f32>().map_err(|_| Error::InvalidParms)?;
        let y = parts.next().ok_or(Error::InvalidParms)?.parse::<f32>().map_err(|_| Error::InvalidParms)?;
        Ok(Point32 { x, y })
    }
}
// ------------------- Float point -------------------
// ------------------- Binary data -------------------
#[derive(Debug, Clone)]
pub struct Binary(Vec<u8>);
impl FromStr for Binary {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use base64::prelude::*;
        Ok(Binary(BASE64_STANDARD.decode(s).map_err(|_| Error::InvalidParms)?))
    }
}
impl Deref for Binary {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Binary {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
// ------------------- Binary data -------------------
// -------------------- Keyframes --------------------
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Keyframes(pub String); // TODO: not implemented

impl FromStr for Keyframes {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned())) // TODO implement parsing this
    }
}
// -------------------- Keyframes --------------------

macro_rules! define_properties {
    ($(($field:ident, $ty:ident, $prop:ident),)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        pub enum Property {
            $( $field, )*
            Unknown(String)
        }

        impl Property {
            pub fn as_id(&self) -> &'static [u8] {
                match self {
                    $( Self::$field => $prop, )*
                    Self::Unknown(_) => b"Unknown",
                }
            }
            pub fn from_id(id: &[u8]) -> Self {
                $(
                    if id == &$prop[..$prop.len() - 1] { return Self::$field; }
                )*
                return Self::Unknown(String::from_utf8_lossy(id).to_string())
            }
            pub fn parse_result(&self, s: &str) -> PropertyData {
                match self {
                    $( Self::$field => {
                        match s.parse() {
                            Ok(v)  => PropertyData::$ty(v),
                            Err(_) => PropertyData::Unknown(s.to_owned())
                        }
                    } )*
                    Self::Unknown(_) => PropertyData::Unknown(s.to_owned()),
                }
            }
        }
    };
}

define_properties! {
    (Media_ClipID,                                   Int32,         kVideoSegmentProperty_Media_ClipID),
    (Media_ProxyClipID,                              Int32,         kVideoSegmentProperty_Media_ProxyClipID),
    (Media_InstanceString,                           String,        kVideoSegmentProperty_Media_InstanceString),
    (Media_ProxyInstanceString,                      String,        kVideoSegmentProperty_Media_ProxyInstanceString),
    (Media_ImplementationID,                         String,        kVideoSegmentProperty_Media_ImplementationID),
    (Media_StreamGroup,                              USize,         kVideoSegmentProperty_Media_StreamGroup),
    (Media_StreamLabel,                              String,        kVideoSegmentProperty_Media_StreamLabel),
    (Media_IsDraft,                                  Bool,          kVideoSegmentProperty_Media_IsDraft),
    (Media_ModState,                                 Binary,        kVideoSegmentProperty_Media_ModState),
    (Media_IsOffline,                                Bool,          kVideoSegmentProperty_Media_IsOffline),
    (Media_IsPending,                                Bool,          kVideoSegmentProperty_Media_IsPending),
    (Media_CaptioningID,                             String,        kVideoSegmentProperty_Media_CaptioningID),
    (Media_StreamFrameRate,                          Time,          kVideoSegmentProperty_Media_StreamFrameRate),
    (Media_StreamAlphaType,                          Int32,         kVideoSegmentProperty_Media_StreamAlphaType),
    (Media_StreamIgnoreAlpha,                        Bool,          kVideoSegmentProperty_Media_StreamIgnoreAlpha),
    (Media_StreamInvertAlpha,                        Bool,          kVideoSegmentProperty_Media_StreamInvertAlpha),
    (Media_StreamAlphaMatteColor,                    Int32,         kVideoSegmentProperty_Media_StreamAlphaMatteColor),
    (Media_StreamRemovePulldown,                     Bool,          kVideoSegmentProperty_Media_StreamRemovePulldown),
    (Media_StreamPixelAspectRatioNum,                Int32,         kVideoSegmentProperty_Media_StreamPixelAspectRatioNum),
    (Media_StreamFrameWidth,                         Int32,         kVideoSegmentProperty_Media_StreamFrameWidth),
    (Media_StreamFrameHeight,                        Int32,         kVideoSegmentProperty_Media_StreamFrameHeight),
    (Media_StreamPixelAspectRatioDen,                Int32,         kVideoSegmentProperty_Media_StreamPixelAspectRatioDen),
    (Media_StreamFieldType,                          Int32,         kVideoSegmentProperty_Media_StreamFieldType),
    (Media_StreamOpaqueData,                         Binary,        kVideoSegmentProperty_Media_StreamOpaqueData),
    (Media_ProxyStreamOpaqueData,                    Binary,        kVideoSegmentProperty_Media_ProxyStreamOpaqueData),
    (Media_StreamPullDownCadence,                    Int32,         kVideoSegmentProperty_Media_StreamPullDownCadence),
    (Media_StreamFrameVidSubType,                    Int32,         kVideoSegmentProperty_Media_StreamFrameVidSubType),
    (Media_StreamIsContinuousTime,                   Bool,          kVideoSegmentProperty_Media_StreamIsContinuousTime),
    (Media_StreamIsRollCrawl,                        Bool,          kVideoSegmentProperty_Media_StreamIsRollCrawl),
    (Media_RollCrawlDuration,                        Int32,         kVideoSegmentProperty_Media_RollCrawlDuration),
    (Media_ContentStart,                             Time,          kVideoSegmentProperty_Media_ContentStart),
    (Media_ContentEnd,                               Time,          kVideoSegmentProperty_Media_ContentEnd),
    (Media_StartTimecodeOffset,                      Time,          kVideoSegmentProperty_Media_StartTimecodeOffset),
    (Media_ProxyStartTimecodeOffset ,                Time,          kVideoSegmentProperty_Media_ProxyStartTimecodeOffset),
    (Media_NestedSequenceHash,                       String,        kVideoSegmentProperty_Media_NestedSequenceHash),
    (Media_SelectedMulticamTrackFromNestedSequenc,   Int32,         kVideoSegmentProperty_Media_SelectedMulticamTrackFromNestedSequence),
    (Media_MulticamCameraOrderFromNestedSequence,    String,        kVideoSegmentProperty_Media_MulticamCameraOrderFromNestedSequence),
    (Media_MulticamCamerasPerPage,                   String,        kVideoSegmentProperty_Media_MulticamCamerasPerPage),
    (Media_MulticamCurrentPage,                      String,        kVideoSegmentProperty_Media_MulticamCurrentPage),
    (Media_SelectedMulticamTrackTimelineID,          Int32,         kVideoSegmentProperty_Media_SelectedMulticamTrackTimelineID),
    (Media_NestedSequenceTimelineID,                 Int32,         kVideoSegmentProperty_Media_NestedSequenceTimelineID),
    (Media_TrackItemIsMuted,                         Bool,          kVideoSegmentProperty_Media_TrackItemIsMuted),
    (Media_ClipSpeed,                                Float64,       kVideoSegmentProperty_Media_ClipSpeed),
    (Media_ClipBackwards,                            Bool,          kVideoSegmentProperty_Media_ClipBackwards),
    (Media_StreamFrameBlend,                         Bool,          kVideoSegmentProperty_Media_StreamFrameBlend),
    (Media_StreamTimeInterpolationType,              UInt32,        kVideoSegmentProperty_Media_StreamTimeInterpolationType), // dvamediatypes::TimeInterpolationType
    (Media_ClipScaleToFrameSize,                     Bool,          kVideoSegmentProperty_Media_ClipScaleToFrameSize),
    (Media_ClipScaleToFramePolicy,                   Int32,         kVideoSegmentProperty_Media_ClipScaleToFramePolicy), // int, optional see PrNodeScalePolicy
    (Media_StreamReverseFieldDominance,              Bool,          kVideoSegmentProperty_Media_StreamReverseFieldDominance),
    (Media_DeinterlaceAlways,                        Bool,          kVideoSegmentProperty_Media_DeinterlaceAlways),
    (Media_RemoveFlicker,                            Bool,          kVideoSegmentProperty_Media_RemoveFlicker),
    (Media_InterlaceConsecutiveFrames,               Bool,          kVideoSegmentProperty_Media_InterlaceConsecutiveFrames),
    (Media_SequenceColorSpace,                       String,        kVideoSegmentProperty_Media_SequenceColorSpace),
    (Media_StreamColorSpace,                         String,        kVideoSegmentProperty_Media_StreamColorSpace),
    (Media_StreamInputLUTID,                         String,        kVideoSegmentProperty_Media_StreamInputLUTID),
    (Media_ScanlineOffsetToImproveVerticalCentering, Int32,         kVideoSegmentProperty_Media_ScanlineOffsetToImproveVerticalCentering), // positive values mean shift up, negative means shift down
    (Media_InPointMediaTimeAsTicks,                  Int64,         kVideoSegmentProperty_Media_InPointMediaTimeAsTicks),                  // media in point in units of ticks in media time
    (Media_OutPointMediaTimeAsTicks,                 Int64,         kVideoSegmentProperty_Media_OutPointMediaTimeAsTicks),                 // media out point in units of ticks in media time
    (Media_SequenceFieldType,                        Int32,         kVideoSegmentProperty_Media_SequenceFieldType),                        // containing sequence field type
    (Media_SequenceFrameRate,                        Int32,         kVideoSegmentProperty_Media_SequenceFrameRate),                        // containing sequence frame rate
    (Media_SequenceWidth,                            Int32,         kVideoSegmentProperty_Media_SequenceWidth),                            // containing sequence width
    (Media_SequenceHeight,                           Int32,         kVideoSegmentProperty_Media_SequenceHeight),                           // containing sequence height
    (Media_SequencePixelAspectRatioNum,              Int32,         kVideoSegmentProperty_Media_SequencePixelAspectRatioNum),              // containing sequence pixel aspect ratio num
    (Media_SequencePixelAspectRatioDen,              Int32,         kVideoSegmentProperty_Media_SequencePixelAspectRatioDen),              // containing sequence pixel aspect ratio den
    // (Media_OrientationType,                       ???,           kVideoSegmentProperty_Media_OrientationType),                          // Orientation for primary source
    // (Media_OrientationTypeProxy,                  ???,           kVideoSegmentProperty_Media_OrientationTypeProxy),                     // Orientation for Proxy

    (Clip_Speed,                                     Float64,       kVideoSegmentProperty_Clip_ClipSpeed),
    (Clip_Backwards,                                 Bool,          kVideoSegmentProperty_Clip_ClipBackwards),
    (Clip_TimeRemapping,                             Keyframes,     kVideoSegmentProperty_Clip_TimeRemapping),
    (Clip_FrameHoldAtTime,                           Time,          kVideoSegmentProperty_Clip_FrameHoldAtTime),
    (Clip_FrameHoldFilters,                          Bool,          kVideoSegmentProperty_Clip_FrameHoldFilters),
    (Clip_GrowingDuration,                           Time,          kVideoSegmentProperty_Clip_GrowingDuration),
    (Clip_FrameHoldDeinterlace,                      Bool,          kVideoSegmentProperty_Clip_FrameHoldDeinterlace),
    (Clip_TrackID,                                   Int32,         kVideoSegmentProperty_Clip_TrackID),
    (Clip_TrackItemStartAsTicks,                     Int64,         kVideoSegmentProperty_Clip_TrackItemStartAsTicks),          // the start of the track item in units of ticks in sequence time
    (Clip_TrackItemEndAsTicks,                       Int64,         kVideoSegmentProperty_Clip_TrackItemEndAsTicks),            // the end of the track item in units of ticks in sequence time
    (Clip_EffectiveTrackItemStartAsTicks,            Int64,         kVideoSegmentProperty_Clip_EffectiveTrackItemStartAsTicks), // the start of the track item, adjusted for transitions at the head, in units of ticks in sequence time
    (Clip_EffectiveTrackItemEndAsTicks,              Int64,         kVideoSegmentProperty_Clip_EffectiveTrackItemEndAsTicks),   // the start of the track item, adjusted for transitions at the tail, in units of ticks in sequence time
    (Clip_AllowLinearCompositing,                    Bool,          kVideoSegmentProperty_Clip_AllowLinearCompositing),         // only set if false
    (Clip_HasCaptions,                               Bool,          kVideoSegmentProperty_Clip_HasCaptions),                    // only set if true
    (Clip_UntrimmedDuration,                         Int64,         kVideoSegmentProperty_Clip_UntrimmedDuration),              // the untrimmed duration of the clip in ticks
    (Clip_ToneMapSettings,                           String,        kVideoSegmentProperty_Clip_ToneMapSettings),                // in JSON format

    (Multicam_SelectedTrack,                         Int32,         kVideoSegmentProperty_Multicam_SelectedTrack),
    (Multicam_CameraOrder,                           String,        kVideoSegmentProperty_Multicam_CameraOrder),
    (Multicam_CamerasPerPage,                        String,        kVideoSegmentProperty_Multicam_CamerasPerPage),
    (Multicam_CurrentPage,                           String,        kVideoSegmentProperty_Multicam_CurrentPage),
    (Multicam_ShowProgram,                           Bool,          kVideoSegmentProperty_Multicam_ShowProgram),
    (Multicam_Recording,                             Bool,          kVideoSegmentProperty_Multicam_Recording),
    (Multicam_IsNonMultiCam,                         Bool,          kVideoSegmentProperty_Multicam_IsNonMultiCam),
    (Multicam_UseTimeRemapping,                      Bool,          kVideoSegmentProperty_Multicam_UseTimeRemapping),

    (SolidColor_Color,                               Int32,         kVideoSegmentProperty_SolidColor_Color),

    (Effect_FilterMatchName,                         String,        kVideoSegmentProperty_Effect_FilterMatchName),
    (Effect_FilterCategoryName,                      String,        kVideoSegmentProperty_Effect_FilterCategoryName),
    (Effect_FilterOpaqueData,                        Binary,        kVideoSegmentProperty_Effect_FilterOpaqueData),
    (Effect_FilterParams,                            Keyframes,     kVideoSegmentProperty_Effect_FilterParams),
    (Effect_EffectDuration,                          Time,          kVideoSegmentProperty_Effect_EffectDuration),
    (Effect_RuntimeInstanceID,                       UInt32,        kVideoSegmentProperty_Effect_RuntimeInstanceID),
    (Effect_LayerInputHashes,                        String,        kVideoSegmentProperty_Effect_LayerInputHashes),
    (Effect_RuntimeHash,                             String,        kVideoSegmentProperty_Effect_RuntimeHash),
    (Effect_StreamLabel,                             String,        kVideoSegmentProperty_Effect_StreamLabel),
    (Effect_ClipName,                                String,        kVideoSegmentProperty_Effect_ClipName),
    (Effect_MasterClipName,                          String,        kVideoSegmentProperty_Effect_MasterClipName),
    (Effect_FileName,                                String,        kVideoSegmentProperty_Effect_FileName),
    (Effect_SourceTrackClipNameHashes,               String,        kVideoSegmentProperty_Effect_SourceTrackClipNameHashes),

    (Transition_MatchName,                           String,        kVideoSegmentProperty_Transition_TransitionMatchName),
    (Transition_OpaqueData,                          Binary,        kVideoSegmentProperty_Transition_TransitionOpaqueData),
    (Transition_StartPosition,                       Point32,       kVideoSegmentProperty_Transition_TransitionStartPosition),
    (Transition_EndPosition,                         Point32,       kVideoSegmentProperty_Transition_TransitionEndPosition),
    (Transition_CenterPosition,                      Point32,       kVideoSegmentProperty_Transition_TransitionCenterPosition),
    (Transition_StartPercent,                        Float32,       kVideoSegmentProperty_Transition_TransitionStartPercent),
    (Transition_EndPercent,                          Float32,       kVideoSegmentProperty_Transition_TransitionEndPercent),
    (Transition_BorderWidth,                         Float32,       kVideoSegmentProperty_Transition_TransitionBorderWidth),
    (Transition_BorderColor,                         Int32,         kVideoSegmentProperty_Transition_TransitionBorderColor),
    (Transition_SwitchSources,                       Bool,          kVideoSegmentProperty_Transition_TransitionSwitchSources),
    (Transition_Reverse,                             Bool,          kVideoSegmentProperty_Transition_TransitionReverse),
    (Transition_Direction,                           Int32,         kVideoSegmentProperty_Transition_TransitionDirection),
    (Transition_AntiAliasQuality,                    Int32,         kVideoSegmentProperty_Transition_TransitionAntiAliasQuality),
    (Transition_Duration,                            Time,          kVideoSegmentProperty_Transition_TransitionDuration),
    (Transition_Params,                              Keyframes,     kVideoSegmentProperty_Transition_TransitionParams),
    (Transition_RuntimeInstanceID,                   UInt32,        kVideoSegmentProperty_Transition_RuntimeInstanceID),
    (Transition_TransitionCategoryName,              String,        kVideoSegmentProperty_Transition_TransitionCategoryName),

    (Adjustment_MediaIsOpaque,                       Bool,          kVideoSegmentProperty_Adjustment_AdjustmentMediaIsOpaque),
    (Adjustment_InvertAlpha,                         Bool,          kVideoSegmentProperty_Adjustment_InvertAlpha),
}
