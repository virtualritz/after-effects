use aftereffects_sys as ae_sys;

// FIXME: wrap this nicely
pub struct EffectWorld {
    pub effect_world: Box<ae_sys::PF_EffectWorld>,
}
