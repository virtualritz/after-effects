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

    pub fn buffer(&self) -> &[u8] {
        // Stride can be negative, so we need to offset the pointer to get to the real beginning of the buffer
        let offset = if self.row_bytes() < 0 {
            self.row_bytes() * (self.height() as isize - 1)
        } else {
            0
        };
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

    pub fn fill(&mut self, color: Option<Pixel8>, rect: Option<Rect>) -> Result<(), Error> {
        if self.bit_depth() == 16 {
            return self.fill16(color.map(pixel8_to_16), rect);
        }
        if !self.in_data_ptr.is_null() && unsafe { (*self.in_data_ptr).appl_id != i32::from_be_bytes(*b"PrMr") } {
            if let Ok(fill_suite) = pf::suites::FillMatte::new() {
                return fill_suite.fill(unsafe { (*self.in_data_ptr).effect_ref }, self, color, rect);
            }
        }
        self.utils().fill(self, color, rect)
    }
    pub fn fill16(&mut self, color: Option<Pixel16>, rect: Option<Rect>) -> Result<(), Error> {
        if self.in_data_ptr.is_null() && unsafe { (*self.in_data_ptr).appl_id != i32::from_be_bytes(*b"PrMr") } {
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

    pub unsafe fn data_ptr(&self) -> *const u8 {
        self.layer.data as *const u8
    }

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
        unsafe { &mut *(self.data_ptr_mut().offset(y as isize * self.row_bytes()) as *mut Pixel8).offset(x as isize) }
    }

    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        self.as_pixel8_mut(x, y)
    }

    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(x < self.width() && y < self.height(), "Coordinate is outside EffectWorld bounds.");
        unsafe { &mut *(self.data_ptr_mut().offset(y as isize * self.row_bytes()) as *mut Pixel16).offset(x as isize) }
    }

    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        self.as_pixel16_mut(x, y)
    }

    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut PixelF32 {
        debug_assert!(x < self.width() && y < self.height(), "Coordinate is outside EffectWorld bounds.");
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
            (self.row_bytes().abs() as f32 / self.width() as f32).floor() as i16 / 4 * 8
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

