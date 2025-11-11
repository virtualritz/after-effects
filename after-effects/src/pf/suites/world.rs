use crate::*;
use ae_sys::PF_ProgPtr;

define_suite!(
    /// Use these functions to create and destroy [`Layer`], and to find out their bit-depth.
    WorldSuite,
    PF_WorldSuite2,
    kPFWorldSuite,
    kPFWorldSuiteVersion2
);

impl WorldSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Creates a new [`Layer`].
    pub fn new_world(&self, in_data: impl AsPtr<*const ae_sys::PF_InData>, width: i32, height: i32, clear_pix: bool, pixel_format: PixelFormat) -> Result<Layer, Error> {
        let layer = call_suite_fn_single!(self, PF_NewWorld -> ae_sys::PF_EffectWorld, (*in_data.as_ptr()).effect_ref, width, height, clear_pix as _, pixel_format.into())?;
        Ok(Layer::from_owned(layer, in_data, |self_layer| {
            // SAFETY: Dereferencing in_data_ptr to access effect_ref for disposal.
            // Detailed explanation: (1) in_data_ptr is guaranteed valid for the lifetime of the Layer as it's stored during Layer creation,
            // (2) PF_InData is provided by After Effects and remains valid throughout the effect's lifetime,
            // (3) effect_ref is a required field that After Effects always initializes before passing PF_InData to plugins.
            // Would be UB if: in_data_ptr was null, dangling, or pointed to uninitialized memory, but Layer construction ensures validity.
            WorldSuite::new().unwrap().dispose_world(unsafe { (*self_layer.in_data_ptr).effect_ref }, self_layer.as_mut_ptr()).unwrap();
        }))

    }

    /// Dispose of an [`Layer`].
    pub fn dispose_world(&self, effect_ref: impl AsPtr<PF_ProgPtr>, effect_world: *mut ae_sys::PF_EffectWorld) -> Result<(), Error> {
        call_suite_fn!(self, PF_DisposeWorld, effect_ref.as_ptr(), effect_world)
    }

    /// Get the pixel format for a given [`Layer`].
    ///
    /// Result can be:
    ///
    /// * [`PixelFormat::Argb32`] - standard 8-bit RGB
    /// * [`PixelFormat::Argb64`] - 16-bit RGB
    /// * [`PixelFormat::Argb128`] - 32-bit floating point RGB
    pub fn pixel_format(&self, effect_world: impl AsPtr<*const ae_sys::PF_EffectWorld>) -> Result<PixelFormat, Error> {
        Ok(call_suite_fn_single!(self, PF_GetPixelFormat -> ae_sys::PF_PixelFormat, effect_world.as_ptr())?.into())
    }
}
