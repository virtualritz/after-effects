use super::*;
use std::ffi::{ CStr, CString };
use ae_sys::PF_PathID;
use serde::{de::DeserializeOwned, Serialize};

const MAX_NAME_LEN: usize = 32;

define_enum! {
    ae_sys::PF_ParamType,
    ParamType {
        Reserved      = ae_sys::PF_Param_RESERVED,
        Layer         = ae_sys::PF_Param_LAYER,
        Slider        = ae_sys::PF_Param_SLIDER,
        FixSlider     = ae_sys::PF_Param_FIX_SLIDER,
        Angle         = ae_sys::PF_Param_ANGLE,
        CheckBox      = ae_sys::PF_Param_CHECKBOX,
        Color         = ae_sys::PF_Param_COLOR,
        Point         = ae_sys::PF_Param_POINT,
        PopUp         = ae_sys::PF_Param_POPUP,
        Custom        = ae_sys::PF_Param_CUSTOM,
        NoData        = ae_sys::PF_Param_NO_DATA,
        FloatSlider   = ae_sys::PF_Param_FLOAT_SLIDER,
        ArbitraryData = ae_sys::PF_Param_ARBITRARY_DATA,
        Path          = ae_sys::PF_Param_PATH,
        GroupStart    = ae_sys::PF_Param_GROUP_START,
        GroupEnd      = ae_sys::PF_Param_GROUP_END,
        Button        = ae_sys::PF_Param_BUTTON,
        Reserved2     = ae_sys::PF_Param_RESERVED2,
        Reserved3     = ae_sys::PF_Param_RESERVED3,
        Point3D       = ae_sys::PF_Param_POINT_3D,
    }
}

bitflags! {
    pub struct ParamUIFlags: ae_sys::A_long {
        const NONE = ae_sys::PF_PUI_NONE as ae_sys::A_long;
        /// Effect has custom UI and wants events for this params' title (portion visible when twirled up).
        const TOPIC = ae_sys::PF_PUI_TOPIC as ae_sys::A_long;
        /// Effect has custom UI and wants events for this params' control (portion invisible when twirled up).
        const CONTROL = ae_sys::PF_PUI_CONTROL as ae_sys::A_long;
        /// Param will be used as UI only, no data.
        const CONTROL_ONLY = ae_sys::PF_PUI_STD_CONTROL_ONLY as ae_sys::A_long;
        /// Stop param from appearing in Effect Controls (which in PPro also means you won't see a keyframe track there).
        const NO_ECW_UI = ae_sys::PF_PUI_NO_ECW_UI as ae_sys::A_long;
        /// Draw a thick separating line above this param; not used by Ae.
        const ECW_SEPARATOR = ae_sys::PF_PUI_ECW_SEPARATOR as ae_sys::A_long;
        /// Disable (gray-out) UI for this parameter.
        const DISABLED = ae_sys::PF_PUI_DISABLED as ae_sys::A_long;
        /// Ae will not erase the ECW topic, it's up to the FX to erase/draw every pixel.
        /// Handy if FX author implements an offscreen, prevents flashing.
        const DO_NOT_ERASE_TOPIC = ae_sys::PF_PUI_DONT_ERASE_TOPIC as ae_sys::A_long;
        const DO_NOT_ERASE_CONTROL = ae_sys::PF_PUI_DONT_ERASE_CONTROL as ae_sys::A_long;
        /// Display as a radio-button group; only valid for PF_Param_POPUP; ignored by Ae.
        const RADIO_BUTTON = ae_sys::PF_PUI_RADIO_BUTTON as ae_sys::A_long;
        /// In Ae as of CS6, this hides the parameter UI in both the Effect Controls and Timeline.
        /// in Premiere since earlier than that, this hides the parameter UI in the Effect Controls,
        /// which includes the keyframe track; for PPro only, the flag is dynamic and can be cleared to make the parameter visible again.
        const INVISIBLE = ae_sys::PF_PUI_INVISIBLE as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ParamFlag: ae_sys::A_long {
        /// If this is passed, the parameter will not be allowed to vary over time -- no keyframe controller will appear at the right.
        const CANNOT_TIME_VARY = ae_sys::PF_ParamFlag_CANNOT_TIME_VARY as ae_sys::A_long;
        /// If this is passed, parameter values are not interpolated between. You can still use no interp and discontinuous interp.
        const CANNOT_INTERP = ae_sys::PF_ParamFlag_CANNOT_INTERP as ae_sys::A_long;
        /// Set this flag if you want the parameter's twirly arrow in the Effect Control Window to be twirled up by default when the effect is first applied.
        /// New in AE 4.0: you can now set & clear this bit when handling `PF_Cmd_UPDATE_PARAMS_UI` and `PF_Cmd_USER_CHANGED_PARAM` messages, so as to twirl your parameters and groups up and down at will.
        ///
        /// Same as [`ParamFlag::START_COLLAPSED`]
        const TWIRLY = ae_sys::PF_ParamFlag_COLLAPSE_TWIRLY as ae_sys::A_long;
        /// If this is passed, PF_Cmd_USER_CHANGED_PARAM will be sent when this parameter changes.
        const SUPERVISE = ae_sys::PF_ParamFlag_SUPERVISE as ae_sys::A_long;
        /// Set this flag if you want the parameter's twirly arrow in the Effect Control Window to be twirled up by default when the effect is first applied.
        /// New in AE 4.0: you can now set & clear this bit when handling `PF_Cmd_UPDATE_PARAMS_UI` and `PF_Cmd_USER_CHANGED_PARAM` messages, so as to twirl your parameters and groups up and down at will.
        ///
        /// Same as [`ParamFlag::TWIRLY`]
        const START_COLLAPSED = ae_sys::PF_ParamFlag_START_COLLAPSED as ae_sys::A_long;
        /// This only affects the loading of projects saved with an older version of the effect which lacks parameters added later.
        /// When set, the PF_ParamDef "value" field set in PF_ADD_PARAM will be used to initialize the missing parameter,
        /// but the "dephault" field will still be used for initial value of the parameter when the effect is newly applied or reset.
        /// This is useful for when you want a parameter to default to one value but need it set to something else to preserve rendering behaviour for older projects.
        ///
        /// This flag is valid for all PF_Param types except PF_Param_LAYER
        const USE_VALUE_FOR_OLD_PROJECTS = ae_sys::PF_ParamFlag_USE_VALUE_FOR_OLD_PROJECTS as ae_sys::A_long;
        /// For PF_Param_LAYER, this flag indicates that the layer parameter is to be presented as a track matte. Supported by Premiere, ignored in AE.
        const LAYER_PARAM_IS_TRACKMATTE = ae_sys::PF_ParamFlag_LAYER_PARAM_IS_TRACKMATTE as ae_sys::A_long;
        /// See doc for [`pf::suites::ParamUtils::are_states_identical()`].
        const EXCLUDE_FROM_HAVE_INPUTS_CHANGED = ae_sys::PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED as ae_sys::A_long;
        /// When this param is "un hidden" (cuz it may hide and show), then the GUI is NOT to cause the parameter to be "revealed", ie: it won't twirl down it's parents and scroll it into view
        const SKIP_REVEAL_WHEN_UNHIDDEN = ae_sys::PF_ParamFlag_SKIP_REVEAL_WHEN_UNHIDDEN as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ChangeFlag: ae_sys::A_long {
        const NONE = ae_sys::PF_ChangeFlag_NONE as ae_sys::A_long;
        /// Set this flag for each param whose value you change when handling a `PF_Cmd_USER_CHANGED_PARAM` or specific `PF_Cmd_EVENT` events (`PF_Event_DO_CLICK`, `PF_Event_DRAG`, & `PF_Event_KEYDOWN`).
        /// If set during `PF_Cmd_EVENT`, but sure to also set `PF_EO_HANDLED_EVENT` before returning.
        ///
        /// You can change as many params as you want at once. These changes are undoable and re-doable by the user.
        ///
        /// Exception: do not set PF_PUI_STD_CONTROL_ONLY param values with this flag, use PF_UpdateParamUI() instead.
        const CHANGED_VALUE = ae_sys::PF_ChangeFlag_CHANGED_VALUE as ae_sys::A_long;
        /// Not yet implemented.  Same restrictions as PF_ChangeFlag_CHANGED_VALUE.
        const SET_TO_VARY = ae_sys::PF_ChangeFlag_SET_TO_VARY as ae_sys::A_long;
        /// Not yet implemented.  Same restrictions as PF_ChangeFlag_CHANGED_VALUE.
        const SET_TO_CONSTANT = ae_sys::PF_ChangeFlag_SET_TO_CONSTANT as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ValueDisplayFlag: u16 {
        const NONE = ae_sys::PF_ValueDisplayFlag_NONE as u16;
        /// Append % to value display for A_FpShort sliders (for fixed-point sliders, also maps range into 0-100%)
        const PERCENT = ae_sys::PF_ValueDisplayFlag_PERCENT as u16;
        /// Assume 0..1 is a pixel value, either 0..255,  0..32768, or 0..1.0 in UI (value will always be 0..1),
        const PIXEL = ae_sys::PF_ValueDisplayFlag_PIXEL as u16;
        /// Presentation negates values. eg: a true -5 would be presented as "5", and typing in "22" would store in the model as -22
        const REVERSE = ae_sys::PF_ValueDisplayFlag_REVERSE as u16;
    }
}

bitflags! {
    pub struct FSliderFlag: u16 {
        const NONE = ae_sys::PF_FSliderFlag_NONE as u16;
        /// Works for audio effects only
        const WANT_PHASE = ae_sys::PF_FSliderFlag_WANT_PHASE as u16;
    }
}

// ―――――――――――――――――――――――――――――――――――― Angle ―――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_ANGLE, PF_AngleDef, ad,
    Param::Angle,
    AngleDef { },
    impl value: Fixed,
}
impl AngleDef<'_> {
    pub fn set_default(&mut self, v: f32) -> &mut Self {
        self.def.dephault = Fixed::from(v).as_fixed();
        self
    }
    pub fn default(&self) -> f32 {
        Fixed::from_fixed(self.def.dephault).into()
    }
    pub fn float_value(&self) -> Result<f64, Error> {
        if self._in_data.is_null() || self._parent_ptr.is_none() {
            return Err(Error::InvalidParms);
        }
        Ok(pf::suites::AngleParam::new()?
            .floating_point_value_from_angle_def(unsafe { (*self._in_data).effect_ref }, self._parent_ptr.unwrap())?)
    }
}
// ―――――――――――――――――――――――――――――――――――― Angle ―――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Button ―――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_BUTTON, PF_ButtonDef, button_d,
    Param::Button,
    ButtonDef {
        label: CString,
    },
    impl label: String,
}
// ―――――――――――――――――――――――――――――――――――― Button ―――――――――――――――――――――――――――――――――――――

