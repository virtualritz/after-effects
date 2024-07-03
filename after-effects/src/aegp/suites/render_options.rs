use crate::*;
use crate::aegp::*;
use ae_sys::{ AEGP_ItemH, AEGP_RenderOptionsH, A_Time, AEGP_WorldType, AEGP_MatteMode, PF_Field, AEGP_ChannelOrder, AEGP_ItemQuality, A_Boolean };

define_suite!(
    /// Since we introduced the AEGP API, we've been asked to provide functions for retrieving rendered frames.
    ///
    /// These function suites allows you to do just that.
    ///
    /// First, specify what you want rendered in the [`aegp::suites::RenderOptions`] or [`aegp::suites::LayerRenderOptions`].
    ///
    /// Then do the rendering with [`aegp::suites::Render`].
    RenderOptionsSuite,
    AEGP_RenderOptionsSuite4,
    kAEGPRenderOptionsSuite,
    kAEGPRenderOptionsSuiteVersion4
);

impl RenderOptionsSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns the [`RenderOptionsHandle`] associated with a given [`aegp::Item`].
    /// If there are no options yet specified, After Effects passes back an [`RenderOptionsHandle`] with render time set to 0,
    /// time step set to the current frame duration, field render set to ``PF_Field_FRAME``, and the depth set to the highest resolution specified within the item.
    ///
    /// The returned object will be disposed on drop
    pub fn new_from_item(&self, item: impl AsPtr<AEGP_ItemH>, plugin_id: aegp::PluginId) -> Result<RenderOptionsHandle, Error> {
        Ok(RenderOptionsHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_NewFromItem -> AEGP_RenderOptionsH, plugin_id, item.as_ptr())?
        ))
    }

    /// Duplicates the given [`RenderOptionsHandle`].
    pub fn duplicate(&self, options: impl AsPtr<AEGP_RenderOptionsH>, plugin_id: aegp::PluginId) -> Result<RenderOptionsHandle, Error> {
        Ok(RenderOptionsHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_Duplicate -> AEGP_RenderOptionsH, plugin_id, options.as_ptr())?
        ))
    }

    /// Deletes the given [`RenderOptionsHandle`].
    pub fn dispose(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_Dispose, options.as_ptr())
    }

    /// Sets the render time of the given [`RenderOptionsHandle`].
    pub fn set_time(&self, options: impl AsPtr<AEGP_RenderOptionsH>, time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetTime, options.as_ptr(), time.into())
    }

    /// Retrieves the render time of the given [`RenderOptionsHandle`].
    pub fn time(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetTime -> A_Time, options.as_ptr())?.into())
    }

    /// Specifies the time step (duration of a frame) for the referenced [`RenderOptionsHandle`].
    pub fn set_time_step(&self, options: impl AsPtr<AEGP_RenderOptionsH>, time_step: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetTimeStep, options.as_ptr(), time_step.into())
    }

    /// Retrieves the time step (duration of a frame) for the given [`RenderOptionsHandle`].
    pub fn time_step(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetTimeStep -> A_Time, options.as_ptr())?.into())
    }

    /// Specifies the field settings for the given [`RenderOptionsHandle`].
    pub fn set_field_render(&self, options: impl AsPtr<AEGP_RenderOptionsH>, field_render: pf::Field) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetFieldRender, options.as_ptr(), field_render.into())
    }

    /// Retrieves the field settings for the given [`RenderOptionsHandle`].
    pub fn field_render(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<pf::Field, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetFieldRender -> PF_Field, options.as_ptr())?.into())
    }

    /// Specifies the AEGP_WorldType of the output of a given [`RenderOptionsHandle`].
    pub fn set_world_type(&self, options: impl AsPtr<AEGP_RenderOptionsH>, typ: aegp::WorldType) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetWorldType, options.as_ptr(), typ.into())
    }

    /// Retrieves the AEGP_WorldType of the given [`RenderOptionsHandle`].
    pub fn world_type(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<aegp::WorldType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetWorldType -> AEGP_WorldType, options.as_ptr())?.into())
    }

    /// Specifies the downsample factor (with independent horizontal and vertical settings) for the given [`RenderOptionsHandle`].
    pub fn set_downsample_factor(&self, options: impl AsPtr<AEGP_RenderOptionsH>, x: i16, y: i16) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetDownsampleFactor, options.as_ptr(), x, y)
    }

    /// Retrieves the downsample factor for the given [`RenderOptionsHandle`].
    pub fn downsample_factor(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<(i16, i16), Error> {
        call_suite_fn_double!(self, AEGP_GetDownsampleFactor -> i16, i16, options.as_ptr())
    }

    /// Specifies the region of interest sub-rectangle for the given [`RenderOptionsHandle`].
    pub fn set_region_of_interest(&self, options: impl AsPtr<AEGP_RenderOptionsH>, roi: Rect) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRegionOfInterest, options.as_ptr(), &roi.into())
    }

    /// Retrieves the region of interest sub-rectangle for the given [`RenderOptionsHandle`].
    pub fn region_of_interest(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<Rect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRegionOfInterest -> ae_sys::A_LRect, options.as_ptr())?.into())
    }

    /// Specifies the matte mode for the given [`RenderOptionsHandle`].
    /// `mode` will be one of the following:
    /// - [`MatteMode::Straight`]
    /// - [`MatteMode::PremulBlack`]
    /// - [`MatteMode::PremulBgColor`]
    pub fn set_matte_mode(&self, options: impl AsPtr<AEGP_RenderOptionsH>, mode: MatteMode) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMatteMode, options.as_ptr(), mode.into())
    }

    /// Retrieves the matte mode for the given [`RenderOptionsHandle`].
    pub fn matte_mode(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<MatteMode, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMatteMode -> AEGP_MatteMode, options.as_ptr())?.into())
    }

    /// Specifies the [`ChannelOrder`] for the given [`RenderOptionsHandle`].
    ///
    /// Factoid: this was added to facilitate live linking with Premiere Pro.
    pub fn set_channel_order(&self, options: impl AsPtr<AEGP_RenderOptionsH>, channel_order: ChannelOrder) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetChannelOrder, options.as_ptr(), channel_order.into())
    }

    /// Retrieves the [`ChannelOrder`] for the given [`RenderOptionsHandle`].
    pub fn channel_order(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<ChannelOrder, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetChannelOrder -> AEGP_ChannelOrder, options.as_ptr())?.into())
    }

    /// Passes back a boolean that is true if the render guide layers setting is on.
    pub fn render_guide_layers(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRenderGuideLayers -> A_Boolean, options.as_ptr())? != 0)
    }

    /// Specify whether or not to render guide layers.
    pub fn set_render_guide_layers(&self, options: impl AsPtr<AEGP_RenderOptionsH>, render_them: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRenderGuideLayers, options.as_ptr(), render_them as _)
    }

    /// Get the render quality of the render queue item.
    /// Quality can be either [`ItemQuality::Draft`] or [`ItemQuality::Best`].
    pub fn render_quality(&self, options: impl AsPtr<AEGP_RenderOptionsH>) -> Result<ItemQuality, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRenderQuality -> AEGP_ItemQuality, options.as_ptr())?.into())
    }

    /// Set the render quality of the render queue item.
    /// Quality can be either [`ItemQuality::Draft`] or [`ItemQuality::Best`].
    pub fn set_render_quality(&self, options: impl AsPtr<AEGP_RenderOptionsH>, quality: ItemQuality) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRenderQuality, options.as_ptr(), quality.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_RenderOptionsH);
