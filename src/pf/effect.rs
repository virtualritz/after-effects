use crate::*;

register_handle!(PF_ProgPtr);
define_handle_wrapper!(EffectHandle, PF_ProgPtr);

define_suite_item_wrapper!(
    ae_sys::PF_ProgPtr, EffectHandle,
    pf_interface: aegp::suites::PFInterface,
    ///
    Effect {
        dispose: ;

        /// Returns the layer the effect is applied to
        layer() -> aegp::Layer => pf_interface.effect_layer,
    }
);

impl Effect {
}
