use crate::*;
use crate::aegp::*;

define_suite!(
    /// Output Module Suite provides information about and control over output modules
    /// attached to render queue items.
    ///
    /// NOTE: All `OutputModuleRefHandle`s are invalidated by ANY re-ordering, addition or removal
    /// of output modules from a render item. DO NOT CACHE THEM.
    OutputModuleSuite,
    AEGP_OutputModuleSuite4,
    kAEGPOutputModuleSuite,
    kAEGPOutputModuleSuiteVersion4
);

impl OutputModuleSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves an output module by index from a render queue item.
    pub fn output_module_by_index(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        index: i32,
    ) -> Result<OutputModuleRefHandle, Error> {
        Ok(OutputModuleRefHandle::from_raw(
            call_suite_fn_single!(
                self,
                AEGP_GetOutputModuleByIndex -> ae_sys::AEGP_OutputModuleRefH,
                rq_item.as_ptr(),
                index as ae_sys::A_long
            )?
        ))
    }

    /// Retrieves the embedding options for an output module.
    pub fn embed_options(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<EmbeddingType, Error> {
        Ok(
            call_suite_fn_single!(
                self,
                AEGP_GetEmbedOptions -> ae_sys::AEGP_EmbeddingType,
                rq_item.as_ptr(),
                output_module.as_ptr()
            )?
            .into()
        )
    }

    /// Sets the embedding options for an output module.
    pub fn set_embed_options(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        embed_options: EmbeddingType,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetEmbedOptions,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            embed_options.into()
        )
    }

    /// Retrieves the post-render action for an output module.
    pub fn post_render_action(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<PostRenderAction, Error> {
        Ok(
            call_suite_fn_single!(
                self,
                AEGP_GetPostRenderAction -> ae_sys::AEGP_PostRenderAction,
                rq_item.as_ptr(),
                output_module.as_ptr()
            )?
            .into()
        )
    }

    /// Sets the post-render action for an output module.
    pub fn set_post_render_action(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        action: PostRenderAction,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetPostRenderAction,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            action.into()
        )
    }

    /// Retrieves which output types (video, audio) are enabled for an output module.
    pub fn enabled_outputs(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<OutputTypes, Error> {
        Ok(OutputTypes::from_bits_truncate(
            call_suite_fn_single!(
                self,
                AEGP_GetEnabledOutputs -> ae_sys::AEGP_OutputTypes,
                rq_item.as_ptr(),
                output_module.as_ptr()
            )?
        ))
    }

    /// Sets which output types (video, audio) are enabled for an output module.
    pub fn set_enabled_outputs(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        enabled_types: OutputTypes,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetEnabledOutputs,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            enabled_types.bits()
        )
    }

    /// Retrieves the output channels setting for an output module.
    pub fn output_channels(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<VideoChannels, Error> {
        Ok(
            call_suite_fn_single!(
                self,
                AEGP_GetOutputChannels -> ae_sys::AEGP_VideoChannels,
                rq_item.as_ptr(),
                output_module.as_ptr()
            )?
            .into()
        )
    }

    /// Sets the output channels for an output module.
    pub fn set_output_channels(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        channels: VideoChannels,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetOutputChannels,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            channels.into()
        )
    }

    /// Retrieves the stretch info for an output module.
    ///
    /// Returns a tuple of (is_enabled, stretch_quality, is_locked).
    pub fn stretch_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<(bool, StretchQuality, bool), Error> {
        let mut is_enabled: ae_sys::A_Boolean = 0;
        let mut stretch_quality: ae_sys::AEGP_StretchQuality = 0;
        let mut locked: ae_sys::A_Boolean = 0;

        call_suite_fn!(
            self,
            AEGP_GetStretchInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            &mut is_enabled,
            &mut stretch_quality,
            &mut locked
        )?;

        Ok((is_enabled != 0, stretch_quality.into(), locked != 0))
    }

    /// Sets the stretch info for an output module.
    pub fn set_stretch_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        is_enabled: bool,
        stretch_quality: StretchQuality,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetStretchInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            is_enabled as ae_sys::A_Boolean,
            stretch_quality.into()
        )
    }

    /// Retrieves the crop info for an output module.
    ///
    /// Returns a tuple of (is_enabled, crop_rect).
    pub fn crop_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<(bool, ae_sys::A_Rect), Error> {
        let mut is_enabled: ae_sys::A_Boolean = 0;
        let mut crop_rect: ae_sys::A_Rect = unsafe { std::mem::zeroed() };

        call_suite_fn!(
            self,
            AEGP_GetCropInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            &mut is_enabled,
            &mut crop_rect
        )?;

        Ok((is_enabled != 0, crop_rect))
    }

    /// Sets the crop info for an output module.
    pub fn set_crop_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        enable: bool,
        crop_rect: ae_sys::A_Rect,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetCropInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            enable as ae_sys::A_Boolean,
            crop_rect
        )
    }

    /// Retrieves the sound format info for an output module.
    ///
    /// Returns a tuple of (sound_format, audio_enabled).
    pub fn sound_format_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<(ae_sys::AEGP_SoundDataFormat, bool), Error> {
        let mut sound_format: ae_sys::AEGP_SoundDataFormat = unsafe { std::mem::zeroed() };
        let mut audio_enabled: ae_sys::A_Boolean = 0;

        call_suite_fn!(
            self,
            AEGP_GetSoundFormatInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            &mut sound_format,
            &mut audio_enabled
        )?;

        Ok((sound_format, audio_enabled != 0))
    }

    /// Sets the sound format info for an output module.
    pub fn set_sound_format_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        sound_format: ae_sys::AEGP_SoundDataFormat,
        audio_enabled: bool,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_SetSoundFormatInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            sound_format,
            audio_enabled as ae_sys::A_Boolean
        )
    }

    /// Retrieves the output file path for an output module.
    /// Returns an empty string if not specified.
    pub fn output_file_path(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(
            self,
            AEGP_GetOutputFilePath -> ae_sys::AEGP_MemHandle,
            rq_item.as_ptr(),
            output_module.as_ptr()
        )?;
        unsafe {
            Ok(
                U16CString::from_ptr_str(MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr())
                    .to_string_lossy()
            )
        }
    }

    /// Sets the output file path for an output module.
    /// The path should use platform-native separators.
    pub fn set_output_file_path(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
        path: &str,
    ) -> Result<(), Error> {
        let path_utf16 = U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(
            self,
            AEGP_SetOutputFilePath,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            path_utf16.as_ptr()
        )
    }

    /// Adds a default output module to a render queue item.
    /// Returns a handle to the newly created output module.
    pub fn add_default_output_module(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
    ) -> Result<OutputModuleRefHandle, Error> {
        Ok(OutputModuleRefHandle::from_raw(
            call_suite_fn_single!(
                self,
                AEGP_AddDefaultOutputModule -> ae_sys::AEGP_OutputModuleRefH,
                rq_item.as_ptr()
            )?
        ))
    }

    /// Retrieves extra information about an output module.
    ///
    /// Returns a tuple of (format_name, info, is_sequence, is_multi_frame).
    pub fn extra_output_module_info(
        &self,
        rq_item: impl AsPtr<ae_sys::AEGP_RQItemRefH>,
        output_module: impl AsPtr<ae_sys::AEGP_OutputModuleRefH>,
    ) -> Result<(String, String, bool, bool), Error> {
        let mut format_handle: ae_sys::AEGP_MemHandle = std::ptr::null_mut();
        let mut info_handle: ae_sys::AEGP_MemHandle = std::ptr::null_mut();
        let mut is_sequence: ae_sys::A_Boolean = 0;
        let mut multi_frame: ae_sys::A_Boolean = 0;

        call_suite_fn!(
            self,
            AEGP_GetExtraOutputModuleInfo,
            rq_item.as_ptr(),
            output_module.as_ptr(),
            &mut format_handle,
            &mut info_handle,
            &mut is_sequence,
            &mut multi_frame
        )?;

        unsafe {
            let format = U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(format_handle)?.lock()?.as_ptr()
            ).to_string_lossy();

            let info = U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(info_handle)?.lock()?.as_ptr()
            ).to_string_lossy();

            Ok((format, info, is_sequence != 0, multi_frame != 0))
        }
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ―――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_EmbeddingType,
    EmbeddingType {
        None        = ae_sys::AEGP_Embedding_NONE,
        Nothing     = ae_sys::AEGP_Embedding_NOTHING,
        Link        = ae_sys::AEGP_Embedding_LINK,
        LinkAndCopy = ae_sys::AEGP_Embedding_LINK_AND_COPY,
    }
}

