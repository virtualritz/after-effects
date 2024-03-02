use super::*;
use c_vec::CVec;
use std::ffi::CStr;
use std::ffi::CString;

#[derive(Clone, Copy, Debug)]
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

bitflags! {
    pub struct ParamUIFlags: ae_sys::A_long {
        const NONE = ae_sys::PF_PUI_NONE as ae_sys::A_long;
        /// Effect has custom UI and wants events for this params' title (portion visible when twirled up).
        const TOPIC = ae_sys::PF_PUI_TOPIC as ae_sys::A_long;
        /// Effect has custom UI and wants events for this params' control (portion invisible when twirled up).
        const CONTROL = ae_sys::PF_PUI_CONTROL as ae_sys::A_long;
        // Param will be used as UI only, no data.
        const CONTROL_ONLY = ae_sys::PF_PUI_STD_CONTROL_ONLY as ae_sys::A_long;
        // Stop param from appearing in Effect Controls (which in PPro also means you won't see a keyframe track there).
        const NO_ECW_UI = ae_sys::PF_PUI_NO_ECW_UI as ae_sys::A_long;
        // Draw a thick separating line above this param; not used by Ae.
        const ECW_SEPARATOR = ae_sys::PF_PUI_ECW_SEPARATOR as ae_sys::A_long;
        // Disable (gray-out) UI for this parameter.
        const DISABLED = ae_sys::PF_PUI_DISABLED as ae_sys::A_long;
        // Ae will not erase the ECW topic, it's up to the FX to erase/draw every pixel.
        // Handy if FX author implements an offscreen, prevents flashing.
        const DO_NOT_ERASE_TOPIC = ae_sys::PF_PUI_DONT_ERASE_TOPIC as ae_sys::A_long;
        const DO_NOT_ERASE_CONTROL = ae_sys::PF_PUI_DONT_ERASE_CONTROL as ae_sys::A_long;
        /// Display as a radio-button group; only valid for PF_Param_POPUP; ignored by Ae.
        const RADIO_BUTTON = ae_sys::PF_PUI_RADIO_BUTTON as ae_sys::A_long;
        /// In Ae as of CS6, this hides the parameter UI in both the Effect Controls and Timeline.
        /// in Premiere since earlier than that, this hides the parameter UI in the Effect Controls,
        ///	which includes the keyframe track; for PPro only, the flag is dynamic and can be cleared
        ///	to make the parameter visible again.
        const INVISIBLE = ae_sys::PF_PUI_INVISIBLE as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ParamFlag: ae_sys::A_long {
        const RESERVED1                        = ae_sys::PF_ParamFlag_RESERVED1                        as ae_sys::A_long;
        const CANNOT_TIME_VARY                 = ae_sys::PF_ParamFlag_CANNOT_TIME_VARY                 as ae_sys::A_long;
        const CANNOT_INTERP                    = ae_sys::PF_ParamFlag_CANNOT_INTERP                    as ae_sys::A_long;
        const RESERVED2                        = ae_sys::PF_ParamFlag_RESERVED2                        as ae_sys::A_long;
        const RESERVED3                        = ae_sys::PF_ParamFlag_RESERVED3                        as ae_sys::A_long;
        const TWIRLY                           = ae_sys::PF_ParamFlag_COLLAPSE_TWIRLY                  as ae_sys::A_long;
        const SUPERVISE                        = ae_sys::PF_ParamFlag_SUPERVISE                        as ae_sys::A_long;
        const START_COLLAPSED                  = ae_sys::PF_ParamFlag_START_COLLAPSED                  as ae_sys::A_long;
        const USE_VALUE_FOR_OLD_PROJECTS       = ae_sys::PF_ParamFlag_USE_VALUE_FOR_OLD_PROJECTS       as ae_sys::A_long;
        const LAYER_PARAM_IS_TRACKMATTE        = ae_sys::PF_ParamFlag_LAYER_PARAM_IS_TRACKMATTE        as ae_sys::A_long;
        const EXCLUDE_FROM_HAVE_INPUTS_CHANGED = ae_sys::PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED as ae_sys::A_long;
        const SKIP_REVEAL_WHEN_UNHIDDEN        = ae_sys::PF_ParamFlag_SKIP_REVEAL_WHEN_UNHIDDEN        as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ChangeFlag: ae_sys::A_long {
        const NONE            = ae_sys::PF_ChangeFlag_NONE            as ae_sys::A_long;
        const CHANGED_VALUE   = ae_sys::PF_ChangeFlag_CHANGED_VALUE   as ae_sys::A_long;
        const RESERVED        = ae_sys::PF_ChangeFlag_RESERVED        as ae_sys::A_long;
        const SET_TO_VARY     = ae_sys::PF_ChangeFlag_SET_TO_VARY     as ae_sys::A_long;
        const SET_TO_CONSTANT = ae_sys::PF_ChangeFlag_SET_TO_CONSTANT as ae_sys::A_long;
    }
}

bitflags! {
    pub struct ValueDisplayFlag: u16 {
        const NONE = ae_sys::PF_ValueDisplayFlag_NONE as u16;
        const PERCENT = ae_sys::PF_ValueDisplayFlag_PERCENT as u16;
        const PIXEL = ae_sys::PF_ValueDisplayFlag_PIXEL as u16;
        const RESERVED = ae_sys::PF_ValueDisplayFlag_RESERVED1 as u16;
        const REVERSE = ae_sys::PF_ValueDisplayFlag_REVERSE as u16;
    }
}

//define_param_wrapper!(ButtonDef, PF_ButtonDef, button_def);

#[repr(C)]
#[derive(Clone)]
pub struct ButtonDef(ae_sys::PF_ButtonDef, CString);

//define_param_value_str_wrapper!(ButtonDef, button_def);
//define_param_value_desc_wrapper!(ButtonDef, button_def);

impl ButtonDef {
    pub fn new() -> Self {
        Self(
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            CString::new("").unwrap(),
        )
    }

    pub fn from_raw(def: ae_sys::PF_ButtonDef) -> Self {
        Self(def, CString::new("").unwrap())
    }

    pub fn label(mut self, label: &str) -> Self {
        self.1 = CString::new(label).unwrap();
        self.0.u.namesptr = self.1.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_BUTTON == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.button_d }, unsafe {
                CString::from_raw(param.param_def_boxed.u.button_d.u.namesptr as _)
            }))
        } else {
            None
        }
    }

    pub fn into_raw(def: ButtonDef) -> ae_sys::PF_ButtonDef {
        let ret = def.0;
        std::mem::forget(def);
        ret
    }
}
impl Into<Param> for ButtonDef {
    fn into(self) -> Param {
        Param::Button(self)
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PopupDef(ae_sys::PF_PopupDef, CString);

define_param_basic_wrapper!(PopupDef, PF_PopupDef, i32, u16);
//define_param_value_str_wrapper!(PopupDef, popup_def);
//define_param_value_desc_wrapper!(PopupDef, popup_def);

impl PopupDef {
    pub fn new() -> Self {
        Self(
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            CString::new("").unwrap(),
        )
    }

    pub fn from_raw(def: ae_sys::PF_PopupDef) -> Self {
        Self(def, CString::new("").unwrap())
    }

    pub fn into_raw(def: Self) -> ae_sys::PF_PopupDef {
        def.0
    }

    pub fn names(&mut self, names: Vec<&str>) -> &mut Self {
        // Build a string in the format "list|of|choices|", the
        // format Ae expects. Ae ignores the trailing '|'.
        let mut names_tmp = String::new();
        names
            .iter()
            .for_each(|s| write!(names_tmp, "{}|", *s).unwrap());
        self.1 = CString::new(names_tmp).unwrap();
        self.0.u.namesptr = self.1.as_ptr();
        self.0.num_choices = names.len().try_into().unwrap();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_POPUP == param.param_def_boxed.param_type {
            Some(Self(
                unsafe { param.param_def_boxed.u.pd },
                CString::new("").unwrap(),
            ))
        } else {
            None
        }
    }

    //pub fn check_out()

    pub fn value(&self) -> u16 {
        self.0.value as u16
    }
}
impl Into<Param> for PopupDef {
    fn into(self) -> Param {
        Param::Popup(self)
    }
}

define_param_wrapper!(AngleDef, PF_AngleDef);
//define_param_value_str_wrapper!(AngleDef, angle_def);
//define_param_value_desc_wrapper!(AngleDef, angle_def);

impl AngleDef {
    pub fn set_value(mut self, value: f32) -> Self {
        self.0.value = Fixed::from(value).into();
        self
    }

    pub fn set_default(mut self, default: f32) -> Self {
        self.0.dephault = Fixed::from(default).into();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_ANGLE == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.ad }))
        } else {
            None
        }
    }

    pub fn value(&self) -> f32 {
        Fixed::from(self.0.value).into()
    }
}
impl Into<Param> for AngleDef {
    fn into(self) -> Param {
        Param::Angle(self)
    }
}

