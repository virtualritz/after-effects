use crate::*;

define_suite!(
    /// Although not strictly concerned with parameters, this suite can change the name of the options button.
    EffectUISuite,
    PF_EffectUISuite1,
    kPFEffectUISuite,
    kPFEffectUISuiteVersion1
);

impl EffectUISuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Changes the text on the options button in the effect controls palette.
    ///
    /// Button name can be up to 31 characters.
    ///
    /// NOTE: This must be called during [`Command::ParamsSetup`].
    pub fn set_options_button_name(&self, effect_ref: impl AsPtr<ae_sys::PF_ProgPtr>, name: &str) -> Result<(), Error> {
        assert!(name.len() < 31);
        let name = std::ffi::CString::new(name).unwrap();
        call_suite_fn!(self, PF_SetOptionsButtonName, effect_ref.as_ptr(), name.as_ptr())
    }
}