// ――――――――――――――――――――――――――――――――――― Checkbox ――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_CHECKBOX, PF_CheckBoxDef, bd,
    Param::CheckBox,
    CheckBoxDef {
        label: CString,
    },
    impl value: bool,
    fn init(param) {
        param.set_label(" ");
    }
}
impl<'a> CheckBoxDef<'_> {
    pub fn set_default(&mut self, v: bool) -> &mut Self {
        self.def.dephault = if v { 1 } else { 0 };
        self
    }
    pub fn default(&self) -> bool {
        self.def.dephault != 0
    }
    pub fn set_label(&mut self, v: &str) -> &mut Self {
        self.label = CString::new(v).unwrap();
        self.def.u.nameptr = self.label.as_ptr();
        self
    }
    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(self.def.u.nameptr).to_str().unwrap() }
    }
}
// ――――――――――――――――――――――――――――――――――― Checkbox ――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Color ――――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_COLOR, PF_ColorDef, cd,
    Param::Color,
    ColorDef { },
    impl value: Pixel8,
    impl default: Pixel8,
}
impl ColorDef<'_> {
    pub fn float_value(&self) -> Result<PixelF32, Error> {
        if self._in_data.is_null() || self._parent_ptr.is_none() {
            return Err(Error::InvalidParms);
        }
        Ok(pf::suites::ColorParam::new()?
            .floating_point_value_from_color_def(unsafe { (*self._in_data).effect_ref }, self._parent_ptr.unwrap())?)
    }
}
// ―――――――――――――――――――――――――――――――――――― Color ――――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Slider ―――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_SLIDER, PF_SliderDef, sd,
    Param::Slider,
    SliderDef { },
    impl value: i32,
    impl default: i32,
    impl valid_min: i32,
    impl valid_max: i32,
    impl slider_min: i32,
    impl slider_max: i32,
    impl value_str: ShortString,
    impl value_desc: ShortString,
}
// ―――――――――――――――――――――――――――――――――――― Slider ―――――――――――――――――――――――――――――――――――――

// Adobe recommends not using fixed (point) sliders any more and instead use float sliders.
// Do not define FixedSliderDef

// ―――――――――――――――――――――――――――――――――― FloatSlider ――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_FLOAT_SLIDER, PF_FloatSliderDef, fs_d,
    Param::FloatSlider,
    FloatSliderDef { },
    impl value: f64,
    impl phase: f64,
    impl default: f64,
    impl precision: i16,
    impl curve_tolerance: f32,
    impl valid_min: f32,
    impl valid_max: f32,
    impl slider_min: f32,
    impl slider_max: f32,
    impl value_desc: ShortString,
}
impl FloatSliderDef<'_> {
    pub fn set_display_flags(&mut self, flags: ValueDisplayFlag) -> &mut Self {
        self.def.display_flags = flags.bits() as _;
        self
    }
    pub fn display_flags(&self) -> ValueDisplayFlag {
        ValueDisplayFlag::from_bits_truncate(self.def.display_flags as _)
    }
    pub fn set_flags(&mut self, flags: FSliderFlag) -> &mut Self {
        self.def.fs_flags = flags.bits() as _;
        self
    }
    pub fn flags(&self) -> FSliderFlag {
        FSliderFlag::from_bits_truncate(self.def.fs_flags as _)
    }
    pub fn set_exponent(&mut self, v: f32) -> &mut Self {
        self.def.exponent = v;
        self.def.useExponent = 1;
        self
    }
    pub fn exponent(&self) -> Option<f32> {
        if self.def.useExponent == 1 {
            Some(self.def.exponent)
        } else {
            None
        }
    }
}

// This is not an enum because the set_precision method of the FloatSlider takes i16,
// and since this is a preference thing (some people prefer to use 0 instead of Precision::Integer),
// having it as a module with constants is more flexible.
#[allow(non_upper_case_globals, non_snake_case)]
pub mod Precision {
    pub const Integer: i16 = 0;
    pub const Tenths: i16 = 1;
    pub const Hundredths: i16 = 2;
    pub const Thousandths: i16 = 3;
    pub const TenThousandths: i16 = 4;
}

