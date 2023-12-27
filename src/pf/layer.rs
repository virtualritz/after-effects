use super::*;

pub struct Layer {
    pub(crate) in_data: *const ae_sys::PF_InData,
    pub(crate) layer_ptr: *mut ae_sys::PF_LayerDef,
}

impl Layer {
    pub fn from_raw(in_data: *const ae_sys::PF_InData, layer_ptr: *mut ae_sys::PF_LayerDef) -> Self {
        assert!(!in_data.is_null());
        assert!(!layer_ptr.is_null());
        Self { in_data, layer_ptr }
    }

    pub fn extent_hint(&self) -> Rect {
        unsafe { (*self.layer_ptr).extent_hint.into() }
    }

    pub fn copy_from(&mut self, src: &Self, src_rect: Option<Rect>, dst_rect: Option<Rect>) -> Result<(), Error> {
        unsafe {
            if let Some(copy_fn) = (*(*self.in_data).utils).copy {
                let err = copy_fn(
                    (*self.in_data).effect_ref,
                    src.layer_ptr,
                    self.layer_ptr,
                    src_rect.map(|x| x.into()).map(|mut x| &mut x as *mut _).unwrap_or(std::ptr::null_mut()),
                    dst_rect.map(|x| x.into()).map(|mut x| &mut x as *mut _).unwrap_or(std::ptr::null_mut()),
                );
            }
        }
        Ok(()) // TODO
    }
    pub fn fill(&mut self, color: Pixel, rect: Option<Rect>) -> Result<(), Error> {
        unsafe {
            if let Some(copy_fn) = (*(*self.in_data).utils).fill {
                let err = copy_fn(
                    (*self.in_data).effect_ref,
                    &color.into() as *const _,
                    rect.map(|x| x.into()).map(|x| &x as *const _).unwrap_or(std::ptr::null_mut()),
                    self.layer_ptr,
                );
            }
        }
        Ok(()) // TODO
    }


    /*pub iterate: ::std::option::Option<
        unsafe extern "C" fn(
            in_data: *mut PF_InData,
            progress_base: A_long,
            progress_final: A_long,
            src: *mut PF_EffectWorld,
            area: *const PF_Rect,
            refcon: *mut ::std::os::raw::c_void,
            pix_fn: PF_IteratePixel8Func,
            dst: *mut PF_EffectWorld,
        ) -> PF_Err,
    >,*/

    pub fn iterate<'a, F>(&self, output: &mut Self, progress_base: i32, progress_final: i32, extent_hint: Rect, cb: F) -> Result<(), Error>
    where F: Fn(i32, i32, Pixel) -> Result<Pixel, Error> + 'a {
        let callback = Box::<Iterate8Cb>::new(Iterate8Cb::<'a>(Box::new(cb)));
        unsafe {
            if let Some(iterate_fn) = (*(*self.in_data).utils).iterate {
                let err = iterate_fn(
                    self.in_data as *mut _,
                    progress_base,
                    progress_final,
                    self.layer_ptr,
                    &ae_sys::PF_LRect::from(extent_hint),
                    Box::into_raw(callback) as *mut _,
                    Some(iterate_8_func),
                    output.layer_ptr,
                );
            }
        }
        Ok(()) // TODO
    }
}
struct Iterate8Cb<'a>(Box<dyn Fn(i32, i32, Pixel) -> Result<Pixel, Error> + 'a>);

unsafe extern "C" fn iterate_8_func(refcon: *mut std::ffi::c_void, x: i32, y: i32, in_p: *mut ae_sys::PF_Pixel, out_p: *mut ae_sys::PF_Pixel) -> ae_sys::PF_Err {
    if refcon.is_null() || in_p.is_null() || out_p.is_null() {
        return ae_sys::PF_Err_BAD_CALLBACK_PARAM as ae_sys::PF_Err;
    }
    let refcon = refcon as *mut Iterate8Cb;

    let cb = Box::from_raw(refcon);
    let ret = match (cb.0)(x, y, (*in_p).into()) {
        Ok(px) => {
            *out_p = px.into();
            ae_sys::PF_Err_NONE as ae_sys::PF_Err
        },
        Err(e) => e as ae_sys::PF_Err,
    };

    let _ = Box::into_raw(cb); // leak here so it doesn't get deleted

    ret
}
