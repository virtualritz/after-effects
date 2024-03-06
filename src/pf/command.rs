use super::*;

#[rustfmt::skip]
#[derive(Debug)]
pub enum Command {
    About,
    GlobalSetup,
    GlobalSetdown,
    ParamsSetup,
    SequenceSetup,
    SequenceResetup,
    SequenceFlatten,
    SequenceSetdown,
    DoDialog,
    FrameSetup               { in_layer: Layer, out_layer: Layer },
    Render                   { in_layer: Layer, out_layer: Layer },
    FrameSetdown,
    UserChangedParam         { param_index: usize },
    UpdateParamsUi,
    Event                    { extra: EventExtra },
    GetExternalDependencies  { extra: *mut ae_sys::PF_ExtDependenciesExtra },
    CompletelyGeneral,
    QueryDynamicFlags,
    AudioRender,
    AudioSetup,
    AudioSetdown,
    ArbitraryCallback        { extra: ArbParamsExtra },
    SmartPreRender           { extra: PreRenderExtra },
    SmartRender              { extra: SmartRenderExtra },
    GetFlattenedSequenceData,
    TranslateParamsToPrefs   { extra: *mut ae_sys::PF_TranslateParamsToPrefsExtra },
    SmartRenderGpu           { extra: SmartRenderExtra },
    GpuDeviceSetup           { extra: GpuDeviceSetupExtra },
    GpuDeviceSetdown         { extra: GpuDeviceSetdownExtra },
}

impl Command {
    pub fn from_entry_point(
        cmd: impl Into<RawCommand>,
        in_data_ptr: *const ae_sys::PF_InData,
        params: *mut *mut ae_sys::PF_ParamDef,
        output: *mut ae_sys::PF_LayerDef,
        extra: *mut std::ffi::c_void,
    ) -> Self {
        match cmd.into() {
            RawCommand::About => Command::About,
            RawCommand::GlobalSetup => Command::GlobalSetup,
            RawCommand::GlobalSetdown => Command::GlobalSetdown,
            RawCommand::ParamsSetup => Command::ParamsSetup,
            RawCommand::SequenceSetup => Command::SequenceSetup,
            RawCommand::SequenceResetup => Command::SequenceResetup,
            RawCommand::SequenceFlatten => Command::SequenceFlatten,
            RawCommand::SequenceSetdown => Command::SequenceSetdown,
            RawCommand::DoDialog => Command::DoDialog,
            RawCommand::FrameSetup => Command::FrameSetup {
                in_layer: unsafe { Layer::from_raw(&mut (*(*params)).u.ld, InData::from_raw(in_data_ptr), None) },
                out_layer: Layer::from_raw(output, InData::from_raw(in_data_ptr), None),
            },
            RawCommand::Render => Command::Render {
                in_layer: unsafe { Layer::from_raw(&mut (*(*params)).u.ld, InData::from_raw(in_data_ptr), None) },
                out_layer: Layer::from_raw(output, InData::from_raw(in_data_ptr), None),
            },
            RawCommand::FrameSetdown => Command::FrameSetdown,
            RawCommand::UserChangedParam => Command::UserChangedParam {
                param_index: unsafe {
                    (*(extra as *mut ae_sys::PF_UserChangedParamExtra)).param_index as usize
                },
            },
            RawCommand::UpdateParamsUi => Command::UpdateParamsUi,
            RawCommand::Event => Command::Event {
                extra: EventExtra::from_raw(extra as *mut ae_sys::PF_EventExtra),
            },
            RawCommand::GetExternalDependencies => Command::GetExternalDependencies {
                extra: extra as *mut ae_sys::PF_ExtDependenciesExtra,
            },
            RawCommand::CompletelyGeneral => Command::CompletelyGeneral,
            RawCommand::QueryDynamicFlags => Command::QueryDynamicFlags,
            RawCommand::AudioRender => Command::AudioRender,
            RawCommand::AudioSetup => Command::AudioSetup,
            RawCommand::AudioSetdown => Command::AudioSetdown,
            RawCommand::ArbitraryCallback => Command::ArbitraryCallback {
                extra: ArbParamsExtra::from_raw(extra as *mut ae_sys::PF_ArbParamsExtra),
            },
            RawCommand::SmartPreRender => Command::SmartPreRender {
                extra: PreRenderExtra::from_raw(in_data_ptr, extra as *mut _),
            },
            RawCommand::SmartRender => Command::SmartRender {
                extra: SmartRenderExtra::from_raw(in_data_ptr, extra as *mut _),
            },
            RawCommand::GetFlattenedSequenceData => Command::GetFlattenedSequenceData,
            RawCommand::TranslateParamsToPrefs => Command::TranslateParamsToPrefs {
                extra: extra as *mut ae_sys::PF_TranslateParamsToPrefsExtra,
            },
            RawCommand::SmartRenderGpu => Command::SmartRenderGpu {
                extra: SmartRenderExtra::from_raw(in_data_ptr, extra as *mut _),
            },
            RawCommand::GpuDeviceSetup => Command::GpuDeviceSetup {
                extra: GpuDeviceSetupExtra::from_raw(extra as *mut _),
            },
            RawCommand::GpuDeviceSetdown => Command::GpuDeviceSetdown {
                extra: GpuDeviceSetdownExtra::from_raw(extra as *mut _),
            }
        }
    }

