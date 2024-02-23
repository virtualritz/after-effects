use crate::*;
use crate::aegp::*;

define_suite!(
    /// These functions provide a way for effects (and AEGPs) to obtain information about the context of an applied effect.
    /// ## Notes On Effect Context
    /// Any time you modify or rely on data from outside the normal render pipeline, you run the risk of dependency problems.
    ///
    /// There is no way for After Effects to know that you depend on this external information; consequently, you will not be notified if it changes out from under you.
    PFInterfaceSuite,
    AEGP_PFInterfaceSuite1,
    kAEGPPFInterfaceSuite,
    kAEGPPFInterfaceSuiteVersion1
);

impl PFInterfaceSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Obtain the layer handle of the layer to which the effect is applied.
    pub fn effect_layer(&self, effect_ref: impl Into<pf::ProgPtr>) -> Result<LayerHandle, Error> {
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetEffectLayer -> ae_sys::AEGP_LayerH, effect_ref.into())?
        ))
    }

    /// Obtain the [`EffectRefHandle`] corresponding to the effect.
    pub fn new_effect_for_effect(&self, plugin_id: PluginId, effect_ref: pf::ProgPtr) -> Result<EffectRefHandle, Error> {
        Ok(EffectRefHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetNewEffectForEffect -> ae_sys::AEGP_EffectRefH, plugin_id, effect_ref.into())?
        ))
    }

    /// Retreive the composition time corresponding to the effect's layer time.
    pub fn convert_effect_to_comp_time(&self, effect_ref: pf::ProgPtr, time: i32, time_scale: u32) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ConvertEffectToCompTime -> ae_sys::A_Time, effect_ref.into(), time, time_scale)?.into())
    }

    /// Obtain the camera (if any) being used by After Effects to view the effect's layer.
    pub fn effect_camera(&self, effect_ref: pf::ProgPtr, time: Time) -> Result<Option<LayerHandle>, Error> {
        let camera_handle = call_suite_fn_single!(self, AEGP_GetEffectCamera -> ae_sys::AEGP_LayerH, effect_ref.into(), &time.into() as *const _)?;
        if camera_handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(LayerHandle::from_raw(camera_handle)))
        }
    }

    /// Obtain the transform used to move between the layer's coordinate space and that of the containing composition.
    ///
    /// NOTE: In cases where the effect's input layer has square pixels, but is in a non-square pixel composition,
    /// you must correct for the pixel aspect ratio by premultiplying the matrix by `(1/parF, 1, 1)`.
    ///
    /// The model view for the camera matrix is inverse of the matrix obtained from [`effect_camera_matrix()`](Self::effect_camera_matrix).
    ///
    /// Also note that our matrix is row-based; OpenGL's is column-based.
    ///
    /// Returns a tuple containing: (matrix, dist_to_image_plane, image_plane_width, image_plane_height)
    pub fn effect_camera_matrix(&self, effect_ref: pf::ProgPtr, time: Time) -> Result<(Matrix4, f64, i16, i16), Error> {
        let mut matrix: ae_sys::A_Matrix4 = unsafe { std::mem::zeroed() };
        let mut dist_to_image_plane: f64 = 0.0;
        let mut image_plane_width = 0;
        let mut image_plane_height = 0;
        call_suite_fn!(self, AEGP_GetEffectCameraMatrix, effect_ref.into(), &time.into() as *const _, &mut matrix, &mut dist_to_image_plane, &mut image_plane_width, &mut image_plane_height)?;
        Ok((matrix.into(), dist_to_image_plane, image_plane_width, image_plane_height))
    }
}
