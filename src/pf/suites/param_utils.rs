use crate::*;

define_suite!(
    /// A parameter's value (not just UI) can be modified during `Command::UserChangedParam` and during `Command::Event` (*PF_Event_DO_CLICK*, *PF_Event_DRAG*, & *PF_Event_KEYDOWN*).
    /// After Effects will not honor changes made at other times.
    ///
    /// When changing parameter *values* (and not just the UI), modify the original parameter, and set ``PF_Paramdef.uu.change_flags`` to ``PF_ChangeFlag_CHANGED_VALUE``.
    ///
    /// This change will be also update the UI, and will be undoable by the user. Note that ``PF_ChangeFlag_CHANGED_VALUE`` isn't supported for layer parameters.
    ///
    /// This suite is provided to give effect plug-ins some access to their parameter streams, without requiring AEGP suite usage.
    /// At least some of these functions are provided by several third-party hosts. These functions are especially handy for effects with supervised parameters.
    ///
    ///
    /// ### [`current_state()`](Self::current_state) / [`are_states_identical()`](Self::are_states_identical)
    /// This API lets you determine if a set of your inputs (either layers, other properties, or both) are different between when you first called [`current_state()`](Self::current_state) and a current call, so it can/ be used for caching.
    /// You can specify a range of time to consider or all of time.
    ///
    /// For effects that do simulation across time and therefore set PF_OutFlag2_AUTOMATIC_WIDE_TIME_INPUT, when you ask about a time range, it will be expanded to include any times needed to produce that range.
    ///
    /// IMPORTANT: as of 13.5 to avoid threading deadlock problems, [`current_state()`](Self::current_state) returns a random state
    /// if used in the context of UPDATE_PARAMS_UI only. In other selectors this will behave normally.
    ///
    /// # Parameters & Floating Point Values
    ///
    /// We have something to admit to you; for years, even though we've given you 8 bit color values,
    /// we've internally used floating point representations behind your back.
    ///
    /// That's right, even with over-bright colors, we'd only ever tell you '255, 255, 255'. Yeah, right.
    ///
    /// Use the [`AngleParamSuite`](crate::pf::suites::AngleParamSuite) to get floating point values for angle parameters.
    ///
    /// Use the [`ColorParamSuite`](crate::pf::suites::ColorParamSuite) to get floating point values for color parameters.
    ///
    /// Use the [`PointParamSuite`](crate::pf::suites::PointParamSuite) to get floating point values for point parameters.
    ParamUtilsSuite,
    PF_ParamUtilsSuite3,
    kPFParamUtilsSuite,
    kPFParamUtilsSuiteVersion3
);

