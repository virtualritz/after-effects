use crate::*;

define_suite!(
    /// Use these functions to create and destroy [`EffectWorld`], and to find out their bit-depth.
    WorldSuite2,
    PF_WorldSuite2,
    kPFWorldSuite,
    kPFWorldSuiteVersion2
);

impl WorldSuite2 {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Creates a new [`EffectWorld`].
    pub fn new_world(&self, effect_ref: ProgressInfo, width: i32, height: i32, clear_pix: bool, pixel_format: pf::PixelFormat) -> Result<EffectWorld, Error> {
        Ok(EffectWorld {
            effect_world: call_suite_fn_single!(self, PF_NewWorld -> ae_sys::PF_EffectWorld, effect_ref.as_ptr(), width, height, clear_pix as _, pixel_format.into())?
        })
    }

    /// Dispose of an [`EffectWorld`].
    pub fn dispose_world(&self, effect_ref: ProgressInfo, effect_world: EffectWorld) -> Result<(), Error> {
        call_suite_fn!(self, PF_DisposeWorld, effect_ref.as_ptr(), effect_world.as_ptr() as *mut _)
    }

    /// Get the pixel format for a given [`EffectWorld`].
    ///
    /// Result can be:
    ///
    /// * [`PixelFormat::Argb32`] - standard 8-bit RGB
    /// * [`PixelFormat::Argb64`] - 16-bit RGB
    /// * [`PixelFormat::Argb128`] - 32-bit floating point RGB
    pub fn pixel_format(&self, effect_world: &EffectWorld) -> Result<PixelFormat, Error> {
        Ok(call_suite_fn_single!(self, PF_GetPixelFormat -> ae_sys::PF_PixelFormat, effect_world.as_ptr())?.into())
    }
}
