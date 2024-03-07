

use crate::*;
use ae_sys::*;

define_suite!(
    /// Long ago, we helped a developer integrate their stand-alone tracker with After Effects by exposing
    /// a set of functions to give them some way to notify us of, and be notified of, changes to the timeline.
    ///
    /// With the numerous AEGP API calls available, these aren't used much, but they're still available.
    ///
    /// Don't confuse this suite with [`aegp::suites::Item`].
    AdvItemSuite,
    PF_AdvItemSuite1,
    kPFAdvItemSuite,
    kPFAdvItemSuiteVersion1
);

impl AdvItemSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Moves current time `num_steps` in the specified direction.
    pub fn move_time_step(&self, in_data: impl AsPtr<*mut PF_InData>, world: impl AsPtr<*mut PF_EffectWorld>, time_dir: Step, num_steps: i32) -> Result<(), Error> {
        call_suite_fn!(self, PF_MoveTimeStep, in_data.as_ptr(), world.as_ptr(), time_dir.into(), num_steps as _)
    }

    /// Moves `num_steps` in the specified direction, for the active item.
    pub fn move_time_step_active_item(&self, time_dir: Step, num_steps: i32) -> Result<(), Error> {
        call_suite_fn!(self, PF_MoveTimeStepActiveItem, time_dir.into(), num_steps as _)
    }

    /// Tells After Effects that the active item must be updated.
    pub fn touch_active_item(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_TouchActiveItem, )
    }

    /// Forces After Effects to rerender the current frame.
    pub fn force_rerender(&self, in_data: impl AsPtr<*mut PF_InData>, world: impl AsPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self, PF_ForceRerender, in_data.as_ptr(), world.as_ptr())
    }

    /// Returns whether the effect which owns the `PF_ContextH` is currently active or enabled (if it isn't, After Effects won't be listening for function calls from it).
    pub fn effect_is_active_or_enabled(&self, context: impl AsPtr<PF_ContextH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_EffectIsActiveOrEnabled -> PF_Boolean, context.as_ptr())? != 0)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::PF_Step,
    Step {
        Forward  = ae_sys::PF_Step_FORWARD,
        Backward = ae_sys::PF_Step_BACKWARD,
    }
}
