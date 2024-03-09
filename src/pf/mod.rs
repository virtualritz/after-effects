use crate::*;
use bitflags::bitflags;
use std::{
    fmt::Debug,
    marker::PhantomData,
};

mod command;    pub use command::*;
mod events;     pub use events::*;
mod gpu;        pub use gpu::*;
mod handles;    pub use handles::*;
mod in_data;    pub use in_data::*;
mod layer;      pub use layer::*;
mod out_data;   pub use out_data::*;
mod parameters; pub use parameters::*;
mod pixel;      pub use pixel::*;
mod render;     pub use render::*;
mod effect;     pub use effect::*;
mod interact_callbacks;    pub use interact_callbacks::*;
mod util_callbacks;        pub use util_callbacks::*;
mod external_dependencies; pub use external_dependencies::*;

pub mod suites {
    pub(crate) mod adv_item;              pub use adv_item            ::AdvItemSuite               as AdvItem;
    pub(crate) mod background_frame;      pub use background_frame    ::BackgroundFrameSuite       as BackgroundFrame;
    pub(crate) mod cache_on_load;         pub use cache_on_load       ::CacheOnLoadSuite           as CacheOnLoad;
    pub(crate) mod channel;               pub use channel             ::ChannelSuite               as Channel;
    pub(crate) mod color_callbacks;       pub use color_callbacks     ::{ ColorCallbacksSuite      as ColorCallbacks,
                                                                          ColorCallbacks16Suite    as ColorCallbacks16,
                                                                          ColorCallbacksFloatSuite as ColorCallbacksFloat };
    pub(crate) mod effect_sequence_data;  pub use effect_sequence_data::EffectSequenceDataSuite    as EffectSequenceData;
    pub(crate) mod effect_ui;             pub use effect_ui           ::EffectUISuite              as EffectUI;
    pub(crate) mod app;                   pub use app                 ::{ AppSuite                 as App,
                                                                          AdvAppSuite              as AdvApp };
    pub(crate) mod custom_ui;             pub use custom_ui           ::{ EffectCustomUISuite      as EffectCustomUI,
                                                                          EffectCustomUIOverlayThemeSuite as EffectCustomUIOverlayTheme };
    pub(crate) mod iterate;               pub use iterate             ::{ Iterate8Suite            as Iterate8,
                                                                          Iterate16Suite           as Iterate16,
                                                                          IterateFloatSuite        as IterateFloat };
    pub(crate) mod pixel_data;            pub use pixel_data          ::PixelDataSuite             as PixelData;
    pub(crate) mod pixel_format;          pub use pixel_format        ::PixelFormatSuite           as PixelFormat;
    pub(crate) mod source_settings;       pub use source_settings     ::SourceSettingsSuite        as SourceSettings;
    pub(crate) mod transition;            pub use transition          ::TransitionSuite            as Transition;
    pub(crate) mod utility;               pub use utility             ::UtilitySuite               as Utility;
    pub(crate) mod world;                 pub use world               ::WorldSuite                 as World;
    pub(crate) mod world_transform;       pub use world_transform     ::WorldTransformSuite        as WorldTransform;
    pub(crate) mod handle;                pub use handle              ::HandleSuite                as Handle;
    pub(crate) mod helper;                pub use helper              ::{ HelperSuite              as Helper,
                                                                          HelperSuite2             as Helper2 };
    pub(crate) mod param_utils;           pub use param_utils         ::{ ParamUtilsSuite          as ParamUtils,
                                                                          AngleParamSuite          as AngleParam,
                                                                          ColorParamSuite          as ColorParam,
                                                                          PointParamSuite          as PointParam };
    pub(crate) mod gpu_device;            pub use gpu_device          ::GPUDeviceSuite             as GPUDevice;
    pub(crate) mod fill_matte;            pub use fill_matte          ::FillMatteSuite             as FillMatte;
}

pub use suites::adv_item::Step;
pub use suites::app::{
    AppColorType,
    AppPersonalTextInfo,
    AppProgressDialog,
    CursorType,
    EyeDropperSampleMode,
    FontStyleSheet,
};
pub use suites::custom_ui::{
    ContextHandle,
    CustomUIInfo,
};
pub use suites::channel::{
    DataType,
    ChannelType,
};
pub use suites::helper::{
    SuiteTool,
    ExtendedSuiteTool
};
pub use suites::param_utils::{
    PARAM_INDEX_NONE,
    PARAM_INDEX_CHECK_ALL,
    PARAM_INDEX_CHECK_ALL_EXCEPT_LAYER_PARAMS,
    PARAM_INDEX_CHECK_ALL_HONOR_EXCLUDE,
    TimeDir,
};
pub use suites::pixel_format::PixelFormat;

