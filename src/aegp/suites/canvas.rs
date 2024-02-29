use crate::*;
use crate::aegp::*;
use ae_sys::{ PR_RenderContextH, AEGP_RenderLayerContextH };

define_suite!(
    /// [`render_texture()`](Self::render_texture) supplies the raw pixels of a layer, untransformed, into an arbitrarily-sized buffer.
    ///
    /// [`render_layer_plus()`](Self::render_layer_plus) invokes the entire After Effects render pipeline, including transforms, masking, et cetera, providing the layer as it appears in its composition, in a composition-sized buffer.
    ///
    /// If the layer being rendered is 3D, the default (Standard 3D) Artisan is invoked to perform any 3D geometrics.
    ///
    /// Your Artisan can use this to render track matte layers, and apply them only in a strictly 2D sense, to the transformed 3D layer.
    ///
    /// Before rendering, the Artisans that ship with After Effects apply an inverse transform to get square pixels, then re-apply the transform before display.
    ///
    /// For example, if the pixel aspect ratio is 10/11 (DV NTSC), we multiply by 11/10 to get square pixels. We process and composite 3D layers, then re-divide to get back to the original pixel aspect ratio.
    ///
    /// The following suite supplies the layers, compositions, texture and destination buffers. This is a vital suite for all artisans.
    CanvasSuite,
    AEGP_CanvasSuite8,
    kAEGPCanvasSuite,
    kAEGPCanvasSuiteVersion8
);