define_enum! {
    ae_sys::AEGP_PostRenderAction,
    PostRenderAction {
        None              = ae_sys::AEGP_PostRenderOptions_NONE,
        Import            = ae_sys::AEGP_PostRenderOptions_IMPORT,
        ImportAndReplace  = ae_sys::AEGP_PostRenderOptions_IMPORT_AND_REPLACE_USAGE,
        SetProxy          = ae_sys::AEGP_PostRenderOptions_SET_PROXY,
    }
}

bitflags::bitflags! {
    /// Output type flags for specifying which types of output to render.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OutputTypes: ae_sys::AEGP_OutputTypes {
        const NONE  = ae_sys::AEGP_OutputType_NONE  as ae_sys::AEGP_OutputTypes;
        const VIDEO = ae_sys::AEGP_OutputType_VIDEO as ae_sys::AEGP_OutputTypes;
        const AUDIO = ae_sys::AEGP_OutputType_AUDIO as ae_sys::AEGP_OutputTypes;
    }
}

define_enum! {
    ae_sys::AEGP_VideoChannels,
    VideoChannels {
        None  = ae_sys::AEGP_VideoChannels_NONE,
        Rgb   = ae_sys::AEGP_VideoChannels_RGB,
        Rgba  = ae_sys::AEGP_VideoChannels_RGBA,
        Alpha = ae_sys::AEGP_VideoChannels_ALPHA,
    }
}

define_enum! {
    ae_sys::AEGP_StretchQuality,
    StretchQuality {
        None = ae_sys::AEGP_StretchQual_NONE,
        Low  = ae_sys::AEGP_StretchQual_LOW,
        High = ae_sys::AEGP_StretchQual_HIGH,
    }
}
