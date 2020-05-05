pub use crate::*;
use aftereffects_sys as ae_sys;
use std::{convert::TryInto, ffi::{CString, CStr}};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel8 {
    pub alpha: ae_sys::A_u_char,
    pub red: ae_sys::A_u_char,
    pub green: ae_sys::A_u_char,
    pub blue: ae_sys::A_u_char,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel16 {
    pub alpha: ae_sys::A_u_short,
    pub red: ae_sys::A_u_short,
    pub green: ae_sys::A_u_short,
    pub blue: ae_sys::A_u_short,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel32 {
    pub alpha: ae_sys::PF_FpShort,
    pub red: ae_sys::PF_FpShort,
    pub green: ae_sys::PF_FpShort,
    pub blue: ae_sys::PF_FpShort,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(i32)]
pub enum TransferMode {
    None = ae_sys::PF_Xfer_NONE,
    Copy = ae_sys::PF_Xfer_COPY,
    Behind = ae_sys::PF_Xfer_BEHIND,
    InFront = ae_sys::PF_Xfer_IN_FRONT,
    Dissolve = ae_sys::PF_Xfer_DISSOLVE,
    Add = ae_sys::PF_Xfer_ADD,
    Mulitply = ae_sys::PF_Xfer_MULTIPLY,
    Screen = ae_sys::PF_Xfer_SCREEN,
    Overlay = ae_sys::PF_Xfer_OVERLAY,
    SoftLight = ae_sys::PF_Xfer_SOFT_LIGHT,
    HardLight = ae_sys::PF_Xfer_HARD_LIGHT,
    Darken = ae_sys::PF_Xfer_DARKEN,
    Lighten = ae_sys::PF_Xfer_LIGHTEN,
    Difference = ae_sys::PF_Xfer_DIFFERENCE,
    Hue = ae_sys::PF_Xfer_HUE,
    Saturation = ae_sys::PF_Xfer_SATURATION,
    Color = ae_sys::PF_Xfer_COLOR,
    Luminosity = ae_sys::PF_Xfer_LUMINOSITY,
    MultiplyAlpha = ae_sys::PF_Xfer_MULTIPLY_ALPHA,
    MultiplyAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_ALPHA_LUMA,
    MultiplyNotAlpha = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA,
    MultiplyNotAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA_LUMA,
    AddiditivePremul = ae_sys::PF_Xfer_ADDITIVE_PREMUL,
    AlphaAdd = ae_sys::PF_Xfer_ALPHA_ADD,
    ColorDodge = ae_sys::PF_Xfer_COLOR_DODGE,
    ColorBurn = ae_sys::PF_Xfer_COLOR_BURN,
    Exclusion = ae_sys::PF_Xfer_EXCLUSION,

    Difference2 = ae_sys::PF_Xfer_DIFFERENCE2,
    ColorDodge2 = ae_sys::PF_Xfer_COLOR_DODGE2,
    ColorBurn2 = PF_Xfer_COLOR_BURN2,

    LinearDodge = ae_sys::PF_Xfer_LINEAR_DODGE,
    LinearBurn = ae_sys::PF_Xfer_LINEAR_BURN,
    LinearLight = ae_sys::PF_Xfer_LINEAR_LIGHT,
    VividLight = ae_sys::PF_Xfer_VIVID_LIGHT,
    PinLight = ae_sys::PF_Xfer_PIN_LIGHT,

    HardMix = ae_sys::PF_Xfer_HARD_MIX,

    LighterColor = ae_sys::PF_Xfer_LIGHTER_COLOR,
    DarkerColor = ae_sys::PF_Xfer_DARKER_COLOR,

    Subtract = ae_sys::PF_Xfer_SUBTRACT,
    Divide = ae_sys::PF_Xfer_DIVIDE,

    Reserved0 = ae_sys::PF_Xfer_RESERVED0,
    Reserved1 = ae_sys::PF_Xfer_RESERVED1,

    NumModes = ae_sys::PF_Xfer_NUM_MODES,
}

pub type XferMode = TransferMode;

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct CompositeMode {
    pub xfer: TransferMode,
    // For TransferMode::DissolveRandomized.
    pub rand_seed: i32,
    // 0–255.
    pub opacity: u8,
    // Ignored TransferMode::MutiplyAlpha* modes.
    pub rgb_only: u8,
    // For deep color only.
    pub opacity_su: u16,
}

pub type ProgPtr = ae_sys::PF_ProgPtr;

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Point {
    pub h: i32,
    pub v: i32,
}

pub type MaskFlags = u32;

#[derive(Debug)]
#[repr(C)]
pub struct MaskWorld {
    pub mask: EffectWorld,
    pub offset: Point,
    pub what_is_mask: MaskFlags,
}

#[derive(Copy, Clone, Debug, Hash)]
#[repr(i32)]
pub enum Quality {
    DrawingAudio = ae_sys::PF_Quality_DRAWING_AUDIO,
    Lo = ae_sys::PF_Quality_LO,
    Hi = ae_sys::PF_Quality_HI,
}

#[repr(u32)]
pub enum ModeFlags {
    AlphaPremul = ae_sys::PF_MF_Alpha_PREMUL,
    AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
}

#[repr(u32)]
pub enum Field {
    Frame = ae_sys::PF_Field_FRAME,
    Upper = ae_sys::PF_Field_UPPER,
    Lower = ae_sys::PF_Field_LOWER,
}

// FIXME: wrap this nicely
#[derive(Debug, Copy, Clone)]
pub struct EffectWorld {
    pub effect_world: ae_sys::PF_EffectWorld,
}

pub struct EffectWorldConst {
    pub effect_world: ae_sys::PF_EffectWorld,
}

unsafe impl Send for EffectWorldConst {}
unsafe impl Sync for EffectWorldConst {}

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables, blending_tabpe_ptr);

impl EffectWorld {
    #[inline]
    pub fn new(world_handle: WorldHandle) -> Result<Self, crate::Error> {
        WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
    }

    pub fn from_raw(effect_world_ptr: *const ae_sys::PF_EffectWorld) -> Result<Self, crate::Error> {
        if std::ptr::null() == effect_world_ptr {
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
                    WorldType::Integer => 2,
                    WorldType::Byte => 1,
                    WorldType::Float => 4,
                    WorldType::None => panic!(),
                }
    }

    #[inline]
    pub fn get_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel8).add(x) }
    }

    #[inline]
    pub fn get_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        self.get_pixel8_mut(x, y)
    }

    #[inline]
    pub fn get_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel16).add(x) }
    }

    #[inline]
    pub fn get_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        self.get_pixel16_mut(x, y)
    }

    #[inline]
    pub fn get_pixel32_mut(&self, x: usize, y: usize) -> &mut Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel32).add(x) }
    }

    #[inline]
    pub fn get_pixel32(&self, x: usize, y: usize) -> &Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &*((self.effect_world.data as *const u8).add(y * self.row_bytes()) as *const Pixel32).add(x) }
    }

    #[inline]
    pub fn world_type(&self) -> WorldType {
        let flags = self.effect_world.world_flags;
        // Most frequent case is 16bit integer.
        if ae_sys::PF_WorldFlag_DEEP & flags as u32 != 0 {
            WorldType::Integer
        } else if ae_sys::PF_WorldFlag_RESERVED1 & flags as u32 != 0 {
            WorldType::Float
        } else {
            WorldType::Byte
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Err {
    None = ae_sys::PF_Err_NONE as isize,
    OutOfMemory = ae_sys::PF_Err_OUT_OF_MEMORY as isize,
    InternalStructDamaged = ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED as isize,
    // Out of range, or action not allowed on this index.
    InvalidIndex = ae_sys::PF_Err_INVALID_INDEX as isize,
    UnrecogizedParamType = ae_sys::PF_Err_UNRECOGNIZED_PARAM_TYPE as isize,
    InvalidCallback = ae_sys::PF_Err_INVALID_CALLBACK as isize,
    BadCallbackParam = ae_sys::PF_Err_BAD_CALLBACK_PARAM as isize,
    // Returned when user interrupts rendering.
    InterruptCancel = ae_sys::PF_Interrupt_CANCEL as isize,
    // Returned from PF_Arbitrary_SCAN_FUNC when effect cannot parse
    // arbitrary data from text
    CannonParseKeyframeText = ae_sys::PF_Err_CANNOT_PARSE_KEYFRAME_TEXT as isize,
}

#[macro_export]
macro_rules! add_param {
    (in_data: expr,
    index: expr,
    def: expr) => {
        in_data.inter.add_param.unwrap()(in_data.effect_ref, (index), &(def))
    };
}

#[repr(i32)]
pub enum ParamType {
    Reserved = ae_sys::PF_Param_RESERVED,
    Layer = ae_sys::PF_Param_LAYER,
    Slider = ae_sys::PF_Param_SLIDER,
    FixSlider = ae_sys::PF_Param_FIX_SLIDER,
    Angle = ae_sys::PF_Param_ANGLE,
    CheckBox = ae_sys::PF_Param_CHECKBOX,
    Color = ae_sys::PF_Param_COLOR,
    Point = ae_sys::PF_Param_POINT,
    PopUp = ae_sys::PF_Param_POPUP,
    Custom = ae_sys::PF_Param_CUSTOM,
    NoData = ae_sys::PF_Param_NO_DATA,
    FloatSlider = ae_sys::PF_Param_FLOAT_SLIDER,
    ArbitraryData = ae_sys::PF_Param_ARBITRARY_DATA,
    Path = ae_sys::PF_Param_PATH,
    GroupStart = ae_sys::PF_Param_GROUP_START,
    GroupEnd = ae_sys::PF_Param_GROUP_END,
    Button = ae_sys::PF_Param_BUTTON,
    Reserved2 = ae_sys::PF_Param_RESERVED2,
    Reserved3 = ae_sys::PF_Param_RESERVED3,
    Point3D = ae_sys::PF_Param_POINT_3D,
}

pub const PARAM_FLAG_RESERVED1: u32 = ae_sys::PF_ParamFlag_RESERVED1;
pub const PARAM_FLAG_CANNOT_TIME_VARY: u32 = ae_sys::PF_ParamFlag_CANNOT_TIME_VARY;
pub const PARAM_FLAG_CANNOT_INTERP: u32 = ae_sys::PF_ParamFlag_CANNOT_INTERP;
pub const PARAM_FLAG_RESERVED2: u32 = ae_sys::PF_ParamFlag_RESERVED2;
pub const PARAM_FLAG_RESERVED3: u32 = ae_sys::PF_ParamFlag_RESERVED3;
pub const PARAM_FLAG_TWIRLY: u32 = ae_sys::PF_ParamFlag_COLLAPSE_TWIRLY;
pub const PARAM_FLAG_SUPERVIDE: u32 = ae_sys::PF_ParamFlag_SUPERVISE;
pub const PARAM_FLAG_START_COLLAPSED: u32 = ae_sys::PF_ParamFlag_START_COLLAPSED;
pub const PARAM_FLAG_USE_VALUE_FOR_OLD_PROJECTS: u32 = ae_sys::PF_ParamFlag_USE_VALUE_FOR_OLD_PROJECTS;
pub const PARAM_FLAG_LAYER_PARAM_IS_TRACKMATTE: u32 = ae_sys::PF_ParamFlag_LAYER_PARAM_IS_TRACKMATTE;
pub const PARAM_FLAG_EXCLUDE_FROM_HAVE_INPUTS_CHANGED: u32 = ae_sys::PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED;
pub const PARAM_FLAG_SKIP_REVEAL_WHEN_UNHIDDEN: u32 = ae_sys::PF_ParamFlag_SKIP_REVEAL_WHEN_UNHIDDEN;

#[repr(C)]
#[derive(Clone)]
pub struct PopupDef {
    popup_def: ae_sys::PF_PopupDef,
    names: CString,
}

impl PopupDef {
    pub fn new() -> Self {
        Self {
            popup_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            names: CString::new("").unwrap()
        }
    }

    pub fn value<'a>(&'a mut self, value: u16) -> &'a mut PopupDef {
        self.popup_def.value = value as i32;
        self
    }

    pub fn default<'a>(&'a mut self, default: u16) -> &'a mut PopupDef {
        self.popup_def.dephault = default as i16;
        self
    }

    pub fn num_choices<'a>(&'a mut self, num_choices: u16) -> &'a mut PopupDef {
        self.popup_def.num_choices = num_choices as i16;
        self
    }

    pub fn names<'a>(&'a mut self, names: Vec<&str>) -> &'a mut PopupDef {
        let mut names_tmp = String::new();
        names.iter().map(|s| names_tmp += format!("{}|", *s).as_str());
        self.names = CString::new(names_tmp).unwrap();
        self.popup_def.u.namesptr = self.names.as_c_str().as_ptr();
        self.popup_def.num_choices = names.len().try_into().unwrap();
        //(names.to_string_lossy().matches("|").count() + 1).try_into().unwrap();
        self
    }

    pub fn into_raw(pd: PopupDef) -> PF_PopupDef {
        pd.popup_def
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AngleDef {
    angle_def: PF_AngleDef,
}

impl AngleDef {
    pub fn new() -> Self {
        Self {
            angle_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        }
    }

    pub fn value<'a>(&'a mut self, value: i32) -> &'a mut AngleDef {
        self.angle_def.value = value;
        self
    }

    pub fn default<'a>(&'a mut self, default: i32) -> &'a mut AngleDef {
        self.angle_def.dephault = default;
        self
    }

    pub fn valid_min<'a>(&'a mut self, valid_min: i32) -> &'a mut AngleDef {
        self.angle_def.valid_min = valid_min;
        self
    }

    pub fn valid_max<'a>(&'a mut self, valid_max: i32) -> &'a mut AngleDef {
        self.angle_def.valid_max = valid_max;
        self
    }

    pub fn into_raw(ad: AngleDef) -> PF_AngleDef {
        ad.angle_def
    }
}

