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
mod suites;     pub use suites::*;

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
    /// 0–255.
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

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct RationalScale {
    pub num: ae_sys::A_long,
    pub den: ae_sys::A_u_long,
}

impl From<RationalScale> for ae_sys::PF_RationalScale {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<ae_sys::PF_RationalScale> for RationalScale {
    #[inline]
    fn from(ratio: ae_sys::PF_RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
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
    pub fn new(world_handle: aegp::WorldHandle) -> Result<Self, crate::Error> {
        aegp::WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
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
    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut Pixel32 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel32).add(x) }
    }

    #[inline]
    pub fn as_pixel32(&self, x: usize, y: usize) -> &Pixel32 {
        self.as_pixel32_mut(x, y)
    }

    #[inline]
    pub fn world_type(&self) -> aegp::WorldType {
        let flags = self.effect_world.world_flags as ae_sys::PF_WorldFlags;
        // Most frequent case is 16bit integer.
        if ae_sys::PF_WorldFlag_DEEP & flags != 0 {
            aegp::WorldType::U15
        } else if ae_sys::PF_WorldFlag_RESERVED1 & flags != 0 {
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

define_handle_wrapper!(ProgressInfo, PF_ProgPtr);

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

/*
pub enum GameState {
    FreePlacement(FreePlacement),
    Play(PlayState),
    Scoring(ScoringState),
    Done(ScoringState),
}

assume!(GameState);
assume!(GameState, Play(x) => x, PlayState);
assume!(GameState, Scoring(x) => x, ScoringState);
assume!(GameState, FreePlacement(x) => x, FreePlacement);

// and then I can:

state.assume::<PlayState>()
*/

pub trait AssumeFrom<T> {
    fn assume(x: &T) -> &Self;
    fn assume_mut(x: &mut T) -> &mut Self;
}