// ―――――――――――――――――――――――――――――――――― FloatSlider ――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Path ―――――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_PATH, PF_PathDef, path_d,
    Param::Path,
    /// Path parameters give access to the mask/path/shapes of the layer on which the effect is applied.
    /// For more information on how to use these paths, see the `PF_PathQuerySuite`, and the `PF_PathDataSuite`
    /// * `path_id` - to be used with `PF_CheckoutPath()` note that path_id != `PF_PathID_NONE` does not guarantee that `PF_CheckoutPath` will return a valid path (it may have been deleted)
    /// * `default` - 0 means that the default is NONE, other numbers are the 1-based index of the path, if the path doesn't exist, the `path_id` value will be `PF_PathID_NONE`.
    PathDef { },
    impl path_id: PF_PathID,
    impl default: i32,
}
// ―――――――――――――――――――――――――――――――――――― Path ―――――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Point ――――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_POINT, PF_PointDef, td,
    Param::Point,
    /// The values for the point use the source's coordinate system, with the origin at the top left.
    ///
    /// The defaults are expressed as percentages with the origin at the top left.
    /// The percent can be negative, but should not be smaller than -600%. It should not be greater than 600%.
    ///
    /// If restrict_bounds is `true`, the user will not be allowed to specify points outside the bounds of the layer to which they are applying the effect.
    /// If this is `true`, the dephaults should be between 0.0 and 100.0.
    PointDef { },
    impl restrict_bounds: bool,
    impl x_value: Fixed,
    impl y_value: Fixed,
}
impl PointDef<'_> {
    pub fn set_default_x(&mut self, v: f32) -> &mut Self { self.def.x_dephault = Fixed::from(v).as_fixed(); self }
    pub fn set_default_y(&mut self, v: f32) -> &mut Self { self.def.y_dephault = Fixed::from(v).as_fixed(); self }
    pub fn default_x(&self) -> f32 { Fixed::from_fixed(self.def.x_dephault).into() }
    pub fn default_y(&self) -> f32 { Fixed::from_fixed(self.def.y_dephault).into() }

    pub fn set_default(&mut self, v: (f32, f32)) -> &mut Self {
        self.def.x_dephault = Fixed::from(v.0).as_fixed();
        self.def.y_dephault = Fixed::from(v.1).as_fixed();
        self
    }
    pub fn default(&self) -> (f32, f32) {
        (Fixed::from_fixed(self.def.x_dephault).into(), Fixed::from_fixed(self.def.y_dephault).into())
    }

    pub fn set_value(&mut self, v: (f32, f32)) -> &mut Self {
        self.def.x_value = Fixed::from(v.0).as_fixed();
        self.def.y_value = Fixed::from(v.1).as_fixed();
        self
    }
    pub fn value(&self) -> (f32, f32) {
        (Fixed::from_fixed(self.def.x_value).into(), Fixed::from_fixed(self.def.y_value).into())
    }
    pub fn float_value(&self) -> Result<ae_sys::A_FloatPoint, Error> {
        if self._in_data.is_null() || self._parent_ptr.is_none() {
            return Err(Error::InvalidParms);
        }
        Ok(pf::suites::PointParam::new()?
            .floating_point_value_from_point_def(unsafe { (*self._in_data).effect_ref }, self._parent_ptr.unwrap())?)
    }
}

define_param_wrapper! {
    PF_Param_POINT_3D, PF_Point3DDef, point3d_d,
    Param::Point3D,
    /// Just like POINT, with an extra dimension. Supported in AE starting with version 10.5 (CS 5.5).
    /// * `x_dephault` - percentage of layer width; note: use 50 for halfway, not 0.5; this matches the old PF_PointDef behavior
    /// * `y_dephault` - percentage of layer height
    /// * `z_dephault` - percentage of layer _height_ (since typical layers are zero depth)
    Point3DDef { },
    impl x_value: f64,
    impl y_value: f64,
    impl z_value: f64,
}
impl Point3DDef<'_> {
    pub fn set_default_x(&mut self, v: f64) -> &mut Self { self.def.x_dephault = v; self }
    pub fn set_default_y(&mut self, v: f64) -> &mut Self { self.def.y_dephault = v; self }
    pub fn set_default_z(&mut self, v: f64) -> &mut Self { self.def.z_dephault = v; self }
    pub fn default_x(&self) -> f64 { self.def.x_dephault }
    pub fn default_y(&self) -> f64 { self.def.y_dephault }
    pub fn default_z(&self) -> f64 { self.def.z_dephault }

    pub fn set_default(&mut self, v: (f64, f64, f64)) -> &mut Self {
        self.def.x_dephault = v.0;
        self.def.y_dephault = v.1;
        self.def.z_dephault = v.2;
        self
    }
    pub fn default(&self) -> (f64, f64, f64) {
        (self.def.x_dephault, self.def.y_dephault, self.def.z_dephault)
    }

    pub fn set_value(&mut self, v: (f64, f64, f64)) -> &mut Self {
        self.def.x_value = v.0;
        self.def.y_value = v.1;
        self.def.z_value = v.2;
        self
    }
    pub fn value(&self) -> (f64, f64, f64) {
        (self.def.x_value, self.def.y_value, self.def.z_value)
    }
}
// ―――――――――――――――――――――――――――――――――――― Point ――――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Popup ―――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_POPUP, PF_PopupDef, pd,
    Param::Popup,
    PopupDef {
        options: CString,
    },
    impl value: i32,
    impl default: i32,
}
impl<'a> PopupDef<'a> {
    pub fn set_options(&mut self, options: &[&str]) {
        // Build a string in the format "list|of|choices|", the format Ae expects.
        self.options = CString::new(options.join("|")).unwrap();
        self.def.u.namesptr = self.options.as_ptr();
        self.def.num_choices = options.len().try_into().unwrap();
    }
    pub fn options(&self) -> Vec<&str> {
        let options = unsafe { CStr::from_ptr(self.def.u.namesptr).to_str().unwrap() };
        options.split('|').collect()
    }
}
// ―――――――――――――――――――――――――――――――――――― Popup ―――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Layer ―――――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_LAYER, PF_LayerDef, ld,
    Param::Layer,
    LayerDef { },
}
impl<'a> LayerDef<'a> {
    pub fn set_default_to_this_layer(&mut self) {
        self.def.dephault = ae_sys::PF_LayerDefault_MYSELF;
    }
    pub fn value(&self) -> Option<Layer> {
        if self.def.data.is_null() {
            None
        } else {
            Some(Layer::from_raw(&*self.def as *const _ as _, self._in_data, None))
        }
    }
}
// ―――――――――――――――――――――――――――――――――――― Layer ―――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――――― Null ―――――――――――――――――――――――――――――――――――――
#[allow(dead_code)]
pub struct NullDef<'a>(&'a ());
impl<'a> NullDef<'a> {
    pub fn new() -> Self {
        Self(&())
    }
}
impl<'p> Into<Param<'p>> for NullDef<'p> {
    fn into(self) -> Param<'p> {
        Param::Null(self)
    }
}
// ―――――――――――――――――――――――――――――――――――― Null ―――――――――――――――――――――――――――――――――――――

