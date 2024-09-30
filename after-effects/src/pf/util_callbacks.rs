use crate::*;
use std::ffi::c_void;
use ae_sys::{ PF_ProgPtr, PF_EffectWorld, _PF_UtilCallbacks, PF_Pixel, PF_Pixel16, PF_FloatMatrix };

define_enum! {
    ae_sys::PF_Quality,
    Quality {
        DrawingAudio = ae_sys::PF_Quality_DRAWING_AUDIO,
        Lo           = ae_sys::PF_Quality_LO,
        Hi           = ae_sys::PF_Quality_HI,
    }
}
define_enum! {
    ae_sys::PF_ModeFlags,
    ModeFlags {
        AlphaPremul   = ae_sys::PF_MF_Alpha_PREMUL,
        AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
    }
}

macro_rules! call_fn {
    ($self:ident, $fn:ident, $($args:expr),*) => {
        unsafe {
            let in_data = &(*$self.0.as_ptr());
            if in_data.utils.is_null() || in_data.effect_ref.is_null() {
                return Err(Error::BadCallbackParameter);
            }
            let $fn = (*in_data.utils).$fn.ok_or(Error::BadCallbackParameter)?;
            match $fn(in_data.effect_ref, $($args),*) {
                0 => Result::<(), Error>::Ok(()),
                e => Err(e.into()),
            }
        }
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! define_iterate {
    ($(+ $in_data:ident: $in_data_ty:ty, )? $name:ident, $pixel:ident, $pixel_ptr:ident $(, $additional:ident: $additional_type:ty)?) => {
        /// This invokes a function you specify on a region of pixels in the source and dest images.
        /// The function is invoked with the x and y coordinates of the current pixel, plus a pointer to that pixel in the src and dest images.
        /// You can specify a rectangle to iterate over (for instance, the extent_hint), or pass `None` to iterate over every pixel where the worlds overlap.
        ///
        /// If you set `src` to `None`, this will just iterate over the `dst`.
        ///
        /// This function will automatically make the progress bar go as it iterates.
        /// To allow your effect to have the progress bar go across just once and still perform multiple iterations, "iterate" starts progress at a base
        /// number you specify, and goes to that number + the height of the image, reporting the progress out of a possible maximum that you also specify.
        /// Pass the max number as zero to turn off progress reporting.
        ///
        /// This is quality independent.
        pub fn $name<F>(&self $(, $in_data: $in_data_ty)?, src: Option<*const PF_EffectWorld>, dst: *mut PF_EffectWorld, progress_base: i32, progress_final: i32, area: Option<Rect> $(, $additional: $additional_type)?, cb: F) -> Result<(), Error>
        where
            F: Fn(i32, i32, &$pixel, &mut $pixel) -> Result<(), Error>,
        {
            unsafe extern "C" fn iterate_c_fn(refcon: *mut c_void, x: i32, y: i32, mut in_p: *mut $pixel_ptr, out_p: *mut $pixel_ptr) -> ae_sys::PF_Err {
                if refcon.is_null() || out_p.is_null() {
                    return ae_sys::PF_Err_BAD_CALLBACK_PARAM as ae_sys::PF_Err;
                }
                let cb = &*(refcon as *const Box<Box<dyn Fn(i32, i32, &$pixel, &mut $pixel) -> Result<(), Error>>>);

                // If `src` is None, there will be no source pixels, so just use the output pixel in both places to simplify the callback
                if in_p.is_null() { in_p = out_p; }

                match cb(x, y, &*in_p, &mut *out_p) {
                    Ok(_)  => ae_sys::PF_Err_NONE as _,
                    Err(e) => e.into(),
                }
            }
            unsafe {
                let _in_data_ptr = self.get_in_data();
                $(
                    let _in_data_ptr = $in_data.as_ptr();
                )?
                let ptr = self.get_funcs_ptr();
                if dst.is_null() || ptr.is_null() {
                    return Err(Error::BadCallbackParameter);
                }
                let iterate_fn = (*ptr).$name.ok_or(Error::BadCallbackParameter)?;

                let callback = Box::<Box<dyn Fn(i32, i32, &$pixel, &mut $pixel) -> Result<(), Error>>>::new(Box::new(cb));
                let refcon = &callback as *const _;
                match iterate_fn(
                    _in_data_ptr as *mut _,
                    progress_base,
                    progress_final,
                    src.map_or(std::ptr::null_mut(), |x| x as *mut _),
                    area.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
                    $($additional.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), )?
                    refcon as *mut _,
                    Some(iterate_c_fn),
                    dst,
                ) {
                    0 => Ok(()),
                    e => Err(e.into()),
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_iterate_lut_and_generic {
    ($(+ $in_data:ident: $in_data_ty:ty, )?) => {
        /// Allows a Look-Up Table (LUT) to be passed for iteration; you can pass the same or different LUTs for each color channel.
        ///
        /// If no LUT is passed, an identity LUT is used.
        pub fn iterate_lut(&self $(, $in_data: $in_data_ty)?, src: *mut PF_EffectWorld, dst: *mut PF_EffectWorld, progress_base: i32, progress_final: i32, area: Option<Rect>, a_lut: Option<&[u8]>, r_lut: Option<&[u8]>, g_lut: Option<&[u8]>, b_lut: Option<&[u8]>) -> Result<(), Error> {
            if src.is_null() || dst.is_null() { return Err(Error::BadCallbackParameter); }
            unsafe {
                let _in_data_ptr = self.get_in_data();
                $(
                    let _in_data_ptr = $in_data.as_ptr();
                )?
                let ptr = self.get_funcs_ptr();
                if dst.is_null() || ptr.is_null() {
                    return Err(Error::BadCallbackParameter);
                }
                let iterate_fn = (*ptr).iterate_lut.ok_or(Error::BadCallbackParameter)?;

                match iterate_fn(
                    _in_data_ptr as *mut _,
                    progress_base,
                    progress_final,
                    src,
                    area.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
                    a_lut.as_ref().map_or(std::ptr::null_mut(), |x| x.as_ptr() as *mut _),
                    r_lut.as_ref().map_or(std::ptr::null_mut(), |x| x.as_ptr() as *mut _),
                    g_lut.as_ref().map_or(std::ptr::null_mut(), |x| x.as_ptr() as *mut _),
                    b_lut.as_ref().map_or(std::ptr::null_mut(), |x| x.as_ptr() as *mut _),
                    dst,
                ) {
                    0 => Ok(()),
                    e => Err(e.into()),
                }
            }
        }

        /// If you want to do something once per available CPU, this is the function to use (pass [`ONCE_PER_PROCESSOR`] for `iterations`).
        ///
        /// Only call abort and progress functions from thread index 0.
        ///
        /// The `cb` callback parameters are: `thread_index`, `i`, `iterations`.
        ///
        /// Inside the callback, if you want to call abort or progress, you can only do it if `thread_index == 0`.
        ///
        /// Note: You can iterate over more than pixels. Internally, we use it for row-based image processing, and for once-per-entity updates of complex sequence data.
        pub fn iterate_generic<F>(&self, iterations: i32, cb: F) -> Result<(), Error>
        where
            F: Fn(i32, i32, i32) -> Result<(), Error>,
        {
            unsafe extern "C" fn iterate_c_fn(refcon: *mut c_void, thread_index: ae_sys::A_long, i: ae_sys::A_long, iterations: ae_sys::A_long) -> ae_sys::PF_Err {
                let cb = &*(refcon as *const Box<Box<dyn Fn(i32, i32, i32) -> Result<(), Error>>>);
                match cb(thread_index, i, iterations) {
                    Ok(_)  => ae_sys::PF_Err_NONE as _,
                    Err(e) => e.into(),
                }
            }
            unsafe {
                let ptr = self.get_funcs_ptr();
                if ptr.is_null() {
                    return Err(Error::BadCallbackParameter);
                }
                let f = (*ptr).iterate_generic.ok_or(Error::BadCallbackParameter)?;

                let callback = Box::<Box<dyn Fn(i32, i32, i32) -> Result<(), Error>>>::new(Box::new(cb));
                let refcon = &callback as *const _ as *mut _;

                match f(iterations, refcon, Some(iterate_c_fn)) {
                    0 => Ok(()),
                    e => Err(e.into()),
                }
            }
        }
    }
}

pub const ONCE_PER_PROCESSOR: i32 = ae_sys::PF_Iterations_ONCE_PER_PROCESSOR as i32;

pub struct UtilCallbacks(*const ae_sys::PF_InData);

impl UtilCallbacks {
    pub fn new(in_data: impl AsPtr<*const ae_sys::PF_InData>) -> Self {
        assert!(!in_data.as_ptr().is_null());
        Self(in_data.as_ptr())
    }

    /// Composite a rectangle from one `PF_EffectWorld` into another, using one of After Effects' transfer modes.
    /// * `src_rect` - rectangle in source image
    /// * `src_opacity` - opacity of src
    /// * `src` - source PF world
    /// * `dest_x`, `dest_y` - upper left-hand corner of src rect in composite image
    /// * `field` - which scanlines to render ([`Field::Frame`], [`Field::Upper`] or [`Field::Lower`])
    /// * `transfer_mode` - can be [`TransferMode::Copy`], [`TransferMode::Behind`] or [`TransferMode::InFront`]
    pub fn composite_rect(&self, src_rect: Option<Rect>, src_opacity: i32, src: impl AsPtr<*mut PF_EffectWorld>, dest_x: i32, dest_y: i32, field: Field, transfer_mode: TransferMode, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, composite_rect, src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x), src_opacity, src.as_ptr(), dest_x, dest_y, field.into(), transfer_mode.into(), dst.as_mut_ptr())
    }

    /// Blends two images, alpha-weighted. Does not deal with different-sized sources, though the destination may be either `PF_EffectWorld`.
    /// - `ratio` should be between 0.0 and 1.0
    pub fn blend(&self, src1: impl AsPtr<*const PF_EffectWorld>, src2: impl AsPtr<*const PF_EffectWorld>, ratio: f32, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src1.as_ptr().is_null() || src2.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, blend, src1.as_ptr(), src2.as_ptr(), Fixed::from(ratio).as_fixed(), dst.as_mut_ptr())
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
    pub fn convolve(&self, src: impl AsPtr<*const PF_EffectWorld>, area: Option<Rect>, flags: KernelFlags, kernel_size: i32, a_kernel: *mut c_void, r_kernel: *mut c_void, g_kernel: *mut c_void, b_kernel: *mut c_void, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, convolve, src.as_ptr() as _, area.map(Into::into).as_ref().map_or(std::ptr::null_mut(), |x| x), flags.bits() as _, kernel_size, a_kernel, r_kernel, g_kernel, b_kernel, dst.as_mut_ptr())
    }

    /// Generate a kernel with a Gaussian distribution of values.
    /// * `radius` - desired gaussian radius
    /// * `multiplier` - this value is multiplied by every value generated; in general, you should pass 1.0, but this lets you adjust the "fuzziness" of the kernel.
    /// * `diameter` - actual integral width of generated kernel; this will always currently be `radius.ceil() as i32 * 2 + 1` you need to know this because the "kernel" array must be already allocated upon entry to this routine.
    /// * `kernel` - kernel is a "diameter" by "diameter" array of values allocated by you, of longs, chars, or Fixeds. It points to the kernel upper left corner.
    ///
    /// Describe the convolution using [`KernelFlags`]:
    /// * 1D or 2D
    /// * Normalized or Unnormalized
    /// * Use longs-chars-fixeds
    ///
    /// This filter will be the same high and low quality.
    pub fn gaussian_kernel(&self, radius: f64, flags: KernelFlags, multiplier: f64, diameter: &mut i32, kernel: *mut c_void) -> Result<(), Error> {
        call_fn!(self, gaussian_kernel, radius, flags.bits() as _, multiplier, diameter, kernel)
    }

    /// Converts to (and from) r, g, and b color values pre-multiplied with black to represent the alpha channel.
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// Quality independent.
    pub fn premultiply(&self, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, premultiply, forward as _, dst.as_mut_ptr())
    }

    /// Converts to (and from) having r, g, and b color values premultiplied with any color to represent the alpha channel.
    /// * `color` - color to premultiply/unmultiply with
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// To convert between premul and straight pixel buffers where the color channels were matted with a color other than black.
    pub fn premultiply_color(&self, src: impl AsPtr<*mut PF_EffectWorld>, color: &Pixel8, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, premultiply_color, src.as_ptr(), color, forward as _, dst.as_mut_ptr())
    }

    /// Converts to (and from) having r, g, and b color values premultiplied with any color to represent the alpha channel.
    /// * `color` - color to premultiply/unmultiply with
    /// * `forward` - `true` means convert non-premultiplied to pre-multiplied; `false` means un-pre-multiply.
    ///
    /// To convert between premul and straight pixel buffers where the color channels were matted with a color other than black.
    pub fn premultiply_color16(&self, src: impl AsPtr<*mut PF_EffectWorld>, color: &Pixel16, forward: bool, mut dst: impl AsMutPtr<*mut PF_EffectWorld>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, premultiply_color16, src.as_ptr(), color, forward as _, dst.as_mut_ptr())
    }

    /// Call this routine before you plan to perform a large number of image resamplings.
    /// Depending on platform, this routine could start up the DSP chip, compute an index table to each scanline
    /// of the buffer, or whatever might be needed to speed up image resampling.
    pub fn begin_sampling(&self, quality: Quality, mode_flags: ModeFlags) -> Result<Sampling, Error> {
        let mut params = Sampling {
            in_data_ptr: self.0.as_ptr(),
            params: unsafe { std::mem::zeroed() },
            quality: quality.into(),
            mode_flags: mode_flags.into(),
        };
        let _ = call_fn!(self, begin_sampling, quality.into(), mode_flags.into(), &mut params.params)?;
        Ok(params)
    }

    /// This fills a rectangle in the 8-bit image with the given color.
    /// Setting `color` to `None` will fill the rectangle with black.
    /// Setting `rect` to `None` will fill the entire image.
    /// Quality setting doesn't matter.
    pub fn fill(&self, mut world: impl AsMutPtr<*mut PF_EffectWorld>, color: Option<Pixel8>, rect: Option<Rect>) -> Result<(), Error> {
        if world.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, fill, color.as_ref().map_or(std::ptr::null(), |x| x), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), world.as_mut_ptr())
    }

    /// This fills a rectangle in the 8-bit image with the given color.
    /// Setting `color` to `None` will fill the rectangle with black.
    /// Setting `rect` to `None` will fill the entire image.
    /// Quality setting doesn't matter.
    pub fn fill16(&self, mut world: impl AsMutPtr<*mut PF_EffectWorld>, color: Option<Pixel16>, rect: Option<Rect>) -> Result<(), Error> {
        if world.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, fill16, color.as_ref().map_or(std::ptr::null(), |x| x), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x), world.as_mut_ptr())
    }

    /// This blits a region from one PF_EffectWorld to another.
    /// This is an alpha-preserving (unlike CopyBits), 32-bit only, non-antialiased stretch blit.
    /// The high qual version does an anti-aliased blit (ie. it interpolates).
    pub fn copy(&self, src: impl AsPtr<*const PF_EffectWorld>, mut dst: impl AsMutPtr<*mut PF_EffectWorld>, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        if src.as_ptr().is_null() || dst.as_mut_ptr().is_null() { return Err(Error::BadCallbackParameter); }
        call_fn!(self, copy, src.as_ptr() as *mut _, dst.as_mut_ptr(), src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x), dst_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x))
    }

    // Helpers for the define_iterate macros
    #[inline(always)] fn get_in_data(&self) -> *const ae_sys::PF_InData { self.0.as_ptr() }
    #[inline(always)] fn get_funcs_ptr(&self) -> *mut ae_sys::_PF_UtilCallbacks { unsafe { (*self.0.as_ptr()).utils } }

    define_iterate!(iterate,                       Pixel8,  PF_Pixel);
    define_iterate!(iterate16,                     Pixel16, PF_Pixel16);
    define_iterate!(iterate_origin,                Pixel8,  PF_Pixel,   origin: Option<Point>);
    define_iterate!(iterate_origin16,              Pixel16, PF_Pixel16, origin: Option<Point>);
    define_iterate!(iterate_origin_non_clip_src,   Pixel8,  PF_Pixel,   origin: Option<Point>);
    define_iterate!(iterate_origin_non_clip_src16, Pixel16, PF_Pixel16, origin: Option<Point>);
    define_iterate_lut_and_generic!();

    pub fn host_new_handle(&self, size: usize) -> Result<RawHandle, Error> {
        unsafe {
            let in_data = &(*self.0);
            if size == 0 || in_data.utils.is_null() {
                return Err(Error::BadCallbackParameter);
            }
            let new    = (*in_data.utils).host_new_handle   .ok_or(Error::BadCallbackParameter)?;
            let lock   = (*in_data.utils).host_lock_handle  .ok_or(Error::BadCallbackParameter)?;
            let unlock = (*in_data.utils).host_unlock_handle.ok_or(Error::BadCallbackParameter)?;
            let ptr = new(size as _);
            if ptr.is_null() {
                return Err(Error::OutOfMemory);
            }
            let locked = lock(ptr);
            unlock(ptr); // we just want to check for null ptr
            if locked.is_null() {
                return Err(Error::OutOfMemory);
            }
            Ok(RawHandle {
                utils_ptr: in_data.utils,
                handle: ptr
            })
        }
    }

    /// This creates a new [`Layer`] for scratch for you. You must dispose of it. This is quality independent.
    pub fn new_world(&self, width: usize, height: usize, flags: NewWorldFlags) -> Result<Layer, Error> {
        let mut world = unsafe { std::mem::zeroed() };
        call_fn!(self, new_world, width as _, height as _, flags.bits() as _, &mut world)?;
        Ok(Layer::from_owned(world, self.0.clone(), |self_layer| {
            UtilCallbacks::new(self_layer.in_data_ptr).dispose_world(self_layer.as_mut_ptr()).unwrap();
        }))
    }

    /// This disposes a [`Layer`], deallocating pixels, etc. Only call it on worlds you have created. Quality independent.
    pub fn dispose_world(&self, world: *mut ae_sys::PF_EffectWorld) -> Result<(), Error> {
        call_fn!(self, dispose_world, world)
    }

    /// Blends using a transfer mode, with an optional mask.
    pub fn transfer_rect(&self, quality: Quality, flags: ModeFlags, field: Field, src_rect: Option<Rect>, src: *const PF_EffectWorld, comp_mode: CompositeMode, mask_world: Option<MaskWorld>, dest_x: i32, dest_y: i32, dst: *mut PF_EffectWorld) -> Result<(), Error> {
        if src.is_null() || dst.is_null() { return Err(Error::BadCallbackParameter); }

        let comp_mode: ae_sys::PF_CompositeMode = comp_mode.into();

        call_fn!(self,
            transfer_rect,
            quality.into(),
            flags.into(),
            field.into(),
            src_rect.map(Into::into).as_mut().map_or(std::ptr::null_mut(), |x| x),
            src,
            &comp_mode,
            mask_world.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
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
    pub fn transform_world(&self, quality: Quality, mode_flags: ModeFlags, field: Field, src: *const PF_EffectWorld, comp_mode: CompositeMode, mask_world: Option<MaskWorld>, matrices: &[Matrix3], src2dst_matrix: bool, dest_rect: Option<Rect>, dst: *mut PF_EffectWorld) -> Result<(), Error> {
        if src.is_null() || dst.is_null() { return Err(Error::BadCallbackParameter); }

        const _: () = assert!(std::mem::size_of::<PF_FloatMatrix>() == std::mem::size_of::<Matrix3>());

        let comp_mode: ae_sys::PF_CompositeMode = comp_mode.into();

        call_fn!(self,
            transform_world,
            quality.into(),
            mode_flags.into(),
            field.into(),
            src,
            &comp_mode,
            mask_world.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
            matrices.as_ptr() as *const _,
            matrices.len() as _,
            src2dst_matrix as _,
            dest_rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x),
            dst
        )
    }

    pub fn platform_data(&self, type_: PlatformDataType) -> Result<PlatformData, Error> {
        match type_ {
            PlatformDataType::MainWnd => {
                let mut hwnd: usize = 0;
                call_fn!(self, get_platform_data, type_.into(), &mut hwnd as *mut _ as *mut c_void)?;
                Ok(PlatformData::MainWnd(hwnd as _))
            },
            PlatformDataType::ResDllinstance => {
                let mut handle = std::ptr::null_mut();
                call_fn!(self, get_platform_data, type_.into(), &mut handle as *mut _ as *mut c_void)?;
                Ok(PlatformData::ResDllinstance(handle))
            },
            PlatformDataType::BundleRef => {
                let mut handle = std::ptr::null_mut();
                call_fn!(self, get_platform_data, type_.into(), &mut handle as *mut _ as *mut c_void)?;
                Ok(PlatformData::BundleRef(handle))
            },
            PlatformDataType::ExeFilePath => {
                let mut path = [0u16; 260]; // AEFX_MAX_PATH
                call_fn!(self, get_platform_data, type_.into(), path.as_mut_ptr() as *mut c_void)?;
                Ok(PlatformData::ExeFilePath(String::from_utf16_lossy(&path).trim_end_matches('\0').to_string()))
            },
            PlatformDataType::ResFilePath => {
                let mut path = [0u16; 260]; // AEFX_MAX_PATH
                call_fn!(self, get_platform_data, type_.into(), path.as_mut_ptr() as *mut c_void)?;
                Ok(PlatformData::ResFilePath(String::from_utf16_lossy(&path).trim_end_matches('\0').to_string()))
            },
        }
    }

    /// Obtain a pointer to a 8-bpc pixel within the specified world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 8-bpc.
    ///
    /// The second parameter is optional; if it is `Some`, the returned pixel will be an interpretation of the values in the passed-in pixel, as if it were in the specified PF_EffectWorld.
    pub fn pixel_data8(&self, world: *mut PF_EffectWorld, pixels: Option<*mut Pixel8>) -> Result<*mut Pixel8, Error> {
        let mut ret = std::ptr::null_mut();
        unsafe {
            let in_data = &(*self.0.as_ptr());
            if in_data.utils.is_null() || in_data.effect_ref.is_null() || world.is_null() {
                return Err(Error::BadCallbackParameter);
            }
            let f = (*in_data.utils).get_pixel_data8.ok_or(Error::BadCallbackParameter)?;
            match f(world, pixels.unwrap_or(std::ptr::null_mut()), &mut ret) {
                0 => {
                    if ret.is_null() {
                        return Err(Error::BadCallbackParameter);
                    }
                    Ok(ret)
                },
                e => Err(e.into()),
            }
        }
    }
    /// Obtain a pointer to a 16-bpc pixel within the specified world.
    ///
    /// It will return [`Error::BadCallbackParameter`] if the world is not 16-bpc.
    ///
    /// The second parameter is optional; if it is `Some`, the returned pixel will be an interpretation of the values in the passed-in pixel, as if it were in the specified PF_EffectWorld.
    pub fn pixel_data16(&self, world: *mut PF_EffectWorld, pixels: Option<*mut Pixel8>) -> Result<*mut Pixel16, Error> {
        let mut ret = std::ptr::null_mut();
        unsafe {
            let in_data = &(*self.0.as_ptr());
            if in_data.utils.is_null() || in_data.effect_ref.is_null() || world.is_null() {
                return Err(Error::BadCallbackParameter);
            }
            let f = (*in_data.utils).get_pixel_data16.ok_or(Error::BadCallbackParameter)?;
            match f(world, pixels.unwrap_or(std::ptr::null_mut()), &mut ret) {
                0 => {
                    if ret.is_null() {
                        return Err(Error::BadCallbackParameter);
                    }
                    Ok(ret)
                },
                e => Err(e.into()),
            }
        }
    }

    /// Plug-ins can draw on image processing algorithms written for nearly any color space by using the following callback functions.
    pub fn color(&self) -> ColorCallbacks {
        unsafe {
            let in_data = &(*self.0.as_ptr());
            assert!(!in_data.utils.is_null() && !in_data.effect_ref.is_null());
            ColorCallbacks {
                effect_ref: in_data.effect_ref,
                utils: in_data.utils
            }
        }
    }

    // fn get_callback_addr( effect_ref: PF_ProgPtr, quality: PF_Quality, mode_flags: PF_ModeFlags, which_callback: PF_CallbackID, fn_ptr: *mut PF_CallbackFunc) -> PF_Err,
    // fn app(arg1: PF_ProgPtr, arg2: A_long, ...) -> PF_Err,
    // ansi: PF_ANSICallbacks,
}