define_enum! {
    ae_sys::PF_XferMode,
    TransferMode {
        None                 = ae_sys::PF_Xfer_NONE,
        Copy                 = ae_sys::PF_Xfer_COPY,
        Behind               = ae_sys::PF_Xfer_BEHIND,
        InFront              = ae_sys::PF_Xfer_IN_FRONT,
        Dissolve             = ae_sys::PF_Xfer_DISSOLVE,
        Add                  = ae_sys::PF_Xfer_ADD,
        Mulitply             = ae_sys::PF_Xfer_MULTIPLY,
        Screen               = ae_sys::PF_Xfer_SCREEN,
        Overlay              = ae_sys::PF_Xfer_OVERLAY,
        SoftLight            = ae_sys::PF_Xfer_SOFT_LIGHT,
        HardLight            = ae_sys::PF_Xfer_HARD_LIGHT,
        Darken               = ae_sys::PF_Xfer_DARKEN,
        Lighten              = ae_sys::PF_Xfer_LIGHTEN,
        Difference           = ae_sys::PF_Xfer_DIFFERENCE,
        Hue                  = ae_sys::PF_Xfer_HUE,
        Saturation           = ae_sys::PF_Xfer_SATURATION,
        Color                = ae_sys::PF_Xfer_COLOR,
        Luminosity           = ae_sys::PF_Xfer_LUMINOSITY,
        MultiplyAlpha        = ae_sys::PF_Xfer_MULTIPLY_ALPHA,
        MultiplyAlphaLuma    = ae_sys::PF_Xfer_MULTIPLY_ALPHA_LUMA,
        MultiplyNotAlpha     = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA,
        MultiplyNotAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA_LUMA,
        AddiditivePremul     = ae_sys::PF_Xfer_ADDITIVE_PREMUL,
        AlphaAdd             = ae_sys::PF_Xfer_ALPHA_ADD,
        ColorDodge           = ae_sys::PF_Xfer_COLOR_DODGE,
        ColorBurn            = ae_sys::PF_Xfer_COLOR_BURN,
        Exclusion            = ae_sys::PF_Xfer_EXCLUSION,
        Difference2          = ae_sys::PF_Xfer_DIFFERENCE2,
        ColorDodge2          = ae_sys::PF_Xfer_COLOR_DODGE2,
        ColorBurn2           = ae_sys::PF_Xfer_COLOR_BURN2,
        LinearDodge          = ae_sys::PF_Xfer_LINEAR_DODGE,
        LinearBurn           = ae_sys::PF_Xfer_LINEAR_BURN,
        LinearLight          = ae_sys::PF_Xfer_LINEAR_LIGHT,
        VividLight           = ae_sys::PF_Xfer_VIVID_LIGHT,
        PinLight             = ae_sys::PF_Xfer_PIN_LIGHT,
        HardMix              = ae_sys::PF_Xfer_HARD_MIX,
        LighterColor         = ae_sys::PF_Xfer_LIGHTER_COLOR,
        DarkerColor          = ae_sys::PF_Xfer_DARKER_COLOR,
        Subtract             = ae_sys::PF_Xfer_SUBTRACT,
        Divide               = ae_sys::PF_Xfer_DIVIDE,
        Reserved0            = ae_sys::PF_Xfer_RESERVED0,
        Reserved1            = ae_sys::PF_Xfer_RESERVED1,
        NumModes             = ae_sys::PF_Xfer_NUM_MODES,
    }
}

pub type XferMode = TransferMode;

#[derive(Debug, Copy, Clone, Hash)]
pub struct CompositeMode {
    pub xfer: TransferMode,
    /// For TransferMode::DissolveRandomized.
    pub rand_seed: i32,
    /// 0-255.
    pub opacity: u8,
    /// Ignored TransferMode::MutiplyAlpha* modes.
    pub rgb_only: bool,
    /// For deep color only.
    pub opacity_su: u16,
}

impl From<ae_sys::PF_CompositeMode> for CompositeMode {
    fn from(mode: ae_sys::PF_CompositeMode) -> Self {
        Self {
            xfer: mode.xfer.into(),
            rand_seed: mode.rand_seed,
            opacity: mode.opacity,
            rgb_only: mode.rgb_only != 0,
            opacity_su: mode.opacitySu,
        }
    }
}
impl From<CompositeMode> for ae_sys::PF_CompositeMode {
    fn from(mode: CompositeMode) -> Self {
        Self {
            xfer: mode.xfer.into(),
            rand_seed: mode.rand_seed,
            opacity: mode.opacity,
            rgb_only: mode.rgb_only as _,
            opacitySu: mode.opacity_su,
        }
    }
}