define_param_wrapper!(ColorDef, PF_ColorDef);

impl ColorDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_COLOR == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.cd }))
        } else {
            None
        }
    }

    pub fn value(&self) -> Pixel8 {
        Pixel8::from(self.0.value)
    }

    pub fn set_value(&mut self, value: Pixel8) -> &mut Self {
        self.0.value = ae_sys::PF_Pixel::from(value);
        self
    }

    pub fn default(&mut self, default: Pixel8) -> &mut Self {
        self.0.dephault = ae_sys::PF_Pixel::from(default);
        self
    }
}
impl Into<Param> for ColorDef {
    fn into(self) -> Param {
        Param::Color(self)
    }
}

define_param_wrapper!(SliderDef, PF_SliderDef);
define_param_basic_wrapper!(SliderDef, PF_SliderDef, i32, i32);
define_param_valid_min_max_wrapper!(SliderDef, i32);
define_param_slider_min_max_wrapper!(SliderDef, i32);
define_param_value_str_wrapper!(SliderDef);
define_param_value_desc_wrapper!(SliderDef);

impl SliderDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_SLIDER == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.sd }))
        } else {
            None
        }
    }

    pub fn value(&self) -> i32 {
        self.0.value
    }
}
impl Into<Param> for SliderDef {
    fn into(self) -> Param {
        Param::Slider(self)
    }
}
/* Adobe recommends not useing fixed (point) sliders any more and
 * instead to use float sliders instead.

    define_param_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def);
    define_param_basic_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32, i32);
    define_param_slider_min_max_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32);
    define_param_value_str_wrapper!(FixedSliderDef, slider_def);
    define_param_value_desc_wrapper!(FixedSliderDef, slider_def);

    impl FixedSliderDef {
        pub fn precision<'a>(&'a mut self, precision: u16) -> &'a mut FixedSliderDef {
            self.slider_def.precision = precision as i16;
            self
        }

        pub fn display_flags<'a>(&'a mut self, display_flags: ValueDisplayFlag) -> &'a mut FixedSliderDef {
            self.slider_def.display_flags = display_flags.bits() as i16;
            self
        }

 *
}*/

