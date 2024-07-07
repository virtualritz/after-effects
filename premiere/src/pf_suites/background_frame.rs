
use crate::*;
use pr_sys::*;

define_suite!(
    ///
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

    pub fn add_supported_background_transfer_mode(&self, effect_ref: impl AsPtr<PF_ProgPtr>, supported_transfer_mode: TransferMode, supported_pixel_format: PixelFormat) -> Result<(), Error> {
        call_suite_fn!(self, AddSupportedBackgroundTransferMode, effect_ref.as_ptr(), supported_transfer_mode.into(), supported_pixel_format.into())
    }

    pub fn background_frame(&self, in_data: impl AsPtr<*const PF_InData>) -> Result<(*mut PF_EffectWorld, TransferMode), Error> {
        let mut background_frame = std::ptr::null_mut();
        let mut background_transfer_mode: pr_sys::PF_TransferMode = 0;
        call_suite_fn!(self, GetBackgroundFrame, (*in_data.as_ptr()).effect_ref, &mut background_frame, &mut background_transfer_mode)?;
        Ok((
            background_frame,
            background_transfer_mode.into()
        ))
    }
}

define_enum! {
    pr_sys::PF_XferMode,
    TransferMode {
        None                 = pr_sys::PF_Xfer_NONE,
        Copy                 = pr_sys::PF_Xfer_COPY,
        Behind               = pr_sys::PF_Xfer_BEHIND,
        InFront              = pr_sys::PF_Xfer_IN_FRONT,
        Dissolve             = pr_sys::PF_Xfer_DISSOLVE,
        Add                  = pr_sys::PF_Xfer_ADD,
        Mulitply             = pr_sys::PF_Xfer_MULTIPLY,
        Screen               = pr_sys::PF_Xfer_SCREEN,
        Overlay              = pr_sys::PF_Xfer_OVERLAY,
        SoftLight            = pr_sys::PF_Xfer_SOFT_LIGHT,
        HardLight            = pr_sys::PF_Xfer_HARD_LIGHT,
        Darken               = pr_sys::PF_Xfer_DARKEN,
        Lighten              = pr_sys::PF_Xfer_LIGHTEN,
        Difference           = pr_sys::PF_Xfer_DIFFERENCE,
        Hue                  = pr_sys::PF_Xfer_HUE,
        Saturation           = pr_sys::PF_Xfer_SATURATION,
        Color                = pr_sys::PF_Xfer_COLOR,
        Luminosity           = pr_sys::PF_Xfer_LUMINOSITY,
        MultiplyAlpha        = pr_sys::PF_Xfer_MULTIPLY_ALPHA,
        MultiplyAlphaLuma    = pr_sys::PF_Xfer_MULTIPLY_ALPHA_LUMA,
        MultiplyNotAlpha     = pr_sys::PF_Xfer_MULTIPLY_NOT_ALPHA,
        MultiplyNotAlphaLuma = pr_sys::PF_Xfer_MULTIPLY_NOT_ALPHA_LUMA,
        AddiditivePremul     = pr_sys::PF_Xfer_ADDITIVE_PREMUL,
        AlphaAdd             = pr_sys::PF_Xfer_ALPHA_ADD,
        ColorDodge           = pr_sys::PF_Xfer_COLOR_DODGE,
        ColorBurn            = pr_sys::PF_Xfer_COLOR_BURN,
        Exclusion            = pr_sys::PF_Xfer_EXCLUSION,
        Difference2          = pr_sys::PF_Xfer_DIFFERENCE2,
        ColorDodge2          = pr_sys::PF_Xfer_COLOR_DODGE2,
        ColorBurn2           = pr_sys::PF_Xfer_COLOR_BURN2,
        LinearDodge          = pr_sys::PF_Xfer_LINEAR_DODGE,
        LinearBurn           = pr_sys::PF_Xfer_LINEAR_BURN,
        LinearLight          = pr_sys::PF_Xfer_LINEAR_LIGHT,
        VividLight           = pr_sys::PF_Xfer_VIVID_LIGHT,
        PinLight             = pr_sys::PF_Xfer_PIN_LIGHT,
        HardMix              = pr_sys::PF_Xfer_HARD_MIX,
        LighterColor         = pr_sys::PF_Xfer_LIGHTER_COLOR,
        DarkerColor          = pr_sys::PF_Xfer_DARKER_COLOR,
        Subtract             = pr_sys::PF_Xfer_SUBTRACT,
        Divide               = pr_sys::PF_Xfer_DIVIDE,
        Reserved0            = pr_sys::PF_Xfer_RESERVED0,
        Reserved1            = pr_sys::PF_Xfer_RESERVED1,
        NumModes             = pr_sys::PF_Xfer_NUM_MODES,
    }
}
