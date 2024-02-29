use crate::*;
use bitflags::bitflags;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::{Debug, Write},
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
mod interact_callbacks; pub use interact_callbacks::*;
mod util_callbacks;     pub use util_callbacks::*;

pub mod suites {
    pub(crate) mod effect_sequence_data;  pub use effect_sequence_data::EffectSequenceDataSuite  as EffectSequenceData;
    pub(crate) mod custom_ui;             pub use custom_ui           ::{ EffectCustomUISuite as EffectCustomUI,
                                                                          EffectCustomUIOverlayThemeSuite as EffectCustomUIOverlayTheme };
    pub(crate) mod iterate;               pub use iterate             ::{ Iterate8Suite     as Iterate8,
                                                                          Iterate16Suite    as Iterate16,
                                                                          IterateFloatSuite as IterateFloat };
    pub(crate) mod pixel_format;          pub use pixel_format        ::PixelFormatSuite    as PixelFormat;
    pub(crate) mod utility;               pub use utility             ::UtilitySuite        as Utility;
    pub(crate) mod world;                 pub use world               ::WorldSuite          as World;
    pub(crate) mod handle;                pub use handle              ::HandleSuite         as Handle;
    pub(crate) mod param_utils;           pub use param_utils         ::ParamUtilsSuite     as ParamUtils;
    pub(crate) mod gpu_device;            pub use gpu_device          ::GPUDeviceSuite      as GPUDevice;
}

pub use suites::custom_ui::{
    ContextHandle,
    CustomUIInfo
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
#[repr(C)]
pub struct CompositeMode {
    pub xfer: TransferMode,
    /// For TransferMode::DissolveRandomized.
    pub rand_seed: i32,
    /// 0-255.
    pub opacity: u8,
    /// Ignored TransferMode::MutiplyAlpha* modes.
    pub rgb_only: u8,
    /// For deep color only.
    pub opacity_su: u16,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Point {
    pub h: i32,
    pub v: i32,
}
impl Into<ae_sys::PF_Point> for Point {
    fn into(self) -> ae_sys::PF_Point {
        ae_sys::PF_Point {
            h: self.h,
            v: self.v,
        }
    }
}
impl From<ae_sys::PF_Point> for Point {
    fn from(point: ae_sys::PF_Point) -> Self {
        Self {
            h: point.h,
            v: point.v,
        }
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

pub type MaskFlags = u32;

#[derive(Debug)]
#[repr(C)]
pub struct MaskWorld {
    pub mask: EffectWorld,
    pub offset: Point,
    pub what_is_mask: MaskFlags,
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
// FIXME: wrap this nicely
/// An EffectWorld is a view on a WorldHandle that can be used to write to.
#[derive(Debug, Copy, Clone)]
pub struct EffectWorld {
    pub effect_world: ae_sys::PF_EffectWorld,
}

pub struct EffectWorldConst {
    pub effect_world: ae_sys::PF_EffectWorld,
}

unsafe impl Send for EffectWorldConst {}
unsafe impl Sync for EffectWorldConst {}

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables);

// FIXME: this is not safe.
// We just use EffectWorld responsibly but another user of this care may not.
unsafe impl Send for EffectWorld {}
unsafe impl Sync for EffectWorld {}

impl EffectWorld {
    #[inline]
    pub fn new(world_handle: &aegp::WorldHandle) -> Result<Self, crate::Error> {
        aegp::suites::World::new()?.effect_world(world_handle)
    }

    pub fn from_raw(effect_world_ptr: *const ae_sys::PF_EffectWorld) -> Result<Self, crate::Error> {
        if effect_world_ptr.is_null() {
            Err(crate::Error::Generic)
        } else {
            Ok(EffectWorld {
                effect_world: unsafe { *effect_world_ptr },
            })
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.effect_world.width as usize
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.effect_world.height as usize
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        self.effect_world.rowbytes as usize
    }

    #[inline]
    pub fn data_as_ptr(&self) -> *const u8 {
        self.effect_world.data as *const u8
    }

    #[inline]
    pub fn data_as_ptr_mut(&self) -> *mut u8 {
        self.effect_world.data as *mut u8
    }

    #[inline]
    pub fn data_len(&self) -> usize {
        self.height() * self.row_bytes()
    }

    pub fn row_padding_bytes(&self) -> usize {
        self.row_bytes()
            - self.width()
                * 4
                * match self.world_type() {
                    aegp::WorldType::U15 => 2,
                    aegp::WorldType::U8 => 1,
                    aegp::WorldType::F32 => 4,
                    aegp::WorldType::None => panic!(),
                }
    }

    #[inline]
    pub fn as_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel8).add(x) }
    }

    #[inline]
    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        self.as_pixel8_mut(x, y)
    }

    #[inline]
    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel16).add(x) }
    }

    #[inline]
    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        self.as_pixel16_mut(x, y)
    }

    #[inline]
    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut PixelF32 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut PixelF32).add(x) }
    }

    #[inline]
    pub fn as_pixel32(&self, x: usize, y: usize) -> &PixelF32 {
        self.as_pixel32_mut(x, y)
    }

    #[inline]
    pub fn world_type(&self) -> aegp::WorldType {
        let flags = WorldFlags::from_bits(self.effect_world.world_flags as _).unwrap();
        // Most frequent case is 16bit integer.
        if flags.contains(WorldFlags::DEEP) {
            aegp::WorldType::U15
        } else if flags.contains(WorldFlags::RESERVED1) {
            aegp::WorldType::F32
        } else {
            aegp::WorldType::U8
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        &self.effect_world as *const ae_sys::PF_EffectWorld
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ae_sys::PF_EffectWorld {
        &mut self.effect_world as *mut ae_sys::PF_EffectWorld
    }
}

pub fn progress(in_data: InData, count: u16, total: u16) -> i32 {
    unsafe {
        (*in_data.as_ptr()).inter.progress.unwrap()(
            (*in_data.as_ptr()).effect_ref,
            count as i32,
            total as i32,
        )
    }
}

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
        // Option on macOS, alt on Windows.
        const OPT_ALT_KEY     = ae_sys::PF_Mod_OPT_ALT_KEY     as ae_sys::A_long;
        // Mac control key only
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
        self.to_int() + 32768
    }
    pub fn from_int(value: i32) -> Self {
        Self(value << 16)
    }
}
impl Into<ae_sys::PF_Fixed> for Fixed {
    fn into(self) -> ae_sys::PF_Fixed {
        self.0
    }
}
impl From<ae_sys::PF_Fixed> for Fixed {
    fn from(value: ae_sys::PF_Fixed) -> Self {
        Fixed(value)
    }
}
impl From<f32> for Fixed {
    fn from(value: f32) -> Self {
        Fixed((value * 65536.0 + (if value < 0.0 { -0.5 } else { 0.5 })) as _)
    }
}
impl Into<f32> for Fixed {
    fn into(self) -> f32 {
        self.0 as f32 / 65536.0
    }
}
impl Into<f64> for Fixed {
    fn into(self) -> f64 {
        self.0 as f64 / 65536.0
    }
}
