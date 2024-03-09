use crate::*;

// Don't define separate wrappers for pixel types, because any potential
// additional per-pixel allocation may have a significant performance impact.

pub type Pixel8 = ae_sys::PF_Pixel;
pub type Pixel16 = ae_sys::PF_Pixel16;
pub type PixelF32 = ae_sys::PF_Pixel32;
pub type PixelF64 = ae_sys::AEGP_ColorVal;

pub use ae_sys::PF_MAX_CHAN8 as MAX_CHANNEL8;
pub use ae_sys::PF_HALF_CHAN8 as HALF_CHANNEL8;
pub use ae_sys::PF_MAX_CHAN16 as MAX_CHANNEL16;
pub use ae_sys::PF_HALF_CHAN16 as HALF_CHANNEL16;

pub fn pixel8_to_16(p: Pixel8) -> Pixel16 {
    fn convert_8_to_16(x: u8) -> u16 {
        (((x as u32 * ae_sys::PF_MAX_CHAN16) + ae_sys::PF_HALF_CHAN8) / ae_sys::PF_MAX_CHAN8) as u16
    }

    Pixel16 {
        alpha: convert_8_to_16(p.alpha),
        red:   convert_8_to_16(p.red),
        green: convert_8_to_16(p.green),
        blue:  convert_8_to_16(p.blue),
    }
}

pub fn pixel16_to_8(p: Pixel16) -> Pixel8 {
    fn convert_16_to_8(x: u16) -> u8 {
        (((x as u32 * ae_sys::PF_MAX_CHAN8) + ae_sys::PF_HALF_CHAN16) / ae_sys::PF_MAX_CHAN16) as u8
    }

    Pixel8 {
        alpha: convert_16_to_8(p.alpha),
        red:   convert_16_to_8(p.red),
        green: convert_16_to_8(p.green),
        blue:  convert_16_to_8(p.blue),
    }
}


pub enum GenericPixel<'a> {
    Pixel8(&'a Pixel8),
    Pixel16(&'a Pixel16),
    PixelF32(&'a PixelF32),
    PixelF64(&'a PixelF64),
}

pub enum GenericPixelMut<'a> {
    Pixel8(&'a mut Pixel8),
    Pixel16(&'a mut Pixel16),
    PixelF32(&'a mut PixelF32),
    PixelF64(&'a mut PixelF64),
}

impl<'a> GenericPixel<'a> {
    pub fn as_u8(&self) -> Pixel8 {
        match self {
            Self::Pixel8 (p)  => **p,
            Self::Pixel16(p)  => Pixel8 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::PixelF32(p) => Pixel8 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::PixelF64(p) => Pixel8 { alpha: p.alphaF as _, red: p.redF as _, green: p.greenF as _, blue: p.blueF as _ },
        }
    }
    pub fn as_u16(&self) -> Pixel16 {
        match self {
            Self::Pixel8 (p)  => Pixel16 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::Pixel16(p)  => **p,
            Self::PixelF32(p) => Pixel16 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::PixelF64(p) => Pixel16 { alpha: p.alphaF as _, red: p.redF as _, green: p.greenF as _, blue: p.blueF as _ },
        }
    }
    pub fn as_f32(&self) -> PixelF32 {
        match self {
            Self::Pixel8 (p)  => PixelF32 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::Pixel16(p)  => PixelF32 { alpha: p.alpha as _, red: p.red as _, green: p.green as _, blue: p.blue as _ },
            Self::PixelF32(p) => **p,
            Self::PixelF64(p) => PixelF32 { alpha: p.alphaF as _, red: p.redF as _, green: p.greenF as _, blue: p.blueF as _ },
        }
    }
}

impl<'a> GenericPixelMut<'a> {
    pub fn set_from_u8(&mut self, px: Pixel8) {
        match self {
            Self::Pixel8 (s)  => { **s = px; },
            Self::Pixel16(s)  => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::PixelF32(s) => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::PixelF64(s) => { s.alphaF = px.alpha as _; s.redF = px.red as _; s.greenF = px.green as _; s.blueF = px.blue as _; },
        }
    }
    pub fn set_from_u16(&mut self, px: Pixel16) {
        match self {
            Self::Pixel8 (s)  => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::Pixel16(s)  => { **s = px; },
            Self::PixelF32(s) => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::PixelF64(s) => { s.alphaF = px.alpha as _; s.redF = px.red as _; s.greenF = px.green as _; s.blueF = px.blue as _; },
        }
    }
    pub fn set_from_f32(&mut self, px: PixelF32) {
        match self {
            Self::Pixel8 (s)  => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::Pixel16(s)  => { s.alpha = px.alpha as _; s.red = px.red as _; s.green = px.green as _; s.blue = px.blue as _; },
            Self::PixelF32(s) => { **s = px; },
            Self::PixelF64(s) => { s.alphaF = px.alpha as _; s.redF = px.red as _; s.greenF = px.green as _; s.blueF = px.blue as _; },
        }
    }
}