// Float Slider
define_param_wrapper!(FloatSliderDef, PF_FloatSliderDef);
define_param_basic_wrapper!(FloatSliderDef, PF_FloatSliderDef, f64, f32);
define_param_valid_min_max_wrapper!(FloatSliderDef, f32);
define_param_slider_min_max_wrapper!(FloatSliderDef, f32);
define_param_value_desc_wrapper!(FloatSliderDef);

impl FloatSliderDef {
    pub fn display_flags(mut self, display_flags: ValueDisplayFlag) -> Self {
        self.0.display_flags = display_flags.bits() as i16;
        self
    }

    pub fn precision(mut self, precision: u8) -> Self {
        self.0.precision = precision as i16;
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_FLOAT_SLIDER == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.fs_d }))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0.value
    }
}
impl Into<Param> for FloatSliderDef {
    fn into(self) -> Param {
        Param::FloatSlider(self)
    }
}

// Checkbox

// PF_CheckBoxDef does not implement Debug trait so we can't use
// the define_param_basic_wrapper!() macro.
#[repr(C)]
#[derive(Clone)]
pub struct CheckBoxDef(ae_sys::PF_CheckBoxDef, CString);

impl CheckBoxDef {
    pub fn new() -> Self {
        Self(
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            CString::new("").unwrap(),
        )
    }

    pub fn from_raw(def: ae_sys::PF_CheckBoxDef) -> Self {
        Self(def, CString::new("").unwrap())
    }

    pub fn into_raw(def: Self) -> ae_sys::PF_CheckBoxDef {
        let ret = def.0;
        std::mem::forget(def);
        ret
    }

    pub fn label(mut self, label: &str) -> Self {
        self.1 = CString::new(label).unwrap();
        self.0.u.nameptr = self.1.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_CHECKBOX == param.param_def_boxed.param_type {
            Some(Self(unsafe { param.param_def_boxed.u.bd }, unsafe {
                CString::from_raw(param.param_def_boxed.u.bd.u.nameptr as _)
            }))
        } else {
            None
        }
    }

    pub fn value(&self) -> bool {
        self.0.value != 0
    }
}
impl Into<Param> for CheckBoxDef {
    fn into(self) -> Param {
        Param::CheckBox(self)
    }
}

define_param_basic_wrapper!(CheckBoxDef, PF_CheckBoxDef, i32, bool);
pub struct ArbitraryDef(ae_sys::PF_ArbitraryDef);

impl ArbitraryDef {
    pub fn new() -> Self {
        Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
    }

    pub fn into_raw(def: ArbitraryDef) -> ae_sys::PF_ArbitraryDef {
        def.0
    }

    pub fn from_raw(def: ae_sys::PF_ArbitraryDef) -> Self {
        Self(def)
    }

    pub fn finalize(self) -> Self {
        self
    }

    pub fn set_value(mut self, value_handle: FlatHandle) -> Self {
        self.0.value = FlatHandle::into_raw(value_handle);
        self
    }

    pub fn default(mut self, value_handle: FlatHandle) -> Self {
        self.0.dephault = FlatHandle::into_raw(value_handle);
        self
    }

    pub fn refcon(mut self, refcon: usize) -> Self {
        self.0.refconPV = refcon as _;
        self
    }

    pub fn has_refcon(&self, refcon: usize) -> bool {
        self.0.refconPV == refcon as _
    }

    pub fn value(&self) -> Result<FlatHandle, Error> {
        FlatHandle::from_raw(self.0.value)
    }

    pub fn value_owned(&self) -> Result<FlatHandle, Error> {
        FlatHandle::from_raw_owned(self.0.value)
    }
}
impl Into<Param> for ArbitraryDef {
    fn into(self) -> Param {
        Param::Arbitrary(self)
    }
}

pub trait ArbitraryData<T> {
    fn default() -> T;
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
    pub fn id(&self) -> isize {
        self.as_ref().id as _
    }

    pub fn refcon(&self) -> usize {
        unsafe { std::mem::transmute(self.as_ref().u.new_func_params.refconPV) }
    }