define_struct! {
    ae_sys::PF_Point,
    #[derive(Eq)]
    Point {
        h: i32,
        v: i32,
    }
}
impl Point {
    pub fn empty() -> Self {
        Self { h: 0, v: 0 }
    }
}

define_struct! {
    ae_sys::PF_RationalScale,
    #[derive(Eq)]
    RationalScale {
        num: i32,
        den: u32,
    }
}
impl RationalScale {
    pub fn inv(&self) -> RationalScale {
        RationalScale { num: self.den as _, den: self.num as _ }
    }
}

impl From<RationalScale> for f64 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(
            ratio.den != 0,
            "Denominator is zero. This would lead to a division by zero."
        );
        ratio.num as Self / ratio.den as Self
    }
}

impl From<RationalScale> for f32 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(
            ratio.den != 0,
            "Denominator is zero. This would lead to a division by zero."
        );
        ratio.num as Self / ratio.den as Self
    }
}
define_enum! {
    ae_sys::PF_MaskFlags,
    MaskFlags {
        None      = ae_sys::PF_MaskFlag_NONE,
        Inverted  = ae_sys::PF_MaskFlag_INVERTED,
        Luminance = ae_sys::PF_MaskFlag_LUMINANCE,
    }
}

#[derive(Debug)]
pub struct MaskWorld {
    pub mask: ae_sys::PF_EffectWorld,
    pub offset: Point,
    pub what_is_mask: MaskFlags,
}
impl From<ae_sys::PF_MaskWorld> for MaskWorld {
    fn from(mask: ae_sys::PF_MaskWorld) -> Self {
        Self {
            mask: mask.mask,
            offset: Point {
                v: mask.offset.v,
                h: mask.offset.h,
            },
            what_is_mask: mask.what_is_mask.into(),
        }
    }
}
impl From<MaskWorld> for ae_sys::PF_MaskWorld {
    fn from(mask: MaskWorld) -> Self {
        Self {
            mask: mask.mask,
            offset: ae_sys::PF_Point {
                v: mask.offset.v,
                h: mask.offset.h,
            },
            what_is_mask: mask.what_is_mask.into(),
        }
    }
}

define_enum! {
    ae_sys::PF_Quality,
    Quality {
        DrawingAudio = ae_sys::PF_Quality_DRAWING_AUDIO,
        Lo           = ae_sys::PF_Quality_LO,
        Hi           = ae_sys::PF_Quality_HI,
    }
}

define_enum! {
    ae_sys::PF_ModeFlags,
    ModeFlags {
        AlphaPremul   = ae_sys::PF_MF_Alpha_PREMUL,
        AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
    }
}

define_enum! {
    ae_sys::PF_Field,
    Field {
        Frame = ae_sys::PF_Field_FRAME,
        Upper = ae_sys::PF_Field_UPPER,
        Lower = ae_sys::PF_Field_LOWER,
    }
}
define_enum! {
    ae_sys::PF_TimeDisplay,
    TimeDisplay {
        Fps24        = ae_sys::PF_TimeDisplay_24,
        Fps25        = ae_sys::PF_TimeDisplay_25,
        Fps30Drop    = ae_sys::PF_TimeDisplay_30Drop,
        Fps30NonDrop = ae_sys::PF_TimeDisplay_30NonDrop,
        Fps50        = ae_sys::PF_TimeDisplay_50,
        Fps60Drop    = ae_sys::PF_TimeDisplay_60Drop,
        Fps60NonDrop = ae_sys::PF_TimeDisplay_60NonDrop,
        NonStandard  = ae_sys::PF_TimeDisplay_NonStandard,
        Invalid      = ae_sys::PF_TimeDisplay_Invalid,
    }
}

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables);

define_enum! {
    ae_sys::PF_ParamIndex,
    ParamIndex {
        None                      = ae_sys::PF_ParamIndex_NONE,
        CheckAll                  = ae_sys::PF_ParamIndex_CHECK_ALL,
        CheckAllExceptLayerParams = ae_sys::PF_ParamIndex_CHECK_ALL_EXCEPT_LAYER_PARAMS,
        CheckAllHonorExclude      = ae_sys::PF_ParamIndex_CHECK_ALL_HONOR_EXCLUDE,
    }
}

