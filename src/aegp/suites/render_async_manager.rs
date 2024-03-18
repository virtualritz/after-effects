use crate::*;
use ae_sys::{ PF_AsyncManagerP, AEGP_FrameReceiptH, AEGP_RenderOptionsH, AEGP_LayerRenderOptionsH };

define_suite!(
    /// For cases where such renders formerly were triggered by side-effect or cancelled implicity
    /// (such as custom UI histogram drawing), and lifetime is less clear from inside the plug-in, use the new "Async Manager" which
    /// can handle multiple simultaneous async requests for effect Custom UI and will automatically support interactions with other AE UI behavior.
    ///
    /// Note: Async retrieval of frames is preferred for handling passive drawing situations, but not when a user action will update the project state.
    /// If you are (1) responding to a specific user click, AND 2) you need to update the project as a result, the synchronous AEGP_RenderAndCheckoutLayerFrame() is recommended.
    RenderAsyncManagerSuite,
    AEGP_RenderAsyncManagerSuite1,
    kAEGPRenderAsyncManagerSuite,
    kAEGPRenderAsyncManagerSuiteVersion1
);

impl RenderAsyncManagerSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn checkout_or_render_item_frame_async_manager(&self, async_manager: impl AsPtr<PF_AsyncManagerP>, purpose_id: u32, ro: impl AsPtr<AEGP_RenderOptionsH>) -> Result<AEGP_FrameReceiptH, Error> {
        call_suite_fn_single!(self, AEGP_CheckoutOrRender_ItemFrame_AsyncManager -> AEGP_FrameReceiptH, async_manager.as_ptr(), purpose_id as _, ro.as_ptr())
    }

    pub fn checkout_or_render_layer_frame_async_manager(&self, async_manager: impl AsPtr<PF_AsyncManagerP>, purpose_id: u32, lro: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<AEGP_FrameReceiptH, Error> {
        call_suite_fn_single!(self, AEGP_CheckoutOrRender_LayerFrame_AsyncManager -> AEGP_FrameReceiptH, async_manager.as_ptr(), purpose_id as _, lro.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(PF_AsyncManagerP);
register_handle!(AEGP_FrameReceiptH);
define_handle_wrapper!(AsyncManager, PF_AsyncManagerP);

impl AsyncManager {
    pub fn checkout_or_render_item_frame_async_manager(&self, purpose_id: u32, ro: impl AsPtr<AEGP_RenderOptionsH>) -> Result<AEGP_FrameReceiptH, Error> {
        RenderAsyncManagerSuite::new()?.checkout_or_render_item_frame_async_manager(self.0, purpose_id, ro)
    }

    pub fn checkout_or_render_layer_frame_async_manager(&self, purpose_id: u32, lro: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<AEGP_FrameReceiptH, Error> {
        RenderAsyncManagerSuite::new()?.checkout_or_render_layer_frame_async_manager(self.0, purpose_id, lro)
    }
}
