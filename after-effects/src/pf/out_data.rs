use super::*;
use std::any::Any;

// struct PF_OutData {
//     pub start_sampL: A_long,             // Used only for Audio commands
//     pub dur_sampL: A_long,               // --^
//     pub dest_snd: PF_SoundWorld,         // --^
//     ...
// }

#[derive(Clone, Copy, Debug)]
pub struct OutData {
    pub(crate) ptr: *mut ae_sys::PF_OutData,
}

impl AsRef<ae_sys::PF_OutData> for OutData {
    fn as_ref(&self) -> &ae_sys::PF_OutData {
        unsafe { &*self.ptr }
    }
}
impl AsMut<ae_sys::PF_OutData> for OutData {
    fn as_mut(&mut self) -> &mut ae_sys::PF_OutData {
        unsafe { &mut *self.ptr }
    }
}

impl OutData {
    pub fn from_raw(ptr: *mut ae_sys::PF_OutData) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_OutData {
        self.ptr
    }

    pub fn width(&self) -> u32 {
        self.as_ref().width as u32
    }

    /// Set during [`Command::FrameSetup`] if the output image size differs from the input. width and height are the size of the output buffer, and origin is the point the input should map to in the output.
    /// To create a 5-pixel drop shadow up and left, set origin to (5, 5).
    pub fn set_width(&mut self, width: u32) {
        self.as_mut().width = width as ae_sys::A_long;
    }

    pub fn height(&self) -> u32 {
        self.as_ref().height as u32
    }

    /// Set during [`Command::FrameSetup`] if the output image size differs from the input. width and height are the size of the output buffer, and origin is the point the input should map to in the output.
    /// To create a 5-pixel drop shadow up and left, set origin to (5, 5).
    pub fn set_height(&mut self, height: u32) {
        self.as_mut().height = height as ae_sys::A_long;
    }

    pub fn origin(&self) -> Point {
        self.as_ref().origin.into()
    }

    /// Set during [`Command::FrameSetup`] if the output image size differs from the input. width and height are the size of the output buffer, and origin is the point the input should map to in the output.
    /// To create a 5-pixel drop shadow up and left, set origin to (5, 5).
    pub fn set_origin(&mut self, origin: Point) {
        self.as_mut().origin = origin.into();
    }

    /// After Effects displays any string you put here (checked and cleared after every command selector).
    pub fn set_return_msg(&mut self, msg: &str) {
        let msg = msg.as_bytes();
        assert!(msg.len() < 256);
        let return_msg = &mut self.as_mut().return_msg;
        return_msg.fill(0);
        return_msg[..msg.len()].copy_from_slice(unsafe { std::mem::transmute(msg) });
    }

    /// Whether the plug-in already supplied a message for the current command.
    pub fn has_return_msg(&self) -> bool {
        self.as_ref().return_msg[0] != 0
    }

    /// After Effects displays any string you put here as an error (checked and cleared after every command selector).
    pub fn set_error_msg(&mut self, msg: &str) {
        self.set_return_msg(msg);
        self.set_out_flag(OutFlags::DisplayErrorMessage, true);
    }

    /// Sets an error message only if the plug-in has not already supplied one.
    pub fn set_error_msg_if_empty(&mut self, msg: &str) {
        if self.has_return_msg() {
            self.set_out_flag(OutFlags::DisplayErrorMessage, true);
        } else {
            self.set_error_msg(msg);
        }
    }

    /// Set this flag to the version of your plug-in code. After Effects uses this data to decide which of duplicate effects to load.
    pub fn set_version(&mut self, v: u32) {
        self.as_mut().my_version = v as ae_sys::A_u_long;
    }

    /// Send messages to After Effects. OR together multiple values.
    pub fn set_out_flags(&mut self, v: OutFlags) {
        self.as_mut().out_flags = v.into();
    }

    /// Send messages to After Effects. OR together multiple values.
    pub fn set_out_flags2(&mut self, v: OutFlags2) {
        self.as_mut().out_flags2 = v.into();
    }

    /// Send messages to After Effects. OR together multiple values.
    pub fn set_out_flag(&mut self, flag: OutFlags, enabled: bool) {
        if enabled {
            self.as_mut().out_flags |= Into::<ae_sys::PF_OutFlags>::into(flag);
        } else {
            self.as_mut().out_flags &= !(Into::<ae_sys::PF_OutFlags>::into(flag));
        }
    }

    /// Send messages to After Effects. OR together multiple values.
    pub fn set_out_flag2(&mut self, flag: OutFlags2, enabled: bool) {
        if enabled {
            self.as_mut().out_flags2 |= Into::<ae_sys::PF_OutFlags2>::into(flag);
        } else {
            self.as_mut().out_flags2 &= !(Into::<ae_sys::PF_OutFlags2>::into(flag));
        }
    }