pub type ProgPtr = ae_sys::PF_ProgPtr;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct CustomEventFlags: ae_sys::A_long {
        const NONE    = ae_sys::PF_CustomEFlag_NONE    as ae_sys::A_long;
        const COMP    = ae_sys::PF_CustomEFlag_COMP    as ae_sys::A_long;
        const LAYER   = ae_sys::PF_CustomEFlag_LAYER   as ae_sys::A_long;
        const EFFECT  = ae_sys::PF_CustomEFlag_EFFECT  as ae_sys::A_long;
        const PREVIEW = ae_sys::PF_CustomEFlag_PREVIEW as ae_sys::A_long;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    struct _UIAlignment: ae_sys::A_long {
        // No values other than PF_UIAlignment_NONE are honored, in Ae or PPro.
        const NONE   = ae_sys::PF_UIAlignment_NONE   as ae_sys::A_long;
        const TOP    = ae_sys::PF_UIAlignment_TOP    as ae_sys::A_long;
        const LEFT   = ae_sys::PF_UIAlignment_LEFT   as ae_sys::A_long;
        const BOTTOM = ae_sys::PF_UIAlignment_BOTTOM as ae_sys::A_long;
        const RIGHT  = ae_sys::PF_UIAlignment_RIGHT  as ae_sys::A_long;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Modifiers: ae_sys::A_long {
        const NONE            = ae_sys::PF_Mod_NONE            as ae_sys::A_long;
        /// Cmd on macOS, Ctrl on Windows.
        const CMD_CTRL_KEY    = ae_sys::PF_Mod_CMD_CTRL_KEY    as ae_sys::A_long;
        const SHIFT_KEY       = ae_sys::PF_Mod_SHIFT_KEY       as ae_sys::A_long;
        const CAPS_LOCK_KEY   = ae_sys::PF_Mod_CAPS_LOCK_KEY   as ae_sys::A_long;
        /// Option on macOS, alt on Windows.
        const OPT_ALT_KEY     = ae_sys::PF_Mod_OPT_ALT_KEY     as ae_sys::A_long;
        /// Mac control key only
        const MAC_CONTROL_KEY = ae_sys::PF_Mod_MAC_CONTROL_KEY as ae_sys::A_long;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct WorldFlags: ae_sys::A_long {
        const DEEP        = ae_sys::PF_WorldFlag_DEEP as ae_sys::A_long;
        const WRITEABLE   = ae_sys::PF_WorldFlag_WRITEABLE   as ae_sys::A_long;
        const RESERVED0   = ae_sys::PF_WorldFlag_RESERVED0   as ae_sys::A_long;
        const RESERVED1   = ae_sys::PF_WorldFlag_RESERVED1   as ae_sys::A_long;
        const RESERVED2   = ae_sys::PF_WorldFlag_RESERVED2   as ae_sys::A_long;
        const RESERVED3   = ae_sys::PF_WorldFlag_RESERVED3   as ae_sys::A_long;
        const RESERVED4   = ae_sys::PF_WorldFlag_RESERVED4   as ae_sys::A_long;
        const RESERVED5   = ae_sys::PF_WorldFlag_RESERVED5   as ae_sys::A_long;
        const RESERVED6   = ae_sys::PF_WorldFlag_RESERVED6   as ae_sys::A_long;
        const RESERVED    = ae_sys::PF_WorldFlag_RESERVED    as ae_sys::A_long;
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct Fixed(ae_sys::PF_Fixed);
impl Fixed {
    pub const ONE: Self = Self(0x00010000);
    pub const HALF: Self = Self(0x00008000);

    pub fn to_int(self) -> i32 {
        self.0 as ae_sys::A_long >> 16
    }
    pub fn to_int_rounded(self) -> i32 {
        (self.0 as ae_sys::A_long + 32768) >> 16
    }
    pub fn from_int(value: i32) -> Self {
        Self(value << 16)
    }
    pub fn as_f32(&self) -> f32 {
        self.0 as f32 / 65536.0
    }

    pub fn as_fixed(&self) -> ae_sys::PF_Fixed {
        self.0
    }
    pub fn from_fixed(value: ae_sys::PF_Fixed) -> Self {
        Self(value)
    }
}
impl From<f32> for Fixed {
    fn from(value: f32) -> Self {
        Fixed((value * 65536.0 + (if value < 0.0 { -0.5 } else { 0.5 })) as _)
    }
}
impl From<Fixed> for f32 {
    fn from(val: Fixed) -> Self {
        val.0 as f32 / 65536.0
    }
}
impl From<Fixed> for f64 {
    fn from(val: Fixed) -> Self {
        val.0 as f64 / 65536.0
    }
}
