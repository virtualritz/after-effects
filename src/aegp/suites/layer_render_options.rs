use crate::*;
use ae_sys::{ AEGP_LayerH, AEGP_EffectRefH, AEGP_LayerRenderOptionsH, A_Time, AEGP_WorldType, AEGP_MatteMode };

define_suite!(
    /// New in 13.0
    LayerRenderOptionsSuite,
    AEGP_LayerRenderOptionsSuite2,
    kAEGPLayerRenderOptionsSuite,
    kAEGPLayerRenderOptionsSuiteVersion2
);

impl LayerRenderOptionsSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns the [`LayerRenderOptionsHandle`] associated with a given `AEGP_LayerH`.
    /// Render time is set to the layer's current time, time step is set to layer's frame duration,
    /// ROI to the layer's nominal bounds, and EffectsToRender to "all".
    ///
    /// The returned object will be disposed on drop
    pub fn new_from_layer(&self, layer: impl AsPtr<AEGP_LayerH>, plugin_id: aegp::PluginId) -> Result<LayerRenderOptionsHandle, Error> {
        Ok(LayerRenderOptionsHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_NewFromLayer -> AEGP_LayerRenderOptionsH, plugin_id, layer.as_ptr())?
        ))
    }

    /// Returns the [`LayerRenderOptionsHandle`] from the layer associated with a given `AEGP_EffectRefH`.
    /// Render time is set to the layer's current time, time step is set to layer's frame duration,
    /// ROI to the layer's nominal bounds, and EffectsToRender to the index of `effect_ref`.
    ///
    /// The returned object will be disposed on drop
    pub fn new_from_upstream_of_effect(&self, effect_ref: impl AsPtr<AEGP_EffectRefH>, plugin_id: aegp::PluginId) -> Result<LayerRenderOptionsHandle, Error> {
        Ok(LayerRenderOptionsHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_NewFromUpstreamOfEffect -> AEGP_LayerRenderOptionsH, plugin_id, effect_ref.as_ptr())?
        ))
    }

    /// Duplicates the given [`LayerRenderOptionsHandle`].
    pub fn duplicate(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, plugin_id: aegp::PluginId) -> Result<LayerRenderOptionsHandle, Error> {
        Ok(LayerRenderOptionsHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_Duplicate -> AEGP_LayerRenderOptionsH, plugin_id, options.as_ptr())?
        ))
    }

    /// Deletes the given [`LayerRenderOptionsHandle`].
    pub fn dispose(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_Dispose, options.as_ptr())
    }

    /// Sets the render time of the given [`LayerRenderOptionsHandle`].
    pub fn set_time(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetTime, options.as_ptr(), time.into())
    }

    /// Retrieves the render time of the given [`LayerRenderOptionsHandle`].
    pub fn time(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetTime -> A_Time, options.as_ptr())?.into())
    }

    /// Specifies the time step (duration of a frame) for the referenced [`LayerRenderOptionsHandle`].
    pub fn set_time_step(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, time_step: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetTimeStep, options.as_ptr(), time_step.into())
    }

    /// Retrieves the time step (duration of a frame) for the given [`LayerRenderOptionsHandle`].
    pub fn time_step(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetTimeStep -> A_Time, options.as_ptr())?.into())
    }

    /// Specifies the AEGP_WorldType of the output of a given [`LayerRenderOptionsHandle`].
    pub fn set_world_type(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, typ: aegp::WorldType) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetWorldType, options.as_ptr(), typ.into())
    }

    /// Retrieves the AEGP_WorldType of the given [`LayerRenderOptionsHandle`].
    pub fn world_type(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<aegp::WorldType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetWorldType -> AEGP_WorldType, options.as_ptr())?.into())
    }

    /// Specifies the downsample factor (with independent horizontal and vertical settings) for the given [`LayerRenderOptionsHandle`].
    pub fn set_downsample_factor(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, x: i16, y: i16) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetDownsampleFactor, options.as_ptr(), x, y)
    }

    /// Retrieves the downsample factor for the given [`LayerRenderOptionsHandle`].
    pub fn downsample_factor(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<(i16, i16), Error> {
        call_suite_fn_double!(self, AEGP_GetDownsampleFactor -> i16, i16, options.as_ptr())
    }

    /// Specifies the matte mode for the given [`LayerRenderOptionsHandle`].
    /// `mode` will be one of the following:
    /// - [`MatteMode::Straight`]
    /// - [`MatteMode::PremulBlack`]
    /// - [`MatteMode::PremulBgColor`]
    pub fn set_matte_mode(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>, mode: MatteMode) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetMatteMode, options.as_ptr(), mode.into())
    }

    /// Retrieves the matte mode for the given [`LayerRenderOptionsHandle`].
    pub fn matte_mode(&self, options: impl AsPtr<AEGP_LayerRenderOptionsH>) -> Result<MatteMode, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetMatteMode -> AEGP_MatteMode, options.as_ptr())?.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_LayerRenderOptionsH);
