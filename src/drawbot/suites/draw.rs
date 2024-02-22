use crate::*;

define_suite!(
    DrawbotSuite,
    DRAWBOT_DrawbotSuite1,
    kDRAWBOT_DrawSuite,
    kDRAWBOT_DrawSuite_Version1
);

impl DrawbotSuite {
    /// Get the supplier reference.
    pub fn get_supplier(&self, draw_ref: &DrawRef) -> Result<SupplierRef, Error> {
        Ok(SupplierRef::from_raw(
            call_suite_fn_single!(self, GetSupplier -> ae_sys::DRAWBOT_SupplierRef, draw_ref.as_ptr())?
        ))
    }

    /// Get the surface reference.
    pub fn get_surface(&self, draw_ref: &DrawRef) -> Result<SurfaceRef, Error> {
        Ok(SurfaceRef::from_raw(
            call_suite_fn_single!(self, GetSurface -> ae_sys::DRAWBOT_SurfaceRef, draw_ref.as_ptr())?
        ))
    }
}
