use crate::*;
use ae_sys::AEGP_ItemViewP;

define_suite!(
    /// Query the state of an item view (a Composition, Layer or Footage panel).
    ///
    /// Get the view of an item with [`ItemSuite::item_mru_view()`](aegp::suites::Item::item_mru_view).
    ItemViewSuite,
    AEGP_ItemViewSuite1,
    kAEGPItemViewSuite,
    kAEGPItemViewSuiteVersion1
);
define_suite!(
    ItemViewSuite2,
    AEGP_ItemViewSuite2,
    kAEGPItemViewSuite,
    kAEGPItemViewSuiteVersion2
);

impl ItemViewSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns whether the view is currently previewing, and the playback time of the view.
    pub fn item_view_playback_time(&self, item_view: impl AsPtr<AEGP_ItemViewP>) -> Result<(bool, Time), Error> {
        let (previewing, time) = call_suite_fn_double!(self, AEGP_GetItemViewPlaybackTime -> ae_sys::A_Boolean, ae_sys::A_Time, item_view.as_ptr())?;
        Ok((previewing != 0, time.into()))
    }

    /// New in AE 26.0. Returns whether guides are shown in the view.
    pub fn item_view_guides_visible(&self, item_view: impl AsPtr<AEGP_ItemViewP>) -> Result<bool, Error> {
        let v2 = ItemViewSuite2::new()?;
        Ok(call_suite_fn_single!(v2, AEGP_GetItemViewGuidesVisible -> ae_sys::A_Boolean, item_view.as_ptr())? != 0)
    }

    /// New in AE 26.0. Shows or hides guides in the view.
    pub fn set_item_view_guides_visible(&self, item_view: impl AsPtr<AEGP_ItemViewP>, visible: bool) -> Result<(), Error> {
        let v2 = ItemViewSuite2::new()?;
        call_suite_fn!(v2, AEGP_SetItemViewGuidesVisible, item_view.as_ptr(), visible as _)
    }

    /// New in AE 26.0. Returns whether snapping to guides is enabled in the view.
    pub fn item_view_guides_snap(&self, item_view: impl AsPtr<AEGP_ItemViewP>) -> Result<bool, Error> {
        let v2 = ItemViewSuite2::new()?;
        Ok(call_suite_fn_single!(v2, AEGP_GetItemViewGuidesSnap -> ae_sys::A_Boolean, item_view.as_ptr())? != 0)
    }

    /// New in AE 26.0. Enables or disables snapping to guides in the view.
    pub fn set_item_view_guides_snap(&self, item_view: impl AsPtr<AEGP_ItemViewP>, snap: bool) -> Result<(), Error> {
        let v2 = ItemViewSuite2::new()?;
        call_suite_fn!(v2, AEGP_SetItemViewGuidesSnap, item_view.as_ptr(), snap as _)
    }

    /// New in AE 26.0. Returns whether guides are locked in the view.
    pub fn item_view_guides_locked(&self, item_view: impl AsPtr<AEGP_ItemViewP>) -> Result<bool, Error> {
        let v2 = ItemViewSuite2::new()?;
        Ok(call_suite_fn_single!(v2, AEGP_GetItemViewGuidesLocked -> ae_sys::A_Boolean, item_view.as_ptr())? != 0)
    }

    /// New in AE 26.0. Locks or unlocks guides in the view.
    pub fn set_item_view_guides_locked(&self, item_view: impl AsPtr<AEGP_ItemViewP>, locked: bool) -> Result<(), Error> {
        let v2 = ItemViewSuite2::new()?;
        call_suite_fn!(v2, AEGP_SetItemViewGuidesLocked, item_view.as_ptr(), locked as _)
    }
}