define_owned_handle_wrapper!(RenderOptionsHandle, AEGP_RenderOptionsH);
impl Drop for RenderOptionsHandle {
    fn drop(&mut self) {
        if self.is_owned() {
            RenderOptionsSuite::new().unwrap().dispose(self.as_ptr()).unwrap();
        }
    }
}

define_enum! {
    ae_sys::AEGP_ItemQuality,
    ItemQuality {
        Draft = ae_sys::AEGP_ItemQuality_DRAFT,
        Best  = ae_sys::AEGP_ItemQuality_BEST,
    }
}

define_enum! {
    ae_sys::AEGP_ChannelOrder,
    ChannelOrder {
        Argb = ae_sys::AEGP_ChannelOrder_ARGB,
        Bgra = ae_sys::AEGP_ChannelOrder_BGRA,
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_RenderOptionsH, RenderOptionsHandle,
    suite: RenderOptionsSuite,
    /// New in 13.0
    RenderOptions {
        dispose: ; // Handled by the Drop impl for RenderOptionsHandle

        /// Duplicates the given [`RenderOptionsHandle`].
        duplicate(plugin_id: aegp::PluginId) -> RenderOptions => suite.duplicate,

        /// Sets the render time of the given [`RenderOptionsHandle`].
        set_time(time: Time) -> () => suite.set_time,

        /// Retrieves the render time of the given [`RenderOptionsHandle`].
        time() -> Time => suite.time,

        /// Specifies the time step (duration of a frame) for the referenced [`RenderOptionsHandle`].
        set_time_step(time_step: Time) -> () => suite.set_time_step,

        /// Retrieves the time step (duration of a frame) for the given [`RenderOptionsHandle`].
        time_step() -> Time => suite.time_step,

        /// Specifies the field settings for the given [`RenderOptionsHandle`].
        set_field_render(field_render: pf::Field) -> () => suite.set_field_render,

        /// Retrieves the field settings for the given [`RenderOptionsHandle`].
        field_render() -> pf::Field => suite.field_render,

        /// Specifies the AEGP_WorldType of the output of a given [`RenderOptionsHandle`].
        set_world_type(typ: aegp::WorldType) -> () => suite.set_world_type,

        /// Retrieves the AEGP_WorldType of the given [`RenderOptionsHandle`].
        world_type() -> aegp::WorldType => suite.world_type,

        /// Specifies the downsample factor (with independent horizontal and vertical settings) for the given [`RenderOptionsHandle`].
        set_downsample_factor(x: i16, y: i16) -> () => suite.set_downsample_factor,

        /// Retrieves the downsample factor for the given [`RenderOptionsHandle`].
        downsample_factor() -> (i16, i16) => suite.downsample_factor,

        /// Specifies the region of interest sub-rectangle for the given [`RenderOptionsHandle`].
        set_region_of_interest(roi: Rect) -> () => suite.set_region_of_interest,

        /// Retrieves the region of interest sub-rectangle for the given [`RenderOptionsHandle`].
        region_of_interest() -> Rect => suite.region_of_interest,

        /// Specifies the matte mode for the given [`RenderOptionsHandle`].
        /// `mode` will be one of the following:
        /// - [`MatteMode::Straight`]
        /// - [`MatteMode::PremulBlack`]
        /// - [`MatteMode::PremulBgColor`]
        set_matte_mode(mode: MatteMode) -> () => suite.set_matte_mode,

        /// Retrieves the matte mode for the given [`RenderOptionsHandle`].
        matte_mode() -> MatteMode => suite.matte_mode,

        /// Specifies the [`ChannelOrder`] for the given [`RenderOptionsHandle`].
        ///
        /// Factoid: this was added to facilitate live linking with Premiere Pro.
        set_channel_order(channel_order: ChannelOrder) -> () => suite.set_channel_order,

        /// Retrieves the [`ChannelOrder`] for the given [`RenderOptionsHandle`].
        channel_order() -> ChannelOrder => suite.channel_order,

        /// Passes back a boolean that is true if the render guide layers setting is on.
        render_guide_layers() -> bool => suite.render_guide_layers,

        /// Specify whether or not to render guide layers.
        set_render_guide_layers(render_them: bool) -> () => suite.set_render_guide_layers,

        /// Get the render quality of the render queue item.
        /// Quality can be either [`ItemQuality::Draft`] or [`ItemQuality::Best`].
        render_quality() -> ItemQuality => suite.render_quality,

        /// Set the render quality of the render queue item.
        /// Quality can be either [`ItemQuality::Draft`] or [`ItemQuality::Best`].
        set_render_quality(quality: ItemQuality) -> () => suite.set_render_quality,
    }
);

impl RenderOptions {
    /// Returns the [`RenderOptionsHandle`] associated with a given [`aegp::Item`].
    /// If there are no options yet specified, After Effects passes back an [`RenderOptionsHandle`] with render time set to 0,
    /// time step set to the current frame duration, field render set to ``PF_Field_FRAME``, and the depth set to the highest resolution specified within the item.
    ///
    /// The returned object will be disposed on drop
    pub fn from_item(layer: impl AsPtr<AEGP_ItemH>, plugin_id: aegp::PluginId) -> Result<Self, Error> {
        Ok(Self::from_handle(
            RenderOptionsSuite::new().unwrap().new_from_item(layer, plugin_id)?,
            false
        ))
    }
}
