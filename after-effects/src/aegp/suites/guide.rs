use crate::*;
use ae_sys::{ AEGP_ItemH, AEGP_LayerH };

define_suite!(
    /// New in AE 26.0. Read and write the guides of compositions, footage items and layers.
    ///
    /// The `item_*` functions work on the guides of an item (composition or footage), the `layer_*` functions on the guides of a layer.
    ///
    /// The `*2` functions require `AEGP_GuideSuite2` (AE 26.2) and add percentage positioning, per-guide color and edge pinning.
    GuideSuite,
    AEGP_GuideSuite1,
    kAEGPGuideSuite,
    kAEGPGuideSuiteVersion1
);
define_suite!(
    GuideSuite2,
    AEGP_GuideSuite2,
    kAEGPGuideSuite,
    kAEGPGuideSuiteVersion2
);

impl GuideSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns the number of guides of the layer.
    pub fn layer_num_guides(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetLayerNumGuides -> ae_sys::A_long, layer_handle.as_ptr())
    }

    /// Returns the number of guides of the item (composition or footage).
    pub fn item_num_guides(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetItemNumGuides -> ae_sys::A_long, item_handle.as_ptr())
    }

    /// Returns the orientation and the position (in pixels) of the layer guide at `index`.
    pub fn layer_guide_by_index(&self, layer_handle: impl AsPtr<AEGP_LayerH>, index: i32) -> Result<(GuideOrientation, f64), Error> {
        let (orientation, position) = call_suite_fn_double!(self, AEGP_GetLayerGuideByIndex -> ae_sys::AEGP_GuideOrientationType, ae_sys::A_FpLong, layer_handle.as_ptr(), index)?;
        Ok((orientation.into(), position))
    }

    /// Returns the orientation and the position (in pixels) of the item guide at `index`.
    pub fn item_guide_by_index(&self, item_handle: impl AsPtr<AEGP_ItemH>, index: i32) -> Result<(GuideOrientation, f64), Error> {
        let (orientation, position) = call_suite_fn_double!(self, AEGP_GetItemGuideByIndex -> ae_sys::AEGP_GuideOrientationType, ae_sys::A_FpLong, item_handle.as_ptr(), index)?;
        Ok((orientation.into(), position))
    }

    /// Adds a guide to the layer at `position` (in pixels) and returns its index.
    pub fn add_layer_guide(&self, layer_handle: impl AsPtr<AEGP_LayerH>, orientation: GuideOrientation, position: f64) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_AddLayerGuide -> ae_sys::A_long, layer_handle.as_ptr(), orientation.into(), position)
    }

    /// Adds a guide to the item at `position` (in pixels) and returns its index.
    pub fn add_item_guide(&self, item_handle: impl AsPtr<AEGP_ItemH>, orientation: GuideOrientation, position: f64) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_AddItemGuide -> ae_sys::A_long, item_handle.as_ptr(), orientation.into(), position)
    }

    /// Moves the layer guide at `index` to `position` (in pixels).
    pub fn set_layer_guide(&self, layer_handle: impl AsPtr<AEGP_LayerH>, index: i32, position: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLayerGuide, layer_handle.as_ptr(), index, position)
    }

    /// Moves the item guide at `index` to `position` (in pixels).
    pub fn set_item_guide(&self, item_handle: impl AsPtr<AEGP_ItemH>, index: i32, position: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemGuide, item_handle.as_ptr(), index, position)
    }

    /// Removes the layer guide at `index`.
    pub fn remove_layer_guide(&self, layer_handle: impl AsPtr<AEGP_LayerH>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveLayerGuide, layer_handle.as_ptr(), index)
    }

    /// Removes the item guide at `index`.
    pub fn remove_item_guide(&self, item_handle: impl AsPtr<AEGP_ItemH>, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveItemGuide, item_handle.as_ptr(), index)
    }

    /// New in AE 26.2. Returns all properties of the layer guide at `index`.
    pub fn layer_guide_by_index2(&self, layer_handle: impl AsPtr<AEGP_LayerH>, index: i32) -> Result<Guide, Error> {
        let v2 = GuideSuite2::new()?;
        let mut raw = RawGuide::zeroed();
        call_suite_fn!(v2, AEGP_GetLayerGuideByIndex2, layer_handle.as_ptr(), index, &mut raw.orientation, &mut raw.position_type, &mut raw.position, &mut raw.color, &mut raw.pinned)?;
        Ok(raw.into())
    }

    /// New in AE 26.2. Returns all properties of the item guide at `index`.
    pub fn item_guide_by_index2(&self, item_handle: impl AsPtr<AEGP_ItemH>, index: i32) -> Result<Guide, Error> {
        let v2 = GuideSuite2::new()?;
        let mut raw = RawGuide::zeroed();
        call_suite_fn!(v2, AEGP_GetItemGuideByIndex2, item_handle.as_ptr(), index, &mut raw.orientation, &mut raw.position_type, &mut raw.position, &mut raw.color, &mut raw.pinned)?;
        Ok(raw.into())
    }

    /// New in AE 26.2. Adds a guide to the layer and returns its index.
    ///
    /// The position is clamped to ±100,000 px or ±300%; non-finite positions are rejected.
    pub fn add_layer_guide2(&self, layer_handle: impl AsPtr<AEGP_LayerH>, guide: &Guide) -> Result<i32, Error> {
        let v2 = GuideSuite2::new()?;
        call_suite_fn_single!(v2, AEGP_AddLayerGuide2 -> ae_sys::A_long, layer_handle.as_ptr(), guide.orientation.into(), guide.position_type.into(), guide.position, &guide.color, guide.pinned as _)
    }

    /// New in AE 26.2. Adds a guide to the item and returns its index.
    ///
    /// The position is clamped to ±100,000 px or ±300%; non-finite positions are rejected.
    pub fn add_item_guide2(&self, item_handle: impl AsPtr<AEGP_ItemH>, guide: &Guide) -> Result<i32, Error> {
        let v2 = GuideSuite2::new()?;
        call_suite_fn_single!(v2, AEGP_AddItemGuide2 -> ae_sys::A_long, item_handle.as_ptr(), guide.orientation.into(), guide.position_type.into(), guide.position, &guide.color, guide.pinned as _)
    }

    /// New in AE 26.2. Replaces all properties of the layer guide at `index`.
    ///
    /// The position is clamped to ±100,000 px or ±300%; non-finite positions are rejected.
    pub fn set_layer_guide2(&self, layer_handle: impl AsPtr<AEGP_LayerH>, index: i32, guide: &Guide) -> Result<(), Error> {
        let v2 = GuideSuite2::new()?;
        call_suite_fn!(v2, AEGP_SetLayerGuide2, layer_handle.as_ptr(), index, guide.orientation.into(), guide.position_type.into(), guide.position, &guide.color, guide.pinned as _)
    }

    /// New in AE 26.2. Replaces all properties of the item guide at `index`.
    ///
    /// The position is clamped to ±100,000 px or ±300%; non-finite positions are rejected.
    pub fn set_item_guide2(&self, item_handle: impl AsPtr<AEGP_ItemH>, index: i32, guide: &Guide) -> Result<(), Error> {
        let v2 = GuideSuite2::new()?;
        call_suite_fn!(v2, AEGP_SetItemGuide2, item_handle.as_ptr(), index, guide.orientation.into(), guide.position_type.into(), guide.position, &guide.color, guide.pinned as _)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_GuideOrientationType,
    GuideOrientation {
        Horizontal = ae_sys::AEGP_GuideOrientationType_HORIZONTAL,
        Vertical   = ae_sys::AEGP_GuideOrientationType_VERTICAL,
    }
}