#[derive(Default)]
#[repr(C)]
pub struct HLSPixel {
    pub h: Fixed,
    pub l: Fixed,
    pub s: Fixed,
}

#[derive(Default)]
#[repr(C)]
pub struct YIQPixel {
    pub y: Fixed,
    pub i: Fixed,
    pub q: Fixed,
}

/// Plug-ins can draw on image processing algorithms written for nearly any color space by using the following callback functions.
pub struct ColorCallbacks {
    effect_ref: PF_ProgPtr,
    utils: *const _PF_UtilCallbacks,
}
impl ColorCallbacks {
    /// Given an RGB pixel, returns an HLS (hue, lightness, saturation) pixel. HLS values are scaled from 0 to 1 in fixed point.
    pub fn rgb_to_hls(&self, rgb: &Pixel8) -> Result<HLSPixel, Error> {
        let mut hls = HLSPixel::default();
        unsafe {
            let f = (*self.utils).colorCB.RGBtoHLS.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut hls as *mut _ as *mut _) {
                0 => Ok(hls),
                e => Err(e.into()),
            }
        }
    }

    /// Given an HLS pixel, returns an RGB pixel.
    pub fn hls_to_rgb(&self, hls: &HLSPixel) -> Result<Pixel8, Error> {
        unsafe {
            let mut rgb = std::mem::zeroed();
            let f = (*self.utils).colorCB.HLStoRGB.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, hls as *const _ as *mut _, &mut rgb) {
                0 => Ok(rgb),
                e => Err(e.into()),
            }
        }
    }

    /// Given an RGB pixel, returns a YIQ (luminance, inphase chrominance, quadrature chrominance) pixel.
    /// Y is 0 to 1 in fixed point, I is -0.5959 to 0.5959 in fixed point, and Q is -0.5227 to 0.5227 in fixed point.
    pub fn rgb_to_yiq(&self, rgb: &Pixel8) -> Result<YIQPixel, Error> {
        let mut yiq = YIQPixel::default();
        unsafe {
            let f = (*self.utils).colorCB.RGBtoYIQ.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut yiq as *mut _ as *mut _) {
                0 => Ok(yiq),
                e => Err(e.into()),
            }
        }
    }

    /// Given a YIQ pixel, returns an RGB pixel.
    pub fn yiq_to_rgb(&self, yiq: &YIQPixel) -> Result<Pixel8, Error> {
        unsafe {
            let mut rgb = std::mem::zeroed();
            let f = (*self.utils).colorCB.YIQtoRGB.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, yiq as *const _ as *mut _, &mut rgb) {
                0 => Ok(rgb),
                e => Err(e.into()),
            }
        }
    }

    /// Given an RGB pixel, returns 100 times its luminance value (0 to 25500).
    pub fn luminance(&self, rgb: &Pixel8) -> Result<i32, Error> {
        let mut x = 0;
        unsafe {
            let f = (*self.utils).colorCB.Luminance.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut x) {
                0 => Ok(x),
                e => Err(e.into()),
            }
        }
    }

    /// Given an RGB pixel, returns its hue angle mapped from 0 to 255, where 0 is 0 degrees and 255 is 360 degrees.
    pub fn hue(&self, rgb: &Pixel8) -> Result<i32, Error> {
        let mut x = 0;
        unsafe {
            let f = (*self.utils).colorCB.Hue.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut x) {
                0 => Ok(x),
                e => Err(e.into()),
            }
        }
    }

    /// Given an RGB pixel, returns its lightness value (0 to 255).
    pub fn lightness(&self, rgb: &Pixel8) -> Result<i32, Error> {
        let mut x = 0;
        unsafe {
            let f = (*self.utils).colorCB.Lightness.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut x) {
                0 => Ok(x),
                e => Err(e.into()),
            }
        }
    }

    /// Given an RGB pixel, returns its saturation value (0 to 255).
    pub fn saturation(&self, rgb: &Pixel8) -> Result<i32, Error> {
        let mut x = 0;
        unsafe {
            let f = (*self.utils).colorCB.Saturation.ok_or(Error::BadCallbackParameter)?;
            match f(self.effect_ref, rgb as *const _ as *mut _, &mut x) {
                0 => Ok(x),
                e => Err(e.into()),
            }
        }
    }
}