    /// Set the [`OutFlags::ForceRerender`] flag
    pub fn set_force_rerender(&mut self) {
        self.set_out_flag(OutFlags::ForceRerender, true);
    }

    /// Data you (might have) allocated during [`Command::FrameSetup`].
    /// This is never written to disk; it was used to pass information from your [`Command::FrameSetup`] response to your [`Command::Render`] or [`Command::FrameSetdown`]
    /// (which you must do if you resize the output buffer). Otherwise, this memory is rarely used.
    pub fn set_frame_data<T: Any>(&mut self, val: T) {
        let boxed: Box<Box<dyn Any>> = Box::new(Box::new(val));
        self.as_mut().frame_data = Box::<Box<dyn Any>>::into_raw(boxed) as *mut _;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn return_msg(raw: &ae_sys::PF_OutData) -> &str {
        unsafe { std::ffi::CStr::from_ptr(raw.return_msg.as_ptr()) }
            .to_str()
            .unwrap()
    }

    #[test]
    fn return_message_is_nul_terminated_and_detectable() {
        let mut raw: ae_sys::PF_OutData = unsafe { std::mem::zeroed() };
        raw.return_msg.fill(b'x' as ae_sys::A_char);
        let mut out = OutData::from_raw(&mut raw);

        out.set_return_msg("custom error");

        assert!(out.has_return_msg());
        assert_eq!(raw.return_msg[12], 0);
    }

    #[test]
    fn empty_return_message_is_not_detected() {
        let mut raw: ae_sys::PF_OutData = unsafe { std::mem::zeroed() };
        let out = OutData::from_raw(&mut raw);

        assert!(!out.has_return_msg());
    }

    #[test]
    fn error_fallback_preserves_existing_message_and_fills_empty_buffer() {
        let display_error: ae_sys::PF_OutFlags = OutFlags::DisplayErrorMessage.into();

        let mut existing: ae_sys::PF_OutData = unsafe { std::mem::zeroed() };
        let mut out = OutData::from_raw(&mut existing);
        out.set_return_msg("custom error");
        out.set_error_msg_if_empty("fallback");

        assert_eq!(return_msg(&existing), "custom error");
        assert_ne!(existing.out_flags & display_error, 0);

        let mut empty: ae_sys::PF_OutData = unsafe { std::mem::zeroed() };
        let mut out = OutData::from_raw(&mut empty);
        out.set_error_msg_if_empty("fallback");

        assert_eq!(return_msg(&empty), "fallback");
        assert_ne!(empty.out_flags & display_error, 0);
    }
}

define_enum! {
    ae_sys::PF_OutFlags,
    OutFlags {
        None                         = ae_sys::PF_OutFlag_NONE,
        KeepResourceOpen             = ae_sys::PF_OutFlag_KEEP_RESOURCE_OPEN,
        WideTimeInput                = ae_sys::PF_OutFlag_WIDE_TIME_INPUT,
        NonParamVary                 = ae_sys::PF_OutFlag_NON_PARAM_VARY,
        SequenceDataNeedsFlattening  = ae_sys::PF_OutFlag_SEQUENCE_DATA_NEEDS_FLATTENING,
        IDoDialog                    = ae_sys::PF_OutFlag_I_DO_DIALOG,
        UseOutputExtent              = ae_sys::PF_OutFlag_USE_OUTPUT_EXTENT,
        SendDoDialog                 = ae_sys::PF_OutFlag_SEND_DO_DIALOG,
        DisplayErrorMessage          = ae_sys::PF_OutFlag_DISPLAY_ERROR_MESSAGE,
        IExpandBuffer                = ae_sys::PF_OutFlag_I_EXPAND_BUFFER,
        PixIndependent               = ae_sys::PF_OutFlag_PIX_INDEPENDENT,
        IWriteInputBuffer            = ae_sys::PF_OutFlag_I_WRITE_INPUT_BUFFER,
        IShrinkBuffer                = ae_sys::PF_OutFlag_I_SHRINK_BUFFER,
        WorksInPlace                 = ae_sys::PF_OutFlag_WORKS_IN_PLACE,
        CustomUi                     = ae_sys::PF_OutFlag_CUSTOM_UI,
        RefreshUi                    = ae_sys::PF_OutFlag_REFRESH_UI,
        NopRender                    = ae_sys::PF_OutFlag_NOP_RENDER,
        IUseShutterAngle             = ae_sys::PF_OutFlag_I_USE_SHUTTER_ANGLE,
        IUseAudio                    = ae_sys::PF_OutFlag_I_USE_AUDIO,
        IAmObsolete                  = ae_sys::PF_OutFlag_I_AM_OBSOLETE,
        ForceRerender                = ae_sys::PF_OutFlag_FORCE_RERENDER,
        PiplOverridesOutdataOutflags = ae_sys::PF_OutFlag_PiPL_OVERRIDES_OUTDATA_OUTFLAGS,
        IHaveExternalDependencies    = ae_sys::PF_OutFlag_I_HAVE_EXTERNAL_DEPENDENCIES,
        DeepColorAware               = ae_sys::PF_OutFlag_DEEP_COLOR_AWARE,
        SendUpdateParamsUi           = ae_sys::PF_OutFlag_SEND_UPDATE_PARAMS_UI,
        AudioFloatOnly               = ae_sys::PF_OutFlag_AUDIO_FLOAT_ONLY,
        AudioIir                     = ae_sys::PF_OutFlag_AUDIO_IIR,
        ISynthesizeAudio             = ae_sys::PF_OutFlag_I_SYNTHESIZE_AUDIO,
        AudioEffectToo               = ae_sys::PF_OutFlag_AUDIO_EFFECT_TOO,
        AudioEffectOnly              = ae_sys::PF_OutFlag_AUDIO_EFFECT_ONLY,
    }
}


define_enum! {
    ae_sys::PF_OutFlags2,
    OutFlags2 {
        None                                = ae_sys::PF_OutFlag2_NONE,
        SupportsQueryDynamicFlags           = ae_sys::PF_OutFlag2_SUPPORTS_QUERY_DYNAMIC_FLAGS,
        IUse3DCamera                        = ae_sys::PF_OutFlag2_I_USE_3D_CAMERA,
        IUse3DLights                        = ae_sys::PF_OutFlag2_I_USE_3D_LIGHTS,
        ParamGroupStartCollapsedFlag        = ae_sys::PF_OutFlag2_PARAM_GROUP_START_COLLAPSED_FLAG,
        IAmThreadsafe                       = ae_sys::PF_OutFlag2_I_AM_THREADSAFE,
        CanCombineWithDestination           = ae_sys::PF_OutFlag2_CAN_COMBINE_WITH_DESTINATION,
        DoesntNeedEmptyPixels               = ae_sys::PF_OutFlag2_DOESNT_NEED_EMPTY_PIXELS,
        RevealsZeroAlpha                    = ae_sys::PF_OutFlag2_REVEALS_ZERO_ALPHA,
        PreservesFullyOpaquePixels          = ae_sys::PF_OutFlag2_PRESERVES_FULLY_OPAQUE_PIXELS,
        SupportsSmartRender                 = ae_sys::PF_OutFlag2_SUPPORTS_SMART_RENDER,
        FloatColorAware                     = ae_sys::PF_OutFlag2_FLOAT_COLOR_AWARE,
        IUseColorspaceEnumeration           = ae_sys::PF_OutFlag2_I_USE_COLORSPACE_ENUMERATION,
        IAmDeprecated                       = ae_sys::PF_OutFlag2_I_AM_DEPRECATED,
        PproDoNotCloneSequenceDataForRender = ae_sys::PF_OutFlag2_PPRO_DO_NOT_CLONE_SEQUENCE_DATA_FOR_RENDER,
        AutomaticWideTimeInput              = ae_sys::PF_OutFlag2_AUTOMATIC_WIDE_TIME_INPUT,
        IUseTimecode                        = ae_sys::PF_OutFlag2_I_USE_TIMECODE,
        DependsOnUnreferencedMasks          = ae_sys::PF_OutFlag2_DEPENDS_ON_UNREFERENCED_MASKS,
        OutputIsWatermarked                 = ae_sys::PF_OutFlag2_OUTPUT_IS_WATERMARKED,
        IMixGuidDependencies                = ae_sys::PF_OutFlag2_I_MIX_GUID_DEPENDENCIES,
        Ae135Threadsafe                     = ae_sys::PF_OutFlag2_AE13_5_THREADSAFE,
        SupportsGetFlattenedSequenceData    = ae_sys::PF_OutFlag2_SUPPORTS_GET_FLATTENED_SEQUENCE_DATA,
        CustomUiAsyncManager                = ae_sys::PF_OutFlag2_CUSTOM_UI_ASYNC_MANAGER,
        SupportsGpuRenderF32                = ae_sys::PF_OutFlag2_SUPPORTS_GPU_RENDER_F32,
        SupportsThreadedRendering           = ae_sys::PF_OutFlag2_SUPPORTS_THREADED_RENDERING,
        MutableRenderSequenceDataSlower     = ae_sys::PF_OutFlag2_MUTABLE_RENDER_SEQUENCE_DATA_SLOWER,
        SupportsDirectXRendering            = ae_sys::PF_OutFlag2_SUPPORTS_DIRECTX_RENDERING,
    }
}
