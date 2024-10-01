
use crate::*;
use ae_sys::*;

define_suite!(
    /// The [`FillMatteSuite`] can be used to fill a [`pf::Layer`], either with a specific color or premultiplied with an alpha value.
    FillMatteSuite,
    PF_FillMatteSuite2,
    kPFFillMatteSuite,
    kPFFillMatteSuiteVersion2
);

impl FillMatteSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Fills a `rect` with a `color` (or, if the color is `None`, fills with black and alpha zero).
    ///
    /// If the `rect` is `None`, it fills the entire image.
    pub fn fill(&self, effect_ref: impl AsPtr<PF_ProgPtr>, mut world: impl AsMutPtr<*mut PF_EffectWorld>, color: Option<Pixel8>, rect: Option<Rect>) -> Result<(), Error> {
        call_suite_fn!(self, fill, effect_ref.as_ptr(), color.as_ref().map_or(std::ptr::null(), |x| x), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), world.as_mut_ptr())
    }

    /// Fills a `rect` with a `color` (or, if the color is `None`, fills with black and alpha zero).
    ///
    /// If the `rect` is `None`, it fills the entire image.
    pub fn fill16(&self, effect_ref: impl AsPtr<PF_ProgPtr>, mut world: impl AsMutPtr<*mut PF_EffectWorld>, color: Option<Pixel16>, rect: Option<Rect>) -> Result<(), Error> {
        call_suite_fn!(self, fill16, effect_ref.as_ptr(), color.as_ref().map_or(std::ptr::null(), |x| x), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), world.as_mut_ptr())
    }

    /// Fills a `rect` with a `color` (or, if the color is `None`, fills with black and alpha zero).
    ///
    /// If the `rect` is `None`, it fills the entire image.
    pub fn fill_float(&self, effect_ref: impl AsPtr<PF_ProgPtr>, mut world: impl AsMutPtr<*mut PF_EffectWorld>, color: Option<PixelF32>, rect: Option<Rect>) -> Result<(), Error> {
        call_suite_fn!(self, fill_float, effect_ref.as_ptr(), color.as_ref().map_or(std::ptr::null(), |x| x), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), world.as_mut_ptr())
    }

    /// Converts to (and from) r, g, and b color values pre-multiplied with black to represent the alpha channel.
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    pub fn premultiply(&self, effect_ref: impl AsPtr<PF_ProgPtr>, mut world: impl AsMutPtr<*mut PF_EffectWorld>, forward: bool) -> Result<(), Error> {
        call_suite_fn!(self, premultiply, effect_ref.as_ptr(), forward as _, world.as_mut_ptr())
    }

    /// Converts to (and from) having r, g, and b color values premultiplied with any color to represent the alpha channel.
    /// * `color` - color to premultiply/unmultiply with
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// To convert between premul and straight pixel buffers where the color channels were matted with a color other than black.
    pub fn premultiply_color(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, color: &Pixel8, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self, premultiply_color, effect_ref.as_ptr(), src.as_ptr() as *mut _, color, forward as _, dst.as_mut_ptr())
    }

    /// Converts to (and from) having r, g, and b color values premultiplied with any color to represent the alpha channel.
    /// * `color` - color to premultiply/unmultiply with
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// To convert between premul and straight pixel buffers where the color channels were matted with a color other than black.
    pub fn premultiply_color16(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, color: &Pixel16, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self, premultiply_color16, effect_ref.as_ptr(), src.as_ptr() as *mut _, color, forward as _, dst.as_mut_ptr())
    }

    /// Converts to (and from) having r, g, and b color values premultiplied with any color to represent the alpha channel.
    /// * `color` - color to premultiply/unmultiply with
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// To convert between premul and straight pixel buffers where the color channels were matted with a color other than black.
    pub fn premultiply_color_float(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, color: &PixelF32, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self, premultiply_color_float, effect_ref.as_ptr(), src.as_ptr() as *mut _, color, forward as _, dst.as_mut_ptr())
    }
}
