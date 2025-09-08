
use crate::*;
use ae_sys::*;

define_suite!(
    BackgroundFrameSuite,
    PF_BackgroundFrameSuite1,
    kPFBackgroundFrameSuite,
    kPFBackgroundFrameSuiteVersion1
);

impl BackgroundFrameSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn add_supported_background_transfer_mode(&self, effect_ref: impl AsPtr<PF_ProgPtr>, supported_transfer_mode: TransferMode, supported_pixel_format: pr::PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedBackgroundTransferMode, effect_ref.as_ptr(), supported_transfer_mode.into(), supported_pixel_format.into())
    }

    pub fn background_frame(&self, in_data: impl AsPtr<*const PF_InData>) -> Result<(Layer, TransferMode), Error> {
        let mut background_frame = std::ptr::null_mut();
        let mut background_transfer_mode: ae_sys::PF_TransferMode = 0;
        call_suite_fn!(self, GetBackgroundFrame, (*in_data.as_ptr()).effect_ref, &mut background_frame, &mut background_transfer_mode)?;
        Ok((
            Layer::from_raw(background_frame, in_data.as_ptr(), None),
            background_transfer_mode.into()
        ))
    }
}
