use crate::*;

define_enum!(
    ae_sys::AEGP_RenderQueueState,
    RenderQueueState {
        Paused        = ae_sys::AEGP_RenderQueueState_PAUSED,
        Rendering  = ae_sys::AEGP_RenderQueueState_RENDERING,
        Stopped       = ae_sys::AEGP_RenderQueueState_STOPPED,
    }
);

define_suite!(
    RenderQueueSuite,
    AEGP_RenderQueueSuite1,
    kAEGPRenderQueueSuite,
    kAEGPRenderQueueSuiteVersion1
);

impl RenderQueueSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /*
        AEGP_AddCompToRenderQueue(
            AEGP_CompH     compH,
            const A_char*  pathZ);
    */
    /// Adds a composition to the render queue, using default options.
    pub fn add_comp_to_render_queue(
        &self,
        comp_handle: impl AsPtr<after_effects_sys::AEGP_CompH>,
        path: &str,
    ) -> Result<(), Error> {
        let path = std::ffi::CString::new(path).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(
            self,
            AEGP_AddCompToRenderQueue,
            comp_handle.as_ptr(),
            path.as_ptr()
        )?;

        Ok(())
    }

    /*
       AEGP_SetRenderQueueState(
           AEGP_RenderQueueState  state);
    */
    /// Sets the render queue to one of three valid states. It is not possible to go from stopped to paused.
    pub fn set_render_queue_state(&self, state: RenderQueueState) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRenderQueueState, state.into())?;
        Ok(())
    }

    /*
        AEGP_GetRenderQueueState(
            AEGP_RenderQueueState  *stateP);
    */
    /// Obtains the current render queue state.
    pub fn get_render_queue_state(&self) -> Result<RenderQueueState, Error> {
        Ok(
            call_suite_fn_single!(self, AEGP_GetRenderQueueState -> ae_sys::AEGP_RenderQueueState)?
                .into(),
        )
    }
}