    pub fn which_function(&self) -> u32 {
        self.as_ref().which_function as _
    }

    pub fn dispatch<T: ArbitraryData<T> + DeserializeOwned + Serialize + PartialEq + PartialOrd>(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.as_ref().which_function as _ {
            ae_sys::PF_Arbitrary_NEW_FUNC => unsafe {
                //println!("NEW_FUNC");
                // Create a new instance, serialize it to a Vec<u8>
                // pass it to a FlatHandle and turn that into a raw
                // Ae handle that we stash in the PF_ArbParamsExtra
                // struct wrapper.
                self.as_ref()
                    .u
                    .new_func_params
                    .arbPH
                    .write(FlatHandle::into_raw(FlatHandle::new(bincode::serialize(
                        &T::default(),
                    )?)?));
            },

            ae_sys::PF_Arbitrary_DISPOSE_FUNC => {
                //println!("DISPOSE_FUNC");

                // Create a new handle from the raw Ae handle. This
                // disposes then handle when it goes out of scope
                // and is dropped just after.
                assert!(unsafe { !self.as_ref().u.dispose_func_params.arbH.is_null() });

                FlatHandle::from_raw_owned(unsafe { self.as_ref().u.dispose_func_params.arbH })?;
            }

            ae_sys::PF_Arbitrary_COPY_FUNC => unsafe {
                //println!("COPY_FUNC");
                // Create a new handle wraper from the sources,
                // get a referece to that as a slice create a new
                // handle from that and write that to the
                // destination pointer.

                assert!(!self.as_ref().u.copy_func_params.src_arbH.is_null());

                let src_handle = FlatHandle::from_raw(self.as_ref().u.copy_func_params.src_arbH)?;

                let _src_handle_lock = src_handle.lock()?;

                self.as_ref()
                    .u
                    .copy_func_params
                    .dst_arbPH
                    .write(FlatHandle::into_raw(FlatHandle::new(
                        src_handle.as_slice().unwrap(),
                    )?));
            },

            ae_sys::PF_Arbitrary_FLAT_SIZE_FUNC => unsafe {
                //println!("FLAT_SIZE_FUNC");

                let handle = FlatHandle::from_raw(self.as_ref().u.flat_size_func_params.arbH)?;

                self.as_ref()
                    .u
                    .flat_size_func_params
                    .flat_data_sizePLu
                    .write(handle.size() as _);
            },

            ae_sys::PF_Arbitrary_FLATTEN_FUNC => {
                //println!("FLATTEN_FUNC");

                let handle = FlatHandle::from_raw(unsafe { self.as_ref().u.flatten_func_params.arbH })?;

                let _handle_lock = handle.lock()?;

                debug_assert!(
                    handle.size() <= unsafe { self.as_ref().u.flatten_func_params.buf_sizeLu } as _
                );

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        handle.as_ptr(),
                        self.as_ref().u.flatten_func_params.flat_dataPV as _,
                        handle.size(),
                    );
                }
            }

            ae_sys::PF_Arbitrary_UNFLATTEN_FUNC => unsafe {
                //println!("UNFLATTEN_FUNC");

                self.as_ref()
                    .u
                    .unflatten_func_params
                    .arbPH
                    .write(FlatHandle::into_raw(FlatHandle::new(CVec::<u8>::new(
                        self.as_ref().u.unflatten_func_params.flat_dataPV as *mut u8,
                        self.as_ref().u.unflatten_func_params.buf_sizeLu as _,
                    ))?));
            },

            ae_sys::PF_Arbitrary_INTERP_FUNC => unsafe {
                //println!("INTERP_FUNC");

                let left = FlatHandle::from_raw(self.as_ref().u.interp_func_params.left_arbH)?;

                let _left_lock = left.lock()?;

                let right = FlatHandle::from_raw(self.as_ref().u.interp_func_params.right_arbH)?;

                let _right_lock = right.lock()?;

                self.as_ref()
                    .u
                    .interp_func_params
                    .interpPH
                    .write(FlatHandle::into_raw(FlatHandle::new(bincode::serialize(
                        &bincode::deserialize::<T>(left.as_slice().unwrap())?.interpolate(
                            &bincode::deserialize::<T>(right.as_slice().unwrap())?,
                            self.as_ref().u.interp_func_params.tF,
                        ),
                    )?)?));
            },

            ae_sys::PF_Arbitrary_COMPARE_FUNC => {
                //println!("COMPARE_FUNC");

                let handle_a =
                    FlatHandle::from_raw(unsafe { self.as_ref().u.compare_func_params.a_arbH })?;

                let _handle_a_lock = handle_a.lock()?;

                let a = bincode::deserialize::<T>(handle_a.as_slice().unwrap())?;

                let handle_b =
                    FlatHandle::from_raw(unsafe { self.as_ref().u.compare_func_params.b_arbH })?;

                let _handle_b_lock = handle_b.lock()?;

                let b = bincode::deserialize::<T>(handle_b.as_slice().unwrap())?;

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
                //println!("PRINT_SIZE_FUNC");

                let handle = FlatHandle::from_raw(self.as_ref().u.print_size_func_params.arbH)?;

                let _handle_lock = handle.lock()?;

                self.as_ref().u.print_size_func_params.print_sizePLu.write(
                    (serde_json::to_string(&bincode::deserialize::<T>(
                        handle.as_slice().unwrap(),
                    )?)?
                    // The len + terminating nul byte.
                    .len()
                        + 1) as _,
                );
            },