#[repr(C)]
#[derive(Clone)]
pub enum ParamDefUnion {
    PopupDef(PopupDef),
    AngleDef(AngleDef),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParamDef {
    param_def: ae_sys::PF_ParamDef,
}

impl ParamDef {
    pub fn new() -> Self {
        Self {
            param_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        }
    }

    pub fn param<'a>(&'a mut self, param: ParamDefUnion) -> &'a mut ParamDef {
        match param {
            ParamDefUnion::PopupDef(pd) => {
                self.param_def.u.pd = PopupDef::into_raw(pd);
                self.param_def.param_type = ae_sys::PF_Param_POPUP;
            }
            ParamDefUnion::AngleDef(ad) => {
                self.param_def.u.ad = AngleDef::into_raw(ad);
                self.param_def.param_type = ae_sys::PF_Param_ANGLE;
            }
        }
        self
    }

    /*
    ae_sys::PF_ParamDef {
            uu: PF_ParamDef__bindgen_ty_1 { id: 0 },
            ui_flags: 0,
            ui_width: 0,
            ui_height: 0,
            /* PARAMETER DESCRIPTION */
            param_type: 0,
            name: [0i8; 32],
            flags: 0,
            unused: 0,
            u: ae_sys::PF_ParamDefUnion {
                _bindgen_union_align: [0u64; 15usize],
            },
        },
    }*/

    pub fn param_type<'a>(&'a mut self, param_type: ParamType) -> &'a mut ParamDef {
        self.param_def.param_type = param_type as i32;
        self
    }

    pub fn name<'a>(&'a mut self, name: &CStr) -> &'a mut ParamDef {
        let name_slice = name.to_bytes_with_nul();
        self.param_def.name[0..name_slice.len()].copy_from_slice(unsafe { std::mem::transmute(name_slice) });
        self
    }

    pub fn ui_flags<'a>(&'a mut self, flags: i32) -> &'a mut ParamDef {
        self.param_def.ui_flags = flags;
        self
    }

    pub fn ui_width<'a>(&'a mut self, width: u16) -> &'a mut ParamDef {
        self.param_def.ui_width = width as i16;
        self
    }

    pub fn ui_height<'a>(&'a mut self, height: u16) -> &'a mut ParamDef {
        self.param_def.ui_height = height as i16;
        self
    }

    pub fn flags<'a>(&'a mut self, flags: i32) -> &'a mut ParamDef {
        self.param_def.flags = flags;
        self
    }

    pub fn add(&mut self, in_data: &ae_sys::PF_InData) {
        unsafe {
            in_data.inter.add_param.unwrap()(in_data.effect_ref, -1, &mut self.param_def);
        }
    }
}
