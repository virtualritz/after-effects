use super::*;
use ae_sys::PF_LayerDef;

pub struct Layer {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
    pub(crate) layer: PointerOwnership<PF_LayerDef>,
    drop_fn: Option<fn(&mut Self)>
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("buffer_stride", &self.buffer_stride())
            .field("row_bytes", &self.row_bytes())
            .field("extent_hint", &self.extent_hint())
            .field("bit_depth", &self.bit_depth())
            .finish()
    }
}

//pub world_flags: PF_WorldFlags,
//pub data: PF_PixelPtr,
//pub rowbytes: A_long,
//pub width: A_long,
//pub height: A_long,
//pub extent_hint: PF_UnionableRect,
//pub platform_ref: *mut c_void,
//pub pix_aspect_ratio: PF_RationalScale,
//pub origin_x: A_long,
//pub origin_y: A_long,
//pub dephault: A_long,

impl Layer {
    pub fn from_aegp_world(in_data: impl AsPtr<*const ae_sys::PF_InData>, world_handle: impl AsPtr<ae_sys::AEGP_WorldH>) -> Result<Self, crate::Error> {
        // SAFETY: PF_LayerDef is a C struct from After Effects that is safe to zero-initialize.
        // All its fields are either integers, pointers, or C structs that are valid when zeroed.
        // The zeroed value is immediately overwritten by fill_out_pf_effect_world below.
        // UB would occur if: the struct contained non-nullable references, Rust enums with
        // invalid discriminants, or types that require specific initialization patterns.
        let mut layer: PF_LayerDef = unsafe { std::mem::zeroed() };

        aegp::suites::World::new()?
            .fill_out_pf_effect_world(world_handle.as_ptr(), &mut layer)?;

        Ok(Self { in_data_ptr: in_data.as_ptr(), layer: PointerOwnership::Rust(layer), drop_fn: None })
    }

    pub fn from_owned(layer: PF_LayerDef, in_data: impl AsPtr<*const ae_sys::PF_InData>, drop_fn: fn(&mut Self)) -> Self {
        Self { in_data_ptr: in_data.as_ptr(), layer: PointerOwnership::Rust(layer), drop_fn: Some(drop_fn) }
    }

    pub fn from_raw(layer_ptr: *mut PF_LayerDef, in_data: impl AsPtr<*const ae_sys::PF_InData>, drop_fn: Option<fn(&mut Self)>) -> Self {
        assert!(!layer_ptr.is_null());
        Self { in_data_ptr: in_data.as_ptr(), layer: PointerOwnership::AfterEffects(layer_ptr), drop_fn }
    }

    pub fn width(&self) -> usize {
        self.layer.width as usize
    }
    pub fn height(&self) -> usize {
        self.layer.height as usize
    }
    pub fn buffer_stride(&self) -> usize {
        self.layer.rowbytes.abs() as usize
    }
    pub fn row_bytes(&self) -> isize {
        self.layer.rowbytes as isize
    }
    pub fn extent_hint(&self) -> Rect {
        self.layer.extent_hint.into()
    }
    pub fn pix_aspect_ratio(&self) -> RationalScale {
        self.layer.pix_aspect_ratio.into()
    }
    pub fn origin(&self) -> Point {
        Point {
            h: self.layer.origin_x.into(),
            v: self.layer.origin_y.into(),
        }
    }