define_enum! {
    ae_sys::AEGP_GuidePositionType,
    GuidePositionType {
        /// The position is in pixels.
        Pixel      = ae_sys::AEGP_GuidePositionType_PIXEL,
        /// The position is a percentage of the composition, footage or layer size.
        Percentage = ae_sys::AEGP_GuidePositionType_PERCENTAGE,
    }
}

/// All properties of a guide, as used by the `*2` functions of [`GuideSuite`].
#[derive(Debug, Clone, Copy)]
pub struct Guide {
    pub orientation: GuideOrientation,
    pub position_type: GuidePositionType,
    pub position: f64,
    /// Red, green and blue in the range `0.0..=1.0`.
    pub color: ae_sys::AEGP_GuideColor,
    /// Whether the guide is pinned to the edge it is measured from.
    pub pinned: bool,
}

struct RawGuide {
    orientation: ae_sys::AEGP_GuideOrientationType,
    position_type: ae_sys::AEGP_GuidePositionType,
    position: ae_sys::A_FpLong,
    color: ae_sys::AEGP_GuideColor,
    pinned: ae_sys::A_Boolean,
}

impl RawGuide {
    fn zeroed() -> Self {
        // All fields are plain C data, for which all-zero is a valid value.
        unsafe { std::mem::zeroed() }
    }
}

impl From<RawGuide> for Guide {
    fn from(raw: RawGuide) -> Self {
        Self {
            orientation: raw.orientation.into(),
            position_type: raw.position_type.into(),
            position: raw.position,
            color: raw.color,
            pinned: raw.pinned != 0,
        }
    }
}