// ―――――――――――――――――――――――――――――――――― Arbitrary ―――――――――――――――――――――――――――――――――――
define_param_wrapper! {
    PF_Param_ARBITRARY_DATA, PF_ArbitraryDef, arb_d,
    Param::Arbitrary,
    ArbitraryDef { },
    impl pad: i16,
}
impl ArbitraryDef<'_> {
    pub fn set_default<T>(&mut self, value: T) -> Result<&mut Self, Error> {
        self.def.dephault = Handle::into_raw(Handle::new(value)?);
        Ok(self)
    }

    pub fn set_value<T>(&mut self, value: T) -> Result<&mut Self, Error> {
        if !self.def.value.is_null() {
            let _ = Handle::<T>::from_raw(self.def.value, true);
        }
        self.def.value = Handle::into_raw(Handle::new(value)?);
        self.set_value_changed();
        Ok(self)
    }
    pub fn value<T>(&self) -> Result<BorrowedHandleLock<T>, Error> {
        if self.def.value.is_null() {
            return Err(Error::InvalidParms);
        }
        BorrowedHandleLock::<T>::from_raw(self.def.value)
    }

    pub fn set_refcon(&mut self, refcon: *mut std::ffi::c_void) -> &mut Self {
        self.def.refconPV = refcon as _;
        self
    }
}
// ―――――――――――――――――――――――――――――――――― Arbitrary ―――――――――――――――――――――――――――――――――――

pub trait ArbitraryData<T> {
    fn interpolate(&self, other: &T, value: f64) -> T;
}

define_struct_wrapper!(ArbParamsExtra, PF_ArbParamsExtra);

impl std::fmt::Debug for ArbParamsExtra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArbParamsExtra")
            .field("id", &self.id())
            .field("refcon", &self.refcon())
            .field("which_function", &self.which_function())
            .finish()
    }
}

impl ArbParamsExtra {
    pub fn id(&self) -> i16 {
        self.as_ref().id as _
    }

    pub fn refcon(&self) -> *mut std::ffi::c_void {
        unsafe { self.as_ref().u.new_func_params.refconPV }
    }

    pub fn which_function(&self) -> u32 {
        self.as_ref().which_function as _
    }

    pub fn dispatch<T, P>(&mut self, param: P) -> Result<(), Error>
    where T: ArbitraryData<T> + Default + DeserializeOwned + Serialize + PartialEq + PartialOrd,
          P: Eq + PartialEq + Hash + Copy + Debug
    {
        let param_id = Parameters::param_id(param) as i16;
        if self.id() != param_id {
            // Not our param, nothing to do
            return Ok(());
        }
        match self.as_ref().which_function as _ {
            ae_sys::PF_Arbitrary_NEW_FUNC => unsafe {
                assert!(!self.as_ref().u.new_func_params.arbPH.is_null());
                // log::info!("NEW FUNC");
                // Create a new instance, serialize it to a Vec<u8>
                // pass it to a FlatHandle and turn that into a raw
                // Ae handle that we stash in the PF_ArbParamsExtra
                // struct wrapper.
                self.as_ref()
                    .u
                    .new_func_params
                    .arbPH
                    .write(Handle::into_raw(Handle::<T>::new(T::default())?));
            },

            ae_sys::PF_Arbitrary_DISPOSE_FUNC => {
                // log::info!("DISPOSE_FUNC");

                // Create a new handle from the raw Ae handle. This
                // disposes then handle when it goes out of scope
                // and is dropped just after.
                if unsafe { !self.as_ref().u.dispose_func_params.arbH.is_null() } {
                    Handle::<T>::from_raw(unsafe { self.as_ref().u.dispose_func_params.arbH }, true)?;
                }
            }

            ae_sys::PF_Arbitrary_COPY_FUNC => unsafe {
                // log::info!("COPY_FUNC");
                // Create a new handle wraper from the sources,
                // get a referece to that as a slice create a new
                // handle from that and write that to the
                // destination pointer.

                if self.as_ref().u.copy_func_params.src_arbH.is_null() {
                    // Create a new default value
                    self.as_ref()
                        .u
                        .copy_func_params
                        .dst_arbPH
                        .write(Handle::into_raw(Handle::<T>::new(T::default())?));
                    return Ok(());
                }

                let mut src_handle = Handle::<T>::from_raw(self.as_ref().u.copy_func_params.src_arbH, false)?;
                let lock = src_handle.lock()?;

                let serialized = bincode::serde::encode_to_vec::<&T, _>(lock.as_ref()?, bincode::config::legacy()).map_err(|_| Error::InternalStructDamaged)?;
                let deserialized = bincode::serde::decode_from_slice::<T, _>(&serialized, bincode::config::legacy()).map_err(|_| Error::InternalStructDamaged)?;
                let new_handle = Handle::<T>::new(deserialized.0)?;

                self.as_ref()
                    .u
                    .copy_func_params
                    .dst_arbPH
                    .write(Handle::into_raw(new_handle));
            },

            ae_sys::PF_Arbitrary_FLAT_SIZE_FUNC => unsafe {
                // log::info!("FLAT_SIZE_FUNC");

                let mut handle = Handle::<T>::from_raw(self.as_ref().u.flat_size_func_params.arbH, false)?;
                let lock = handle.lock()?;

                let serialized = bincode::serde::encode_to_vec::<&T, _>(lock.as_ref()?, bincode::config::legacy()).map_err(|_| Error::InternalStructDamaged)?;

                self.as_ref()
                    .u
                    .flat_size_func_params
                    .flat_data_sizePLu
                    .write(serialized.len() as _);
            },

            ae_sys::PF_Arbitrary_FLATTEN_FUNC => unsafe {
                // log::info!("FLATTEN_FUNC");
                assert!(!self.as_ref().u.unflatten_func_params.flat_dataPV.is_null());

                let mut handle = Handle::<T>::from_raw(self.as_ref().u.flatten_func_params.arbH, false)?;
                let lock = handle.lock()?;

                let serialized = bincode::serde::encode_to_vec::<&T, _>(lock.as_ref()?, bincode::config::legacy()).map_err(|_| Error::InternalStructDamaged)?;

                assert!(
                    serialized.len() <= self.as_ref().u.flatten_func_params.buf_sizeLu as _
                );

                std::ptr::copy_nonoverlapping(
                    serialized.as_ptr(),
                    self.as_ref().u.flatten_func_params.flat_dataPV as _,
                    serialized.len(),
                );
            }

            ae_sys::PF_Arbitrary_UNFLATTEN_FUNC => unsafe {
                // log::info!("UNFLATTEN_FUNC");
                assert!(!self.as_ref().u.unflatten_func_params.flat_dataPV.is_null());

                let serialized = std::slice::from_raw_parts(
                    self.as_ref().u.unflatten_func_params.flat_dataPV as *mut u8,
                    self.as_ref().u.unflatten_func_params.buf_sizeLu as _
                );
                let t = bincode::serde::decode_from_slice::<T, _>(serialized, bincode::config::legacy()).map_err(|_| Error::InternalStructDamaged)?;
                let handle = Handle::<T>::new(t.0)?;

                self.as_ref()
                    .u
                    .unflatten_func_params
                    .arbPH
                    .write(Handle::into_raw(handle));
            },

            ae_sys::PF_Arbitrary_INTERP_FUNC => unsafe {
                // log::info!("INTERP_FUNC");

                let mut left = Handle::<T>::from_raw(self.as_ref().u.interp_func_params.left_arbH, false)?;
                let left_lock = left.lock()?;

                let mut right = Handle::<T>::from_raw(self.as_ref().u.interp_func_params.right_arbH, false)?;
                let right_lock = right.lock()?;

                let interpolated = Handle::<T>::new(
                    left_lock.as_ref()?.interpolate(right_lock.as_ref()?, self.as_ref().u.interp_func_params.tF)
                )?;

                self.as_ref()
                    .u
                    .interp_func_params
                    .interpPH
                    .write(Handle::into_raw(interpolated));
            },

            ae_sys::PF_Arbitrary_COMPARE_FUNC => {
                // log::info!("COMPARE_FUNC");

                let mut handle_a = Handle::<T>::from_raw(unsafe { self.as_ref().u.compare_func_params.a_arbH }, false)?;
                let handle_a_lock = handle_a.lock()?;
                let a = handle_a_lock.as_ref()?;

                let mut handle_b = Handle::<T>::from_raw(unsafe { self.as_ref().u.compare_func_params.b_arbH }, false)?;
                let handle_b_lock = handle_b.lock()?;
                let b = handle_b_lock.as_ref()?;

                if a < b {
                    unsafe {
                        self.as_ref()
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_LESS as _);
                    }
                } else if a > b {
                    unsafe {
                        self.as_ref()
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_MORE as _);
                    }
                } else if a == b {
                    unsafe {
                        self.as_ref()
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_EQUAL as _);
                    }
                } else {
                    unsafe {
                        self.as_ref()
                            .u
                            .compare_func_params
                            .compareP
                            .write(ae_sys::PF_ArbCompare_NOT_EQUAL as _);
                    }
                }
            }

            ae_sys::PF_Arbitrary_PRINT_SIZE_FUNC => unsafe {
                // log::info!("PRINT_SIZE_FUNC");

                let mut handle = Handle::<T>::from_raw(self.as_ref().u.print_size_func_params.arbH, false)?;
                let lock = handle.lock()?;

                let serialized = serde_json::to_string::<T>(lock.as_ref()?).map_err(|_| Error::InternalStructDamaged)?;
                let cstr = std::ffi::CString::new(serialized).unwrap();

                self.as_ref().u.print_size_func_params.print_sizePLu.write(
                    cstr.as_bytes_with_nul().len() as _,
                );
            },

            // Print arbitrary data into a string as JSON.
            // Note that we could use any text-based serializer here.
            ae_sys::PF_Arbitrary_PRINT_FUNC => unsafe {
                // log::info!("PRINT_FUNC");

                let mut handle = Handle::<T>::from_raw(self.as_ref().u.print_func_params.arbH, false)?;
                let lock = handle.lock()?;

                let serialized = serde_json::to_string::<T>(lock.as_ref()?).map_err(|_| Error::InternalStructDamaged)?;
                let cstr = std::ffi::CString::new(serialized).unwrap();
                let cstr = cstr.as_bytes_with_nul();

                if cstr.len() <= self.as_ref().u.print_func_params.print_sizeLu as _ && self.as_ref().u.print_func_params.print_flags == 0 {
                    std::ptr::copy_nonoverlapping(
                        cstr.as_ptr(),
                        self.as_ref().u.print_func_params.print_bufferPC as _,
                        cstr.len(),
                    );
                }
            }
            ae_sys::PF_Arbitrary_SCAN_FUNC => unsafe {
                // log::info!("SCAN_FUNC");

                let cstr = CStr::from_ptr(self.as_ref().u.scan_func_params.bufPC).to_str().map_err(|_| Error::InternalStructDamaged)?;

                let t = serde_json::from_str::<T>(cstr).map_err(|_| Error::InternalStructDamaged)?;
                let handle = Handle::<T>::new(t)?;

                self.as_ref()
                    .u
                    .scan_func_params
                    .arbPH
                    .write(Handle::into_raw(handle));
            },
            _ => {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }
}

