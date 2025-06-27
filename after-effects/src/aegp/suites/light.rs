use crate::*;
use ae_sys::AEGP_LayerH;
use crate::aegp::LayerHandle;

define_suite!(
    /// Get and set the type of lights in a composition.
    ///
    /// ## Notes On Light Behavior
    /// The formula for parallel lights is found in Foley and Van Dam's "Introduction to Computer Graphics" (ISBN 0-201-60921-5) as is the formula for point lights.
    ///
    /// We use the half angle variant proposed by Jim Blinn instead.
    ///
    /// Suppose we have a point on a layer and want to shade it with the light.
    ///
    /// Let V be the unit vector from the layer point to the eye point.
    /// Let L be the unit vector to the light (in the parallel light case this is constant). Let H be (V+L)/2 (normalized).
    /// Let N be the unit normal vector to the layer.
    ///
    /// The amount of specular reflected light is S * power(H Dot N, shine), where S is the specular coefficient.
    LightSuite,
    AEGP_LightSuite2,
    kAEGPLightSuite,
    kAEGPLightSuiteVersion2
);
define_suite!(
    LightSuite3,
    AEGP_LightSuite3,
    kAEGPLightSuite,
    kAEGPLightSuiteVersion3
);


impl LightSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the [`LightType`] of the specified camera layer
    pub fn light_type(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LightType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLightType -> ae_sys::AEGP_LightType, layer_handle.as_ptr())?.into())
    }

    /// Sets the [`LightType`] for the specified camera layer.
    pub fn set_light_type(&self, layer_handle: impl AsPtr<AEGP_LayerH>, light_type: LightType) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetLightType, layer_handle.as_ptr(), light_type.into())
    }

    pub fn light_source(&self, layer_handle: impl AsPtr<AEGP_LayerH>) -> Result<LayerHandle, Error> {
        let v3 = LightSuite3::new()?;
        Ok(LayerHandle::from_raw(
            call_suite_fn_single!(v3, AEGP_GetLightSource -> ae_sys::AEGP_LayerH, layer_handle.as_ptr())?
        ))
    }
    pub fn set_light_source(&self, layer_handle: impl AsPtr<AEGP_LayerH>, light_source: impl AsPtr<AEGP_LayerH>) -> Result<(), Error> {
        let v3 = LightSuite3::new()?;
        call_suite_fn!(v3, AEGP_SetLightSource, layer_handle.as_ptr(), light_source.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_LightType,
    LightType {
        None        = ae_sys::AEGP_LightType_NONE,
        Parallel    = ae_sys::AEGP_LightType_PARALLEL,
        Spot        = ae_sys::AEGP_LightType_SPOT,
        Point       = ae_sys::AEGP_LightType_POINT,
        Ambient     = ae_sys::AEGP_LightType_AMBIENT,
        Environment = ae_sys::AEGP_LightType_ENVIRONMENT,
    }
}
