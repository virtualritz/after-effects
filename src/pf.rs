pub use crate::*;
use aftereffects_sys as ae_sys;

// FIXME: wrap this nicely
pub struct EffectWorld {
    pub effect_world_boxed: Box<ae_sys::PF_EffectWorld>,
}

impl EffectWorld {
    pub fn new(
        world_handle: WorldHandle,
    ) -> Result<Self, crate::Error> {
        WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
    }

    pub fn borrow(&self) -> &ae_sys::PF_EffectWorld {
        &(*self.effect_world_boxed)
    }

    /*
    pub fn as_mut_ptr(&self) -> *mut ae_sys::PF_EffectWorld {
        self.effect_world_boxed.as_mut_ptr();
    }*/
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Err {
    None = ae_sys::PF_Err_NONE as isize,
    OutOfMemory = ae_sys::PF_Err_OUT_OF_MEMORY as isize,
    InternalStructDamaged =
        ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED as isize,
    // Out of range, or action not allowed on this index.
    InvalidIndex = ae_sys::PF_Err_INVALID_INDEX as isize,
    UnrecogizedParamType =
        ae_sys::PF_Err_UNRECOGNIZED_PARAM_TYPE as isize,
    InvalidCallback = ae_sys::PF_Err_INVALID_CALLBACK as isize,
    BadCallbackParam = ae_sys::PF_Err_BAD_CALLBACK_PARAM as isize,
    // Returned when user interrupts rendering.
    InterruptCancel = ae_sys::PF_Interrupt_CANCEL as isize,
    // Returned from PF_Arbitrary_SCAN_FUNC when effect cannot parse
    // arbitrary data from text
    CannonParseKeyframeText =
        ae_sys::PF_Err_CANNOT_PARSE_KEYFRAME_TEXT as isize,
}