            // Print arbitrary data into a string as JSON.
            // Note that we could use any text-based serializer here.
            ae_sys::PF_Arbitrary_PRINT_FUNC => {
                //println!("PRINT_FUNC");

                let handle = FlatHandle::from_raw(unsafe { self.as_ref().u.print_func_params.arbH })?;

                let _handle_lock = handle.lock()?;

                let string =
                    serde_json::to_string(&bincode::deserialize::<T>(handle.as_slice().unwrap())?)?;

                if string.len() < unsafe { self.as_ref().u.print_func_params.print_sizeLu } as _
                    && unsafe { self.as_ref().u.print_func_params.print_flags } == 0
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            string.as_ptr(),
                            self.as_ref().u.print_func_params.print_bufferPC as _,
                            string.len(),
                        );
                        // Nul-terminate the C string.
                        self.as_ref()
                            .u
                            .print_func_params
                            .print_bufferPC
                            .add(string.len())
                            .write(0);
                    }
                }
            }
            ae_sys::PF_Arbitrary_SCAN_FUNC => unsafe {
                //println!("SCAN_FUNC");

                self.as_ref()
                    .u
                    .scan_func_params
                    .arbPH
                    .write(FlatHandle::into_raw(FlatHandle::new(
                        bincode::serialize::<T>(&serde_json::from_str(
                            CStr::from_ptr(self.as_ref().u.scan_func_params.bufPC).to_str()?,
                        )?)?,
                    )?));
            },
            _ => {
                return Err(Box::new(Error::Generic));
            }
        }
        Ok(())
    }
}

#[repr(C)]
//#[derive(Clone)]
pub enum Param {
    Popup(PopupDef),
    Angle(AngleDef),
    CheckBox(CheckBoxDef),
    Slider(SliderDef),
    FloatSlider(FloatSliderDef),
    Color(ColorDef),
    Button(ButtonDef),
    //FixedSliderDef(FixedSliderDef),
    Arbitrary(ArbitraryDef),
}

#[derive(Clone)]
pub struct ParamDef {
    param_def_boxed: std::mem::ManuallyDrop<Box<ae_sys::PF_ParamDef>>,
    drop: bool,
    index: Option<i32>,
    in_data_ptr: *const ae_sys::PF_InData,
}

fn zeroed_raw_param_def() -> ae_sys::PF_ParamDef {
    ae_sys::PF_ParamDef {
        // all fields same size, no uninitialized memory
        uu: unsafe { std::mem::zeroed() },
        ui_flags: 0,
        ui_width: 0,
        ui_height: 0,
        param_type: 0,
        name: [0; 32],
        flags: 0,
        unused: 0,
        u: unsafe { std::mem::zeroed() },
    }
}

impl ParamDef {
    pub fn new_from_ptr(in_data_ptr: *const ae_sys::PF_InData) -> Self {
        Self {
            param_def_boxed: std::mem::ManuallyDrop::new(Box::new(zeroed_raw_param_def())),
            drop: true,
            in_data_ptr,
            index: None,
        }
    }
    pub fn new(in_data_handle: InData) -> Self {
        Self {
            param_def_boxed: std::mem::ManuallyDrop::new(Box::new(zeroed_raw_param_def())),
            drop: true,
            in_data_ptr: in_data_handle.as_ptr(),
            index: None,
        }
    }
    pub fn as_ptr(&self) -> *const ae_sys::PF_ParamDef {
        self.param_def_boxed.as_ref()
    }

    pub fn update_param_ui(&self) {
        if let Ok(suite) = pf::suites::ParamUtils::new() {
            if let Some(index) = self.index {
                suite.update_param_ui(unsafe { (*self.in_data_ptr).effect_ref }, index, self);
            }
        } else {
            log::error!("failed to get params suite");
        }
    }
    pub fn keyframe_count(&self) -> i32 {
        (|| -> Option<i32> {
            pf::suites::ParamUtils::new()
                .ok()?
                .keyframe_count(unsafe { (*self.in_data_ptr).effect_ref }, self.index?)
                .ok()
        })()
        .unwrap_or(0)
    }

    pub fn from_raw(
        in_data_ptr: *const ae_sys::PF_InData,
        param_def: *mut ae_sys::PF_ParamDef,
        index: Option<i32>,
    ) -> Self {
        debug_assert!(!param_def.is_null());
        Self {
            param_def_boxed: unsafe { std::mem::ManuallyDrop::new(Box::from_raw(param_def)) },
            drop: false,
            in_data_ptr,
            index,
        }
    }

