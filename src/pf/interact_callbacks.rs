use crate::*;

pub struct InteractCallbacks(InData);

impl InteractCallbacks {
    pub fn new(in_data: InData) -> Self {
        Self(in_data)
    }

    pub fn register_ui(&self, custom_ui_info: CustomUIInfo) -> Result<(), Error> {
        match unsafe {
            (*self.0.as_ptr()).inter.register_ui.unwrap()(
                (*self.0.as_ptr()).effect_ref,
                custom_ui_info.as_ptr() as _,
            )
        } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    // fn checkout_param(effect_ref: PF_ProgPtr, index: PF_ParamIndex, what_time: A_long, time_step: A_long, time_scale: A_u_long, param: *mut PF_ParamDef) -> PF_Err,
    // fn checkin_param(effect_ref: PF_ProgPtr, param: *mut PF_ParamDef) -> PF_Err,
    // fn add_param(effect_ref: PF_ProgPtr, index: PF_ParamIndex, def: PF_ParamDefPtr,) -> PF_Err,
    // fn abort(effect_ref: PF_ProgPtr) -> PF_Err>,
    // fn progress(effect_ref: PF_ProgPtr, current: A_long, total: A_long) -> PF_Err,
    // fn register_ui(effect_ref: PF_ProgPtr, cust_info: *mut PF_CustomUIInfo) -> PF_Err,
    // fn checkout_layer_audio(effect_ref: PF_ProgPtr, index: PF_ParamIndex, start_time: A_long, duration: A_long, time_scale: A_u_long, rate: PF_UFixed, bytes_per_sample: A_long, num_channels: A_long, fmt_signed: A_long, audio: *mut PF_LayerAudio) -> PF_Err,
    // fn checkin_layer_audio(effect_ref: PF_ProgPtr, audio: PF_LayerAudio) -> PF_Err,
    // fn get_audio_data(effect_ref: PF_ProgPtr, audio: PF_LayerAudio, data0: *mut PF_SndSamplePtr, num_samples0: *mut A_long, rate0: *mut PF_UFixed, bytes_per_sample0: *mut A_long, num_channels0: *mut A_long, fmt_signed0: *mut A_long) -> PF_Err,
}
