use crate::*;
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
    FrameSetup,
    Render,
    FrameSetdown,
    UserChangedParam         { extra: *mut ae_sys::PF_UserChangedParamExtra },
    UpdateParamsUi,
    Event,
    GetExternalDependencies  { extra: *mut ae_sys::PF_ExtDependenciesExtra },
    CompletelyGeneral,
    QueryDynamicFlags,
    AudioRender,
    AudioSetup,
    AudioSetdown,
    ArbitraryCallback        { extra: *mut ae_sys::PF_ArbParamsExtra },
    SmartPreRender           { extra: *mut ae_sys::PF_PreRenderExtra },
    SmartRender              { extra: *mut ae_sys::PF_SmartRenderExtra },
    GetFlattenedSequenceData,
    TranslateParamsToPrefs   { extra: *mut ae_sys::PF_TranslateParamsToPrefsExtra },
    SmartRenderGpu,
    GpuDeviceSetup           { extra: *mut ae_sys::PF_GPUDeviceSetupExtra },
    GpuDeviceSetdown         { extra: *mut ae_sys::PF_GPUDeviceSetdownExtra },
}

impl Command {
    pub fn from_entry_point(
        cmd: ae_sys::PF_Cmd,
        in_data_ptr: *const ae_sys::PF_InData,
        params: *mut *mut ae_sys::PF_ParamDef,
        output: *mut ae_sys::PF_LayerDef,
        extra: *mut std::ffi::c_void
    ) -> Self {
        match cmd {
            ae_sys::PF_Cmd_ABOUT                       => Command::About,
            ae_sys::PF_Cmd_GLOBAL_SETUP                => Command::GlobalSetup,
            ae_sys::PF_Cmd_GLOBAL_SETDOWN              => Command::GlobalSetdown,
            ae_sys::PF_Cmd_PARAMS_SETUP                => Command::ParamsSetup,
            ae_sys::PF_Cmd_SEQUENCE_SETUP              => Command::SequenceSetup,
            ae_sys::PF_Cmd_SEQUENCE_RESETUP            => Command::SequenceResetup,
            ae_sys::PF_Cmd_SEQUENCE_FLATTEN            => Command::SequenceFlatten,
            ae_sys::PF_Cmd_SEQUENCE_SETDOWN            => Command::SequenceSetdown,
            ae_sys::PF_Cmd_DO_DIALOG                   => Command::DoDialog,
            ae_sys::PF_Cmd_FRAME_SETUP                 => Command::FrameSetup,
            ae_sys::PF_Cmd_RENDER                      => Command::Render,
            ae_sys::PF_Cmd_FRAME_SETDOWN               => Command::FrameSetdown,
            ae_sys::PF_Cmd_USER_CHANGED_PARAM          => Command::UserChangedParam         { extra: extra as *mut ae_sys::PF_UserChangedParamExtra },
            ae_sys::PF_Cmd_UPDATE_PARAMS_UI            => Command::UpdateParamsUi,
            ae_sys::PF_Cmd_EVENT                       => Command::Event,
            ae_sys::PF_Cmd_GET_EXTERNAL_DEPENDENCIES   => Command::GetExternalDependencies  { extra: extra as *mut ae_sys::PF_ExtDependenciesExtra },
            ae_sys::PF_Cmd_COMPLETELY_GENERAL          => Command::CompletelyGeneral,
            ae_sys::PF_Cmd_QUERY_DYNAMIC_FLAGS         => Command::QueryDynamicFlags,
            ae_sys::PF_Cmd_AUDIO_RENDER                => Command::AudioRender,
            ae_sys::PF_Cmd_AUDIO_SETUP                 => Command::AudioSetup,
            ae_sys::PF_Cmd_AUDIO_SETDOWN               => Command::AudioSetdown,
            ae_sys::PF_Cmd_ARBITRARY_CALLBACK          => Command::ArbitraryCallback        { extra: extra as *mut ae_sys::PF_ArbParamsExtra },
            ae_sys::PF_Cmd_SMART_PRE_RENDER            => Command::SmartPreRender           { extra: extra as *mut ae_sys::PF_PreRenderExtra },
            ae_sys::PF_Cmd_SMART_RENDER                => Command::SmartRender              { extra: extra as *mut ae_sys::PF_SmartRenderExtra },
            ae_sys::PF_Cmd_GET_FLATTENED_SEQUENCE_DATA => Command::GetFlattenedSequenceData,
            ae_sys::PF_Cmd_TRANSLATE_PARAMS_TO_PREFS   => Command::TranslateParamsToPrefs   { extra: extra as *mut ae_sys::PF_TranslateParamsToPrefsExtra },
            ae_sys::PF_Cmd_SMART_RENDER_GPU            => Command::SmartRenderGpu,
            ae_sys::PF_Cmd_GPU_DEVICE_SETUP            => Command::GpuDeviceSetup           { extra: extra as *mut ae_sys::PF_GPUDeviceSetupExtra },
            ae_sys::PF_Cmd_GPU_DEVICE_SETDOWN          => Command::GpuDeviceSetdown         { extra: extra as *mut ae_sys::PF_GPUDeviceSetdownExtra },
            _ => panic!("Unknown command: {}", cmd), // TODO: make this an error
        }
    }