    #[rustfmt::skip]
    pub fn as_raw(&self) -> ae_sys::PF_Cmd {
        (match self {
            Command::About                    { .. } => ae_sys::PF_Cmd_ABOUT,
            Command::GlobalSetup              { .. } => ae_sys::PF_Cmd_GLOBAL_SETUP,
            Command::GlobalSetdown            { .. } => ae_sys::PF_Cmd_GLOBAL_SETDOWN,
            Command::ParamsSetup              { .. } => ae_sys::PF_Cmd_PARAMS_SETUP,
            Command::SequenceSetup            { .. } => ae_sys::PF_Cmd_SEQUENCE_SETUP,
            Command::SequenceResetup          { .. } => ae_sys::PF_Cmd_SEQUENCE_RESETUP,
            Command::SequenceFlatten          { .. } => ae_sys::PF_Cmd_SEQUENCE_FLATTEN,
            Command::SequenceSetdown          { .. } => ae_sys::PF_Cmd_SEQUENCE_SETDOWN,
            Command::DoDialog                 { .. } => ae_sys::PF_Cmd_DO_DIALOG,
            Command::FrameSetup               { .. } => ae_sys::PF_Cmd_FRAME_SETUP,
            Command::Render                   { .. } => ae_sys::PF_Cmd_RENDER,
            Command::FrameSetdown             { .. } => ae_sys::PF_Cmd_FRAME_SETDOWN,
            Command::UserChangedParam         { .. } => ae_sys::PF_Cmd_USER_CHANGED_PARAM,
            Command::UpdateParamsUi           { .. } => ae_sys::PF_Cmd_UPDATE_PARAMS_UI,
            Command::Event                    { .. } => ae_sys::PF_Cmd_EVENT,
            Command::GetExternalDependencies  { .. } => ae_sys::PF_Cmd_GET_EXTERNAL_DEPENDENCIES,
            Command::CompletelyGeneral        { .. } => ae_sys::PF_Cmd_COMPLETELY_GENERAL,
            Command::QueryDynamicFlags        { .. } => ae_sys::PF_Cmd_QUERY_DYNAMIC_FLAGS,
            Command::AudioRender              { .. } => ae_sys::PF_Cmd_AUDIO_RENDER,
            Command::AudioSetup               { .. } => ae_sys::PF_Cmd_AUDIO_SETUP,
            Command::AudioSetdown             { .. } => ae_sys::PF_Cmd_AUDIO_SETDOWN,
            Command::ArbitraryCallback        { .. } => ae_sys::PF_Cmd_ARBITRARY_CALLBACK,
            Command::SmartPreRender           { .. } => ae_sys::PF_Cmd_SMART_PRE_RENDER,
            Command::SmartRender              { .. } => ae_sys::PF_Cmd_SMART_RENDER,
            Command::GetFlattenedSequenceData { .. } => ae_sys::PF_Cmd_GET_FLATTENED_SEQUENCE_DATA,
            Command::TranslateParamsToPrefs   { .. } => ae_sys::PF_Cmd_TRANSLATE_PARAMS_TO_PREFS,
            Command::SmartRenderGpu           { .. } => ae_sys::PF_Cmd_SMART_RENDER_GPU,
            Command::GpuDeviceSetup           { .. } => ae_sys::PF_Cmd_GPU_DEVICE_SETUP,
            Command::GpuDeviceSetdown         { .. } => ae_sys::PF_Cmd_GPU_DEVICE_SETDOWN,
        }) as ae_sys::PF_Cmd
    }
}