impl CanvasSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given the render context provided to the Artisan at render time, returns a handle to the composition.
    pub fn comp_to_render(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<CompHandle, Error> {
        Ok(CompHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetCompToRender -> ae_sys::AEGP_CompH, render_ctx.as_ptr())?
        ))
    }

    /// Given the render context, returns the number of layers the Artisan needs to render.
    pub fn num_layers_to_render(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<u32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumLayersToRender -> i32, render_ctx.as_ptr())? as u32)
    }

    /// Used to build a list of layers to render after determining the total number of layers that need rendering by the Artisan.
    pub fn nth_layer_context_to_render(&self, render_ctx: impl AsPtr<PR_RenderContextH>, n: u32) -> Result<RenderLayerContextHandle, Error> {
        Ok(RenderLayerContextHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_GetNthLayerContextToRender -> ae_sys::AEGP_RenderLayerContextH,
                render_ctx.as_ptr(),
                n as i32
            )?,
        ))
    }

    /// Given a [`RenderContextHandle`], retrieves the associated [`LayerHandle`] (required by many suite functions).
    pub fn layer_from_layer_context(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_GetLayerFromLayerContext -> ae_sys::AEGP_LayerH,
                render_ctx.as_ptr(),
                layer_ctx.as_ptr()
            )?
        ))
    }

    /// Allows for rendering of sub-layers (as within a Photoshop file).
    pub fn layer_and_sub_layer_from_layer_context(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>) -> Result<(LayerHandle, u32), Error> {
        let (layer_handle, sub_layer) = call_suite_fn_double!(self,
            AEGP_GetLayerAndSubLayerFromLayerContext -> ae_sys::AEGP_LayerH, ae_sys::AEGP_SubLayerIndex,
            render_ctx.as_ptr(),
            layer_ctx.as_ptr()
        )?;
        Ok((
            LayerHandle::from_raw(layer_handle),
            sub_layer as u32
        ))
    }

    /// With collapsed geometrics "on" this gives the layer in the root composition containing the layer context.
    ///
    /// With collapsed geometrics off this is the same as [`layer_from_layer_context()`](Self::layer_from_layer_context).
    pub fn top_layer_from_layer_context(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetTopLayerFromLayerContext -> ae_sys::AEGP_LayerH, render_ctx.as_ptr(), layer_ctx.as_ptr())?
        ))
    }

    /// Given the render context, returns the current point in (composition) time to render.
    pub fn comp_render_time(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(Time, Time), Error> {
        let (shutter_frame_start, shutter_frame_duration) =
            call_suite_fn_double!(self, AEGP_GetCompRenderTime -> ae_sys::A_Time, ae_sys::A_Time, render_ctx.as_ptr())?;

        Ok((shutter_frame_start.into(), shutter_frame_duration.into()))
    }

    /// Given the render context, returns a buffer in which to place the final rendered output.
    pub fn comp_destination_buffer(&self, render_ctx: impl AsPtr<PR_RenderContextH>, comp_handle: CompHandle) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetCompDestinationBuffer -> ae_sys::AEGP_WorldH, render_ctx.as_ptr(), comp_handle.as_ptr())?
        ))
    }

    /// Given the render context provided to the Artisan at render time, returns a handle to the composition.
    pub fn region_of_interest(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<Rect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetROI -> ae_sys::A_LegacyRect, render_ctx.as_ptr())?.into())
    }

    /// Given the render context and layer, returns the layer texture.
    ///
    /// The returned [`WorldHandle`] can be null.
    ///
    /// [`RenderHints::NoTransferMode`] prevents application of opacity & transfer mode; for use with `RenderLayer` calls.
    pub fn render_texture(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, render_hints: RenderHints, suggested_scale: Option<FloatPoint>, suggested_src_rect: Option<FloatRect>, src_matrix: Option<Matrix3>) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_RenderTexture -> ae_sys::AEGP_WorldH,
                render_ctx.as_ptr(),
                layer_ctx.as_ptr(),
                render_hints as i32 as _,
                suggested_scale   .map(|x| &mut x.into() as *mut _).unwrap_or(std::ptr::null_mut()),
                suggested_src_rect.map(|x| &mut x.into() as *mut _).unwrap_or(std::ptr::null_mut()),
                src_matrix        .map(|x| &mut x.into() as *mut _).unwrap_or(std::ptr::null_mut())
            )?
        ))
    }

    /// Disposes of an acquired layer texture.
    pub fn dispose_texture(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, world_handle: WorldHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeTexture, render_ctx.as_ptr(), layer_ctx.as_ptr(), world_handle.as_ptr())
    }

    /// Returns the field settings of the given [`RenderContextHandle`].
    pub fn field_render(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<ae_sys::PF_Field, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetFieldRender -> ae_sys::PF_Field, render_ctx.as_ptr())?)
    }

    /// Given the render context provided to the Artisan at render time, returns a handle to the composition.
    ///
    /// Note: this is NOT thread-safe on macOS; only use this function when the current thread ID is 0.
    pub fn report_artisan_progress(&self, render_ctx: impl AsPtr<PR_RenderContextH>, count: i32, total: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReportArtisanProgress, render_ctx.as_ptr(), count, total)
    }

    /// Returns the downsample factor of the [`RenderContextHandle`].
    pub fn render_downsample_factor(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<ae_sys::AEGP_DownsampleFactor, Error> {
        let dsf = call_suite_fn_single!(self, AEGP_GetRenderDownsampleFactor -> ae_sys::AEGP_DownsampleFactor, render_ctx.as_ptr())?;
        Ok(dsf.into())
    }
    pub fn set_render_downsample_factor(&self, render_ctx: impl AsPtr<PR_RenderContextH>, mut dsf: ae_sys::AEGP_DownsampleFactor) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetRenderDownsampleFactor, render_ctx.as_ptr(), &mut dsf as *mut _)
    }

    /// Determines whether the [`RenderContextHandle`] is blank (empty).
    pub fn is_blank_canvas(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsBlankCanvas -> ae_sys::A_Boolean, render_ctx.as_ptr())? != 0)
    }

    /// Given a render context and a layer (at a given time), retrieves the 4 by 4 transform to move between their coordinate spaces.
    pub fn render_layer_to_world_xform(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time) -> Result<Matrix4, Error> {
        let matrix = call_suite_fn_single!(self, AEGP_GetRenderLayerToWorldXform -> ae_sys::A_Matrix4, render_ctx.as_ptr(), layer_ctx.as_ptr(), &comp_time.into() as *const _)?;
        Ok(matrix.into())
    }

    /// Retrieves the bounding rectangle of the layer_contextH (at a given time) within the [`RenderContextHandle`].
    pub fn render_layer_bounds(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time) -> Result<Rect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRenderLayerBounds -> ae_sys::A_LegacyRect, render_ctx.as_ptr(), layer_ctx.as_ptr(), &comp_time.into() as *const _)?.into())
    }

    /// Returns the opacity of the given layer context at the given time, within the render context.
    pub fn render_opacity(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetRenderOpacity -> f64, render_ctx.as_ptr(), layer_ctx.as_ptr(), &comp_time.into() as *const _)
    }

    /// Returns whether or not a given layer context is active within the render context, at the given time.
    pub fn is_render_layer_active(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsRenderLayerActive -> ae_sys::A_Boolean, render_ctx.as_ptr(), layer_ctx.as_ptr(), &comp_time.into() as *const _)? != 0)
    }

    /// Sets the progress information for a rendering Artisan.
    ///
    /// * `count` is the number of layers completed
    /// * `num_layers` is the total number of layers the Artisan is rendering
    pub fn set_artisan_layer_progress(&self, render_ctx: impl AsPtr<PR_RenderContextH>, count: i32, num_layers: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetArtisanLayerProgress, render_ctx.as_ptr(), count, num_layers)
    }

    /// Invokes the entire After Effects render pipeline, including transforms, masking, et cetera,
    /// providing the layer as it appears in its composition, in a composition-sized buffer.
    pub fn render_layer_plus(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_handle: LayerHandle, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, render_hints: RenderHints) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_RenderLayerPlus -> ae_sys::AEGP_WorldH, render_ctx.as_ptr(), layer_handle.as_ptr(), layer_ctx.as_ptr(), render_hints as i32 as _)?
        ))
    }

    /// Retrieves the [`RenderLayerContextHandle`] for the specified render and fill contexts.
    pub fn track_matte_context(&self, render_ctx: impl AsPtr<PR_RenderContextH>, fill_ctx: RenderLayerContextHandle) -> Result<RenderLayerContextHandle, Error> {
        Ok(RenderLayerContextHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetTrackMatteContext -> ae_sys::AEGP_RenderLayerContextH, render_ctx.as_ptr(), fill_ctx.as_ptr())?
        ))
    }

    /// Renders a texture into an [`WorldHandle`], and provides an [`RenderReceiptHandle`] for the operation.
    pub fn render_texture_with_receipt(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, render_hints: RenderHints, num_effects: RenderNumEffects, suggested_scale: Option<FloatPoint>, suggested_src_rect: Option<FloatRect>, src_matrix: Option<Matrix3>) -> Result<(RenderReceiptHandle, WorldHandle), Error> {
        let suggested_scale    = suggested_scale   .map(|x| &mut Into::<ae_sys::A_FloatPoint>::into(x) as *mut _).unwrap_or(std::ptr::null_mut());
        let suggested_src_rect = suggested_src_rect.map(|x| &mut Into::<ae_sys::A_FloatRect> ::into(x) as *mut _).unwrap_or(std::ptr::null_mut());
        let src_matrix         = src_matrix        .map(|x| &mut Into::<ae_sys::A_Matrix3>   ::into(x) as *mut _).unwrap_or(std::ptr::null_mut());
        let (receipt, world) = call_suite_fn_double!(self,
            AEGP_RenderTextureWithReceipt -> ae_sys::AEGP_RenderReceiptH, ae_sys::AEGP_WorldH,
            render_ctx.as_ptr(),
            layer_ctx.as_ptr(),
            render_hints as i32 as _,
            num_effects.into(),
            suggested_scale,
            suggested_src_rect,
            src_matrix
        )?;
        Ok((
            RenderReceiptHandle::from_raw(receipt),
            WorldHandle::from_raw(world)
        ))
    }

    /// Returns the number of software effects applied in the given [`RenderLayerContextHandle`].
    pub fn number_of_software_effects(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumberOfSoftwareEffects -> i16, render_ctx.as_ptr(), layer_ctx.as_ptr())? as i32)
    }

    /// An improvement over [`render_layer_plus()`](Self::render_layer_plus), this function also provides an [`RenderReceiptHandle`] for caching purposes.
    pub fn render_layer_plus_with_receipt(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_handle: LayerHandle, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, render_hints: RenderHints) -> Result<(RenderReceiptHandle, WorldHandle), Error> {
        let (receipt, world) = call_suite_fn_double!(self,
            AEGP_RenderLayerPlusWithReceipt -> ae_sys::AEGP_RenderReceiptH, ae_sys::AEGP_WorldH,
            render_ctx.as_ptr(),
            layer_handle.as_ptr(),
            layer_ctx.as_ptr(),
            render_hints as i32 as _
        )?;

        Ok((
            RenderReceiptHandle::from_raw(receipt),
            WorldHandle::from_raw(world))
        )
    }

    /// Frees an [`ae_sys::AEGP_RenderReceiptH`]
    ///
    /// This is called automatically on [`RenderReceiptHandle::drop()`]
    pub fn dispose_render_receipt(&self, render_receipt_handle: ae_sys::AEGP_RenderReceiptH) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeRenderReceipt, render_receipt_handle)
    }

    /// Checks with After Effects' internal caching to determine whether a given [`RenderReceiptHandle`] is still valid.
    pub fn check_render_receipt(&self, current_render_ctx: impl AsPtr<PR_RenderContextH>, current_layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, old_render_receipt_handle: RenderReceiptHandle, check_geometrics: bool, num_effects: RenderNumEffects) -> Result<RenderReceiptStatus, Error> {
        Ok(call_suite_fn_single!(self,
            AEGP_CheckRenderReceipt -> ae_sys::AEGP_RenderReceiptStatus,
            current_render_ctx.as_ptr(),
            current_layer_ctx.as_ptr(),
            old_render_receipt_handle.as_ptr(),
            check_geometrics as i32 as _,
            num_effects.into()
        )?.into())
    }

    /// Generates a [`RenderReceiptHandle`] for a layer as if the first `num_effects` have been rendered.
    pub fn generate_render_receipt(&self, current_render_ctx: impl AsPtr<PR_RenderContextH>, current_layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, num_effects: RenderNumEffects) -> Result<RenderReceiptHandle, Error> {
        Ok(RenderReceiptHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_GenerateRenderReceipt -> ae_sys::AEGP_RenderReceiptH,
                current_render_ctx.as_ptr(),
                current_layer_ctx.as_ptr(),
                num_effects.into()
            )?
        ))
    }

    /// Returns the number of bins After Effects wants the artisan to render.
    pub fn num_bins_to_render(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetNumBinsToRender -> i32, render_ctx.as_ptr())
    }

    /// Sets the given render context to be the n-th bin to be rendered by After Effects.
    pub fn set_nth_bin(&self, render_ctx: impl AsPtr<PR_RenderContextH>, n: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetNthBin, render_ctx.as_ptr(), n)
    }

    /// Retrieves the type of the given bin.
    pub fn bin_type(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<BinType, Error> {
        Ok(call_suite_fn_single!(self,
            AEGP_GetBinType -> ae_sys::AEGP_BinType,
            render_ctx.as_ptr()
        )?.into())
    }

    /// Retrieves the transform to correctly orient the layer being rendered with the output world.
    ///
    /// Pass `true` for `only_2dB` to constrain the transform to two dimensions.
    pub fn render_layer_to_world_xform_2d_3d(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time, only_2d: bool) -> Result<Matrix4, Error> {
        Ok(call_suite_fn_single!(self,
            AEGP_GetRenderLayerToWorldXform2D3D -> ae_sys::A_Matrix4,
            render_ctx.as_ptr(),
            layer_ctx.as_ptr(),
            &comp_time.into() as *const _,
            if only_2d { 1 } else { 0 }
        )?.into())
    }

    /// Retrieves the platform-specific window context into which to draw the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn platform_window_ref(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<ae_sys::AEGP_PlatformWindowRef, Error> {
        call_suite_fn_single!(self, AEGP_GetPlatformWindowRef -> ae_sys::AEGP_PlatformWindowRef, render_ctx.as_ptr())
    }

    /// Retrieves the source-to-frame downsample factor for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn viewport_scale(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(f64, f64), Error> {
        call_suite_fn_double!(self, AEGP_GetViewportScale -> f64, f64, render_ctx.as_ptr())
    }

    /// Retrieves to origin of the source, within the frame (necessary to translate between the two), for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn viewport_origin(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(i32, i32), Error> {
        call_suite_fn_double!(self, AEGP_GetViewportOrigin -> i32, i32, render_ctx.as_ptr())
    }

    /// Retrieves the bounding rectangle for the area to be drawn, for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn viewport_rect(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<Rect, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetViewportRect -> ae_sys::A_LegacyRect, render_ctx.as_ptr())?.into())
    }

    /// Retrieves the color used for the fallow regions in the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn fallow_color(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<ae_sys::PF_Pixel8, Error> {
        call_suite_fn_single!(self, AEGP_GetFallowColor -> ae_sys::PF_Pixel8, render_ctx.as_ptr())
    }

    pub fn interactive_buffer(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetInteractiveBuffer -> ae_sys::AEGP_WorldH, render_ctx.as_ptr())?
        ))
    }

    /// Retrieves whether or not the checkerboard is currently active for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_checkerboard(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInteractiveCheckerboard -> ae_sys::A_Boolean, render_ctx.as_ptr())? != 0)
    }

    /// Retrieves the colors used in the checkerboard.
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_checkerboard_colors(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(Pixel8, Pixel8), Error> {
        let (px1, px2) = call_suite_fn_double!(self, AEGP_GetInteractiveCheckerboardColors -> ae_sys::PF_Pixel, ae_sys::PF_Pixel, render_ctx.as_ptr())?;
        Ok((
            px1.into(),
            px2.into()
        ))
    }

    /// Retrieves the width and height of one checkerboard square.
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_checkerboard_size(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(u32, u32), Error> {
        call_suite_fn_double!(self, AEGP_GetInteractiveCheckerboardSize -> u32, u32, render_ctx.as_ptr())
    }

    /// Retrieves the cached AEGP_WorldH last used for the [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_cached_buffer(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetInteractiveCachedBuffer -> ae_sys::AEGP_WorldH, render_ctx.as_ptr())?
        ))
    }

    /// Determines whether or not the artisan must render the current [`RenderLayerContextHandle`] as a layer.
    ///
    /// This function is valid for interactive artisans only.
    pub fn artisan_must_render_as_layer(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ArtisanMustRenderAsLayer -> ae_sys::A_Boolean, render_ctx.as_ptr(), layer_ctx.as_ptr())? != 0)
    }

    /// Returns which channels should be displayed by the interactive artisan.
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_display_channel(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<DisplayChannel, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInteractiveDisplayChannel -> ae_sys::AEGP_DisplayChannelType, render_ctx.as_ptr())?.into())
    }

    /// Returns the exposure for the given [`RenderContextHandle`], expressed as a floating point number.
    ///
    /// This function is valid for interactive artisans only.
    pub fn interactive_exposure(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetInteractiveExposure -> f64, render_ctx.as_ptr())
    }

    // TODO: what's xform?
    /// Returns the color transform for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn color_transform(&self, render_ctx: impl AsPtr<PR_RenderContextH>, xform: *mut std::ffi::c_void) -> Result<(bool, u32), Error> {
        let mut cms_on = 0;
        let mut xform_key = 0;
        call_suite_fn!(self, AEGP_GetColorTransform, render_ctx.as_ptr(), &mut cms_on, &mut xform_key, xform)?;
        Ok((cms_on != 0, xform_key))
    }

    /// Returns the shutter angle for the given [`RenderContextHandle`].
    ///
    /// This function is valid for interactive artisans only.
    pub fn comp_shutter_time(&self, render_ctx: impl AsPtr<PR_RenderContextH>) -> Result<(Time, Time), Error> {
        let (shutter_time, shutter_dur) = call_suite_fn_double!(self,
            AEGP_GetCompShutterTime -> ae_sys::A_Time, ae_sys::A_Time,
            render_ctx.as_ptr()
        )?;
        Ok((shutter_time.into(), shutter_dur.into()))
    }

    /// New in CC. Unlike [`suites::Layer::convert_comp_to_layer_time()`](aegp::suites::Layer::convert_comp_to_layer_time), this handles time remapping with collapsed or nested comps.
    ///
    /// This function is valid for interactive artisans only.
    pub fn map_comp_to_layer_time(&self, render_ctx: impl AsPtr<PR_RenderContextH>, layer_ctx: impl AsPtr<AEGP_RenderLayerContextH>, comp_time: Time) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_MapCompToLayerTime -> ae_sys::A_Time, render_ctx.as_ptr(), layer_ctx.as_ptr(), &comp_time.into() as *const _)?.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_RenderLayerContextH);
