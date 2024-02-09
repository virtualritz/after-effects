use crate::*;

define_suite!(
    EffectSequenceDataSuite1,
    PF_EffectSequenceDataSuite1,
    kPFEffectSequenceDataSuite,
    kPFEffectSequenceDataSuiteVersion1
);

impl EffectSequenceDataSuite1 {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn get_const_sequence_data(&self, in_data_handle: InData) -> Result<ae_sys::PF_ConstHandle, Error> {
        call_suite_fn_single!(self, PF_GetConstSequenceData -> ae_sys::PF_ConstHandle, in_data_handle.effect_ref().as_ptr())
    }
}
