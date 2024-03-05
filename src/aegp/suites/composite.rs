use crate::*;

define_suite!(
    /// Use the [`CompositeSuite`] to copy pixel worlds, operate on track mattes, and apply transfer functions.
    CompositeSuite,
    AEGP_CompositeSuite2,
    kAEGPCompositeSuite,
    kAEGPCompositeSuiteVersion2
);

impl CompositeSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// For the given [`EffectWorld`], sets the alpha to fully transparent except for the specified rectangle.
    pub fn clear_alpha_except_rect(&self, clipped_dest_rect: Rect, dst_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ClearAlphaExceptRect, &mut clipped_dest_rect.into() as *mut _, dst_world.as_ptr())
    }

    /// Blends two [`EffectWorld`]s using a transfer mode, with an optional mask.
    ///
    /// Pass `None` for the `blending_tables` parameter to perform blending in the current working color space.
    pub fn transfer_rect(
        &self,
        quality: pf::Quality,
        alpha: pf::ModeFlags,
        field: pf::Field,
        src_rect: &Rect,
        src_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>,
        comp_mode: &pf::CompositeMode,
        blending_tables: Option<&EffectBlendingTables>,
        mask_world: Option<pf::MaskWorld>,
        dst_x: u32,
        dst_y: u32,
        dst_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>,
    ) -> Result<(), Error> {
        let mask_world = mask_world.map(|m| ae_sys::PF_MaskWorld {
            mask: m.mask,
            offset: ae_sys::PF_Point {
                v: m.offset.v,
                h: m.offset.h,
            },
            what_is_mask: m.what_is_mask as i32,
        });
        call_suite_fn!(
            self,
            AEGP_TransferRect,
            quality as i32,
            alpha as i32,
            field as i32,
            src_rect as *const _ as _,
            src_world.as_ptr(),
            comp_mode as *const _ as _,
            blending_tables.map_or(std::ptr::null(), |b| b.as_ptr()) as _,
            mask_world.map_or(std::ptr::null(), |m| &m) as _,
            dst_x as i32,
            dst_y as i32,
            dst_world.as_ptr()
        )
    }

    /// Mattes the pixels in a [`EffectWorld`] with the `PF_Pixel` described in src_masks, putting the output into an array of pixels dst_mask.
    ///
    /// NOTE: Unlike most of the other pixel mangling functions provided by After Effects, this one doesn't take [`EffectWorld`] arguments;
    /// rather, you can simply pass the data pointer from within the [`EffectWorld`].
    /// This can be confusing, but as a bonus, the function pads output appropriately so that `num_pix` pixels are always output.
    pub fn prep_track_matte(&self, num_pix: i32, deep: bool, src_mask: &[ae_sys::PF_Pixel], mask_flags: MaskFlags, dst_mask: &mut [ae_sys::PF_Pixel]) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_PrepTrackMatte, num_pix, deep as _, src_mask.as_ptr(), mask_flags as i32, dst_mask.as_mut_ptr())
    }

    /// Copies a rectangle of pixels (pass a `None` rectangle to get all pixels) from one [`EffectWorld`] to another, at low quality.
    pub fn copy_bits_lq(&self, src_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>, src_r: Option<Rect>, dst_r: Option<Rect>, dst_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self,
            AEGP_CopyBits_LQ,
            src_world.as_ptr(),
            src_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_world.as_ptr()
        )
    }

    /// Copies a rectangle of pixels (pass a `None` rectangle to get all pixels) from one [`EffectWorld`] to another, at high quality, with a straight alpha channel.
    pub fn copy_bits_hq_straight(&self, src_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>, src_r: Option<Rect>, dst_r: Option<Rect>, dst_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self,
            AEGP_CopyBits_HQ_Straight,
            src_world.as_ptr(),
            src_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_world.as_ptr()
        )
    }

    /// Copies a rectangle of pixels (pass a `None` rectangle to get all pixels) from one [`EffectWorld`] to another, at high quality, premultiplying the alpha channel.
    pub fn copy_bits_hq_premul(&self, src_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>, src_r: Option<Rect>, dst_r: Option<Rect>, dst_world: impl AsPtr<*mut ae_sys::PF_EffectWorld>) -> Result<(), Error> {
        call_suite_fn!(self,
            AEGP_CopyBits_HQ_Premul,
            src_world.as_ptr(),
            src_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_r.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |r| r),
            dst_world.as_ptr()
        )
    }
}