bitflags::bitflags! {
    pub struct NewWorldFlags: ae_sys::A_long {
        const NONE         = ae_sys::PF_NewWorldFlag_NONE         as ae_sys::A_long;
        const CLEAR_PIXELS = ae_sys::PF_NewWorldFlag_CLEAR_PIXELS as ae_sys::A_long;
        const DEEP_PIXELS  = ae_sys::PF_NewWorldFlag_DEEP_PIXELS  as ae_sys::A_long;
    }
}

bitflags::bitflags! {
    /// Functions such as `convolve` or gaussian kernel work with kernels, or matrices of filter weight values. These matrices can be in any format.
    /// The kernel flags describe how the matrices should be created and used. OR together any flags you need.
    ///
    /// The flags relevant to given routines are documented along with the routine prototype.
    /// The first entry in the left column is always the default and has value 0.
    pub struct KernelFlags: ae_sys::A_long {
        /// Specifies a two dimensional kernel.
        const TWO_D                 = ae_sys::PF_KernelFlag_2D                    as ae_sys::A_long;
        /// Specifies an one dimensional kernel.
        const ONE_D                 = ae_sys::PF_KernelFlag_1D                    as ae_sys::A_long;
        const UNNORMALIZED          = ae_sys::PF_KernelFlag_UNNORMALIZED          as ae_sys::A_long;
        /// `NORMALIZED` equalizes the kernel; the volume under the kernel surface is the same as the volume under the covered area of pixels.
        const NORMALIZED            = ae_sys::PF_KernelFlag_NORMALIZED            as ae_sys::A_long;
        /// `CLAMP` restricts values to the valid range for their data type.
        const CLAMP                 = ae_sys::PF_KernelFlag_CLAMP                 as ae_sys::A_long;
        const NO_CLAMP              = ae_sys::PF_KernelFlag_NO_CLAMP              as ae_sys::A_long;
        /// `USE_LONG` defines the kernel as an array of longs valued from 0 to 255. This is the only only implemented flag.
        const USE_LONG              = ae_sys::PF_KernelFlag_USE_LONG              as ae_sys::A_long;
        /// `USE_CHAR` defines the kernel as an array of unsigned chars from 0 to 255.
        const USE_CHAR              = ae_sys::PF_KernelFlag_USE_CHAR              as ae_sys::A_long;
        /// `USE_FIXED` defines the kernel as an array of fixeds from 0 to 1.
        const USE_FIXED             = ae_sys::PF_KernelFlag_USE_FIXED             as ae_sys::A_long;
        const USE_UNDEFINED         = ae_sys::PF_KernelFlag_USE_UNDEFINED         as ae_sys::A_long;
        /// Specifies the direction of the convolution.
        const HORIZONTAL            = ae_sys::PF_KernelFlag_HORIZONTAL            as ae_sys::A_long;
        /// Specifies the direction of the convolution.
        const VERTICAL              = ae_sys::PF_KernelFlag_VERTICAL              as ae_sys::A_long;
        /// Use `REPLICATE_BORDERS` to replicate border pixels when sampling off the edge. `REPLICATE_BORDERS` is not implemented and will be ignored.
        const TRANSPARENT_BORDERS   = ae_sys::PF_KernelFlag_TRANSPARENT_BORDERS   as ae_sys::A_long;
        /// Use `TRANSPARENT_BORDERS` to treat pixels off the edge as alpha zero (black).
        const REPLICATE_BORDERS     = ae_sys::PF_KernelFlag_REPLICATE_BORDERS     as ae_sys::A_long;
        /// Use `STRAIGHT_CONVOLVE` to indicate straight convolution,
        const STRAIGHT_CONVOLVE     = ae_sys::PF_KernelFlag_STRAIGHT_CONVOLVE     as ae_sys::A_long;
        /// Use `ALPHA_WEIGHT_CONVOLVE` to tell the convolution code to alpha-weight the contributions of pixels to the resulting convolved output. `ALPHA_WEIGHT_CONVOLVE` is not implemented and will be ignored.
        const ALPHA_WEIGHT_CONVOLVE = ae_sys::PF_KernelFlag_ALPHA_WEIGHT_CONVOLVE as ae_sys::A_long;
    }
}