    pub fn add(&mut self, index: i32) {
        unsafe {
            (*self.in_data_ptr).inter.add_param.unwrap()(
                (*self.in_data_ptr).effect_ref,
                index,
                &mut **self.param_def_boxed,
            );
        }
        // Parameters we just added are not checked out
        // so they do not need to be checked in.
        self.drop = false;
        if index != -1 {
            self.index = Some(index);
        }
    }

    pub fn checkout(
        in_data_handle: InData,
        index: i32,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> ParamDef {
        let mut param_def_boxed = std::mem::ManuallyDrop::new(Box::new(zeroed_raw_param_def()));
        let in_data_ptr = in_data_handle.as_ptr();
        unsafe {
            (*in_data_ptr).inter.checkout_param.unwrap()(
                (*in_data_ptr).effect_ref,
                index,
                what_time,
                time_step,
                time_scale,
                &mut **param_def_boxed,
            );
        }
        ParamDef {
            param_def_boxed,
            drop: true,
            in_data_ptr,
            index: Some(index),
        }
    }

    pub fn do_not_checkin(&mut self) {
        self.drop = false;
    }

    pub fn param(&mut self, param: Param) -> &mut ParamDef {
        match param {
            Param::Popup(pd) => {
                self.param_def_boxed.u.pd = PopupDef::into_raw(pd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_POPUP;
            }
            Param::Angle(ad) => {
                self.param_def_boxed.u.ad = AngleDef::into_raw(ad);
                self.param_def_boxed.param_type = ae_sys::PF_Param_ANGLE;
            }
            Param::CheckBox(bd) => {
                self.param_def_boxed.u.bd = CheckBoxDef::into_raw(bd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_CHECKBOX;
            }
            Param::Color(cd) => {
                self.param_def_boxed.u.cd = ColorDef::into_raw(cd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_COLOR;
            }
            Param::Slider(sd) => {
                self.param_def_boxed.u.sd = SliderDef::into_raw(sd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_SLIDER;
            }
            Param::FloatSlider(fs_d) => {
                self.param_def_boxed.u.fs_d = FloatSliderDef::into_raw(fs_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_FLOAT_SLIDER;
            } /* Param::FixedSliderDef(sd) => { */
            //self.param_def_boxed.u.fd = FixedSliderDef::into_raw(sd);
            //self.param_def_boxed.param_type = ae_sys::PF_Param_FIX_SLIDER;
            //}
            Param::Button(button_d) => {
                self.param_def_boxed.u.button_d = ButtonDef::into_raw(button_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_BUTTON;
            }
            Param::Arbitrary(arb_d) => {
                self.param_def_boxed.u.arb_d = ArbitraryDef::into_raw(arb_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_ARBITRARY_DATA;
            }
        }
        self
    }

    pub fn to_param(&self) -> Param {
        match self.param_def_boxed.param_type {
            ae_sys::PF_Param_ANGLE => {
                Param::Angle(AngleDef::from_raw(unsafe { self.param_def_boxed.u.ad }))
            }
            ae_sys::PF_Param_ARBITRARY_DATA => Param::Arbitrary(ArbitraryDef::from_raw(unsafe {
                self.param_def_boxed.u.arb_d
            })),
            ae_sys::PF_Param_BUTTON => Param::Button(ButtonDef::from_raw(unsafe {
                self.param_def_boxed.u.button_d
            })),
            ae_sys::PF_Param_CHECKBOX => {
                Param::CheckBox(CheckBoxDef::from_raw(unsafe { self.param_def_boxed.u.bd }))
            }
            ae_sys::PF_Param_COLOR => {
                Param::Color(ColorDef::from_raw(unsafe { self.param_def_boxed.u.cd }))
            }
            ae_sys::PF_Param_FLOAT_SLIDER => Param::FloatSlider(FloatSliderDef::from_raw(unsafe {
                self.param_def_boxed.u.fs_d
            })),
            ae_sys::PF_Param_POPUP => {
                Param::Popup(PopupDef::from_raw(unsafe { self.param_def_boxed.u.pd }))
            }
            ae_sys::PF_Param_SLIDER => {
                Param::Slider(SliderDef::from_raw(unsafe { self.param_def_boxed.u.sd }))
            }
            _ => unreachable!(),
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(
            self.param_def_boxed.param_type,
            ae_sys::PF_Param_ANGLE
                | ae_sys::PF_Param_ARBITRARY_DATA
                | ae_sys::PF_Param_BUTTON
                | ae_sys::PF_Param_CHECKBOX
                | ae_sys::PF_Param_COLOR
                | ae_sys::PF_Param_FLOAT_SLIDER
                | ae_sys::PF_Param_GROUP_START
                | ae_sys::PF_Param_GROUP_END
                | ae_sys::PF_Param_POPUP
                | ae_sys::PF_Param_SLIDER
        )
    }
    pub fn param_type(&self) -> ParamType {
        unsafe { std::mem::transmute(self.param_def_boxed.param_type) }
    }

    pub fn set_param_type(&mut self, param_type: ParamType) -> &mut Self {
        self.param_def_boxed.param_type = param_type as i32;
        self
    }

    pub unsafe fn layer_def(&mut self) -> *mut ae_sys::PF_LayerDef {
        &mut self.param_def_boxed.u.ld
    }
    pub fn name(&mut self, name: &str) -> &mut Self {
        assert!(name.len() < 32);
        let name_cstr = CString::new(name).unwrap();
        let name_slice = name_cstr.to_bytes_with_nul();
        self.param_def_boxed.name[0..name_slice.len()]
            .copy_from_slice(unsafe { std::mem::transmute(name_slice) });
        self
    }

    pub fn ui_flags(&mut self, flags: ParamUIFlags) -> &mut Self {
        self.param_def_boxed.ui_flags = flags.bits() as _;
        self
    }
    pub fn get_ui_flags(&self) -> ParamUIFlags {
        ParamUIFlags::from_bits(self.param_def_boxed.ui_flags).unwrap()
    }

    pub fn ui_width(&mut self, width: u16) -> &mut Self {
        self.param_def_boxed.ui_width = width as _;
        self
    }

    pub fn ui_height(&mut self, height: u16) -> &mut Self {
        self.param_def_boxed.ui_height = height as _;
        self
    }

    pub fn flags(&mut self, flags: ParamFlag) -> &mut Self {
        self.param_def_boxed.flags = flags.bits() as _;
        self
    }

    pub fn change_flags(&mut self, change_flags: ChangeFlag) -> &mut Self {
        self.param_def_boxed.uu.change_flags = change_flags.bits() as _;
        self
    }

    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.param_def_boxed.uu.id = id;
        self
    }

    pub fn set_value_has_changed(&mut self) -> &mut Self {
        self.param_def_boxed.uu.change_flags = ChangeFlag::CHANGED_VALUE.bits();
        self
    }
}

impl Drop for ParamDef {
    fn drop(&mut self) {
        if self.drop {
            unsafe {
                (*self.in_data_ptr).inter.checkin_param.unwrap()(
                    (*self.in_data_ptr).effect_ref,
                    // ManuallyDrop ensures we do not double free the memory
                    // after passing the pointer to the box contents to the FFI.
                    &mut **self.param_def_boxed,
                );
            }
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.param_def_boxed);
            }
        }
    }
}
impl Debug for ParamDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParamDef")
            .field("type", &self.param_type())
            .field("drop", &self.drop)
            .field("in_data_ptr", &self.in_data_ptr)
            .finish()
    }
}

macro_rules! define_get_param {
    ($name:tt, $enm:ident, $type:ty) => {
        paste::item! {
            pub fn [<get_ $name>](&self, type_: P, time: Option<i32>, time_step: Option<i32>, time_scale: Option<u32>) -> Option<$type> {
                let param = self.get_param_def(type_, time, time_step, time_scale);
                match param.map(|x| x.to_param()) {
                    Some(Param::$enm(pd)) => Some(pd),
                    _ => panic!("Invalid parameter type, expected {}, got {}. type: {type_:?}, params_len: {:?}, map: {:?}", $name, self.get_param_def(type_, time, time_step, time_scale).map(|x| format!("{:?}", x.param_type())).unwrap_or("Invalid".to_owned()), self.params.len(), self.map.borrow()),
                }
            }
        }
    };
}

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
pub struct Parameters<P: Eq + PartialEq + Hash + Copy + Debug> {
    num_params: usize,
    in_data: *const ae_sys::PF_InData,
    pub map: Rc<RefCell<HashMap<P, usize>>>,
    params: Vec<ParamDef>,
}
impl<P: Eq + PartialEq + Hash + Copy + Debug> Default for Parameters<P> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
impl<P: Eq + PartialEq + Hash + Copy + Debug> Parameters<P> {
    pub fn len(&self) -> usize {
        self.map.borrow().len()
    }
    pub fn set_in_data(&mut self, in_data: *const ae_sys::PF_InData) {
        self.in_data = in_data;
    }
    pub fn in_data(&self) -> InData {
        InData::from_raw(self.in_data)
    }
    pub fn new(map: Rc<RefCell<HashMap<P, usize>>>) -> Self {
        Self {
            in_data: std::ptr::null(),
            num_params: 1,
            map,
            params: Vec::new(),
        }
    }
    pub fn with_params(
        in_data: *const ae_sys::PF_InData,
        params: *mut *mut ae_sys::PF_ParamDef,
        map: Rc<RefCell<HashMap<P, usize>>>,
        num_params: usize,
    ) -> Self {
        Self {
            in_data,
            params: if params.is_null() {
                Vec::new()
            } else {
                let params = unsafe { std::slice::from_raw_parts(params, num_params) };
                params
                    .iter()
                    .enumerate()
                    .map(|(i, p)| ParamDef::from_raw(in_data, *p, Some(i as i32)))
                    .collect::<Vec<_>>()
            },
            num_params,
            map,
        }
    }

    fn param_id(type_: P) -> i32 {
        use hash32::Murmur3Hasher;
        use std::hash::Hasher;
        let mut hasher = Murmur3Hasher::default();
        format!("{type_:?}").hash(&mut hasher);
        hasher.finish() as i32
    }

    pub fn add_group<F: FnOnce(&mut Self)>(
        &mut self,
        type_start: P,
        type_end: P,
        name: &str,
        inner_cb: F,
    ) {
        assert!(!self.in_data.is_null());

        let mut param_def = ParamDef::new_from_ptr(self.in_data);
        param_def.name(name);
        param_def.set_param_type(ParamType::GroupStart);
        param_def.set_id(Self::param_id(type_start));
        param_def.add(-1);
        self.map.borrow_mut().insert(type_start, self.num_params);
        self.num_params += 1;

        inner_cb(self);

        let mut param_def = ParamDef::new_from_ptr(self.in_data);
        param_def.set_param_type(ParamType::GroupEnd);
        param_def.set_id(Self::param_id(type_end));
        param_def.add(-1);
        self.map.borrow_mut().insert(type_end, self.num_params);
        self.num_params += 1;
    }

    pub fn add_param(&mut self, type_: P, name: &str, def: impl Into<Param>) {
        assert!(!self.in_data.is_null());

        let mut param_def = ParamDef::new_from_ptr(self.in_data);
        param_def.name(name);
        param_def.param(def.into());
        param_def.set_id(Self::param_id(type_));
        param_def.add(-1);
        self.map.borrow_mut().insert(type_, self.num_params);
        self.num_params += 1;
    }

    pub fn add_param_with_flags(
        &mut self,
        type_: P,
        name: &str,
        def: impl Into<Param>,
        flags: ParamFlag,
        ui_flags: ParamUIFlags,
    ) {
        assert!(!self.in_data.is_null());

        let mut param_def = ParamDef::new_from_ptr(self.in_data);
        param_def.name(name);
        param_def.param(def.into());
        param_def.set_id(Self::param_id(type_));
        param_def.flags(flags);
        param_def.ui_flags(ui_flags);
        param_def.add(-1);
        self.map.borrow_mut().insert(type_, self.num_params);
        self.num_params += 1;
    }

    pub fn add_param_customized<F: FnOnce(&mut ParamDef) -> i32>(
        &mut self,
        type_: P,
        name: &str,
        def: impl Into<Param>,
        cb: F,
    ) -> ParamDef {
        assert!(!self.in_data.is_null());

        let mut param_def = ParamDef::new_from_ptr(self.in_data);
        param_def.name(name);
        param_def.param(def.into());
        param_def.set_id(Self::param_id(type_));
        let mut index = cb(&mut param_def);
        param_def.add(index);
        if index == -1 {
            index = self.num_params as i32;
        }
        self.map.borrow_mut().insert(type_, index as usize);
        self.num_params += 1;
        param_def
    }

    define_get_param!("popup", Popup, PopupDef);
    define_get_param!("angle", Angle, AngleDef);
    define_get_param!("checkbox", CheckBox, CheckBoxDef);
    define_get_param!("color", Color, ColorDef);
    define_get_param!("slider", Slider, SliderDef);
    define_get_param!("float_slider", FloatSlider, FloatSliderDef);
    define_get_param!("button", Button, ButtonDef);
    define_get_param!("arbitrary", Arbitrary, ArbitraryDef);

    pub fn get_param_def(
        &self,
        type_: P,
        time: Option<i32>,
        time_step: Option<i32>,
        time_scale: Option<u32>,
    ) -> Option<Cow<ParamDef>> {
        let index = self.index_for_type(type_)?;
        if self.params.is_empty() || time.is_some() {
            let in_data = self.in_data();
            let param = ParamDef::checkout(
                in_data,
                index as i32,
                time.unwrap_or(in_data.current_time()),
                time_step.unwrap_or(in_data.time_step()),
                time_scale.unwrap_or(in_data.time_scale()),
            );
            if !param.is_valid() {
                return None;
            }
            return Some(Cow::Owned(param));
        }
        Some(Cow::Borrowed(self.params.get(index)?))
    }
    pub fn get_param_def_mut(
        &mut self,
        type_: P,
        time: Option<i32>,
        time_step: Option<i32>,
        time_scale: Option<u32>,
    ) -> Option<mucow::MuCow<ParamDef>> {
        let index = self.index_for_type(type_)?;
        if self.params.is_empty() || time.is_some() {
            let in_data = self.in_data();
            let param = ParamDef::checkout(
                in_data,
                index as i32,
                time.unwrap_or(in_data.current_time()),
                time_step.unwrap_or(in_data.time_step()),
                time_scale.unwrap_or(in_data.time_scale()),
            );
            if !param.is_valid() {
                return None;
            }
            return Some(mucow::MuCow::Owned(param));
        }
        Some(mucow::MuCow::Borrowed(self.params.get_mut(index)?))
    }

    pub fn num_params(&self) -> usize {
        self.num_params
    }

    pub fn index_for_type(&self, type_: P) -> Option<usize> {
        self.map.borrow().get(&type_).copied()
    }
    pub fn type_for_index(&self, index: usize) -> P {
        *self
            .map
            .borrow()
            .iter()
            .find(|(_, v)| **v == index)
            .unwrap()
            .0
    }
}