define_handle_wrapper!(RenderLayerContextHandle, AEGP_RenderLayerContextH);

define_enum! {
    ae_sys::AEGP_RenderHints,
    RenderHints {
        None           = ae_sys::AEGP_RenderHints_NONE,
        IgnoreExtents  = ae_sys::AEGP_RenderHints_IGNORE_EXTENTS,
        NoTransferMode = ae_sys::AEGP_RenderHints_NO_TRANSFER_MODE,
    }
}

define_enum! {
    ae_sys::AEGP_BinType,
    BinType {
        None = ae_sys::AEGP_BinType_NONE,
        TwoD = ae_sys::AEGP_BinType_2D,
        ThreeD = ae_sys::AEGP_BinType_3D,
    }
}

define_enum! {
    ae_sys::AEGP_DisplayChannelType,
    DisplayChannel {
        None     = ae_sys::AEGP_DisplayChannel_NONE,
        Red     = ae_sys::AEGP_DisplayChannel_RED,
        Green   = ae_sys::AEGP_DisplayChannel_GREEN,
        Blue    = ae_sys::AEGP_DisplayChannel_BLUE,
        Alpha    = ae_sys::AEGP_DisplayChannel_ALPHA,
        RedAlt   = ae_sys::AEGP_DisplayChannel_RED_ALT,
        GreenAlt = ae_sys::AEGP_DisplayChannel_GREEN_ALT,
        BlueAlt  = ae_sys::AEGP_DisplayChannel_BLUE_ALT,
        AlphaAlt = ae_sys::AEGP_DisplayChannel_ALPHA_ALT,
    }
}