macro_rules! define_param_cast {
    ($name:tt, $enm:ident, $type:ty) => {
        paste::item! {
            pub fn [<as_ $name>]<'a>(&'a self) -> Result<$type<'a>, Error> where 'p: 'a {
                match self.as_param()? {
                    Param::$enm(x) => Ok(x),
                    x => {
                        log::error!("Invalid param type! Requested {:?}, but the param is {:?}", stringify!($name), x);
                        Err(Error::InvalidParms)
                    }
                }
            }
            pub fn [<as_ $name _mut>]<'a>(&'a mut self) -> Result<$type<'a>, Error> where 'p: 'a {
                match self.as_param_mut()? {
                    Param::$enm(x) => Ok(x),
                    x => {
                        log::error!("Invalid param type! Requested {:?}, but the param is {:?}", stringify!($name), x);
                        Err(Error::InvalidParms)
                    }
                }
            }
        }
    };
}

pub enum Param<'p> {
    Angle(AngleDef<'p>),
    Arbitrary(ArbitraryDef<'p>),
    Button(ButtonDef<'p>),
    CheckBox(CheckBoxDef<'p>),
    Color(ColorDef<'p>),
    FloatSlider(FloatSliderDef<'p>),
    Path(PathDef<'p>),
    Point(PointDef<'p>),
    Point3D(Point3DDef<'p>),
    Popup(PopupDef<'p>),
    Slider(SliderDef<'p>),
    Layer(LayerDef<'p>),
    Null(NullDef<'p>),
}

impl Debug for Param<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::Angle(_)       => write!(f, "Angle"),
            Param::Arbitrary(_)   => write!(f, "Arbitrary"),
            Param::Button(_)      => write!(f, "Button"),
            Param::CheckBox(_)    => write!(f, "CheckBox"),
            Param::Color(_)       => write!(f, "Color"),
            Param::FloatSlider(_) => write!(f, "FloatSlider"),
            Param::Path(_)        => write!(f, "Path"),
            Param::Point(_)       => write!(f, "Point"),
            Param::Point3D(_)     => write!(f, "Point3D"),
            Param::Popup(_)       => write!(f, "Popup"),
            Param::Slider(_)      => write!(f, "Slider"),
            Param::Layer(_)       => write!(f, "Layer"),
            Param::Null(_)        => write!(f, "Null"),
        }
    }
}

#[derive(Clone)]
pub struct ParamDef<'p> {
    param_def: Ownership<'p, ae_sys::PF_ParamDef>,
    checkin_on_drop: bool,
    index: Option<i32>,
    in_data: InData,
}

impl<'p> ParamDef<'p> {
    pub fn new(in_data: InData) -> Self {
        Self {
            param_def: Ownership::Rust(unsafe { std::mem::zeroed() }),
            checkin_on_drop: false,
            in_data,
            index: None,
        }
    }

    pub fn as_ref(&self) -> &ae_sys::PF_ParamDef {
        &*self.param_def
    }
    pub fn as_mut(&mut self) -> &mut ae_sys::PF_ParamDef {
        &mut *self.param_def
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn update_param_ui(&self) -> Result<(), Error> {
        if let Some(index) = self.index {
            self.in_data.effect().update_param_ui(index, self)
        } else {
            Err(Error::InvalidIndex)
        }
    }
    pub fn keyframe_count(&self) -> Result<i32, Error> {
        if let Some(index) = self.index {
            pf::suites::ParamUtils::new()?
                .keyframe_count(self.in_data.effect_ref(), index)
        } else {
            Err(Error::InvalidIndex)
        }
    }

    pub fn from_raw(in_data: InData, param_def: &'p mut ae_sys::PF_ParamDef, index: Option<i32>) -> Self {
        Self {
            param_def: Ownership::AfterEffectsMut(param_def),
            checkin_on_drop: false,
            in_data,
            index,
        }
    }

    pub fn add(&mut self, index: i32) -> Result<(), Error> {
        self.in_data.interact().add_param(index, &*self.param_def)?;
        if index != -1 {
            self.index = Some(index);
        }
        Ok(())
    }

    pub fn checkout(in_data: InData, index: i32, what_time: i32, time_step: i32, time_scale: u32, expected_type: Option<ParamType>) -> Result<Self, Error> {
        let mut param_def = in_data.interact().checkout_param(index, what_time, time_step, time_scale)?;
        if param_def.param_type == ae_sys::PF_Param_RESERVED {
            return Err(Error::InvalidIndex);
        }

        // For some reason the checked out param_type is 0 so we need to override using the info we have from params map.
        if index > 0 && expected_type.is_some() {
            param_def.param_type = expected_type.unwrap().into();
        }

        Ok(Self {
            param_def: Ownership::Rust(param_def),
            checkin_on_drop: true,
            in_data,
            index: Some(index),
        })
    }

    pub fn set_param(&mut self, param: &Param) {
        match param {
            Param::Popup(pd) => {
                self.param_def.u.pd = *pd.def;
                self.param_def.param_type = ae_sys::PF_Param_POPUP;
            }
            Param::Angle(ad) => {
                self.param_def.u.ad = *ad.def;
                self.param_def.param_type = ae_sys::PF_Param_ANGLE;
            }
            Param::CheckBox(bd) => {
                self.param_def.u.bd = *bd.def;
                self.param_def.param_type = ae_sys::PF_Param_CHECKBOX;
            }
            Param::Color(cd) => {
                self.param_def.u.cd = *cd.def;
                self.param_def.param_type = ae_sys::PF_Param_COLOR;
            }
            Param::Slider(sd) => {
                self.param_def.u.sd = *sd.def;
                self.param_def.param_type = ae_sys::PF_Param_SLIDER;
            }
            Param::FloatSlider(fs_d) => {
                self.param_def.u.fs_d = *fs_d.def;
                self.param_def.param_type = ae_sys::PF_Param_FLOAT_SLIDER;
            }
            Param::Button(button_d) => {
                self.param_def.u.button_d = *button_d.def;
                self.param_def.param_type = ae_sys::PF_Param_BUTTON;
            }
            Param::Path(path_d) => {
                self.param_def.u.path_d = *path_d.def;
                self.param_def.param_type = ae_sys::PF_Param_PATH;
            }
            Param::Point(td) => {
                self.param_def.u.td = *td.def;
                self.param_def.param_type = ae_sys::PF_Param_POINT;
            }
            Param::Point3D(point3d_d) => {
                self.param_def.u.point3d_d = *point3d_d.def;
                self.param_def.param_type = ae_sys::PF_Param_POINT_3D;
            }
            Param::Arbitrary(arb_d) => {
                self.param_def.u.arb_d = *arb_d.def;
                self.param_def.param_type = ae_sys::PF_Param_ARBITRARY_DATA;
            }
            Param::Layer(ld) => {
                self.param_def.u.ld = *ld.def;
                self.param_def.param_type = ae_sys::PF_Param_LAYER;
            }
            Param::Null(_) => {
                self.param_def.param_type = ae_sys::PF_Param_NO_DATA;
            }
        }
    }

    define_param_cast!("popup",        Popup,       PopupDef);
    define_param_cast!("angle",        Angle,       AngleDef);
    define_param_cast!("checkbox",     CheckBox,    CheckBoxDef);
    define_param_cast!("color",        Color,       ColorDef);
    define_param_cast!("slider",       Slider,      SliderDef);
    define_param_cast!("float_slider", FloatSlider, FloatSliderDef);
    define_param_cast!("button",       Button,      ButtonDef);
    define_param_cast!("arbitrary",    Arbitrary,   ArbitraryDef);
    define_param_cast!("point",        Point,       PointDef);
    define_param_cast!("point3d",      Point3D,     Point3DDef);
    define_param_cast!("path",         Path,        PathDef);
    define_param_cast!("layer",        Layer,       LayerDef);
    define_param_cast!("null",         Null,        NullDef);

    pub fn as_param<'a>(&'a self) -> Result<Param<'a>, Error> where 'p: 'a {
        let param_def = &*self.param_def;
        let parent_ptr = param_def as *const _;
        unsafe {
            match param_def.param_type {
                ae_sys::PF_Param_ANGLE          => Ok(Param::Angle      (AngleDef      ::from_ref(&param_def.u.ad,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_ARBITRARY_DATA => Ok(Param::Arbitrary  (ArbitraryDef  ::from_ref(&param_def.u.arb_d,     self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_BUTTON         => Ok(Param::Button     (ButtonDef     ::from_ref(&param_def.u.button_d,  self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_CHECKBOX       => Ok(Param::CheckBox   (CheckBoxDef   ::from_ref(&param_def.u.bd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_COLOR          => Ok(Param::Color      (ColorDef      ::from_ref(&param_def.u.cd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_FLOAT_SLIDER   => Ok(Param::FloatSlider(FloatSliderDef::from_ref(&param_def.u.fs_d,      self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POPUP          => Ok(Param::Popup      (PopupDef      ::from_ref(&param_def.u.pd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_SLIDER         => Ok(Param::Slider     (SliderDef     ::from_ref(&param_def.u.sd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POINT          => Ok(Param::Point      (PointDef      ::from_ref(&param_def.u.td,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POINT_3D       => Ok(Param::Point3D    (Point3DDef    ::from_ref(&param_def.u.point3d_d, self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_PATH           => Ok(Param::Path       (PathDef       ::from_ref(&param_def.u.path_d,    self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_LAYER          => Ok(Param::Layer      (LayerDef      ::from_ref(&param_def.u.ld,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_NO_DATA        => Ok(Param::Null       (NullDef       ::new())),
                _ => {
                    log::error!("Invalid parameter type: {}", param_def.param_type);
                    Err(Error::InvalidParms)
                }
            }
        }
    }

    pub fn as_param_mut<'a>(&'a mut self) -> Result<Param<'a>, Error> where 'p: 'a {
        let param_def = &mut *self.param_def;
        let parent_ptr = param_def as *const _;
        unsafe {
            match param_def.param_type {
                ae_sys::PF_Param_ANGLE          => Ok(Param::Angle      (AngleDef      ::from_mut(&mut param_def.u.ad,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_ARBITRARY_DATA => Ok(Param::Arbitrary  (ArbitraryDef  ::from_mut(&mut param_def.u.arb_d,     self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_BUTTON         => Ok(Param::Button     (ButtonDef     ::from_mut(&mut param_def.u.button_d,  self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_CHECKBOX       => Ok(Param::CheckBox   (CheckBoxDef   ::from_mut(&mut param_def.u.bd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_COLOR          => Ok(Param::Color      (ColorDef      ::from_mut(&mut param_def.u.cd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_FLOAT_SLIDER   => Ok(Param::FloatSlider(FloatSliderDef::from_mut(&mut param_def.u.fs_d,      self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POPUP          => Ok(Param::Popup      (PopupDef      ::from_mut(&mut param_def.u.pd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_SLIDER         => Ok(Param::Slider     (SliderDef     ::from_mut(&mut param_def.u.sd,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POINT          => Ok(Param::Point      (PointDef      ::from_mut(&mut param_def.u.td,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_POINT_3D       => Ok(Param::Point3D    (Point3DDef    ::from_mut(&mut param_def.u.point3d_d, self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_PATH           => Ok(Param::Path       (PathDef       ::from_mut(&mut param_def.u.path_d,    self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_LAYER          => Ok(Param::Layer      (LayerDef      ::from_mut(&mut param_def.u.ld,        self.in_data.as_ptr(), parent_ptr))),
                ae_sys::PF_Param_NO_DATA        => Ok(Param::Null       (NullDef       ::new())),
                _ => {
                    log::error!("Invalid parameter type: {}", param_def.param_type);
                    Err(Error::InvalidParms)
                }
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(
            self.param_def.param_type,
            ae_sys::PF_Param_ANGLE
                | ae_sys::PF_Param_ARBITRARY_DATA
                | ae_sys::PF_Param_BUTTON
                | ae_sys::PF_Param_CHECKBOX
                | ae_sys::PF_Param_COLOR
                | ae_sys::PF_Param_FIX_SLIDER
                | ae_sys::PF_Param_FLOAT_SLIDER
                | ae_sys::PF_Param_GROUP_START
                | ae_sys::PF_Param_GROUP_END
                | ae_sys::PF_Param_POPUP
                | ae_sys::PF_Param_SLIDER
                | ae_sys::PF_Param_POINT
                | ae_sys::PF_Param_POINT_3D
                | ae_sys::PF_Param_PATH
                | ae_sys::PF_Param_LAYER
                | ae_sys::PF_Param_NO_DATA
        )
    }
    pub fn param_type(&self) -> ParamType {
        self.param_def.param_type.into()
    }

    pub unsafe fn layer_def(&mut self) -> *mut ae_sys::PF_LayerDef {
        unsafe { &mut self.param_def.u.ld }
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), Error> {
        if name.is_empty() {
            self.param_def.name[0] = 0;
            return Ok(());
        }
        // According to Adobe docs, the encoding expected for the name is the system encoding.
        // Reference: https://ae-plugins.docsforadobe.dev/intro/localization/
        let mut bytes = {
            #[cfg(target_os = "macos")]
            {
                use objc2_core_foundation::CFString;
                let cfstr = CFString::from_str(name);
                let c_string = cfstr.c_string_ptr(CFString::system_encoding());
                if c_string.is_null() {
                    return Err(Error::InvalidParms)
                }
                unsafe {
                    std::ffi::CStr::from_ptr(c_string).to_bytes_with_nul().to_vec()
                }
            }
            #[cfg(target_os = "windows")]
            {
                use std::ffi::OsStr;
                use std::os::windows::ffi::OsStrExt;
                use windows_sys::Win32::Globalization::{WideCharToMultiByte, CP_OEMCP};
                let wstr: Vec<u16> = OsStr::new(name).encode_wide().collect();
                if wstr.is_empty() {
                    Vec::new()
                } else {
                    unsafe {
                        let len = WideCharToMultiByte(CP_OEMCP, 0, wstr.as_ptr(), wstr.len() as i32, std::ptr::null_mut(), 0, std::ptr::null(), std::ptr::null_mut());
                        if len > 0 {
                            let mut bytes: Vec<u8> = Vec::with_capacity(len as usize);
                            let len = WideCharToMultiByte(CP_OEMCP, 0, wstr.as_ptr(), wstr.len() as i32, bytes.as_mut_ptr() as _, len, std::ptr::null(), std::ptr::null_mut());
                            if len > 0 {
                                bytes.set_len(len as usize);
                                if (len as usize) == bytes.len() {
                                    bytes
                                } else {
                                    bytes[0..(len as usize)].to_vec()
                                }
                            } else {
                                return Err(Error::InvalidParms);
                            }
                        } else {
                            return Err(Error::InvalidParms);
                        }
                    }
                }
            }
        };

        let to_copy = bytes.len().min(MAX_NAME_LEN - 1);
        if to_copy > 0 {
            bytes.resize(to_copy, 0);
            bytes.push(0); // Null-terminate
            self.param_def.name[0..bytes.len()].copy_from_slice(unsafe { std::mem::transmute(bytes.as_slice()) });
            return Ok(());
        }

        log::error!("Failed to set the parameter name, \"{name}\" is too long or contains invalid characters for the system encoding.");
        Err(Error::InvalidParms)
    }

    pub fn set_flags       (&mut self, f: ParamFlag)    { self.param_def.flags           = f.bits() as _; }
    pub fn set_change_flags(&mut self, f: ChangeFlag)   { self.param_def.uu.change_flags = f.bits() as _; }
    pub fn set_ui_flags    (&mut self, f: ParamUIFlags) { self.param_def.ui_flags        = f.bits() as _; }

    pub fn set_flag       (&mut self, f: ParamFlag,    set: bool) { let mut v = self.flags();        v.set(f, set); self.set_flags(v);        }
    pub fn set_change_flag(&mut self, f: ChangeFlag,   set: bool) { let mut v = self.change_flags(); v.set(f, set); self.set_change_flags(v); }
    pub fn set_ui_flag    (&mut self, f: ParamUIFlags, set: bool) { let mut v = self.ui_flags();     v.set(f, set); self.set_ui_flags(v);     }

    pub fn flags       (&self) -> ParamFlag    {    ParamFlag::from_bits_truncate(self.param_def.flags) }
    pub fn change_flags(&self) -> ChangeFlag   {   ChangeFlag::from_bits_truncate(unsafe { self.param_def.uu.change_flags }) }
    pub fn ui_flags    (&self) -> ParamUIFlags { ParamUIFlags::from_bits_truncate(self.param_def.ui_flags) }

    pub fn set_ui_width(&mut self, width: u16) {
        self.param_def.ui_width = width as _;
    }
    pub fn set_ui_height(&mut self, height: u16) {
        self.param_def.ui_height = height as _;
    }

    pub fn set_id(&mut self, id: i32) {
        self.param_def.uu.id = id;
        if self.param_def.param_type == ae_sys::PF_Param_ARBITRARY_DATA {
            self.param_def.u.arb_d.id = id as i16; // this truncates the int, but it should be fine
        }
    }

    pub fn set_value_changed(&mut self) {
        self.param_def.uu.change_flags = ChangeFlag::CHANGED_VALUE.bits();
    }
}

impl Drop for ParamDef<'_> {
    fn drop(&mut self) {
        if self.checkin_on_drop {
            self.in_data.interact().checkin_param(&*self.param_def).unwrap()
        }
    }
}
impl Debug for ParamDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParamDef")
            .field("type", &self.param_type())
            .field("checkin_on_drop", &self.checkin_on_drop)
            .field("in_data_ptr", &self.in_data.as_ptr())
            .finish()
    }
}

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct ParamMapInfo {
    pub index: usize,
    pub type_: ParamType,
}
impl ParamMapInfo {
    fn new(index: usize, type_: ParamType) -> Self {
        Self { index, type_ }
    }
}

#[derive(Clone)]
pub struct Parameters<'p, P: Eq + PartialEq + Hash + Copy + Debug> {
    num_params: usize,
    in_data: *const ae_sys::PF_InData,
    pub map: Ownership<'p, HashMap<P, ParamMapInfo>>,
    params: Vec<ParamDef<'p>>,
}
impl<P: Eq + PartialEq + Hash + Copy + Debug> Default for Parameters<'_, P> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'p, P: Eq + PartialEq + Hash + Copy + Debug> Parameters<'p, P> {
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn set_in_data(&mut self, in_data: *const ae_sys::PF_InData) {
        self.in_data = in_data;
    }
    pub fn in_data(&self) -> InData {
        InData::from_raw(self.in_data)
    }
    pub fn new() -> Self {
        Self {
            in_data: std::ptr::null(),
            num_params: 1,
            map: Ownership::Rust(Default::default()),
            params: Vec::new(),
        }
    }
    pub fn with_params(in_data: *const ae_sys::PF_InData, params: &'p [*mut ae_sys::PF_ParamDef], map: Option<&'p HashMap<P, ParamMapInfo>>, num_params: usize) -> Self {
        let in_data_obj = InData::from_raw(in_data);
        Self {
            in_data,
            params: if params.is_empty() || params[0].is_null() {
                Vec::new()
            } else {
                params
                    .iter()
                    .enumerate()
                    .map(|(i, p)| { debug_assert!(!p.is_null()); ParamDef::from_raw(in_data_obj, unsafe { &mut **p }, Some(i as i32)) })
                    .collect::<Vec<_>>()
            },
            num_params,
            map: map.map_or_else(|| Ownership::Rust(HashMap::new()), Ownership::AfterEffects),
        }
    }

    fn param_id(type_: P) -> i32 {
        use hash32::Murmur3Hasher;
        use std::hash::Hasher;
        let mut hasher = Murmur3Hasher::default();
        format!("{type_:?}").hash(&mut hasher);
        hasher.finish() as i32
    }

    pub fn add_group<F: FnOnce(&mut Self) -> Result<(), Error>>(&mut self, type_start: P, type_end: P, name: &str, start_collapsed: bool, inner_cb: F) -> Result<(), Error> {
        assert!(!self.in_data.is_null());

        let mut param_def = ParamDef::new(InData::from_raw(self.in_data));
        param_def.set_name(name)?;
        param_def.as_mut().param_type = ParamType::GroupStart.into();
        param_def.set_id(Self::param_id(type_start));
        if start_collapsed {
            param_def.set_flags(ParamFlag::START_COLLAPSED);
        }
        param_def.add(-1)?;
        self.map.insert(type_start, ParamMapInfo::new(self.num_params, ParamType::GroupStart));
        self.num_params += 1;

        inner_cb(self)?;

        let mut param_def = ParamDef::new(InData::from_raw(self.in_data));
        param_def.as_mut().param_type = ParamType::GroupEnd.into();
        param_def.set_id(Self::param_id(type_end));
        param_def.add(-1)?;
        self.map.insert(type_end, ParamMapInfo::new(self.num_params, ParamType::GroupEnd));
        self.num_params += 1;
        Ok(())
    }

    pub fn add<'a>(&mut self, type_: P, name: &str, def: impl Into<Param<'a>>) -> Result<(), Error> {
        assert!(!self.in_data.is_null());

        let param = def.into(); // This must outlive the call to .add()

        let mut param_def = ParamDef::new(InData::from_raw(self.in_data));
        param_def.set_name(name)?;
        param_def.set_param(&param);
        let param_type = param_def.param_type();
        param_def.set_id(Self::param_id(type_));
        if matches!(param, Param::Button(_)) {
            param_def.set_flags(ParamFlag::SUPERVISE);
        }
        param_def.add(-1)?;
        self.map.insert(type_, ParamMapInfo::new(self.num_params, param_type));
        self.num_params += 1;
        Ok(())
    }

    pub fn add_with_flags<'a>(&mut self, type_: P, name: &str, def: impl Into<Param<'a>>, flags: ParamFlag, ui_flags: ParamUIFlags) -> Result<(), Error> {
        assert!(!self.in_data.is_null());

        let param = def.into(); // This must outlive the call to .add()

        let mut param_def = ParamDef::new(InData::from_raw(self.in_data));
        param_def.set_name(name)?;
        param_def.set_param(&param);
        let param_type = param_def.param_type();
        param_def.set_id(Self::param_id(type_));
        param_def.set_flags(flags);
        param_def.set_ui_flags(ui_flags);
        param_def.add(-1)?;
        self.map.insert(type_, ParamMapInfo::new(self.num_params, param_type));
        self.num_params += 1;
        Ok(())
    }

    pub fn add_customized<'a, F: FnOnce(&mut ParamDef) -> i32>(&mut self, type_: P, name: &str, def: impl Into<Param<'a>>, cb: F) -> Result<(), Error> {
        assert!(!self.in_data.is_null());

        let param = def.into(); // This must outlive the call to .add()

        let mut param_def = ParamDef::new(InData::from_raw(self.in_data));
        param_def.set_name(name)?;
        param_def.set_param(&param);
        let param_type = param_def.param_type();
        param_def.set_id(Self::param_id(type_));
        let mut index = cb(&mut param_def);
        param_def.add(index)?;
        if index == -1 {
            index = self.num_params as i32;
        }
        self.map.insert(type_, ParamMapInfo::new(index as usize, param_type));
        self.num_params += 1;
        Ok(())
    }

    #[inline(always)]
    pub fn get(&self, type_: P) -> Result<ReadOnlyOwnership<'_, ParamDef<'p>>, Error> {
        self.get_at(type_, None, None, None)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, type_: P) -> Result<Ownership<'_, ParamDef<'p>>, Error> {
        self.get_mut_at(type_, None, None, None)
    }

    #[inline(always)]
    pub fn checkout(&self, type_: P) -> Result<Ownership<'_, ParamDef<'p>>, Error> {
        self.checkout_at(type_, None, None, None)
    }

    pub fn get_at(&self, type_: P, time: Option<i32>, time_step: Option<i32>, time_scale: Option<u32>) -> Result<ReadOnlyOwnership<'_, ParamDef<'p>>, Error> {
        if self.params.is_empty() || time.is_some() {
            match self.checkout_at(type_, time, time_step, time_scale) {
                Ok(Ownership::Rust(param)) => Ok(ReadOnlyOwnership::Rust(param)),
                Ok(_) => unreachable!(),
                Err(e) => Err(e)
            }
        } else {
            let index = self.index(type_).ok_or(Error::InvalidIndex)?;
            Ok(ReadOnlyOwnership::AfterEffects(self.params.get(index).ok_or(Error::InvalidIndex)?))
        }
    }

    pub fn get_mut_at(&mut self, type_: P, time: Option<i32>, time_step: Option<i32>, time_scale: Option<u32>) -> Result<Ownership<'_, ParamDef<'p>>, Error> {
        if self.params.is_empty() || time.is_some() {
            self.checkout_at(type_, time, time_step, time_scale)
        } else {
            let index = self.index(type_).ok_or(Error::InvalidIndex)?;
            Ok(Ownership::AfterEffectsMut(self.params.get_mut(index).ok_or(Error::InvalidIndex)?))
        }
    }

    pub fn checkout_at(&self, type_: P, time: Option<i32>, time_step: Option<i32>, time_scale: Option<u32>) -> Result<Ownership<'_, ParamDef<'p>>, Error> {
        let index = self.index(type_).ok_or(Error::InvalidIndex)?;
        let type_ = self.raw_param_type(type_).ok_or(Error::InvalidIndex)?;
        let in_data = self.in_data();
        let param = ParamDef::checkout(
            in_data,
            index as i32,
            time.unwrap_or(in_data.current_time()),
            time_step.unwrap_or(in_data.time_step()),
            time_scale.unwrap_or(in_data.time_scale()),
            Some(type_)
        )?;
        if !param.is_valid() {
            return Err(Error::InvalidParms);
        }
        Ok(Ownership::Rust(param))
    }

    pub fn num_params(&self) -> usize {
        self.num_params
    }

    pub fn index(&self, type_: P) -> Option<usize> {
        self.map.get(&type_).map(|x| x.index)
    }
    pub fn type_at(&self, index: usize) -> P {
        *self.map.iter().find(|(_, v)| v.index == index).unwrap().0
    }

    pub fn raw_params(&self) -> &[ParamDef<'p>] {
        &self.params
    }
    pub fn raw_param_type(&self, type_: P) -> Option<ParamType> {
        self.map.get(&type_).map(|x| x.type_)
    }

    pub fn cloned(&self) -> Parameters<'p, P> {
        Parameters::<'p, P> {
            in_data: self.in_data,
            num_params: self.num_params,
            map: self.map.clone(),
            params: self.params.to_vec(),
        }
    }
}
