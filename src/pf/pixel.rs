use crate::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Pixel {
    pub alpha: ae_sys::A_u_char,
    pub red: ae_sys::A_u_char,
    pub green: ae_sys::A_u_char,
    pub blue: ae_sys::A_u_char,
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

impl From<ae_sys::PF_Pixel> for Pixel {
    fn from(pixel: ae_sys::PF_Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

impl From<Pixel> for ae_sys::PF_Pixel {
    fn from(pixel: Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

pub type Pixel8 = Pixel;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel16 {
    pub alpha: ae_sys::A_u_short,
    pub red: ae_sys::A_u_short,
    pub green: ae_sys::A_u_short,
    pub blue: ae_sys::A_u_short,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel32 {
    pub alpha: ae_sys::PF_FpShort,
    pub red: ae_sys::PF_FpShort,
    pub green: ae_sys::PF_FpShort,
    pub blue: ae_sys::PF_FpShort,
}
