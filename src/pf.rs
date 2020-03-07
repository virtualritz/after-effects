use crate::Suite;
use aftereffects_sys as ae_sys;

// FIXME: wrap this nicely
pub struct EffectWorld {
    pub effect_world_boxed: Box<ae_sys::PF_EffectWorld>,
}

impl EffectWorld {
    pub fn new(
        world_handle: &crate::aegp::WorldHandle,
    ) -> Result<Self, crate::Error> {
        crate::aegp::WorldSuite::new()?
            .fill_out_pf_effect_world(world_handle)
    }

    pub fn borrow(&self) -> &Box<ae_sys::PF_EffectWorld> {
        &self.effect_world_boxed
    }

    /*
    pub fn as_mut_ptr(&self) -> *mut ae_sys::PF_EffectWorld {
        self.effect_world_boxed.as_mut_ptr();
    }*/
}