    pub fn buffer(&self) -> &[u8] {
        // Stride can be negative, so we need to offset the pointer to get to the real beginning of the buffer
        let offset = if self.row_bytes() < 0 {
            self.row_bytes() * (self.height() as isize - 1)
        } else {
            0
        };
        // SAFETY: This function creates a byte slice from the layer's pixel data buffer.
        // Invariants upheld:
        // 1. self.layer.data is non-null (verified by assertion) and points to a valid buffer
        //    allocated by After Effects with lifetime at least as long as this Layer instance.
        // 2. The offset calculation handles negative strides (bottom-up buffers) by computing
        //    the topmost scanline address, ensuring we point to the actual buffer start.
        // 3. The length calculation (height * abs(row_bytes)) produces the total buffer size,
        //    which is guaranteed to be valid memory allocated by the After Effects host.
        // 4. The resulting slice lifetime is tied to &self, preventing use-after-free.
        // 5. No mutable aliases exist since this takes &self immutably.
        // UB would occur if: self.layer.data is a dangling pointer, the buffer size calculation
        // overflows, or the computed memory region extends beyond the allocated buffer bounds.
        unsafe {
            assert!(self.row_bytes().abs() > 0);
            assert!(!self.layer.data.is_null());
            std::slice::from_raw_parts(
                (self.layer.data as *const u8).offset(offset),
                (self.height() as isize * self.row_bytes().abs()) as usize,
            )
        }
    }
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        // Stride can be negative, so we need to offset the pointer to get to the real beginning of the buffer
        let offset = if self.row_bytes() < 0 {
            self.row_bytes() * (self.height() as isize - 1)
        } else {
            0
        };
        // SAFETY: This function creates a mutable byte slice from the layer's pixel data buffer.
        // Invariants upheld:
        // 1. self.layer.data is non-null (verified by assertion) and points to a valid buffer
        //    allocated by After Effects with lifetime at least as long as this Layer instance.
        // 2. The offset calculation handles negative strides (bottom-up buffers) by computing
        //    the topmost scanline address, ensuring we point to the actual buffer start.
        // 3. The length calculation (height * abs(row_bytes)) produces the total buffer size,
        //    which is guaranteed to be valid memory allocated by the After Effects host.
        // 4. The resulting slice lifetime is tied to &mut self, preventing aliasing.
        // 5. Exclusive mutable access is guaranteed by Rust's borrow checker (&mut self).
        // UB would occur if: self.layer.data is a dangling pointer, the buffer size calculation
        // overflows, the computed memory region extends beyond allocated bounds, or another
        // mutable reference to this buffer exists simultaneously.
        unsafe {
            assert!(self.row_bytes().abs() > 0);
            assert!(!self.layer.data.is_null());
            std::slice::from_raw_parts_mut(
                (self.layer.data as *mut u8).offset(offset),
                (self.height() as isize * self.row_bytes().abs()) as usize,
            )
        }
    }

    pub fn copy_from(&mut self, src: &Self, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        self.utils().copy(src, self, src_rect, dst_rect)
    }

    pub fn utils(&self) -> UtilCallbacks {
        UtilCallbacks::new(self.in_data_ptr)
    }

    fn clamp_rect(&self, rect: &mut Option<Rect>) {
        if let Some(rect) = rect {
            if rect.left < 0 { rect.left = 0; }
            if rect.top  < 0 { rect.top  = 0; }
            if rect.width()  > self.width()  as i32 { rect.set_width(self.width() as i32); }
            if rect.height() > self.height() as i32 { rect.set_height(self.height() as i32); }
        }
    }

    pub fn fill(&mut self, color: Option<Pixel8>, mut rect: Option<Rect>) -> Result<(), Error> {
        self.clamp_rect(&mut rect);
        if self.bit_depth() == 16 {
            return self.fill16(color.map(pixel8_to_16), rect);
        }
        // SAFETY: Dereferencing self.in_data_ptr to access After Effects InData structure.
        // Invariants upheld:
        // 1. Pointer validity is checked (non-null) before dereferencing.
        // 2. in_data_ptr is provided by After Effects host and remains valid for the plugin's lifetime.
        // 3. PF_InData.appl_id is a plain integer field, safe to read.
        // 4. The comparison checks if we're running in Premiere Pro (which has different fill behavior).
        // UB would occur if: in_data_ptr points to freed/invalid memory, or if After Effects
        // invalidates InData while the plugin is still executing (violates AE API contract).
        if !self.in_data_ptr.is_null() && unsafe { (*self.in_data_ptr).appl_id != i32::from_be_bytes(*b"PrMr") } {
            // SAFETY: Dereferencing self.in_data_ptr to access effect_ref field.
            // Invariants upheld:
            // 1. We've already verified the pointer is non-null and valid in the condition above.
            // 2. effect_ref is an opaque handle provided by After Effects that remains valid
            //    for the duration of the effect call.
            // 3. The handle is passed to After Effects suite functions which expect this type.
            // UB would occur if: the InData structure becomes invalid between the check above
            // and this access (not possible in single-threaded AE plugin execution model).
            if let Ok(fill_suite) = pf::suites::FillMatte::new() {
                return fill_suite.fill(unsafe { (*self.in_data_ptr).effect_ref }, self, color, rect);
            }
        }
        self.utils().fill(self, color, rect)
    }
    pub fn fill16(&mut self, color: Option<Pixel16>, mut rect: Option<Rect>) -> Result<(), Error> {
        self.clamp_rect(&mut rect);
        // SAFETY: Dereferencing self.in_data_ptr to access After Effects InData structure.
        // Invariants upheld:
        // 1. Pointer validity is checked (non-null) before dereferencing.
        // 2. in_data_ptr is provided by After Effects host and remains valid for the plugin's lifetime.
        // 3. PF_InData.appl_id is a plain integer field, safe to read.
        // 4. The comparison checks if we're running in Premiere Pro (which has different fill behavior).
        // UB would occur if: in_data_ptr points to freed/invalid memory, or if After Effects
        // invalidates InData while the plugin is still executing (violates AE API contract).
        if !self.in_data_ptr.is_null() && unsafe { (*self.in_data_ptr).appl_id != i32::from_be_bytes(*b"PrMr") } {
            // SAFETY: Dereferencing self.in_data_ptr to access effect_ref field.
            // Invariants upheld:
            // 1. We've already verified the pointer is non-null and valid in the condition above.
            // 2. effect_ref is an opaque handle provided by After Effects that remains valid
            //    for the duration of the effect call.
            // 3. The handle is passed to After Effects suite functions which expect this type.
            // UB would occur if: the InData structure becomes invalid between the check above
            // and this access (not possible in single-threaded AE plugin execution model).
            if let Ok(fill_suite) = pf::suites::FillMatte::new() {
                return fill_suite.fill16(unsafe { (*self.in_data_ptr).effect_ref }, self, color, rect);
            }
        }
        self.utils().fill16(self, color, rect)
    }

    pub fn iterate_with<F>(&self, output: &mut Self, progress_base: i32, progress_final: i32, area: Option<Rect>, cb: F) -> Result<(), Error>
    where
        F: Fn(i32, i32, GenericPixel, GenericPixelMut) -> Result<(), Error>,
    {
        assert!(self.bit_depth() == output.bit_depth());

        let self_ptr = Some(self.as_ptr());
        match self.bit_depth() {
            8 => self.utils().iterate(self_ptr, output.as_mut_ptr(), progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
                cb(x, y, GenericPixel::Pixel8(in_pixel), GenericPixelMut::Pixel8(out_pixel))
            }),
            16 => self.utils().iterate16(self_ptr, output.as_mut_ptr(), progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
                cb(x, y, GenericPixel::Pixel16(in_pixel), GenericPixelMut::Pixel16(out_pixel))
            }),
            32 => {
                let suite = pf::suites::IterateFloat::new()?;
                suite.iterate(self.in_data_ptr, self_ptr, output.as_mut_ptr(), progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
                    cb(x, y, GenericPixel::PixelF32(in_pixel), GenericPixelMut::PixelF32(out_pixel))
                })
            },
            _ => Err(Error::BadCallbackParameter),
        }
    }

    pub fn iterate<F>(&mut self, progress_base: i32, progress_final: i32, area: Option<Rect>, cb: F) -> Result<(), Error>
    where
        F: Fn(i32, i32, GenericPixelMut) -> Result<(), Error>,
    {
        let self_ptr = self.as_mut_ptr();
        match self.bit_depth() {
            8 => self.utils().iterate(None, self_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                cb(x, y, GenericPixelMut::Pixel8(out_pixel))
            }),
            16 => self.utils().iterate16(None, self_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                cb(x, y, GenericPixelMut::Pixel16(out_pixel))
            }),
            32 => {
                let suite = pf::suites::IterateFloat::new()?;
                suite.iterate(self.in_data_ptr, None, self_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                    cb(x, y, GenericPixelMut::PixelF32(out_pixel))
                })
            },
            _ => Err(Error::BadCallbackParameter),
        }
    }

    /// Returns a raw pointer to the pixel data buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// 1. The pointer is not dereferenced if the buffer has been freed or invalidated.
    /// 2. The pointer remains valid only for the lifetime of this Layer instance.
    /// 3. Any memory access through this pointer stays within the buffer bounds.
    /// 4. Proper alignment is maintained for the pixel format being accessed.
    pub unsafe fn data_ptr(&self) -> *const u8 {
        self.layer.data as *const u8
    }

    /// Returns a mutable raw pointer to the pixel data buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// 1. The pointer is not dereferenced if the buffer has been freed or invalidated.
    /// 2. The pointer remains valid only for the lifetime of this Layer instance.
    /// 3. Any memory access through this pointer stays within the buffer bounds.
    /// 4. Proper alignment is maintained for the pixel format being accessed.
    /// 5. No other references (mutable or immutable) to the buffer exist while this
    ///    pointer is being used for writes, to prevent data races and aliasing violations.
    pub unsafe fn data_ptr_mut(&self) -> *mut u8 {
        self.layer.data as *mut u8
    }

    pub fn row_padding_bytes(&self) -> usize {
        self.buffer_stride()
            - self.width()
                * 4
                * match self.world_type() {
                    aegp::WorldType::U15 => 2,
                    aegp::WorldType::U8 => 1,
                    aegp::WorldType::F32 => 4,
                    aegp::WorldType::None => panic!(),
                }
    }

    pub fn as_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(x < self.width() && y < self.height(), "Coordinate is outside EffectWorld bounds.");
        // SAFETY: This function creates a mutable reference to a specific pixel in the buffer.
        // Invariants upheld:
        // 1. Bounds checking via debug_assert ensures x and y are within valid dimensions.
        // 2. data_ptr_mut() returns the base buffer pointer from After Effects (assumed valid).
        // 3. Offset arithmetic: y * row_bytes() computes the row start address, handling both
        //    positive (top-down) and negative (bottom-up) strides correctly.
        // 4. Additional offset by x positions to the target Pixel8 within the row.
        // 5. The resulting pointer is aligned for Pixel8 (4-byte ARGB structure).
        // 6. Lifetime of the returned reference is tied to &self, preventing use-after-free.
        // UB would occur if: coordinates are out of bounds (only checked in debug builds),
        // the buffer pointer is invalid, offset calculation overflows, resulting pointer is
        // misaligned, or multiple mutable references to the same pixel exist concurrently.
        unsafe { &mut *(self.data_ptr_mut().offset(y as isize * self.row_bytes()) as *mut Pixel8).offset(x as isize) }
    }

    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        self.as_pixel8_mut(x, y)
    }

    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(x < self.width() && y < self.height(), "Coordinate is outside EffectWorld bounds.");
        // SAFETY: This function creates a mutable reference to a specific 16-bit pixel in the buffer.
        // Invariants upheld:
        // 1. Bounds checking via debug_assert ensures x and y are within valid dimensions.
        // 2. data_ptr_mut() returns the base buffer pointer from After Effects (assumed valid).
        // 3. Offset arithmetic: y * row_bytes() computes the row start address, handling both
        //    positive (top-down) and negative (bottom-up) strides correctly.
        // 4. Additional offset by x positions to the target Pixel16 within the row.
        // 5. The resulting pointer is aligned for Pixel16 (8-byte structure with 16-bit channels).
        // 6. Lifetime of the returned reference is tied to &self, preventing use-after-free.
        // UB would occur if: coordinates are out of bounds (only checked in debug builds),
        // the buffer pointer is invalid, offset calculation overflows, resulting pointer is
        // misaligned for Pixel16, or multiple mutable references to the same pixel exist concurrently.
        unsafe { &mut *(self.data_ptr_mut().offset(y as isize * self.row_bytes()) as *mut Pixel16).offset(x as isize) }
    }

    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        self.as_pixel16_mut(x, y)
    }

    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut PixelF32 {
        debug_assert!(x < self.width() && y < self.height(), "Coordinate is outside EffectWorld bounds.");
        // SAFETY: This function creates a mutable reference to a specific 32-bit float pixel.
        // Invariants upheld:
        // 1. Bounds checking via debug_assert ensures x and y are within valid dimensions.
        // 2. data_ptr_mut() returns the base buffer pointer from After Effects (assumed valid).
        // 3. Offset arithmetic: y * row_bytes() computes the row start address, handling both
        //    positive (top-down) and negative (bottom-up) strides correctly.
        // 4. Additional add(x) positions to the target PixelF32 within the row (add is used
        //    instead of offset as it multiplies by size_of::<PixelF32>()).
        // 5. The resulting pointer is aligned for PixelF32 (16-byte structure with f32 channels).
        // 6. Lifetime of the returned reference is tied to &self, preventing use-after-free.
        // UB would occur if: coordinates are out of bounds (only checked in debug builds),
        // the buffer pointer is invalid, offset calculation overflows, resulting pointer is
        // misaligned for PixelF32, or multiple mutable references to the same pixel exist concurrently.
        unsafe { &mut *(self.data_ptr_mut().offset(y as isize * self.row_bytes()) as *mut PixelF32).add(x) }
    }

    pub fn as_pixel32(&self, x: usize, y: usize) -> &PixelF32 {
        self.as_pixel32_mut(x, y)
    }

    pub fn world_type(&self) -> aegp::WorldType {
        let flags = WorldFlags::from_bits(self.layer.world_flags as _).unwrap();
        // Most frequent case is 16bit integer.
        if flags.contains(WorldFlags::DEEP) {
            aegp::WorldType::U15
        } else if flags.contains(WorldFlags::RESERVED1) {
            aegp::WorldType::F32
        } else {
            aegp::WorldType::U8
        }
    }

    pub fn bit_depth(&self) -> i16 {
        let flags = WorldFlags::from_bits(self.layer.world_flags as _).unwrap();
        if flags.contains(WorldFlags::DEEP) {
            16
        } else if flags.contains(WorldFlags::RESERVED1) {
            32
        } else {
            if InData::from_raw(self.in_data_ptr).is_premiere() {
                match self.pr_pixel_format() {
                    Ok(pr::PixelFormat::Rgb444_10u) |
                    Ok(pr::PixelFormat::V210422_10u601) |
                    Ok(pr::PixelFormat::V210422_10u709) => { 10 }
                    Ok(pr::PixelFormat::Rgb444_12uPq709) |
                    Ok(pr::PixelFormat::Rgb444_12uPqP3) |
                    Ok(pr::PixelFormat::Rgb444_12uPq2020) => { 12 }
                    Ok(pr::PixelFormat::Bgra4444_16u) |
                    Ok(pr::PixelFormat::Vuya4444_16u) |
                    Ok(pr::PixelFormat::Argb4444_16u) |
                    Ok(pr::PixelFormat::Bgrx4444_16u) |
                    Ok(pr::PixelFormat::Xrgb4444_16u) |
                    Ok(pr::PixelFormat::Bgrp4444_16u) |
                    Ok(pr::PixelFormat::Prgb4444_16u) => { 16 }
                    Ok(pr::PixelFormat::Bgra4444_32f) |
                    Ok(pr::PixelFormat::Vuya4444_32f) |
                    Ok(pr::PixelFormat::Vuya4444_32f709) |
                    Ok(pr::PixelFormat::Argb4444_32f) |
                    Ok(pr::PixelFormat::Bgrx4444_32f) |
                    Ok(pr::PixelFormat::Vuyx4444_32f) |
                    Ok(pr::PixelFormat::Vuyx4444_32f709) |
                    Ok(pr::PixelFormat::Xrgb4444_32f) |
                    Ok(pr::PixelFormat::Bgrp4444_32f) |
                    Ok(pr::PixelFormat::Vuyp4444_32f) |
                    Ok(pr::PixelFormat::Vuyp4444_32f709) |
                    Ok(pr::PixelFormat::Prgb4444_32f) |
                    Ok(pr::PixelFormat::Uyvy422_32f601) |
                    Ok(pr::PixelFormat::Uyvy422_32f709) |
                    Ok(pr::PixelFormat::Bgra4444_32fLinear) |
                    Ok(pr::PixelFormat::Bgrp4444_32fLinear) |
                    Ok(pr::PixelFormat::Bgrx4444_32fLinear) |
                    Ok(pr::PixelFormat::Argb4444_32fLinear) |
                    Ok(pr::PixelFormat::Prgb4444_32fLinear) |
                    Ok(pr::PixelFormat::Xrgb4444_32fLinear) => { 32 }
                    _ => { 8 }
                }
            } else {
                match self.pixel_format() {
                    Ok(PixelFormat::Argb64) => { 16 }
                    Ok(PixelFormat::Argb128) |
                    Ok(PixelFormat::GpuBgra128) => { 32 }
                    _ => { 8 }
                }
            }
        }
    }

    pub fn pixel_format(&self) -> Result<PixelFormat, Error> {
        pf::suites::World::new()?.pixel_format(self)
    }

    pub fn pr_pixel_format(&self) -> Result<pr::PixelFormat, Error> {
        pf::suites::PixelFormat::new()?.pixel_format(self)
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        if let Some(drop_fn) = self.drop_fn {
            drop_fn(self);
        }
    }
}

impl AsPtr<*const ae_sys::PF_EffectWorld> for Layer {
    fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        &*self.layer
    }
}
impl AsPtr<*const ae_sys::PF_EffectWorld> for &Layer {
    fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        &*self.layer
    }
}
impl AsPtr<*const ae_sys::PF_EffectWorld> for *const ae_sys::PF_EffectWorld {
    fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        *self
    }
}

impl AsMutPtr<*mut ae_sys::PF_EffectWorld> for Layer {
    fn as_mut_ptr(&mut self) -> *mut ae_sys::PF_EffectWorld {
        &mut *self.layer
    }
}
impl AsMutPtr<*mut ae_sys::PF_EffectWorld> for &mut Layer {
    fn as_mut_ptr(&mut self) -> *mut ae_sys::PF_EffectWorld {
        &mut *self.layer
    }
}