define_enum! {
    ae_sys::PF_PlatDataID,
    PlatformDataType {
        /// Windows only, returns `HWND`
        MainWnd        = ae_sys::PF_PlatData_MAIN_WND,
        /// Windows only, returns `HANDLE`
        ResDllinstance = ae_sys::PF_PlatData_RES_DLLINSTANCE,
        /// macOS only, returns `CFBundleRef`
        BundleRef      = ae_sys::PF_PlatData_BUNDLE_REF,
        ExeFilePath    = ae_sys::PF_PlatData_EXE_FILE_PATH_W,
        ResFilePath    = ae_sys::PF_PlatData_RES_FILE_PATH_W,
    }
}
pub enum PlatformData {
    /// Windows only, returns `HWND`
    MainWnd(usize),
    /// Windows only, returns `HANDLE`
    ResDllinstance(*mut c_void),
    /// macOS only, returns `CFBundleRef`
    BundleRef(*mut c_void),
    ExeFilePath(String),
    ResFilePath(String),
}

pub struct RawHandle {
    utils_ptr: *const ae_sys::_PF_UtilCallbacks,
    handle: ae_sys::PF_Handle,
}
impl RawHandle {
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
    pub fn lock(&self) -> Result<RawHandleLock, Error> {
        unsafe {
            let lock = (*self.utils_ptr).host_lock_handle.ok_or(Error::BadCallbackParameter)?;
            let ptr = lock(self.handle);
            if ptr.is_null() {
                return Err(Error::OutOfMemory);
            }
            Ok(RawHandleLock {
                handle: self,
                ptr,
            })
        }
    }
    pub fn size(&self) -> Result<usize, Error> {
        unsafe {
            let get_size = (*self.utils_ptr).host_get_handle_size.ok_or(Error::BadCallbackParameter)?;
            let size = get_size(self.handle);
            Ok(size as _)
        }
    }
    pub fn resize(&mut self, new_size: usize) -> Result<(), Error> {
        unsafe {
            let resize = (*self.utils_ptr).host_resize_handle.ok_or(Error::BadCallbackParameter)?;
            match resize(new_size as _, &mut self.handle) {
                0 => Ok(()),
                e => Err(e.into()),
            }
        }
    }
}
impl Drop for RawHandle {
    fn drop(&mut self) {
        unsafe {
            let dispose = (*self.utils_ptr).host_dispose_handle.unwrap();
            dispose(self.handle);
        }
    }
}
pub struct RawHandleLock<'a> {
    handle: &'a RawHandle,
    ptr: *mut c_void,
}
impl<'a> RawHandleLock<'a> {
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}
impl<'a> Drop for RawHandleLock<'a> {
    fn drop(&mut self) {
        unsafe {
            let unlock = (*self.handle.utils_ptr).host_unlock_handle.unwrap();
            unlock(self.handle.handle);
        }
    }
}