define_enum! {
    ae_sys::PF_Cmd,
    RawCommand {
        About                    = ae_sys::PF_Cmd_ABOUT,
        GlobalSetup              = ae_sys::PF_Cmd_GLOBAL_SETUP,
        GlobalSetdown            = ae_sys::PF_Cmd_GLOBAL_SETDOWN,
        ParamsSetup              = ae_sys::PF_Cmd_PARAMS_SETUP,
        SequenceSetup            = ae_sys::PF_Cmd_SEQUENCE_SETUP,
        SequenceResetup          = ae_sys::PF_Cmd_SEQUENCE_RESETUP,
        SequenceFlatten          = ae_sys::PF_Cmd_SEQUENCE_FLATTEN,
        SequenceSetdown          = ae_sys::PF_Cmd_SEQUENCE_SETDOWN,
        DoDialog                 = ae_sys::PF_Cmd_DO_DIALOG,
        FrameSetup               = ae_sys::PF_Cmd_FRAME_SETUP,
        Render                   = ae_sys::PF_Cmd_RENDER,
        FrameSetdown             = ae_sys::PF_Cmd_FRAME_SETDOWN,
        UserChangedParam         = ae_sys::PF_Cmd_USER_CHANGED_PARAM,
        UpdateParamsUi           = ae_sys::PF_Cmd_UPDATE_PARAMS_UI,
        Event                    = ae_sys::PF_Cmd_EVENT,
        GetExternalDependencies  = ae_sys::PF_Cmd_GET_EXTERNAL_DEPENDENCIES,
        CompletelyGeneral        = ae_sys::PF_Cmd_COMPLETELY_GENERAL,
        QueryDynamicFlags        = ae_sys::PF_Cmd_QUERY_DYNAMIC_FLAGS,
        AudioRender              = ae_sys::PF_Cmd_AUDIO_RENDER,
        AudioSetup               = ae_sys::PF_Cmd_AUDIO_SETUP,
        AudioSetdown             = ae_sys::PF_Cmd_AUDIO_SETDOWN,
        ArbitraryCallback        = ae_sys::PF_Cmd_ARBITRARY_CALLBACK,
        SmartPreRender           = ae_sys::PF_Cmd_SMART_PRE_RENDER,
        SmartRender              = ae_sys::PF_Cmd_SMART_RENDER,
        GetFlattenedSequenceData = ae_sys::PF_Cmd_GET_FLATTENED_SEQUENCE_DATA,
        TranslateParamsToPrefs   = ae_sys::PF_Cmd_TRANSLATE_PARAMS_TO_PREFS,
        SmartRenderGpu           = ae_sys::PF_Cmd_SMART_RENDER_GPU,
        GpuDeviceSetup           = ae_sys::PF_Cmd_GPU_DEVICE_SETUP,
        GpuDeviceSetdown         = ae_sys::PF_Cmd_GPU_DEVICE_SETDOWN,
    }
}