define_owned_handle_wrapper!(LayerRenderOptionsHandle, AEGP_LayerRenderOptionsH);
impl Drop for LayerRenderOptionsHandle {
    fn drop(&mut self) {
        if self.is_owned() {
            LayerRenderOptionsSuite::new().unwrap().dispose(self.as_ptr()).unwrap();
        }
    }
}

define_enum! {
    ae_sys::AEGP_MatteMode,
    MatteMode {
        Straight      = ae_sys::AEGP_MatteMode_STRAIGHT,
        PremulBlack   = ae_sys::AEGP_MatteMode_PREMUL_BLACK,
        PremulBgColor = ae_sys::AEGP_MatteMode_PREMUL_BG_COLOR,
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_LayerRenderOptionsH, LayerRenderOptionsHandle,
    suite: LayerRenderOptionsSuite,
    /// New in 13.0
    LayerRenderOptions {
        dispose: ; // Handled by the Drop impl for LayerRenderOptionsHandle

        /// Duplicates the given [`LayerRenderOptionsHandle`].
        duplicate(plugin_id: aegp::PluginId) -> LayerRenderOptions => suite.duplicate,

        /// Sets the render time of the given [`LayerRenderOptionsHandle`].
        set_time(time: Time) -> () => suite.set_time,

        /// Retrieves the render time of the given [`LayerRenderOptionsHandle`].
        time() -> Time => suite.time,

        /// Specifies the time step (duration of a frame) for the referenced [`LayerRenderOptionsHandle`].
        set_time_step( time_step: Time) -> () => suite.set_time_step,

        /// Retrieves the time step (duration of a frame) for the given [`LayerRenderOptionsHandle`].
        time_step() -> Time => suite.time_step,

        /// Specifies the AEGP_WorldType of the output of a given [`LayerRenderOptionsHandle`].
        set_world_type(typ: aegp::WorldType) -> () => suite.set_world_type,

        /// Retrieves the AEGP_WorldType of the given [`LayerRenderOptionsHandle`].
        world_type() -> aegp::WorldType => suite.world_type,

        /// Specifies the downsample factor (with independent horizontal and vertical settings) for the given [`LayerRenderOptionsHandle`].
        set_downsample_factor(x: i16, y: i16) -> () => suite.set_downsample_factor,

        /// Retrieves the downsample factor for the given [`LayerRenderOptionsHandle`].
        downsample_factor() -> (i16, i16) => suite.downsample_factor,

        /// Specifies the matte mode for the given [`LayerRenderOptionsHandle`].
        /// `mode` will be one of the following:
        /// - [`MatteMode::Straight`]
        /// - [`MatteMode::PremulBlack`]
        /// - [`MatteMode::PremulBgColor`]
        set_matte_mode(mode: MatteMode) -> () => suite.set_matte_mode,

        /// Retrieves the matte mode for the given [`LayerRenderOptionsHandle`].
        matte_mode() -> MatteMode => suite.matte_mode,
    }
);

impl LayerRenderOptions {
    /// Returns the [`LayerRenderOptionsHandle`] associated with a given `AEGP_LayerH`.
    /// Render time is set to the layer's current time, time step is set to layer's frame duration,
    /// ROI to the layer's nominal bounds, and EffectsToRender to "all".
    ///
    /// The returned object will be disposed on drop
    pub fn from_layer(layer: impl AsPtr<AEGP_LayerH>, plugin_id: aegp::PluginId) -> Result<Self, Error> {
        Ok(Self::from_handle(
            LayerRenderOptionsSuite::new().unwrap().new_from_layer(layer, plugin_id)?,
            false
        ))
    }

    /// Returns the [`LayerRenderOptionsHandle`] from the layer associated with a given `AEGP_EffectRefH`.
    /// Render time is set to the layer's current time, time step is set to layer's frame duration,
    /// ROI to the layer's nominal bounds, and EffectsToRender to the index of `effect_ref`.
    ///
    /// The returned object will be disposed on drop
    pub fn from_upstream_of_effect(effect_ref: impl AsPtr<AEGP_EffectRefH>, plugin_id: aegp::PluginId) -> Result<Self, Error> {
        Ok(Self::from_handle(
            LayerRenderOptionsSuite::new().unwrap().new_from_upstream_of_effect(effect_ref, plugin_id)?,
            false
        ))
    }
}