pub struct Sampling {
    in_data_ptr: *const ae_sys::PF_InData,
    pub params: ae_sys::PF_SampPB,
    pub quality: ae_sys::PF_Quality,
    pub mode_flags: ae_sys::PF_ModeFlags,
}
impl Sampling {
    /// Use this to interpolate the appropriate alpha weighted mix of colors at a non-integral point in a source image, in high quality.
    /// Nearest neighbor sample is used in low quality.
    pub fn subpixel_sample(&self, x: f32, y: f32) -> Result<Pixel8, Error> {
        unsafe {
            let mut pixel = std::mem::zeroed();
            let f = (*(*self.in_data_ptr).utils).subpixel_sample.ok_or(Error::BadCallbackParameter)?;
            match f((*self.in_data_ptr).effect_ref, Fixed::from(x).as_fixed(), Fixed::from(y).as_fixed(), &self.params, &mut pixel) {
                0 => Ok(pixel),
                e => Err(e.into()),
            }
        }
    }

    /// Use this to interpolate the appropriate alpha weighted mix of colors at a non-integral point in a source image, in high quality.
    /// Nearest neighbor sample is used in low quality.
    pub fn subpixel_sample16(&self, x: f32, y: f32) -> Result<Pixel16, Error> {
        unsafe {
            let mut pixel = std::mem::zeroed();
            let f = (*(*self.in_data_ptr).utils).subpixel_sample16.ok_or(Error::BadCallbackParameter)?;
            match f((*self.in_data_ptr).effect_ref, Fixed::from(x).as_fixed(), Fixed::from(y).as_fixed(), &self.params, &mut pixel) {
                0 => Ok(pixel),
                e => Err(e.into()),
            }

        }
    }