    pub fn as_raw(&self) -> ae_sys::PF_Cmd {
        match self {
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
        }
    }
}

// For debugging purposes
pub struct PfCmd(pub ae_sys::PF_Cmd);
impl Debug for PfCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ae_sys::PF_Cmd_ABOUT                       => write!(f, "PF_Cmd_ABOUT"),
            ae_sys::PF_Cmd_GLOBAL_SETUP                => write!(f, "PF_Cmd_GLOBAL_SETUP"),
            ae_sys::PF_Cmd_GLOBAL_SETDOWN              => write!(f, "PF_Cmd_GLOBAL_SETDOWN"),
            ae_sys::PF_Cmd_PARAMS_SETUP                => write!(f, "PF_Cmd_PARAMS_SETUP"),
            ae_sys::PF_Cmd_SEQUENCE_SETUP              => write!(f, "PF_Cmd_SEQUENCE_SETUP"),
            ae_sys::PF_Cmd_SEQUENCE_RESETUP            => write!(f, "PF_Cmd_SEQUENCE_RESETUP"),
            ae_sys::PF_Cmd_SEQUENCE_FLATTEN            => write!(f, "PF_Cmd_SEQUENCE_FLATTEN"),
            ae_sys::PF_Cmd_SEQUENCE_SETDOWN            => write!(f, "PF_Cmd_SEQUENCE_SETDOWN"),
            ae_sys::PF_Cmd_DO_DIALOG                   => write!(f, "PF_Cmd_DO_DIALOG"),
            ae_sys::PF_Cmd_FRAME_SETUP                 => write!(f, "PF_Cmd_FRAME_SETUP"),
            ae_sys::PF_Cmd_RENDER                      => write!(f, "PF_Cmd_RENDER"),
            ae_sys::PF_Cmd_FRAME_SETDOWN               => write!(f, "PF_Cmd_FRAME_SETDOWN"),
            ae_sys::PF_Cmd_USER_CHANGED_PARAM          => write!(f, "PF_Cmd_USER_CHANGED_PARAM"),
            ae_sys::PF_Cmd_UPDATE_PARAMS_UI            => write!(f, "PF_Cmd_UPDATE_PARAMS_UI"),
            ae_sys::PF_Cmd_EVENT                       => write!(f, "PF_Cmd_EVENT"),
            ae_sys::PF_Cmd_GET_EXTERNAL_DEPENDENCIES   => write!(f, "PF_Cmd_GET_EXTERNAL_DEPENDENCIES"),
            ae_sys::PF_Cmd_COMPLETELY_GENERAL          => write!(f, "PF_Cmd_COMPLETELY_GENERAL"),
            ae_sys::PF_Cmd_QUERY_DYNAMIC_FLAGS         => write!(f, "PF_Cmd_QUERY_DYNAMIC_FLAGS"),
            ae_sys::PF_Cmd_AUDIO_RENDER                => write!(f, "PF_Cmd_AUDIO_RENDER"),
            ae_sys::PF_Cmd_AUDIO_SETUP                 => write!(f, "PF_Cmd_AUDIO_SETUP"),
            ae_sys::PF_Cmd_AUDIO_SETDOWN               => write!(f, "PF_Cmd_AUDIO_SETDOWN"),
            ae_sys::PF_Cmd_ARBITRARY_CALLBACK          => write!(f, "PF_Cmd_ARBITRARY_CALLBACK"),
            ae_sys::PF_Cmd_SMART_PRE_RENDER            => write!(f, "PF_Cmd_SMART_PRE_RENDER"),
            ae_sys::PF_Cmd_SMART_RENDER                => write!(f, "PF_Cmd_SMART_RENDER"),
            ae_sys::PF_Cmd_GET_FLATTENED_SEQUENCE_DATA => write!(f, "PF_Cmd_GET_FLATTENED_SEQUENCE_DATA"),
            ae_sys::PF_Cmd_TRANSLATE_PARAMS_TO_PREFS   => write!(f, "PF_Cmd_TRANSLATE_PARAMS_TO_PREFS"),
            ae_sys::PF_Cmd_SMART_RENDER_GPU            => write!(f, "PF_Cmd_SMART_RENDER_GPU"),
            ae_sys::PF_Cmd_GPU_DEVICE_SETUP            => write!(f, "PF_Cmd_GPU_DEVICE_SETUP"),
            ae_sys::PF_Cmd_GPU_DEVICE_SETDOWN          => write!(f, "PF_Cmd_GPU_DEVICE_SETDOWN"),
            _ => write!(f, "Unknown command: {}", self.0),
        }
    }
}
