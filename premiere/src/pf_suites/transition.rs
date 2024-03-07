
use crate::*;
use pr_sys::*;

define_suite!(
    /// # After Effects-Style Transitions
    ///
    /// AE-style Transitions can now get and set transition start and end percentages.
    /// The user can change the start and end parameters in the Effect Controls panel.
    /// To allow a plugin to be informed of changes to these values, there are two new functions in the PF TransitionSuite:
    /// [`register_transition_start_param()`](Self::register_transition_start_param) and [`register_transition_end_param()`](Self::register_transition_end_param),
    /// which register these parameters with the plugin as float parameters.
    /// Once registered, the plugin will receive [`Command::UserChangedParam`] when these params change,
    /// as well as when the transition is first applied, so the plugin can initialize them to the desired value.
    ///
    /// AE-style Transitions can now retrieve GPU frames from arbitrary locations in the underlying clips.
    /// There is a new PrGPUDependency_TransitionInputFrame, and PrGPUFilterFrameDependency has a new member to specify whether frames from the incoming or outgoing clips are needed.
    ///
    /// The call to [`register_transition_input_param()`](Self::register_transition_input_param) call must be made before the `params.add(...)` call during `param_setup`.
    ///
    /// Pass in the param to be used as the input layer for the other side of the transition.
    ///
    /// This enables your effect to be applied between two clips in the timeline just like our native transitions,
    /// but it will show up in the Effect Controls panel with full keyframable parameters similar to existing AE effects.
    TransitionSuite,
    PF_TransitionSuite,
    kPFTransitionSuite,
    kPFTransitionSuiteVersion
);

impl TransitionSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Register an effect as a transition using the passed in input layer as the outgoing clip.
    /// When registered the effect will be available to be dragged directly onto clip ends rather than only applied to layers.
    pub fn register_transition_input_param(&self, effect_ref: impl AsPtr<PF_ProgPtr>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, RegisterTransitionInputParam, effect_ref.as_ptr(), index as _)
    }

    /// Register a PF_ADD_FLOAT_SLIDER parameter to receive changes to the start of the transition region through the [`Command::UserChangedParam`] command.
    pub fn register_transition_start_param(&self, effect_ref: impl AsPtr<PF_ProgPtr>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, RegisterTransitionStartParam, effect_ref.as_ptr(), index as _)
    }

    /// Register a PF_ADD_FLOAT_SLIDER parameter to receive changes to the end of the transition region through the [`Command::UserChangedParam`] command.
    pub fn register_transition_end_param(&self, effect_ref: impl AsPtr<PF_ProgPtr>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, RegisterTransitionEndParam, effect_ref.as_ptr(), index as _)
    }
}