    /// Use this to calculate the appropriate alpha weighted average of an axis-aligned non-integral rectangle of color in a source image, in high quality.
    /// Nearest neighbor in low quality.
    /// Because of overflow issues, this can only average a maximum of a 256 pixel by 256 pixel area (ie. x and y range < 128 pixels).
    pub fn area_sample(&self, x: f32, y: f32) -> Result<Pixel8, Error> {
        unsafe {
            let mut pixel = std::mem::zeroed();
            let f = (*(*self.in_data_ptr).utils).area_sample.ok_or(Error::BadCallbackParameter)?;
            match f((*self.in_data_ptr).effect_ref, Fixed::from(x).as_fixed(), Fixed::from(y).as_fixed(), &self.params, &mut pixel) {
                0 => Ok(pixel),
                e => Err(e.into()),
            }
        }
    }

    /// Use this to calculate the appropriate alpha weighted average of an axis-aligned non-integral rectangle of color in a source image, in high quality.
    /// Nearest neighbor in low quality.
    /// Because of overflow issues, this can only average a maximum of a 256 pixel by 256 pixel area (ie. x and y range < 128 pixels).
    pub fn area_sample16(&self, x: f32, y: f32) -> Result<Pixel16, Error> {
        unsafe {
            let mut pixel = std::mem::zeroed();
            let f = (*(*self.in_data_ptr).utils).area_sample16.ok_or(Error::BadCallbackParameter)?;
            match f((*self.in_data_ptr).effect_ref, Fixed::from(x).as_fixed(), Fixed::from(y).as_fixed(), &self.params, &mut pixel) {
                0 => Ok(pixel),
                e => Err(e.into()),
            }
        }
    }
}
impl Drop for Sampling {
    fn drop(&mut self) {
        unsafe {
            let in_data = &(*self.in_data_ptr);
            if in_data.utils.is_null() || in_data.effect_ref.is_null() {
                return;
            }
            let end_sampling = (*in_data.utils).end_sampling.unwrap(); // We're safe to unwrap because begin_sampling was successful
            let _ = end_sampling(in_data.effect_ref, self.quality, self.mode_flags, &mut self.params);
        }
    }
}
