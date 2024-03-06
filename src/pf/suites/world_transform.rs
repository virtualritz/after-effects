use crate::*;
use ae_sys::{ PF_ProgPtr, PF_EffectWorld, PF_FloatMatrix, PF_CompositeMode, PF_MaskWorld };
use std::ffi::c_void;

define_suite!(
    /// These functions combine [`Layer`]s in interesting ways. When you use these, you're using the same code After Effects does internally.
    WorldTransformSuite,
    PF_WorldTransformSuite1,
    kPFWorldTransformSuite,
    kPFWorldTransformSuiteVersion1
);

impl WorldTransformSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// Composite a rectangle from one `PF_EffectWorld` into another, using one of After Effects' transfer modes.
    /// * `src_rect` - rectangle in source image
    /// * `src_opacity` - opacity of src
    /// * `src` - source PF world
    /// * `dest_x`, `dest_y` - upper left-hand corner of src rect in composite image
    /// * `field` - which scanlines to render ([`Field::Frame`], [`Field::Upper`] or [`Field::Lower`])
    /// * `transfer_mode` - can be [`TransferMode::Copy`], [`TransferMode::Behind`] or [`TransferMode::InFront`]
    pub fn composite_rect(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src_rect: Option<Rect>, src_opacity: i32, src: impl AsPtr<*mut PF_EffectWorld>, dest_x: i32, dest_y: i32, field: Field, transfer_mode: TransferMode, dst: impl AsPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_suite_fn!(self, composite_rect, effect_ref.as_ptr(), src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x), src_opacity, src.as_ptr(), dest_x, dest_y, field.into(), transfer_mode.into(), dst.as_ptr())
    }

    /// Blends two images, alpha-weighted. Does not deal with different-sized sources, though the destination may be either `PF_EffectWorld`.
    pub fn blend(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src1: impl AsPtr<*const PF_EffectWorld>, src2: impl AsPtr<*const PF_EffectWorld>, ratio: i32, dst: impl AsPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src1.as_ptr().is_null() || src2.as_ptr().is_null() || dst.as_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_suite_fn!(self, blend, effect_ref.as_ptr(), src1.as_ptr(), src2.as_ptr(), ratio as _, dst.as_ptr())
    }

    /// Convolve an image with an arbitrary size kernel on each of the a, r, g, and b channels separately.
    ///
    /// You can specify a rectangle to convolve (for instance, the `extent_hint` from `PF_EffectWorld`), or pass `None` to convolve the entire image.
    ///
    /// Do not use if the source *is* the destination.
    ///
    /// Describe the convolution using [`KernelFlags`]:
    /// * 1D or 2D
    /// * Clamp or No Clamp
    /// * Use longs-chars-fixeds
    /// * straight convolve vs. alpha-weighted
    /// * *if 1D is specified:* Horizontal or Vertical
    ///
    /// Note: some 2D convolutions are seperable and can be implemented with a horizontal 1D convolve and a vertical 1D convolve.
    /// This filter may have different high and low quality versions.
    pub fn convolve(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, area: Option<Rect>, flags: KernelFlags, kernel_size: i32, a_kernel: *mut c_void, r_kernel: *mut c_void, g_kernel: *mut c_void, b_kernel: *mut c_void, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_suite_fn!(self, convolve, effect_ref.as_ptr(), src.as_ptr() as _, area.map(Into::into).as_ref().map_or(std::ptr::null_mut(), |x| x), flags.bits() as _, kernel_size, a_kernel, r_kernel, g_kernel, b_kernel, dst.as_mut_ptr())
    }

    /// This blits a region from one PF_EffectWorld to another.
    /// This is an alpha-preserving (unlike CopyBits), 32-bit only, non-antialiased stretch blit.
    /// The high quality version does an anti-aliased blit (ie. it interpolates).
    pub fn copy(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, mut dst: impl AsMutPtr<*mut PF_EffectWorld>, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_suite_fn!(self, copy, effect_ref.as_ptr(), src.as_ptr() as *mut _, dst.as_mut_ptr(), src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x), dst_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x))
    }

    /// This blits a region from one PF_EffectWorld to another.
    /// This is an alpha-preserving (unlike CopyBits), 32-bit only, non-antialiased stretch blit.
    /// The high quality version does an anti-aliased blit (ie. it interpolates).
    pub fn copy_hq(&self, effect_ref: impl AsPtr<PF_ProgPtr>, src: impl AsPtr<*const PF_EffectWorld>, mut dst: impl AsMutPtr<*mut PF_EffectWorld>, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_suite_fn!(self, copy_hq, effect_ref.as_ptr(), src.as_ptr() as *mut _, dst.as_mut_ptr(), src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x), dst_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x))
    }

    /// Blends using a transfer mode, with an optional mask.
    pub fn transfer_rect(&self, effect_ref: impl AsPtr<PF_ProgPtr>, quality: Quality, flags: ModeFlags, field: Field, src_rect: Option<Rect>, src: *const PF_EffectWorld, comp_mode: &CompositeMode, mask_world: Option<&MaskWorld>, dest_x: i32, dest_y: i32, dst: *mut PF_EffectWorld) -> Result<(), Error> {
        if src.is_null() || dst.is_null() { return Err(Error::BadCallbackParameter); }

        const _: () = assert!(std::mem::size_of::<PF_CompositeMode>() == std::mem::size_of::<CompositeMode>());
        const _: () = assert!(std::mem::size_of::<PF_MaskWorld>()     == std::mem::size_of::<MaskWorld>());

        call_suite_fn!(self,
            transfer_rect,
            effect_ref.as_ptr(),
            quality.into(),
            flags.into(),
            field.into(),
            src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x),
            src,
            std::mem::transmute(comp_mode),
            if let Some(mask_world) = mask_world {
                std::mem::transmute(mask_world)
            } else {
                std::ptr::null()
            },
            dest_x,
            dest_y,
            dst
        )
    }

    /// Given a PF_EffectWorld and a matrix (or array of matrices), transforms and blends using an After Effects transfer mode, with an optional mask.
    ///
    /// The matrices pointer points to a matrix array used for motion-blur.
    ///
    /// When is a transform not a transform? A Z-scale transform is not a transform, unless the transformed layer is a parent of other layers that do not all lie in the z=0 plane.
    pub fn transform_world(&self, effect_ref: impl AsPtr<PF_ProgPtr>, quality: Quality, mode_flags: ModeFlags, field: Field, src: *const PF_EffectWorld, comp_mode: &CompositeMode, mask_world: Option<&MaskWorld>, matrices: &[Matrix3], src2dst_matrix: bool, dest_rect: Option<Rect>, dst: *mut PF_EffectWorld) -> Result<(), Error> {
        if src.is_null() || dst.is_null() { return Err(Error::BadCallbackParameter); }

        const _: () = assert!(std::mem::size_of::<PF_FloatMatrix>() == std::mem::size_of::<Matrix3>());

        call_suite_fn!(self,
            transform_world,
            effect_ref.as_ptr(),
            quality.into(),
            mode_flags.into(),
            field.into(),
            src,
            std::mem::transmute(comp_mode),
            if let Some(mask_world) = mask_world {
                std::mem::transmute(mask_world)
            } else {
                std::ptr::null()
            },
            matrices.as_ptr() as *const _,
            matrices.len() as _,
            src2dst_matrix as _,
            dest_rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
            dst
        )
    }
}