impl ParamUtilsSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Force After Effects to refresh the parameter's UI, in the effect controls palette.
    ///
    /// Starting in CC 2014, After Effects will now honor a change to a custom UI height. Simply change the ui_height of your custom UI PF_ParamDef and then call PF_UpdateParamUI.
    /// The effect's custom UI height will be updated in the Effect Control Window.
    ///
    /// Starting in CS6, when a plug-in disables a parameter, we now save that state in the UI flags so that the plug-in can check that flag in the future to see if it is disabled.
    ///
    /// NOTE: Never pass param\[0\] to this function.
    ///
    /// You can call this function for each param whose UI settings you want to change when handling a `Command::UserChangedParam` or `Command::UpdateParamsUi`.
    /// These changes are cosmetic only, and don't go into the undo buffer.
    ///
    /// The ONLY fields that can be changed in this way are:
    ///     PF_ParamDef
    ///         ui_flags: `PF_PUI_ECW_SEPARATOR`, `PF_PUI_DISABLED` only (and `PF_PUI_INVISIBLE` in Premiere).
    ///         ui_width
    ///         ui_height
    ///         name
    ///         flags: `PF_ParamFlag_COLLAPSE_TWIRLY` only
    ///     PF_ParamDefUnion:
    ///         slider_min, slider_max, precision, display_flags of any slider type
    /// For `PF_PUI_STD_CONTROL_ONLY` params, you can also change the value field by setting `PF_ChangeFlag_CHANGED_VALUE` before returning.
    /// But you are not allowed to change the value during `PF_Cmd_UPDATE_PARAMS_UI`.
    pub fn update_param_ui(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, param_def: &ParamDef) -> Result<(), Error> {
        call_suite_fn!(self, PF_UpdateParamUI, effect_ref.as_ptr(), param_index, param_def.as_ref())
    }

    /// This API, combined with [`are_states_identical()`](Self::are_states_identical) below, lets you determine if a set of inputs (either layers, other properties, or both)
    /// are different between when you first called [`current_state()`](Self::current_state) and a current call, so it can be used for caching.
    /// You can specify a range of time to consider or all of time.
    ///
    /// Updated in CS6 to add `param_index`, `start`, and `duration`. Pre-defined constants for `param_index` are as follows:
    ///
    ///   - [`PARAM_INDEX_CHECK_ALL`] - check every parameter, including every layer referred to by a layer parameter.
    ///   - [`PARAM_INDEX_CHECK_ALL_EXCEPT_LAYER_PARAMS`] - omit all layers. Pass a specific layer parameter index to include that as the only layer parameter tested.
    ///   - [`PARAM_INDEX_CHECK_ALL_HONOR_EXCLUDE`] - Similar to CHECK_ALL, but honor `PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED`.
    ///
    /// Passing in `None` for both start and duration indicates all time.
    /// For effects that do simulation across time and therefore set `PF_OutFlag2_AUTOMATIC_WIDE_TIME_INPUT`, when you ask about a time range,
    /// it will be expanded to include any times needed to produce that range.
    ///
    /// Populates a `PF_State`, an opaque data type used as a receipt for the current state of the effect's parameters (the `PF_State` is used in our internal frame caching database).
    pub fn current_state(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, start: Option<Time>, duration: Option<Time>) -> Result<ae_sys::PF_State, Error> {
        call_suite_fn_single!(self,
            PF_GetCurrentState -> ae_sys::PF_State,
            effect_ref.as_ptr(),
            param_index,
            start.map(Into::into).as_ref().map_or(std::ptr::null(), |t| t),
            duration.map(Into::into).as_ref().map_or(std::ptr::null(), |t| t)
        )
    }

    /// New in CS6. Compare two different states, retrieved using `PF_GetCurrentState`, above.
    pub fn are_states_identical(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, state1: &ae_sys::PF_State, state2: &ae_sys::PF_State) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_AreStatesIdentical -> ae_sys::A_Boolean, effect_ref.as_ptr(), state1, state2)? != 0)
    }

    /// Returns `true` if a parameter's value is the same at the two passed times.
    ///
    /// Note: the times need not be contiguous; there could be different intervening values.
    pub fn is_identical_checkout(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, what_time1: i32, time_step1: i32, time_scale1: u32, what_time2: i32, time_step2: i32, time_scale2: u32) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_IsIdenticalCheckout -> ae_sys::PF_Boolean, effect_ref.as_ptr(), param_index, what_time1, time_step1, time_scale1, what_time2, time_step2, time_scale2)? != 0)
    }

    /// Searches (in the specified direction) for the next keyframe in the parameter's stream. The last three parameters are optional.
    ///
    /// Returns a tuple containing: (found, key_index, key_time, key_timescale)
    pub fn find_keyframe_time(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, what_time: i32, time_scale: u32, time_dir: TimeDir) -> Result<(bool, i32, i32, u32), Error> {
        let mut found: ae_sys::PF_Boolean = 0;
        let mut key_index: ae_sys::PF_KeyIndex = 0;
        let mut key_time: ae_sys::A_long = 0;
        let mut key_timescale: ae_sys::A_u_long = 0;
        call_suite_fn!(self, PF_FindKeyframeTime, effect_ref.as_ptr(), param_index, what_time, time_scale, time_dir.into(), &mut found, &mut key_index, &mut key_time, &mut key_timescale)?;

        Ok((
            found != 0,
            key_index as i32,
            key_time as i32,
            key_timescale as u32
        ))
    }

    /// Returns the number of keyframes in the parameter's stream.
    pub fn keyframe_count(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, PF_GetKeyframeCount -> ae_sys::PF_KeyIndex, effect_ref.as_ptr(), param_index)? as i32)
    }

    /// Checks a keyframe for the specified parameter out of our keyframe database. `param_index` is zero-based. You can request time, timescale, or neither; useful if you're performing your own motion blur.
    pub fn checkout_keyframe(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, key_index: i32) -> Result<(i32, u32, ae_sys::PF_ParamDef), Error> {
        let mut key_time: ae_sys::A_long = 0;
        let mut key_timescale: ae_sys::A_u_long = 0;
        let param = call_suite_fn_single!(self, PF_CheckoutKeyframe -> ae_sys::PF_ParamDef, effect_ref.as_ptr(), param_index, key_index, &mut key_time, &mut key_timescale)?;
        Ok((key_time, key_timescale, param))
    }

    /// All calls to `checkout_keyframe` must be balanced with this check-in, or pain will ensue.
    pub fn checkin_keyframe(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, mut param: ae_sys::PF_ParamDef) -> Result<(), Error> {
        call_suite_fn!(self, PF_CheckinKeyframe, effect_ref.as_ptr(), &mut param as *mut _)
    }

    /// Returns the time (and timescale) of the specified keyframe.
    pub fn key_index_to_time(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, param_index: i32, key_index: i32) -> Result<(i32, u32), Error> {
        let (time, timesale) = call_suite_fn_double!(self, PF_KeyIndexToTime -> ae_sys::A_long, ae_sys::A_u_long, effect_ref.as_ptr(), param_index, key_index)?;
        Ok((
            time as i32,
            timesale as u32
        ))
    }
}