pub enum RenderNumEffects {
    AllEffects,
    NumEffects(u16)
}
impl Into<i16> for RenderNumEffects {
    fn into(self) -> i16 {
        match self {
            RenderNumEffects::AllEffects    => -1,
            RenderNumEffects::NumEffects(x) => x as i16
        }
    }
}

define_enum! {
    ae_sys::AEGP_RenderReceiptStatus,
    RenderReceiptStatus {
        Invalid            = ae_sys::AEGP_RenderReceiptStatus_INVALID,
        Valid              = ae_sys::AEGP_RenderReceiptStatus_VALID,
        ValidButIncomplete = ae_sys::AEGP_RenderReceiptStatus_VALID_BUT_INCOMPLETE,
    }
}

#[derive(Clone, Debug, Hash)]
pub struct RenderReceiptHandle(after_effects_sys::AEGP_RenderReceiptH);
impl RenderReceiptHandle {
    pub fn from_raw(raw_handle: after_effects_sys::AEGP_RenderReceiptH) -> Self { Self(raw_handle) }
    pub fn as_ptr(&self) -> after_effects_sys::AEGP_RenderReceiptH { self.0 }
}
impl Drop for RenderReceiptHandle {
    fn drop(&mut self) {
        if let Ok(s) = CanvasSuite::new() {
            let _ = s.dispose_render_receipt(self.0);
        }
    }
}
