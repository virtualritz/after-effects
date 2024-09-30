use super::*;

#[derive(Debug)]
pub enum Command {
    /// Display a dialog describing the plug-in. Use `out_data.set_return_msg()` and After Effects will display it in a simple modal dialog.
    ///
    /// Include your plug-in’s version information in the dialog. On macOS, the current resource file will be set to your effects module during this selector.
    About,
    /// Set any required flags and [`OutData`] fields to describe your plug-in’s behavior.
    GlobalSetup,
    /// Free all global data (only required if you allocated some).
    GlobalSetdown,
    /// Describe your parameters and register them using [`params.add()`](Parameters::add).
    ///
    /// Also, register custom user interface elements.
    ParamsSetup,
    /// Allocate and initialize any sequence-specific data. Sent when the effect is first applied. [`InData`] is initialized at this time.
    SequenceSetup,
    /// Re-create (usually unflatten) sequence data. Sent after sequence data is read from disk, during pre-composition, or when the effect is copied;
    ///
    /// After Effects flattens sequence data before duplication. During duplication, [`Command::SequenceResetup`] is sent for both the old and new sequences.
    ///
    /// Don’t expect a [`Command::SequenceFlatten`] between [`Command::SequenceResetup`]s.
    SequenceResetup,
    /// Sent when saving and when duplicating the sequence. Flatten sequence data containing pointers or handles so it can be written to disk.
    ///
    /// This will saved with the project file. Free the unflat data and set the out_data>sequence_data to point to the new flattened data. Flat data must be correctly byte-ordered for file storage.
    ///
    /// As of 6.0, if an effect’s sequence data has recently been flattened, the effect may be deleted without receiving an additional [`Command::SequenceSetdown`].
    ///
    /// In this case, After Effects will dispose of your flat sequence data.
    SequenceFlatten,
    /// Free all sequence data.
    SequenceSetdown,
    /// Display an options dialog. this is sent when the Options button is clicked (or a menu command has been selected).
    ///
    /// This selector will only be sent if the effect has previously indicated that it has a dialog
    ///
    /// (by setting the global [`OutFlags::IDoDialog`] flag in response to [`Command::GlobalSetup`]).
    ///
    /// In version 3.x, the params passed with [`Command::DoDialog`] were invalid.
    ///
    /// This is no longer the case; plug-ins can access non-layer parameters, check out parameters at other times, and perform UI updates during [`Command::DoDialog`].
    ///
    /// They still may not change the parameter’s values.
    DoDialog,
    /// Allocate any frame-specific data. This is sent immediately before each frame is rendered, to allow for frame-specific setup data. If your effect changes the size of its output buffer, specify the new output height, width, and relative origin. All parameters except the input layer are valid.
    ///
    /// If you set width and height to 0, After Effects ignores your response to the following [`Command::Render`].
    ///
    /// NOTE: If [`OutFlags::IExpandBuffer`] is set, you will receive this selector (and [`Command::FrameSetdown`]) twice, once without [`Command::Render`] between them.
    ///
    /// This is so we know whether or not the given layer will be visible.
    ///
    /// Frame data dates from the days when machines might have 8MB of RAM. Given the calling sequence (above), it’s much more efficient to just allocate during [`Command::Render`].
    FrameSetup { in_layer: Layer, out_layer: Layer },
    /// Render the effect into the output, based on the input frame and any parameters.
    ///
    /// This render call can only support 8-bit or 16-bit per channel rendering. 32-bit per channel rendering must be handled in [`Command::SmartRender`].
    ///
    /// All fields in [`InData`] are valid.
    ///
    /// If your response to this selector is interrupted (your calls to PF_ABORT or PF_PROGRESS returns an error code), your results will not be used.
    ///
    /// You cannot delete frame_data during this selector; you must wait until [`Command::FrameSetdown`].
    Render { in_layer: Layer, out_layer: Layer },
    /// Free any frame data allocated during [`Command::FRAME_SETUP.
    FrameSetdown,
    /// The user changed a parameter value. You will receive this command only if you’ve set the [`ParamFlag::SUPERVISE`] flag.
    ///
    /// You modify the parameter to control values, or make one parameter’s value affect others. A parameter can be modified by different actions.
    ///
    /// in_data.current_time is set to the time of the frame that the user is looking at in the UI (internally, the current time of the comp converted into layer time) while they are changing the param that triggered the [`Command::USER_CHANGED_PARAM.
    ///
    /// It’s also the time of a keyframe that is added automatically (if there isn’t one already, and the stopwatch is enabled).
    ///
    /// This is usually the same as the value passed for the [`Command::Render`] that follows immediately after (unless caps lock is down), but not necessarily – there could be other comp windows open that cause a render at a different time in response to the changed param.
    UserChangedParam { param_index: usize },
    /// The effect controls palette (ECP) needs to be updated. This might occur after opening the ECP or moving to a new time within the composition.
    ///
    /// You can modify parameter characteristics (enabling or disabling them, for example) by calling `param.update_param_ui()`.
    ///
    /// Only cosmetic changes may be made in response to this command. Don’t change parameter values while responding to [`Command::UpdateParamsUi`]; do so during [`Command::UserChangedParam`] instead.
    ///
    /// This command will only be sent regularly if [`OutFlags::SendUpdateParamsUi`]` was set in the PiPL, and during [`Command::GlobalSetup`].
    ///
    /// NOTE: Never check out parameters during this selector. Recursive badness is almost guaranteed to result.
    UpdateParamsUi,
    /// This selector makes use of the extra parameter; the type of event to be handled is indicated by the e_type field, a member of the structure pointed to by extra.
    ///
    /// ee Effect UI & Events.
    Event { extra: EventExtra },
    /// Only sent if [`OutFlags::IHaveExternalDependencies`] was set during [`Command::GlobalSetup`].
    ///
    /// Populate a string handle (in the PF_ExtDependenciesExtra pointed to by extra) with a description of your plug-in’s dependencies, making sure to allocate space for the terminating NULL character.
    ///
    /// Return just a NULL pointer for the string handle if there are no dependencies to report.
    ///
    /// If the check type is PF_DepCheckType_ALL_DEPENDENCIES, report everything that might be required for your plug-in to render.
    ///
    /// Report only missing items (or a null string if nothing’s missing) if the check type is PF_DepCheckType_MISSING_DEPENDENCIES.
    GetExternalDependencies { extra: ExternalDependenciesExtra },
    /// Respond to an AEGP. The extra parameter points to whatever parameter the AEGP sent.
    ///
    /// AEGPs can only communicate with effects which respond to this selector.
    CompletelyGeneral,
    /// Sent only to plug-ins which have specified [`OutFlags2::SupportsQueryDynamicFlags`]` in [`OutFlags2`], in their PiPL and during [`Command::GlobalSetup`].
    ///
    /// With all of the dynamic flags, if you will ever change them during this command, you must have set the flag on during [`Command::GlobalSetup`].
    ///
    /// This selector will be sent at arbitrary times.
    ///
    /// In response, the effect should access its (non-layer) parameters using [`params.checkout()`](Parameters::checkout), and decide whether any of the flags that support [`Command::QueryDynamicFlags`] should be set, such as:
    ///
    /// - [`OutFlags::WideTimeInput`]
    /// - [`OutFlags::NonParamVary`]
    /// - [`OutFlags::PixIndependent`]
    /// - [`OutFlags::IUseShutterAngle`]
    /// - [`OutFlags2::IUse3DCamera`]
    /// - [`OutFlags2::IUse3DLights`]
    /// - [`OutFlags2::DoesntNeedEmptyPixels`]
    /// - [`OutFlags2::RevealsZeroAlpha`]
    /// - [`OutFlags2::DependsOnUnreferencedMasks`]
    /// - [`OutFlags2::OutputIsWatermarked`]
    ///
    /// After Effects uses this information for caching and optimization purposes, so try to respond as quickly as possible.
    QueryDynamicFlags,
    /// Populate `out_data.dest_snd` with effect-ed audio. All fields in [`InData`] are valid.
    ///
    /// If your response to this selector is interrupted (your calls to PF_ABORT or PF_PROGRESS returns an error code), your results will not be used.
    AudioRender,
    ///
    /// Sent before every audio render. Request a time span of input audio. Allocate and initialize any sequence-specific data.
    ///
    /// If your effect requires input from a time span other than the output time span, update the startsampL and endsampL field in [`OutData`].
    AudioSetup,
    /// Free memory allocated during [`Command::AudioSetup`].
    AudioSetdown,
    /// Manage your arbitrary data type. You’ll only receive this if you’ve registered a custom data type parameter.
    ///
    /// The extra parameter indicates which handler function is being called.
    ///
    /// Custom data types are discussed further in Implementing Arbitrary Data.
    ArbitraryCallback { extra: ArbParamsExtra },
    /// SmartFX only. Identify the area(s) of input the effect will need to produce its output, based on whatever criteria the effect implements.
    ///
    /// maybe sent up to twice when MediaCore is hosting. The first will come during GetFrameDependencies to collect the inputs.
    ///
    /// The source checkouts can return full frame dimensions here. Once the sources are rendered, if they are different in size than the first call then this selector will be emitted a second time with the actual source sizes in order to get a correct output size.
    ///
    /// Note that MediaCore wants all of the output, so PF_PreRenderOutput::max_result_rect will be used.
    ///
    /// New in 16.0
    ///
    /// Set `RenderOutputFlag::GpuRenderPossible` in PF_PreRenderOutput to render on the GPU.
    ///
    /// If this flag is not set the requested render is not possible with the requested GPU, because of parameters or render settings.
    ///
    /// The host may re-call PreRender with another what_gpu option (or PF_GPU_Framework_None).
    /// ```ignore
    /// struct PF_PreRenderInput{
    ///   output_request: PF_RenderRequest, // what the effect is being asked to render
    ///   bitdepth: u16,                    // bitdepth the effect is being driven in (in bpc)
    ///   gpu_data: *const c_void,          // (new AE 16.0)
    ///   what_gpu: PF_GPU_Framework,       // (new AE 16.0)
    ///   device_index: u32,                // (new AE 16.0) For use in conjunction with PrSDKGPUDeviceSuite
    /// }
    /// ```
    SmartPreRender { extra: PreRenderExtra },
    /// SmartFX only. Perform rendering and provide output for the area(s) the effect was asked to render.
    SmartRender { extra: SmartRenderExtra },
    ///
    GetFlattenedSequenceData,
    ///
    TranslateParamsToPrefs { extra: *mut ae_sys::PF_TranslateParamsToPrefsExtra },
    /// GPU equivalent to the existing [`Command::SmartRender`] selector.
    ///
    /// At render time, either the [`Command::SmartRender`] or the [`Command::SmartRenderGpu`] selector will be called, depending on whether the effect is expected to produce a CPU or GPU frame as output.
    ///
    /// [`Command::SmartRenderGpu`] will only be called when `what_gpu != PF_GPU_Framework_None`, and has effects on any input / output PF_LayerDef’s.
    ///
    /// All frame check-ins and check-outs will operate on GPU frames when this selector is in progress. Note [`Command::SmartRender`] shares the Extra structs.
    /// ```ignore
    /// struct PF_SmartRenderInput {
    ///   output_request: PF_RenderRequest, // what the effect is being asked to render
    ///   bitdepth: u16,                    // bitdepth the effect is being driven in (in bpc)
    ///   pre_render_data: *mut c_void,     // passed back from value placed in extra->output->pre_render_data during [`Command::PreRender`]
    ///   gpu_data *const c_void,           // (new AE 16.0)
    ///   what_gpu: PF_GPU_Framework,       // (new AE 16.0)
    ///   device_index: u32,                // (new AE 16.0)
    /// }
    ///
    /// struct SmartRenderExtra {
    ///   input: *mut PF_SmartRenderInput,
    ///   cb: *mut PF_SmartRenderCallbacks,
    /// }
    /// ```
    /// The what_gpu and device_index fields are in the extra input for GPU-related selectors indicates to the plug-in the GPU framework to be used for rendering.
    ///
    /// Input and output buffers will be prepared on this framework and device.
    ///
    /// The device, context, command queue, and other associated GPU state can be queried with PrSDKGPUDeviceSuite::GetDeviceInfo.
    ///
    /// what_gpu will be the same between [`Command::SmartPreRender`] and [`Command::SmartRenderGpu`] selector calls.
    SmartRenderGpu { extra: SmartRenderExtra },
    /// This selector can be called at any time by the host. It will be called not more than once for each GPU device.
    ///
    /// Multiple GPU devices may be in the setup state at one time.
    ///
    /// It will be called after GlobalSetup and before SequenceSetup.
    ///
    /// The intent is for the effect to do GPU initialization if necessary and to give the effect an opportunity to opt out of a GPU device based solely on the properties of that device, and not any render context (frame size, etc).
    ///
    /// If the effect rejects the GPU device it will get called for CPU render.
    ///
    /// `InData::what_gpu` != PF_GPU_Framework_None is expected.
    ///
    /// Effect is expected to set one or both of the [`OutFlags2::SupportsGpuRenderF32`] flags in [`OutData::set_out_flags2`] if the device and framework in what_gpu is supported.
    ///
    /// Note that only [`OutFlags2::SupportsGpuRenderF32`] will be in AE 16.0.
    ///
    /// Effects that do not set flags here will NOT be considered to support GPU rendering for any of these devices.
    ///
    /// PF_GPUDeviceSetupOutput::gpu_data is a plug-in owned pointer that must be released with a the [`Command::GpuDeviceSetdown`] selector.
    ///
    /// This pointer is also available at render time.
    GpuDeviceSetup { extra: GpuDeviceSetupExtra },
    /// Release any resources associated with gpu_data. In AE this will be called just before GPU device release.
    /// ```ignore
    /// struct PF_GPUDeviceSetdownInput {
    ///   gpu_data: *mut c_void,  // effect must dispose.
    ///   what_gpu: PF_GPU_Framework,
    ///   device_index: u32 // For use in conjunction with PrSDKGPUDeviceSuite
    /// }
    ///
    /// struct GpuDeviceSetdownExtra {
    ///   input: PF_GPUDeviceSetdownInput
    /// }
    /// ```
    GpuDeviceSetdown { extra: GpuDeviceSetdownExtra },
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
                in_layer: unsafe { Layer::from_raw(&mut (*(*params)).u.ld, in_data_ptr, None) },
                out_layer: Layer::from_raw(output, in_data_ptr, None),
            },
            RawCommand::Render => Command::Render {
                in_layer: unsafe { Layer::from_raw(&mut (*(*params)).u.ld, in_data_ptr, None) },
                out_layer: Layer::from_raw(output, in_data_ptr, None),
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
                extra: ExternalDependenciesExtra::from_raw(extra as *mut ae_sys::PF_ExtDependenciesExtra),
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