define_suite!(
    /// Use this suite to get floating point values for angle parameters.
    AngleParamSuite,
    PF_AngleParamSuite1,
    kPFAngleParamSuite,
    kPFAngleParamSuiteVersion1
);
impl AngleParamSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn floating_point_value_from_angle_def(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, angle_def: *const ae_sys::PF_ParamDef) -> Result<f64, Error> {
        call_suite_fn_single!(self, PF_GetFloatingPointValueFromAngleDef -> ae_sys::A_FpLong, effect_ref.as_ptr(), angle_def)
    }
}
define_suite!(
    /// Use this suite to get floating point values for color parameters.
    ColorParamSuite,
    PF_ColorParamSuite1,
    kPFAngleParamSuite,
    kPFAngleParamSuiteVersion1
);
impl ColorParamSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn floating_point_value_from_color_def(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, color_def: *const ae_sys::PF_ParamDef) -> Result<PixelF32, Error> {
        call_suite_fn_single!(self, PF_GetFloatingPointColorFromColorDef -> ae_sys::PF_PixelFloat, effect_ref.as_ptr(), color_def)
    }
}
define_suite!(
    /// Use this suite to get floating point values for point parameters.
    PointParamSuite,
    PF_PointParamSuite1,
    kPFPointParamSuite,
    kPFPointParamSuiteVersion1
);

impl PointParamSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn floating_point_value_from_point_def(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, point_def: *const ae_sys::PF_ParamDef) -> Result<ae_sys::A_FloatPoint, Error> {
        call_suite_fn_single!(self, PF_GetFloatingPointValueFromPointDef -> ae_sys::A_FloatPoint, effect_ref.as_ptr(), point_def)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

pub const PARAM_INDEX_NONE: i32 = ae_sys::PF_ParamIndex_NONE;

/// check every parameter, including every layer referred to by a layer parameter
pub const PARAM_INDEX_CHECK_ALL: i32 = ae_sys::PF_ParamIndex_CHECK_ALL;

/// omit all layers. Pass a specific layer parameter index to include that as the only layer parameter tested.
pub const PARAM_INDEX_CHECK_ALL_EXCEPT_LAYER_PARAMS: i32 = ae_sys::PF_ParamIndex_CHECK_ALL_EXCEPT_LAYER_PARAMS;

/// Similar to CHECK_ALL, but honor PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED.
pub const PARAM_INDEX_CHECK_ALL_HONOR_EXCLUDE: i32 = ae_sys::PF_ParamIndex_CHECK_ALL_HONOR_EXCLUDE;

define_enum! {
    ae_sys::PF_TimeDir,
    TimeDir {
        GreaterThan        = ae_sys::PF_TimeDir_GREATER_THAN,
        LessThan           = ae_sys::PF_TimeDir_LESS_THAN,
        GreaterThanOrEqual = ae_sys::PF_TimeDir_GREATER_THAN_OR_EQUAL,
        LessThanOrEqual    = ae_sys::PF_TimeDir_LESS_THAN_OR_EQUAL,
    }
}
