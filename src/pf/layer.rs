use super::*;
use ae_sys::PF_LayerDef;

pub struct Layer {
    pub(crate) in_data: InData,
    pub(crate) layer_ptr: *mut PF_LayerDef,
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("buffer_stride", &self.buffer_stride())
            .field("rowbytes", &self.rowbytes())
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
    pub fn from_raw(layer_ptr: *mut PF_LayerDef, in_data: InData) -> Self {
        assert!(!layer_ptr.is_null());
        Self { in_data, layer_ptr }
    }

    pub fn as_ptr(&self) -> *mut PF_LayerDef {
        self.layer_ptr
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.layer_ptr).width }
    }
    pub fn height(&self) -> i32 {
        unsafe { (*self.layer_ptr).height }
    }
    pub fn buffer_stride(&self) -> usize {
        unsafe { (*self.layer_ptr).rowbytes }.abs() as usize
    }
    pub fn rowbytes(&self) -> i32 {
        unsafe { (*self.layer_ptr).rowbytes }
    }
    pub fn extent_hint(&self) -> Rect {
        unsafe { (*self.layer_ptr).extent_hint.into() }
    }
    pub fn bit_depth(&self) -> i16 {
        let flags = WorldFlags::from_bits(unsafe { (*self.layer_ptr).world_flags } as _).unwrap();
        if flags.contains(WorldFlags::DEEP) {
            16
        } else if flags.contains(WorldFlags::RESERVED1) {
            32
        } else {
            (self.rowbytes().abs() as f32 / self.width() as f32).floor() as i16 / 4 * 8
        }
    }

    pub fn buffer(&self) -> &[u8] {
        // Stride can be negative, so we need to offset the pointer to get to the real beginning of the buffer
        let offset = if self.rowbytes() < 0 {
            (self.rowbytes() * (self.height() - 1)) as isize
        } else {
            0
        };
        unsafe {
            assert!(self.rowbytes().abs() > 0);
            assert!(!(*self.layer_ptr).data.is_null());
            std::slice::from_raw_parts(
                ((*self.layer_ptr).data as *const u8).offset(offset),
                (self.height() * self.rowbytes().abs()) as usize,
            )
        }
    }
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        // Stride can be negative, so we need to offset the pointer to get to the real beginning of the buffer
        let offset = if self.rowbytes() < 0 {
            (self.rowbytes() * (self.height() - 1)) as isize
        } else {
            0
        };
        unsafe {
            assert!(self.rowbytes().abs() > 0);
            assert!(!(*self.layer_ptr).data.is_null());
            std::slice::from_raw_parts_mut(
                ((*self.layer_ptr).data as *mut u8).offset(offset),
                (self.height() * self.rowbytes().abs()) as usize,
            )
        }
    }

    pub fn copy_from(&mut self, src: &Self, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        self.in_data.utils().copy(src.layer_ptr, self.layer_ptr, src_rect, dst_rect)
    }

    pub fn fill(&mut self, color: Option<Pixel8>, rect: Option<Rect>) -> Result<(), Error> {
        self.in_data.utils().fill(self.layer_ptr, color, rect)
    }
    pub fn fill16(&mut self, color: Option<Pixel16>, rect: Option<Rect>) -> Result<(), Error> {
        self.in_data.utils().fill16(self.layer_ptr, color, rect)
    }

    pub fn iterate_with<F>(&self, output: &mut Self, progress_base: i32, progress_final: i32, area: Option<Rect>, cb: F) -> Result<(), Error>
    where
        F: Fn(i32, i32, GenericPixel, GenericPixelMut) -> Result<(), Error>,
    {
        if self.layer_ptr.is_null() || output.layer_ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        assert!(self.bit_depth() == output.bit_depth());

        match self.bit_depth() {
            8 => self.in_data.utils().iterate(Some(self.layer_ptr), output.layer_ptr, progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
                cb(x, y, GenericPixel::Pixel8(in_pixel), GenericPixelMut::Pixel8(out_pixel))
            }),
            16 => self.in_data.utils().iterate16(Some(self.layer_ptr), output.layer_ptr, progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
                cb(x, y, GenericPixel::Pixel16(in_pixel), GenericPixelMut::Pixel16(out_pixel))
            }),
            32 => {
                let suite = pf::suites::IterateFloat::new()?;
                suite.iterate(&self.in_data, Some(self.layer_ptr), output.layer_ptr, progress_base, progress_final, area, move |x, y, in_pixel, out_pixel| {
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
        if self.layer_ptr.is_null() {
            return Err(Error::BadCallbackParameter);
        }
        match self.bit_depth() {
            8 => self.in_data.utils().iterate(None, self.layer_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                cb(x, y, GenericPixelMut::Pixel8(out_pixel))
            }),
            16 => self.in_data.utils().iterate16(None, self.layer_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                cb(x, y, GenericPixelMut::Pixel16(out_pixel))
            }),
            32 => {
                let suite = pf::suites::IterateFloat::new()?;
                suite.iterate(&self.in_data, None, self.layer_ptr, progress_base, progress_final, area, move |x, y, _, out_pixel| {
                    cb(x, y, GenericPixelMut::PixelF32(out_pixel))
                })
            },
            _ => Err(Error::BadCallbackParameter),
        }
    }
}
