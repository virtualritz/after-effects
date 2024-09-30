use crate::*;
use ae_sys::*;
//use ae_sys::{ AEGP_FrameReceiptH, AEGP_WorldH };

define_suite!(
    /// Since we introduced the AEGP API, we've been asked to provide functions for retrieving rendered frames.
    ///
    /// These function suites allows you to do just that.
    ///
    /// First, specify what you want rendered in the [`aegp::suites::RenderOptions`] or [`aegp::suites::LayerRenderOptions`].
    ///
    /// Then do the rendering with [`aegp::suites::Render`].
    RenderSuite,
    AEGP_RenderSuite5,
    kAEGPRenderSuite,
    kAEGPRenderSuiteVersion5
);

impl RenderSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves an `AEGP_FrameReceiptH` (not the actual pixels) for the frame requested.
    /// Check in this receipt using ``AEGP_CheckinFrame`` to release memory.
    ///
    /// Create the [`aegp::RenderOptionsHandle`] using the [`aegp::suites::RenderOptions`].
    ///
    /// Optionally, the AEGP can pass a function to be called by After Effects if the user cancels the current render.
    pub fn render_and_checkout_frame<F: Fn() -> bool>(&self, options: impl AsPtr<AEGP_RenderOptionsH>, cancel_function: Option<F>) -> Result<AEGP_FrameReceiptH, Error> {
        unsafe extern "C" fn cancel_fn(refcon: *mut std::ffi::c_void, cancel: *mut ae_sys::A_Boolean) -> A_Err {
            let cb = Box::<Box<dyn Fn() -> bool + Send + Sync + 'static>>::from_raw(refcon as *mut _);
            *cancel = cb() as _;
            ae_sys::PF_Err_NONE as ae_sys::PF_Err
        }

        let cancel_function = cancel_function.map(|x| Box::new(Box::new(x)));

        call_suite_fn_single!(
            self,
            AEGP_RenderAndCheckoutFrame -> AEGP_FrameReceiptH,
            options.as_ptr(),
            if cancel_function.is_some() { Some(cancel_fn) } else { None },
            cancel_function.map_or(std::ptr::null_mut(), |f| Box::into_raw(f) as *mut _)
        )
    }

    /// New in CC 2014. This allows frame checkout of a layer with effects applied at non-render time.
    /// This is useful for an operation that requires the frame, for example, when a button is clicked and it is acceptable to wait for a moment while it is rendering.
    ///
    /// Note: Since it is not asynchronous, it will not solve the general problem where custom UI needs to draw based on the frame.
    ///
    /// Retrieves an `AEGP_FrameReceiptH` (not the actual pixels) for the layer frame requested.
    /// Check in this receipt using [`checkin_frame()`](Self::checkin_frame) to release memory.
    ///
    /// Create the [`aegp::LayerRenderOptions`] using the [`aegp::LayerRenderOptions::from_upstream_of_effect()`].
    ///
    /// You can actually use [`aegp::LayerRenderOptions::from_layer()`] to get other layer param's layers with their effects applied.
    /// However, be careful. If you do it in your effect A, and there's an effect B on the other layer that does the same thing during rendering, you'd create an infinite loop.
    /// If you're not doing it for render purposes then it could be okay.
    ///
    /// Optionally, the AEGP can pass a function to be called by After Effects if the user cancels the current render.
    pub fn render_and_checkout_layer_frame<F: FnMut() -> bool>(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, cancel_function: Option<F>) -> Result<AEGP_FrameReceiptH, Error> {
        unsafe extern "C" fn cancel_fn(refcon: *mut std::ffi::c_void, cancel: *mut ae_sys::A_Boolean) -> A_Err {
            let cb = Box::<Box<dyn Fn() -> bool + Send + Sync + 'static>>::from_raw(refcon as *mut _);
            *cancel = cb() as _;
            ae_sys::PF_Err_NONE as ae_sys::PF_Err
        }

        let cancel_function = cancel_function.map(|x| Box::new(Box::new(x)));

        call_suite_fn_single!(
            self,
            AEGP_RenderAndCheckoutLayerFrame -> AEGP_FrameReceiptH,
            options.as_ptr(),
            if cancel_function.is_some() { Some(cancel_fn) } else { None },
            cancel_function.map_or(std::ptr::null_mut(), |f| Box::into_raw(f) as *mut _)
        )
    }

    pub fn render_and_checkout_layer_frame_async<R: FnMut(AEGP_AsyncRequestId, bool, Error, AEGP_FrameReceiptH)>(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, callback: R) -> Result<AEGP_AsyncRequestId, Error> {
        unsafe extern "C" fn frame_ready_cb(request_id: AEGP_AsyncRequestId, was_canceled: A_Boolean, error: A_Err, receipt: AEGP_FrameReceiptH, refcon: AEGP_AsyncFrameRequestRefcon) -> A_Err {
            let cb = Box::<Box<dyn Fn(AEGP_AsyncRequestId, bool, Error, AEGP_FrameReceiptH)>>::from_raw(refcon as *mut _);
            cb(request_id, was_canceled != 0, Error::from(error), receipt);
            ae_sys::PF_Err_NONE as ae_sys::PF_Err
        }

        let callback = Box::new(Box::new(callback));

        call_suite_fn_single!(
            self,
            AEGP_RenderAndCheckoutLayerFrame_Async -> AEGP_AsyncRequestId,
            options.as_ptr(),
            Some(frame_ready_cb),
            Box::into_raw(callback) as *mut _
        )
    }

    /// Call this function as soon as your AEGP is done accessing the frame.
    /// After Effects makes caching decisions based on which frames are checked out, so don't hog them!
    pub fn checkin_frame(&self, receipt: impl AsPtr<AEGP_FrameReceiptH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_CheckinFrame, receipt.as_ptr())
    }

    /// Retrieves the pixels ([`aegp::WorldHandle`]) associated with the referenced `AEGP_FrameReceiptH`
    pub fn receipt_world(&self, receipt: impl AsPtr<AEGP_FrameReceiptH>) -> Result<aegp::WorldHandle, Error> {
        Ok(aegp::WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetReceiptWorld -> AEGP_WorldH, receipt.as_ptr())?
        ))
    }

    /// Retrieves an [`Rect`] containing the region of the `AEGP_FrameReceiptH`'s `AEGP_WorldH` that has already been rendered.
    /// Remember that it's possible for only those portions of an image that have been changed to be rendered,
    /// so it's important to be able to check whether or not that includes the portion you need.
    pub fn rendered_region(&self, receipt: impl AsPtr<AEGP_FrameReceiptH>) -> Result<Rect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRenderedRegion -> A_LRect, receipt.as_ptr())?.into())
    }

    /// Given two sets of [`aegp::RenderOptions`], After Effects will return `true` if the already-rendered pixels are still valid for the proposed [`aegp::RenderOptions`].
    pub fn is_rendered_frame_sufficient(&self, rendered_options: impl AsPtr<AEGP_RenderOptionsH>, proposed_options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsRenderedFrameSufficient -> A_Boolean, rendered_options.as_ptr(), proposed_options.as_ptr())? != 0)
    }

    /// Obtains an [`aegp::SoundDataHandle`] for the given item at the given time, of the given duration, in the given format.
    ///
    /// NOTE: This function, if called as part of [`aegp::suites::Item`], provides a render interruptible using mouse clicks,
    /// unlike the version published here in [`aegp::suites::Render`].
    pub fn render_new_item_sound_data<F: FnMut() -> bool>(&self, item: impl AsPtr<AEGP_ItemH>, start_time: Time, duration: Time, sound_format: &AEGP_SoundDataFormat, cancel_function: Option<F>) -> Result<aegp::SoundDataHandle, Error> {
        unsafe extern "C" fn cancel_fn(refcon: *mut std::ffi::c_void, cancel: *mut ae_sys::A_Boolean) -> A_Err {
            let cb = Box::<Box<dyn Fn() -> bool + Send + Sync + 'static>>::from_raw(refcon as *mut _);
            *cancel = cb() as _;
            ae_sys::PF_Err_NONE as ae_sys::PF_Err
        }
        let cancel_function = cancel_function.map(|x| Box::new(Box::new(x)));

        Ok(aegp::SoundDataHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_RenderNewItemSoundData -> AEGP_SoundDataH,
                item.as_ptr(),
                &start_time.into(),
                &duration.into(),
                sound_format,
                if cancel_function.is_some() { Some(cancel_fn) } else { None },
                cancel_function.map_or(std::ptr::null_mut(), |f| Box::into_raw(f) as *mut _)
            )?
        ))
    }

    /// Retrieves the current `AEGP_TimeStamp` of the project.
    /// The `AEGP_TimeStamp` is updated whenever an item is touched in a way that affects rendering.
    pub fn current_timestamp(&self) -> Result<AEGP_TimeStamp, Error> {
        call_suite_fn_single!(self, AEGP_GetCurrentTimestamp -> AEGP_TimeStamp)
    }

    /// Returns whether the video of an `AEGP_ItemH` has changed since the given `AEGP_TimeStamp`.
    ///
    /// Note: this does not track changes in audio.
    pub fn has_item_changed_since_timestamp(&self, item: impl AsPtr<AEGP_ItemH>, start_time: Time, duration: Time, timestamp: &AEGP_TimeStamp) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_HasItemChangedSinceTimestamp -> A_Boolean, item.as_ptr(), &start_time.into(), &duration.into(), timestamp)? != 0)
    }

    /// Returns whether this frame would be worth rendering externally and checking in to the cache.
    /// A speculative renderer should check this twice: before sending the frame out to render and when it is complete, before calling `AEGP_NewPlatformWorld()` and checking in.
    ///
    /// This function is to be used with [`has_item_changed_since_timestamp()`](Self::has_item_changed_since_timestamp), not alone.
    pub fn is_item_worthwhile_to_render(&self, render_options: impl AsPtr<AEGP_RenderOptionsH>, timestamp: &AEGP_TimeStamp) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsItemWorthwhileToRender -> A_Boolean, render_options.as_ptr(), timestamp)? != 0)
    }

    /// Provide a rendered frame (`AEGP_PlatformWorldH`) to After Effects, which adopts it.
    /// `ticks` is the approximate time required to render the frame.
    pub fn checkin_rendered_frame(&self, render_options: impl AsPtr<AEGP_RenderOptionsH>, timestamp: &AEGP_TimeStamp, ticks_to_render: u32, image: AEGP_PlatformWorldH) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_CheckinRenderedFrame, render_options.as_ptr(), timestamp, ticks_to_render, image)
    }

    /// New in CS6. Retrieve a GUID for a rendered frame. The memory handle passed back must be disposed.
    pub fn receipt_guid(&self, receipt: impl AsPtr<AEGP_FrameReceiptH>) -> Result<AEGP_MemHandle, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetReceiptGuid -> AEGP_MemHandle, receipt.as_ptr())?)
    }
}
