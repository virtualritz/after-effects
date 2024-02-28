use crate::*;

define_suite!(
    /// When enabling Multi-Frame Rendering on an effect, the `sequence_data` object will be read-only/const during Render and accessible on each render thread via the [`EffectSequenceDataSuite`].
    EffectSequenceDataSuite,
    PF_EffectSequenceDataSuite1,
    kPFEffectSequenceDataSuite,
    kPFEffectSequenceDataSuiteVersion1
);

impl EffectSequenceDataSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the read-only const sequence_data object for a rendering thread when Multi-Frame Rendering is enabled for an effect.
    pub fn const_sequence_data(&self, in_data_handle: &InData) -> Result<ae_sys::PF_ConstHandle, Error> {
        call_suite_fn_single!(self, PF_GetConstSequenceData -> ae_sys::PF_ConstHandle, in_data_handle.effect_ref().as_ptr())
    }
}
