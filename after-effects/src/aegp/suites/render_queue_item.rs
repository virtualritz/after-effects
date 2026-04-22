use crate::*;
use crate::aegp::*;

define_suite!(
    /// Render Queue Item Suite provides information about items in the render queue.
    ///
    /// NOTE: All `RQItemRefHandle`s are invalidated by ANY re-ordering, addition or removal
    /// of render items. DO NOT CACHE THEM.
    RenderQueueItemSuite,
    AEGP_RQItemSuite4,
    kAEGPRQItemSuite,
    kAEGPRQItemSuiteVersion4
);

impl RenderQueueItemSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns the number of items currently in the render queue.
    pub fn num_items(&self) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumRQItems -> ae_sys::A_long)? as i32)
    }

    /// Retrieves a render queue item by index.
    pub fn item_by_index(&self, index: i32) -> Result<RQItemRefHandle, Error> {
        Ok(RQItemRefHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetRQItemByIndex -> ae_sys::AEGP_RQItemRefH, index as ae_sys::A_long)?
        ))
    }

    /// Retrieves the next render queue item.
    /// Pass `None` for `current_item` to get the first item.
    pub fn next_item(&self, current_item: Option<RQItemRefHandle>) -> Result<Option<RQItemRefHandle>, Error> {
        let current = current_item.map_or(std::ptr::null_mut(), |h| h.as_ptr());
        let next = call_suite_fn_single!(self, AEGP_GetNextRQItem -> ae_sys::AEGP_RQItemRefH, current)?;
        if next.is_null() {
            Ok(None)
        } else {
            Ok(Some(RQItemRefHandle::from_raw(next)))
        }
    }

    /// Returns the number of output modules attached to the given render queue item.
    pub fn num_output_modules(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumOutputModulesForRQItem -> ae_sys::A_long, rq_item.as_ptr())? as i32)
    }

    /// Returns the render state of the given render queue item.
    pub fn render_state(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<RenderItemStatus, Error> {
        Ok(
            call_suite_fn_single!(self, AEGP_GetRenderState -> ae_sys::AEGP_RenderItemStatusType, rq_item.as_ptr())?
                .into()
        )
    }

    /// Sets the render state of the given render queue item.
    ///
    /// Will return an error if called while `RenderQueueState` is not `Stopped`.
    ///
    /// Returns `Err_RANGE` if you pass a status that is illegal in any case.
    /// Returns `Err_PARAMETER` if you try to pass a status that doesn't make sense
    /// (e.g., trying to queue something for which you haven't set the output path).
    pub fn set_render_state(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>, status: RenderItemStatus) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRenderState, rq_item.as_ptr(), status.into())
    }

    /// Returns the time at which the given render queue item started rendering.
    /// Returns `Time { value: 0, scale: 1 }` if not started.
    pub fn started_time(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<Time, Error> {
        Ok(
            call_suite_fn_single!(self, AEGP_GetStartedTime -> ae_sys::A_Time, rq_item.as_ptr())?
                .into()
        )
    }

    /// Returns the elapsed rendering time for the given render queue item.
    /// Returns `Time { value: 0, scale: 1 }` if not rendered.
    pub fn elapsed_time(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<Time, Error> {
        Ok(
            call_suite_fn_single!(self, AEGP_GetElapsedTime -> ae_sys::A_Time, rq_item.as_ptr())?
                .into()
        )
    }

    /// Returns the log type for the given render queue item.
    pub fn log_type(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<LogType, Error> {
        Ok(
            call_suite_fn_single!(self, AEGP_GetLogType -> ae_sys::AEGP_LogType, rq_item.as_ptr())?
                .into()
        )
    }

    /// Sets the log type for the given render queue item.
    pub fn set_log_type(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>, log_type: LogType) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLogType, rq_item.as_ptr(), log_type.into())
    }

    /// Removes an output module from a render queue item.
    pub fn remove_output_module(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveOutputModule, rq_item.as_ptr(), output_module.as_ptr())
    }

    /// Retrieves the comment for the given render queue item.
    pub fn comment(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetComment -> ae_sys::AEGP_MemHandle, rq_item.as_ptr())?;
        unsafe {
            Ok(
                U16CString::from_ptr_str(MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr())
                    .to_string_lossy()
            )
        }
    }

    /// Sets the comment for the given render queue item.
    pub fn set_comment(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>, comment: &str) -> Result<(), Error> {
        let comment_utf16 = U16CString::from_str(comment).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetComment, rq_item.as_ptr(), comment_utf16.as_ptr())
    }

    /// Retrieves the composition associated with the given render queue item.
    pub fn comp(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetCompFromRQItem -> ae_sys::AEGP_CompH, rq_item.as_ptr())?
        ))
    }

    /// Deletes a render queue item. This is undoable.
    pub fn delete_item(&self, rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteRQItem, rq_item.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ―――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_RQItemRefH);
define_handle_wrapper!(RQItemRefHandle, AEGP_RQItemRefH);

register_handle!(AEGP_OutputModuleRefH);
define_handle_wrapper!(OutputModuleRefHandle, AEGP_OutputModuleRefH);

define_enum! {
    ae_sys::AEGP_RenderItemStatusType,
    RenderItemStatus {
        None         = ae_sys::AEGP_RenderItemStatus_NONE,
        WillContinue = ae_sys::AEGP_RenderItemStatus_WILL_CONTINUE,
        NeedsOutput  = ae_sys::AEGP_RenderItemStatus_NEEDS_OUTPUT,
        /// Ready to be rendered, but not included in the queue.
        Unqueued     = ae_sys::AEGP_RenderItemStatus_UNQUEUED,
        /// Ready AND queued.
        Queued       = ae_sys::AEGP_RenderItemStatus_QUEUED,
        Rendering    = ae_sys::AEGP_RenderItemStatus_RENDERING,
        UserStopped  = ae_sys::AEGP_RenderItemStatus_USER_STOPPED,
        ErrStopped   = ae_sys::AEGP_RenderItemStatus_ERR_STOPPED,
        Done         = ae_sys::AEGP_RenderItemStatus_DONE,
    }
}

define_enum! {
    ae_sys::AEGP_LogType,
    LogType {
        None         = ae_sys::AEGP_LogType_NONE,
        ErrorsOnly   = ae_sys::AEGP_LogType_ERRORS_ONLY,
        PlusSettings = ae_sys::AEGP_LogType_PLUS_SETTINGS,
        PerFrameInfo = ae_sys::AEGP_LogType_PER_FRAME_INFO,
    }
}
