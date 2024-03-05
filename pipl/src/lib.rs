#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod resource;
pub use resource::*;

#[cfg(target_os = "macos")]
use byteorder::BigEndian as ByteOrder;
#[cfg(target_os = "windows")]
use byteorder::LittleEndian as ByteOrder;

use byteorder::WriteBytesExt;
use std::io::Result;
use std::io::Write;

#[derive(Debug)]
pub enum PIPLType {
    General,
    Filter,
    Parser,
    ImageFormat,
    Extension,
    Acquire,
    Export,
    Selection,
    Picker,
    Actions,
    Test,
    MSPUtility,
    PsModernFilter,
    AEEffect,
    AEImageFormat,
    AEAccelerator,
    AEGeneral,
    PrEffect,
    PrVideoFilter,
    PrAudioFilter,
    PrEDLExport,
    PrDataExport,
    PrDevice,
    PrImporter,
    PrCompile,
    PrRecord,
    PrPlay,
    SweetPea,
    AIGeneral,
}

const fn fourcc(code: &[u8; 4]) -> [u8; 4] {
    // Code order is different between Windows and MacOS
    #[cfg(target_os = "windows")]
    {
        [code[3], code[2], code[1], code[0]]
    }
    #[cfg(target_os = "macos")]
    {
        *code
    }
}
const fn u32_bytes(v: u32) -> [u8; 4] {
    #[cfg(target_os = "windows")]
    {
        v.to_le_bytes()
    }
    #[cfg(target_os = "macos")]
    {
        v.to_be_bytes()
    }
}

impl PIPLType {
    #[rustfmt::skip]
    pub fn as_bytes(&self) -> [u8; 4] {
        match self {
            // Photoshop plug-in types
            Self::General        => fourcc(b"8BPI"),
            Self::Filter         => fourcc(b"8BFM"),
            Self::Parser         => fourcc(b"8BYM"),
            Self::ImageFormat    => fourcc(b"8BIF"),
            Self::Extension      => fourcc(b"8BXM"),
            Self::Acquire        => fourcc(b"8BAM"),
            Self::Export         => fourcc(b"8BEM"),
            Self::Selection      => fourcc(b"8BSM"),
            Self::Picker         => fourcc(b"8BCM"),
            Self::Actions        => fourcc(b"8LIZ"),
            Self::Test           => fourcc(b"8BTS"),
            Self::MSPUtility     => fourcc(b"8SPU"),
            Self::PsModernFilter => fourcc(b"8BFm"),
            // After Effects plug-in types
            Self::AEEffect       => fourcc(b"eFKT"),
            Self::AEImageFormat  => fourcc(b"FXIF"),
            Self::AEAccelerator  => fourcc(b"eFST"),
            Self::AEGeneral      => fourcc(b"AEgp"),
            // Premiere plug-in typefourcc
            Self::PrEffect       => fourcc(b"SPFX"),
            Self::PrVideoFilter  => fourcc(b"VFlt"),
            Self::PrAudioFilter  => fourcc(b"AFlt"),
            Self::PrEDLExport    => fourcc(b"ExpM"),
            Self::PrDataExport   => fourcc(b"ExpD"),
            Self::PrDevice       => fourcc(b"DevC"),
            Self::PrImporter     => fourcc(b"IMPT"),
            Self::PrCompile      => fourcc(b"CMPM"),
            Self::PrRecord       => fourcc(b"RECM"),
            Self::PrPlay         => fourcc(b"PLYM"),
            // Illustrator/SweetPea plug-in types
            Self::SweetPea       => fourcc(b"SPEA"),
            Self::AIGeneral      => fourcc(b"ARPI")
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
	/// The out_flags field of the OutData can be set to an OR-ed combination of these flags to communicate various things to the driver program.
    pub struct OutFlags: u32 {
        /// This is the "empty" setting -- no outflags.
        const None = 0;
        /// Obsoleted in AE 2015 (does nothing when set).
        ///
		/// Set this flag if your effect expects its Macintosh resource fork to be open at any time other than global setup.  Note that this does not mean that
		/// the resource fork will be kept open at all times, just whenever the effect is being executed.
        const KeepResourceOpen             = 1 << 0;  // PF_Cmd_GLOBAL_SETUP
        /// Set this flag if the effect calls get_param to inquire a parameter at a time besides the current one (e.g. to get the previous video frame).
        /// This should be sent, if it is going to be sent, at PF_Cmd_GLOBAL_SETUP. Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        ///
        /// As of AE10, this flag is no longer recommended. It still works the same way and is safe to set, but there's a more efficient option.
        /// See PF_OutFlag2_AUTOMATIC_WIDE_TIME_INPUT.
        const WideTimeInput                = 1 << 1;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// Set this if the effect uses information other than the parameters in the param list to generate its output at the current time.
        /// For instance, if the effect uses the current time of the frame or some random value to decide the output, set this flag.
        /// This flag should be sent at PF_Cmd_GLOBAL_SETUP.
        /// If the effect produces changing frames when applied to a still image and all parameters are constant, that's a sure sign that this bit should be set (e.g. Wave Warp).
        /// Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const NonParamVary                 = 1 << 2;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        // const Reserved6                    = 1 << 3;
        /// When you allocate a sequence data handle, the app may write the handle out to disk and reuse it later.
        /// Pass this flag if the handle is not "flat" (i.e. has pointers or handles hanging off of it).
        /// Basically, this gives you a chance to alter the handle contents before it is written out to disk, so you won't get invalid handles or pointers.
        /// Once you have flattened a handle, you will get an opportunity to un-flatten it before the effect needs to continue.
        /// For sequence data, you will be invoked with a PF_Cmd_SEQUENCE_RESETUP call.
        /// You should store a boolean at a common offset in your unflattened and flattened data that says whether the data is flat or not.
        /// If you get a PF_Cmd_SEQUENCE_RESETUP and the boolean indicated the data is flattened, you should unflatten the data,
        /// free the flattened data handle, and set the sequence_data handle in the PF_OutData.
        /// If you ever set the data to NULL when you flatten it, you will NOT get the sequence resetup call to unflatten it.
        /// Instead, you may just get a RENDER call with NULL data. Forewarned is forearmed. This flag, indicating if the data will need to be flattened, should be set at PF_Cmd_GLOBAL_SETUP time.
        const SequenceDataNeedsFlattening  = 1 << 4;  // PF_Cmd_GLOBAL_SETUP
        /// Set this is the effect responds to a PF_Cmd_DO_DIALOG, i.e. Does this effect bring up an options dialog box.
        /// PF_Cmd_DO_DIALOG is generated when the user presses the Options button on the Effect floater.
        /// This flag should be set at PF_Cmd_GLOBAL_SETUP time.
        const IDoDialog                    = 1 << 5;  // PF_Cmd_GLOBAL_SETUP
        /// The output layer is passed with an "extent rect" indicating the area of the layer that actually contains visible image data.
        /// If the effect changes its behavior based on the extent rect (for instance, by not iterating over the entire image),
        /// set this flag, so the application will know whether having the extent change should cause the frame to re-render.
        /// Specify this flag at PF_Cmd_GLOBAL_SETUP.
        const UseOutputExtent              = 1 << 6;  // PF_Cmd_GLOBAL_SETUP
        /// Some filters need their options dialog box to be brought up at least once to be valid.
        /// You can set this flag, and the driver app will automatically send a PF_Cmd_DO_DIALOG to the effect when it is applied.
        /// The DO_DIALOG will be sent after PF_Cmd_SEQUENCE_SETUP.  This flag should be set in PF_Cmd_SEQUENCE_SETUP if it is going to be set.
        const SendDoDialog                 = 1 << 7;  // PF_Cmd_SEQUENCE_SETUP
        /// Whenever the return_msg field in the PF_OutData is set to a string, After Effects will bring up a simple dialog box containing that string.
        /// If you set this flag, the dialog box will be made to look like an error message dialog box.
        /// If you don't set this flag, it will be an undecorated dialog box.
        /// Using this flag, an effects module can have and display its own error messages and not worry about the code for dialog boxes -- the program will do it for you.
        /// This flag can be sent after any command.
        const DisplayErrorMessage          = 1 << 8;  // all PF_Cmds
        /// Starting with After Effects 2.0, effects will be able to expand their buffers beyond the current layer's dimensions.
        /// This has always been part of the PF specification, but as an extra precaution (and hint to the AE rendering engine) set this flag at PF_Cmd_GLOBAL_SETUP if you plan to expand your buffer.
        const IExpandBuffer                = 1 << 9;  // PF_Cmd_GLOBAL_SETUP
        /// Set this flag if the output at a given pixel is not dependent on the values of the pixels around it.
        /// If this is set, the pixels After Effects does not care about (because of field rendering, for example) could be filled with garbage colors.
        /// Please set this flag at PF_Cmd_GLOBAL_SETUP. Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const PixIndependent               = 1 << 10; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// Set this flag if your effect would like to write into the input buffer.
        /// This can be useful if you need an scratch buffer, but it also invalidates some speedups in the AE rendering pipeline, so use it with some discretion.
        /// Please set this flag at PF_Cmd_GLOBAL_SETUP.
        const IWriteInputBuffer            = 1 << 11; // PF_Cmd_GLOBAL_SETUP
        /// Set this flag if you can shrink your buffer based on the extent-rects passed to you in order to be more memory efficient.
        const IShrinkBuffer                = 1 << 12; // PF_Cmd_GLOBAL_SETUP
        const WorksInPlace                 = 1 << 13; // PF_Cmd_GLOBAL_SETUP
        // const Reserved8                    = 1 << 14;
        /// This flag must be set if your effect has a custom UI in the Effect Controls Window, Layer Window or Comp Window.
        const CustomUI                     = 1 << 15; // PF_Cmd_GLOBAL_SETUP
        // const Reserved7                    = 1 << 16;
        /// Can be returned from PF_Cmd_EVENT, PF_Cmd_RENDER, and PF_Cmd_DO_DIALOG.
        /// Causes the effects control window, layer window, and comp window to be re-drawn.
        const RefreshUI                    = 1 << 17; // PF_Cmd_EVENT, PF_Cmd_RENDER, PF_Cmd_DO_DIALOG
        /// Set this flag in PF_Cmd_GLOBAL_SETUP if the render would never result in changes to the source image (or audio?). For example, an expression control would set this.
        const NopRender                    = 1 << 18; // PF_Cmd_GLOBAL_SETUP
        /// Must be set at PF_Cmd_GLOBAL_SETUP time if the effect uses the shutter_angle or the shutter_phase.
        /// Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const IUseShutterAngle             = 1 << 19; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// Must be set at PF_Cmd_GLOBAL_SETUP time for a visual effect that calls the audio checkout calls.
        const IUseAudio                    = 1 << 20; // PF_Cmd_GLOBAL_SETUP
        /// Set at PF_Cmd_GLOBAL_SETUP time for effects that don't want to appear in the AE Effects menu (but will still be invoked if you load a project that has an old copy of the effect applied).
        const IAmObsolete                  = 1 << 21; // PF_Cmd_GLOBAL_SETUP
        /// Set at PF_Cmd_EVENT if the effect modified sequence data, or did anything else that requires the effect needs to re-render.
        /// Note that setting PF_ChangeFlag_CHANGED_VALUE automatically causes a re-render, so don't worry about setting PF_OutFlag_FORCE_RERENDER in that case.
        /// Also, I_MIX_GUID_DEPENDENCIES can be used to trigger a rerender on dependant changes if sequence_data has not been changed.
        ///
        /// IMPORTANT: FORCE_RERENDER should be used as a last resort. Long term we should be eliminating the need for this
        /// because it causes forced cache invalidation that doesn't work well with undo.
        /// Once we have the full set of APIs in place needed to manage render state, we will be able to deprecate this.
        /// Prefer using ARB data + CHANGED_VALUE or I_MIX_GUID_DEPENDENCIES when possible instead.
        ///
        /// In 13.5 the split between a UI and render threads means that FORCE_RERENDER will now also have the needed side effect of copying sequence_data state to the render project.
        /// This can be expensive if the sequence_data is large.
        /// Support GET_FLATTENED_SEQUENCE_DATA to prevent deallocation of your sequence_data, which can help.
        /// GET_FLATTENED_SEQUENCE_DATA support is required for FORCE_RERENDER use in custom mouse/key events.
        const ForceRerender                = 1 << 22; // PF_Cmd_EVENT, PF_Cmd_USER_CHANGED_PARAM, PF_Cmd_UPDATE_PARAMS_UI
        /// Valid only for setting in your PiPL. When set out_flags will be ignored at PF_Cmd_GLOBAL_SETUP time (& thus don't need to match).
        const PiplOverridesOutdataOutflags = 1 << 23; // PiPL-only-flag
        /// Set this flag at PF_Cmd_GLOBAL_SETUP time if the effect has dependencies that the user should know about before transporting their project to a different machine.
        /// For example, dependencies on an installed font, or on an external file.
        /// If set, the effect will receive a PF_Cmd_GET_EXTERNAL_DEPENDENCIES request, where the extra param will be a PF_ExtDependenciesExtra,
        /// and the effect should report its information based on the given sequence_data.
        const IHaveExternalDependencies    = 1 << 24; // PF_Cmd_GLOBAL_SETUP
        /// Marks the plugin as aware of 16-bpc pixels. This is a hint to the host that the plugin can handle 16-bpc pixels.
        const DeepColorAware               = 1 << 25; // PF_Cmd_GLOBAL_SETUP
        /// Set this flag at PF_Cmd_GLOBAL_SETUP time if you want to receive PF_Cmd_UPDATE_PARAMS_UI messages.
        const SendUpdateParamsUI           = 1 << 26; // PF_Cmd_GLOBAL_SETUP

        // audio flags (pfOutflagAudio_EFFECT_TOO or PF_OutFlag_AUDIO_EFFECT_ONLY required for audio effects)
        /// Set this flag if you only want to receive PF_SIGNED_FLOAT data when processing audio data.
        /// Requires PF_OutFlag_AUDIO_EFFECT_TOO or PF_OutFlag_AUDIO_EFFECT_ONLY.
        const AudioFloatOnly               = 1 << 27; // PF_Cmd_GLOBAL_SETUP
        /// Set this flag at PF_Cmd_GLOBAL_SETUP time if you are an Infinite-Impulse-Response audio filter (i.e. your output at a given time depends on your output from previous times).
        const AudioIir                     = 1 << 28; // PF_Cmd_GLOBAL_SETUP
        /// Set this flag at PF_Cmd_GLOBAL_SETUP time if you generate audio even when handed silence.
        /// Requires PF_OutFlag_AUDIO_EFFECT_TOO or PF_OutFlag_AUDIO_EFFECT_ONLY.
        const ISynthesizeAudio             = 1 << 29; // PF_Cmd_GLOBAL_SETUP
        /// Must be set at PF_Cmd_GLOBAL_SETUP time for an effect that wants to filter the audio too (as opposed to just reading the audio).
        const AudioEffectToo               = 1 << 30; // PF_Cmd_GLOBAL_SETUP
        /// Must be set at PF_Cmd_GLOBAL_SETUP time for an effect that only filters audio (no video).
        const AudioEffectOnly              = 1 << 31; // PF_Cmd_GLOBAL_SETUP
    }
}
bitflags::bitflags! {
    #[derive(Debug)]
    pub struct OutFlags2: u32 {
        const None = 0;
        /// Set this during PF_Cmd_GLOBAL_SETUP if the effect handles PF_Cmd_QUERY_DYNAMIC_FLAGS.
        /// Supporting this command can dramatically improve performance for certain effects, because it provides dynamic
        /// information to the host about what can be cached (as opposed to PIPL bits which cannot be changed at run-time)
        const SupportsQueryDynamicFlags           = 1 << 0;  // PF_Cmd_GLOBAL_SETUP
        /// This bit must be set if the effect ever uses the AEGP PF_Interface suite to access camera layers. Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const IUse3DCamera                        = 1 << 1;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// This bit must be set if the effect ever uses the AEGP PF_Interface suite to access camera layers. Can be over-ridden dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const IUse3DLights                        = 1 << 2;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// If you want a parameter group to honor the PF_ParamFlag_COLLAPSE_TWIRLY or PF_ParamFlag_START_COLLAPSED flag, set this bit.
        /// Otherwise, all parameter groups will be collapsed by default.
        const ParamGroupStartCollapsedFlag        = 1 << 3;  // PF_Cmd_GLOBAL_SETUP
        const IAmThreadsafe                       = 1 << 4;  // PF_Cmd_GLOBAL_SETUP (unused)
        const CanCombineWithDestination           = 1 << 5;  // Premiere only (as of AE 6.0)
        /// Added for render optimizations; shrinks the input buffer passed to the effect to exclude any empty pixels (where empty means "zero alpha" unless PF_OutFlag2_REVEALS_ZERO_ALPHA is set, in which case RGB must be zero as well.)
        /// The origin of the trimmed buffer can be found in in_data->pre_effect_source_origin.
        /// Effects with both this flag and PF_OutFlag_I_EXPAND_BUFFER set may get called with a null input buffer if their input is completely empty, and must be able to handle this case without crashing.
        /// This flag can be cleared dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const DoesntNeedEmptyPixels               = 1 << 6;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// The effect can take pixels with zero alpha and reveal the RGB data in them (like our Set Channels effect).
        /// This tells After Effects not to trim such pixels when determining the input for the effect.
        /// This flag can be cleared dynamically during PF_Cmd_QUERY_DYNAMIC_FLAGS.
        const RevealsZeroAlpha                    = 1 << 7;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const PreservesFullyOpaquePixels          = 1 << 8;  // Premiere only (as of AE 6.0)
        const SupportsSmartRender                 = 1 << 10; // PF_Cmd_GLOBAL_SETUP
        // const Reserved9                           = 1 << 11; // PF_Cmd_GLOBAL_SETUP
        const FloatColorAware                     = 1 << 12; // PF_Cmd_GLOBAL_SETUP, may require PF_OutFlag2_SUPPORTS_SMART_RENDER
        const IUseColorspaceEnumeration           = 1 << 13; // PF_Cmd_GLOBAL_SETUP, not implemented in AE7 (may be impl in Premiere Pro)
        /// this effect is still available, and shows up under user-visible "Obsolete" category in the UI.
        /// Setting this flag means "there's a better way to do this, but this effect may still be useful in some situations".
        /// Distinct from PF_OutFlag_I_AM_OBSOLETE in that these will still show up in the GUI and the user can still apply them to new projects.
        /// The category that is set by the effect is pretty much ignored, as it will instead always go into the "Obsolete" category
        const IAmDeprecated                       = 1 << 14; // PF_Cmd_GLOBAL_SETUP
        const PproDoNotCloneSequenceDataForRender = 1 << 15; // PF_Cmd_GLOBAL_SETUP, Premiere only, CS4.1 and later
        // const Reserved10                          = 1 << 16; // PF_Cmd_GLOBAL_SETUP
        /// New in AE 10. Requires setting of PF_OutFlag_WIDE_TIME_INPUT (which allows you to support old hosts), but effectively overrides that flag.
        /// When set, all parameter checkouts are tracked so over-time dependencies are known by AE.
        /// Note that if you use this new flag, and you cache any time-dependent data in your sequence data (or anywhere else),
        /// you must validate that cache using the new PF_HaveInputsChangedOverTimeSpan() before using it.
        ///
        /// This only works for smart effects (those that set PF_OutFlag2_SUPPORTS_SMART_RENDER).
        /// If you haven't set that, After Effects will silently treat this as PF_OutFlag_WIDE_TIME_INPUT instead.
        ///
        /// To test that it's working, apply your effect with one parameter keyframed on every frame.
        /// RAM Preview to fill the cache, then change one of the keyframes.
        /// The related frame and all dependent frames (e.g. later frames, in the case of a simulation) should lose their cache marks and require re-rendering.
        /// Simlarly, upstream changes to sources of layer parameters should cause time-selective invalidation of the cache.
        const AutomaticWideTimeInput              = 1 << 17; // PF_Cmd_GLOBAL_SETUP, falls back to PF_OutFlag_WIDE_TIME_INPUT if not PF_OutFlag2_SUPPORTS_SMART_RENDER
        /// New in AE 9.0. The effect depends on the Composition's timecode or a layer's source footage timecode.
        /// If the underlying timecode changes the effects will be asked to rerender.
        const IUseTimecode                        = 1 << 18; // PF_Cmd_GLOBAL_SETUP
        /// Set this if you are going to look at paths that aren't directly referenced by a path param, e.g. if you are going to draw a stroke on all masks.
        const DependsOnUnreferencedMasks          = 1 << 19; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// Set this if your output is going to be watermarked in some way that makes it unsuitable for final use, probably because the user is using an unlicensed demo version.
        /// It is ok to change this state during the course of app session, if e.g. a floating license status changes.
        /// Plugin authors that actually do have this state changing asynchronously must be careful to have the next render match the last state
        /// returned from QUERY_DYNAMIC_FLAGS otherwise race conditions could cause incorrect frames to be cached. (This is a non-issue if you only change this in response to DO_DIALOG.)
        const OutputIsWatermarked                 = 1 << 20; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        /// Smart effects only. With this option, FORCE_RERENDER becomes a cache-savvy more efficient MAYBE rerender.
        /// If custom UI or DO_DIALOG change sequence data, returning FORCE_RERENDER requests AE to check whether rerender needs to occur.
        /// During PreRender, the effect uses the GuidMixInPtr callback to mix any additional state that affects the render into our internal GUID for the cached frame.
        /// AE can then tell whether the frame already exists and if so, no longer needs to render.
        /// This also means that DO_DIALOG no longer always blows the cache and that undo works across DO_DIALOG.
        /// Cancelation of DO_DIALOG no longer blows the cache either.
        /// This also means that I_USE_* flags are now basically redundant since any dependency could be mixed in.
        /// Just be sure to mix in everything that can uniquely affect resulting rendered pixels (that is not already an AE stream parameter).
        /// But don't mixin things that are disabled and have no render effect (this results in less cache efficiency).
        const IMixGuidDependencies                = 1 << 21; // PF_Cmd_GLOBAL_SETUP
        const Ae135Threadsafe                     = 1 << 22; // PF_Cmd_GLOBAL_SETUP (unused)
        const SupportsGetFlattenedSequenceData    = 1 << 23; // PF_Cmd_GLOBAL_SETUP, support required if both PF_OutFlag_SEQUENCE_DATA_NEEDS_FLATTENING and PF_OutFlag2_SUPPORTS_THREADED_RENDERING is set
        /// This flags enables use of AEGP_CheckoutOrRender_*_AsyncManager() calls which avoid the need for plugin management of the lifetime of async custom UI renders from the UI thread.
        /// The plugin asks for what frames it needs and the manager calls PF_Event_DRAW again when they are available (or cancels them as needed automatically).
        /// The plugin responds in PF_Event_DRAW by asking for what it needs and drawing what it can from what is available.
        ///
        /// Due to separation of Render thread and UI thread in 13.5, frames for custom UI should no longer be rendered synchronously (see RenderSuite5 for more details).
        /// The manager simplifies this, especially when there are multiple requests needed for DRAW.
        ///
        /// When enabled, this flag associates a "PF_AsyncManager" with the NEW_CONTEXT/CLOSE_CONTEXT and PF_Event_DRAW that will
        /// automatically track completion of 1 or more asynch render requests made for drawing custom UI.
        /// As requests complete, PF_Event_DRAW will be called again and the current state of the CUSTOM_UI can be drawn.
        /// Such requests may be canceled automatically as the user scrubs the time needle or project changes are made and become invalid.
        ///
        /// This flag is used in addition to the CUSTOM_UI flag during PF_Cmd_GLOBAL_SETUP
        const CustomUIAsyncManager                = 1 << 24; // PF_Cmd_GLOBAL_SETUP
        const SupportsGpuRenderF32                = 1 << 25; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_GPU_DEVICE_SETUP. Must also set PF_RenderOutputFlag_GPU_RENDER_POSSIBLE at pre-render to enable GPU rendering.
        // const Reserved12                          = 1 << 26; // PF_Cmd_GLOBAL_SETUP
        /// Indicates the effect supports rendering on multiple threads at the same time.
        /// Single or multiple applications of this effect on a layer can be called to render at the same time on multiple threads.
        ///
        /// UI selectors are still sent on the main thread, however Sequence Setup, Sequence Resetup, Sequence SetDown, PreRender, and Render may be sent on
        /// multiple threads at the same time as the UI selectors are being handled so all of these selectors must be thread safe.
        ///
        /// Global Setup and Global Setdown selectors are unaffected by this flag.
        /// Regardless whether this flag is set or not, they will only be sent on the main thread, and will not be sent at the same time as any other selectors.
        ///
        /// If the effect sets PF_OutFlag_SEQUENCE_DATA_NEEDS_FLATTENING indicating the sequence data needs flattening then it must also set PF_OutFlag2_SUPPORTS_GET_FLATTENED_SEQUENCE_DATA.
        ///
        /// sequence_data is read-only at render time and must be accessed with PF_EffectSequenceDataSuite. in_data->sequence_data will be NULL during render.
        /// AEGP_ComputeCacheSuite is suggested if writing to sequence_data at render time is needed for caching.
        /// This suite unifies cache entries so multiple threads do not recompute the same cache value.
        /// If neither of these solutions work, see the next flag, PF_OutFlag2_MUTABLE_RENDER_SEQUENCE_DATA_SLOWER.
        const SupportsThreadedRendering           = 1 << 27; // PF_Cmd_GLOBAL_SETUP
        /// Indicates the effect needs sequence_data replicated for each render thread, thus allowing each render to have sequence_data which can be written to.
        /// Note that changes to sequence_data will be discarded regularly, currently after each span of frames is rendered such as single RAM Preview or Render Queue export.
        const MutableRenderSequenceDataSlower     = 1 << 28; // PF_Cmd_GLOBAL_SETUP
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct SupportedModes: u32 {
        const Bitmap           = 1 << 15;
        const GrayScale        = 1 << 14;
        const IndexedColor     = 1 << 13;
        const RGBColor         = 1 << 12;
        const CMYKColor        = 1 << 11;
        const HSLColor         = 1 << 10;
        const HSBColor         = 1 << 9;
        const Multichannel     = 1 << 8;
        const Duotone          = 1 << 7;
        const LABColor         = 1 << 6;
        const Gray16           = 1 << 5;
        const RGB48            = 1 << 4;
        const Lab48            = 1 << 3;
        const CMYK64           = 1 << 2;
        const DeepMultichannel = 1 << 1;
        const Duotone16        = 1 << 0;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FilterCaseInfoIn {
    CantFilter = 0,
    StraightData = 1,
    BlackMat = 2,
    GrayMat = 3,
    WhiteMat = 4,
    Defringe = 5,
    BlackZap = 6,
    GrayZap = 7,
    WhiteZap = 8,
    BackgroundZap = 10,
    ForegroundZap = 11,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FilterCaseInfoOut {
    CantFilter = 0,
    StraightData = 1,
    BlackMat = 2,
    GrayMat = 3,
    WhiteMat = 4,
    FillMask = 9,
}
#[derive(Debug)]
pub struct FilterCaseInfoStruct {
    in_handling: FilterCaseInfoIn,
    out_handling: FilterCaseInfoOut,
    write_outside_selection: bool,
    filters_layer_masks: bool,
    works_with_blank_data: bool,
    copy_source_to_destination: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BitTypes {
    None = 0x00,
    Top = 0x01,
    Right = 0x02,
    Bottom = 0x04,
    Left = 0x08,
    UpperRight = 0x10,
    LowerRight = 0x20,
    LowerLeft = 0x40,
    UpperLeft = 0x80,
}
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PixelAspectRatio {
    AnyPAR = 0x10000,
    UnityPAR = 0x20000,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AnimDataType {
    Opaque = 0,
    Char,
    Short,
    Long,
    UnsignedChar,
    UnsignedShort,
    UnsignedLong,
    Fixed,
    UnsignedFixed,
    Extended96,
    Double64,
    Float32,
    ColorRGB,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AnimUIType {
    NoUI = 0,
    Angle,
    Slider,
    Point,
    Rect,
    ColorRGB,
    ColorCMYK,
    ColorLAB,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ClassType {
    None = 0,
    Scanner,
    Camera,
    Video,
    Floppy,
    Cdrom,
    Internet,
}

#[derive(Debug)]
pub enum ButtonIconType {
    None,
    MacCICN,
    WindowsICON,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Stage {
    Develop = 0,
    Alpha,
    Beta,
    Release,
}

pub const fn pf_version(vers: u32, subvers: u32, bugvers: u32, stage: Stage, build: u32) -> u32 {
    const PF_VERS_BUILD_BITS: u32 = 0x1ff;
    const PF_VERS_BUILD_SHIFT: u32 = 0;
    const PF_VERS_STAGE_BITS: u32 = 0x3;
    const PF_VERS_STAGE_SHIFT: u32 = 9;
    const PF_VERS_BUGFIX_BITS: u32 = 0xf;
    const PF_VERS_BUGFIX_SHIFT: u32 = 11;
    const PF_VERS_SUBVERS_BITS: u32 = 0xf;
    const PF_VERS_SUBVERS_SHIFT: u32 = 15;
    const PF_VERS_VERS_BITS: u32 = 0x7; // incomplete without high bits, below
    const PF_VERS_VERS_SHIFT: u32 = 19;
    // skipping these bits for similarity to Up_Vers_ARCH_*, currently unused in PF
    const PF_VERS_VERS_HIGH_BITS: u32 = 0xf; // expand version max from 7 to 127
    const PF_VERS_VERS_HIGH_SHIFT: u32 = 26;
    // b/c we are stripping the stand alone vers value for two fields
    const PF_VERS_VERS_LOW_SHIFT: u32 = 3;

    (((vers >> PF_VERS_VERS_LOW_SHIFT) & PF_VERS_VERS_HIGH_BITS) << PF_VERS_VERS_HIGH_SHIFT)
        | ((vers & PF_VERS_VERS_BITS) << PF_VERS_VERS_SHIFT)
        | ((subvers & PF_VERS_SUBVERS_BITS) << PF_VERS_SUBVERS_SHIFT)
        | ((bugvers & PF_VERS_BUGFIX_BITS) << PF_VERS_BUGFIX_SHIFT)
        | ((stage as u32 & PF_VERS_STAGE_BITS) << PF_VERS_STAGE_SHIFT)
        | ((build & PF_VERS_BUILD_BITS) << PF_VERS_BUILD_SHIFT)
}

#[derive(Debug)]
pub enum Property {
    Kind(PIPLType),
    Version {
        version: u32,
        subversion: u32,
        bugversion: u32,
        stage: Stage,
        build: u32,
    },
    Priority(u32),
    RequiredHost(&'static [u8; 4]),
    Component((u32, &'static str)),
    Name(&'static str),
    Category(&'static str),
    Code68k((PIPLType, u16)),
    Code68kFPU((PIPLType, u16)),
    CodePowerPC((u32, u32, &'static str)),
    CodeCarbonPowerPC((u32, u32, &'static str)),
    CodeMachOPowerPC(&'static str),
    CodeMacIntel32(&'static str),
    CodeMacIntel64(&'static str),
    CodeMacARM64(&'static str),
    CodeWin32X86(&'static str),
    CodeWin64X86(&'static str),
    SupportedModes(SupportedModes),
    EnableInfo(&'static str),
    FilterCaseInfo(&'static [FilterCaseInfoStruct]),
    ExportFlags {
        supports_transparency: bool,
    },
    FmtFileType((&'static [u8; 4], &'static [u8; 4])),
    ReadTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    WriteTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    FilteredTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ReadExtensions(&'static [&'static [u8; 4]]),
    WriteExtensions(&'static [&'static [u8; 4]]),
    FilteredExtensions(&'static [&'static [u8; 4]]),
    FormatFlags {
        saves_image_resources: bool,
        can_read: bool,
        can_write: bool,
        can_write_if_read: bool,
    },
    FormatMaxSize {
        width: u16,
        height: u16,
    },
    FormatMaxChannels(&'static [u16]),
    ParsableTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ParsableClipboardTypes(&'static [&'static [u8; 4]]),
    FilteredParsableTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ParsableExtensions(&'static [&'static [u8; 4]]),
    FilteredParsableExtensions(&'static [&'static [u8; 4]]),
    PickerID(&'static str),
    HasTerminology {
        class_id: u32,
        event_id: u32,
        dictionary_resource_id: u16,
        unique_scope_string: &'static str,
    },
    Persistent,
    AE_PiPL_Version {
        minor: u16,
        major: u16,
    },
    AE_Effect_Spec_Version {
        minor: u16,
        major: u16,
    },
    AE_Effect_Version {
        version: u32,
        subversion: u32,
        bugversion: u32,
        stage: Stage,
        build: u32,
    },
    AE_Effect_Match_Name(&'static str),
    AE_Effect_Info_Flags(u32),
    AE_Effect_Global_OutFlags(OutFlags),
    AE_Effect_Global_OutFlags_2(OutFlags2),
    AE_Reserved(u32),
    AE_Reserved_Info(u32),
    AE_Effect_Support_URL(&'static str),
    AE_ImageFormat_Extension_Info {
        major_version: u16,
        minor_version: u16,
        has_options: bool,
        sequential_only: bool,
        must_interact: bool,
        has_interact_put: bool,
        has_interact_get: bool,
        has_time: bool,
        has_video: bool,
        still: bool,
        has_file: bool,
        output: bool,
        input: bool,
        signature: [u8; 4],
    },
    ANIM_FilterInfo {
        spec_version_major: u32,
        spec_version_minor: u32,
        filter_params_version: u32,
        unity_pixel_aspec_tratio: bool,
        any_pixel_aspect_ratio: bool,
        drive_me: bool,          // ANIM_FF_DONT_DRIVE_ME (AE only)
        needs_dialog: bool,      // ANIM_FF_DOESNT_NEED_DLOG (AE only)
        params_pointer: bool,    // ANIM_FF_PARAMS_ARE PTR (AE only)
        params_handle: bool,     // ANIM_FF_PARAMS_ARE_HANDLE (AE only)
        params_mac_handle: bool, // ANIM_FF_PARAMS_ARE_MAC_HANDLE (AE only)
        dialog_in_render: bool,  // ANIM_FF_DIALOG_IN_RENDER (AE only)
        params_in_globals: bool, // ANIM_FF_PARAMS_IN_GLOBALS (AE only)
        bg_animatable: bool,     // ANIM_FF_BG_ANIMATABLE (AE only)
        fg_animatable: bool,     // ANIM_FF_FG_ANIMATABLE (AE only)
        geometric: bool,         // ANIM_FF_NON_GEOMETRIC (AE only)
        randomness: bool,        // ANIM_FF_HAS_RANDOMNESS (AE only)
        number_of_parameters: u32,
        match_name: &'static str, // 32 bytes
    },
    ANIM_ParamAtom {
        external_name: &'static str, // 32 bytes
        match_id: u32,
        data_type: AnimDataType,
        ui_type: AnimUIType,
        valid_min: f64,
        valid_max: f64,
        ui_min: f64,
        ui_max: f64,

        scale_ui_range: bool,
        animate_param: bool,
        restrict_bounds: bool,
        space_is_relative: bool,
        res_dependant: bool,

        property_size: u32, // size of property described in bytes (short = 2, long = 4, etc.)
    },
    Pr_Effect_Info {
        version: u32,
        valid_corners_mask: BitTypes,
        initial_corners: BitTypes,

        exclusive_dialog: bool,
        needs_callbacks_at_setup: bool,
        direct_comp_data: bool,
        want_initial_setup_call: bool,
        treat_as_transition: bool,
        has_custom_dialog: bool,
        highlight_opposite_corners: bool,
        exclusive: bool,
        reversible: bool,
        have_edges: bool,
        have_start_point: bool,
        have_end_point: bool,

        more_flags: u32,
    },
    Pr_Effect_Description(&'static str),
    InterfaceVersion(u32),
    AdapterVersion(u32),
    SP_STSP(u32),
    InternalName(&'static str),
    Imports(&'static [(&'static str, u32)]), // suite name, version
    Exports(&'static [(&'static str, u32)]), // suite name, version
    Description(&'static str),
    Keywords(&'static [&'static str]),
    Title(&'static str),
    Messages {
        startup_required: bool,
        purge_cache: bool,
        shutdown_required: bool,
        accept_property: bool,
    },
    ButtonIcon {
        version: u32,
        mac_icon_type: ButtonIconType,
        win_icon_type: ButtonIconType,
        resource_id: u32,
        icon_name: &'static str,
    },
    Class {
        version: u32,
        class: ClassType,
    },
    PreviewFile {
        version: u32,
        filename: &'static str,
    },
}

pub fn build_pipl(properties: Vec<Property>) -> Result<Vec<u8>> {
    #[rustfmt::skip]
    fn padding_4(x: u32) -> u32 { if x % 4 != 0 { 4 - x % 4 } else { 0 } }

    fn write(
        buffer: &mut Vec<u8>,
        type_: &[u8; 4],
        key: &[u8; 4],
        mut contents_fn: impl FnMut(&mut Vec<u8>) -> Result<()>,
    ) -> Result<()> {
        buffer.write(&fourcc(type_))?;
        buffer.write(&fourcc(key))?;
        buffer.write_u32::<ByteOrder>(0)?; // pad
        let len = buffer.len();
        buffer.write_u32::<ByteOrder>(0)?; // length placeholder
        contents_fn(buffer)?;
        let aligned_len = (buffer.len() - len - 4) as u32;
        // Overwrite the length
        buffer[len..len + 4].clone_from_slice(&u32_bytes(aligned_len));

        // Padding is done differently between Windows and macOS
        if cfg!(target_os = "macos") {
            let padding = padding_4(aligned_len);
            for _ in 0..padding {
                buffer.write_u8(0)?;
            }
        }
        Ok(())
    }
    // Write pascal string
    fn write_pstring(buffer: &mut Vec<u8>, s: &'static str) -> Result<()> {
        buffer.write_u8(s.len() as u8)?;
        buffer.extend(s.as_bytes());

        // Padding is done differently between Windows and macOS
        if cfg!(target_os = "windows") {
            let padding = padding_4(s.len() as u32 + 1);
            for _ in 0..padding {
                buffer.write_u8(0)?;
            }
        }
        Ok(())
    }
    // Write Long Word padded C String
    fn write_cstring(buffer: &mut Vec<u8>, s: &'static str) -> Result<()> {
        buffer.extend(s.as_bytes());
        buffer.push(0);

        // Padding is done differently between Windows and macOS
        if cfg!(target_os = "windows") {
            let padding = padding_4(s.len() as u32 + 1);
            for _ in 0..padding {
                buffer.write_u8(0)?;
            }
        }
        Ok(())
    }

    let mut buffer = Vec::new();
    if cfg!(target_os = "windows") {
        buffer.write_u8(1)?; // Reserved
        buffer.write_u8(0)?; // Reserved
    }
    buffer.write_u32::<ByteOrder>(0)?; // kPIPropertiesVersion
    buffer.write(&u32_bytes(properties.len() as u32))?;
    for prop in properties {
        match prop {
            Property::Kind(x) => {
                write(&mut buffer, b"8BIM", b"kind", |buffer| {
                    buffer.write(&x.as_bytes())?;
                    Ok(())
                })?;
            }
            Property::Version {
                version,
                subversion,
                bugversion,
                stage,
                build,
            } => {
                write(&mut buffer, b"8BIM", b"vers", |buffer| {
                    buffer.write_u32::<ByteOrder>(pf_version(
                        version, subversion, bugversion, stage, build,
                    ))
                })?;
            }
            Property::Priority(x) => {
                write(&mut buffer, b"8BIM", b"prty", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            Property::Component((version, uuid)) => {
                write(&mut buffer, b"8BIM", b"cmpt", |buffer| {
                    buffer.write_u32::<ByteOrder>(version)?;
                    write_cstring(buffer, uuid)
                })?;
            }
            Property::RequiredHost(x) => {
                write(&mut buffer, b"8BIM", b"host", |buffer| {
                    buffer.write(&fourcc(x))?;
                    Ok(())
                })?;
            }
            Property::Name(x) => {
                write(&mut buffer, b"8BIM", b"name", |buffer| {
                    write_pstring(buffer, x)
                })?;
            }
            Property::Category(x) => {
                // PSHelpMenu = "**Help**";
                write(&mut buffer, b"8BIM", b"catg", |buffer| {
                    write_pstring(buffer, x)
                })?;
            }
            Property::Code68k((type_, x)) => {
                write(&mut buffer, b"8BIM", b"m68k", |buffer| {
                    buffer.write(&type_.as_bytes())?;
                    buffer.write_u16::<ByteOrder>(x)
                })?;
            }
            Property::Code68kFPU((type_, x)) => {
                write(&mut buffer, b"8BIM", b"68fp", |buffer| {
                    buffer.write(&type_.as_bytes())?;
                    buffer.write_u16::<ByteOrder>(x)
                })?;
            }
            Property::CodePowerPC((x, y, entry_point)) => {
                write(&mut buffer, b"8BIM", b"pwpc", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)?;
                    buffer.write_u32::<ByteOrder>(y)?;
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeCarbonPowerPC((x, y, entry_point)) => {
                write(&mut buffer, b"8BIM", b"ppcb", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)?;
                    buffer.write_u32::<ByteOrder>(y)?;
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeMachOPowerPC(entry_point) => {
                write(&mut buffer, b"8BIM", b"mach", |buffer| {
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeMacIntel32(entry_point) => {
                write(&mut buffer, b"8BIM", b"mi32", |buffer| {
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeMacIntel64(entry_point) => {
                write(&mut buffer, b"8BIM", b"mi64", |buffer| {
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeMacARM64(entry_point) => {
                write(&mut buffer, b"8BIM", b"ma64", |buffer| {
                    write_pstring(buffer, entry_point)
                })?;
            }
            Property::CodeWin32X86(entry_point) => {
                write(&mut buffer, b"8BIM", b"wx86", |buffer| {
                    write_cstring(buffer, entry_point)
                })?;
            }
            Property::CodeWin64X86(entry_point) => {
                write(&mut buffer, b"8BIM", b"8664", |buffer| {
                    write_cstring(buffer, entry_point)
                })?;
            }
            Property::SupportedModes(flags) => {
                write(&mut buffer, b"8BIM", b"mode", |buffer| {
                    buffer.write_u32::<ByteOrder>(flags.bits())
                })?;
            }
            Property::EnableInfo(condition) => {
                write(&mut buffer, b"8BIM", b"enbl", |buffer| {
                    write_cstring(buffer, condition)
                })?;
            }
            //-------------------------------------------------------------------
            // Photoshop Filter PiPL properties
            //-------------------------------------------------------------------
            Property::FilterCaseInfo(infos) => {
                write(&mut buffer, b"8BIM", b"fici", |buffer| {
                    for i in 0..7 {
                        if let Some(info) = infos.get(i) {
                            buffer.write_u8(info.in_handling as u8)?;
                            buffer.write_u8(info.out_handling as u8)?;
                            #[rustfmt::skip]
                            let flags = if info.copy_source_to_destination { 0 } else { 1 << 0 } |
                                        if info.works_with_blank_data      { 1 << 1 } else { 0 } |
                                        if info.filters_layer_masks        { 1 << 2 } else { 0 } |
                                        if info.write_outside_selection    { 1 << 3 } else { 0 };
                            buffer.write_u8(flags)?;
                            buffer.write_u8(0)?;
                        } else {
                            buffer.write_u32::<ByteOrder>(0)?;
                        }
                    }
                    Ok(())
                })?;
            }
            //-------------------------------------------------------------------
            // Photoshop Export PiPL properties
            //-------------------------------------------------------------------
            Property::ExportFlags {
                supports_transparency,
            } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"expf", |buffer| {
                    buffer.write_u8(if supports_transparency { 1 << 7 } else { 0 })?;
                    buffer.write_u24::<ByteOrder>(0)
                })?;
            }
            Property::FmtFileType((type_, creator)) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fmTC", |buffer| {
                    buffer.write(&fourcc(type_))?;
                    buffer.write(&fourcc(creator))?;
                    Ok(())
                })?;
            }
            // NOTE: If you specify you can READ type 'foo_', then you will never be called with a FilterFile for type 'foo_'.
            Property::ReadTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"RdTy", |buffer| {
                    for type_ in types {
                        buffer.write(&fourcc(type_.0))?;
                        buffer.write(&fourcc(type_.1))?;
                    }
                    Ok(())
                })?;
            }
            Property::WriteTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"WrTy", |buffer| {
                    for type_ in types {
                        buffer.write(&fourcc(type_.0))?;
                        buffer.write(&fourcc(type_.1))?;
                    }
                    Ok(())
                })?;
            }
            // NOTE: If you specify you want to filter type 'foo_' AND you specify you can read type 'foo_', you will never get a filter call.
            Property::FilteredTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fftT", |buffer| {
                    for type_ in types {
                        buffer.write(&fourcc(type_.0))?;
                        buffer.write(&fourcc(type_.1))?;
                    }
                    Ok(())
                })?;
            }
            // Macintosh plug-ins can use Windows file extensions to determine read/write/parseability.
            // NOTE: If you specify you READ extension '.foo' then you won't be called to Filter that type.
            Property::ReadExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"RdEx", |buffer| {
                    for &ext in exts {
                        buffer.write(&fourcc(ext))?;
                    }
                    Ok(())
                })?;
            }
            Property::WriteExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"WrEx", |buffer| {
                    for &ext in exts {
                        buffer.write(&fourcc(ext))?;
                    }
                    Ok(())
                })?;
            }
            // NOTE: If you specify you want to filter extension '.foo' AND you specify you can read extension '.foo', you will never get a filter call.
            Property::FilteredExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fftE", |buffer| {
                    for &ext in exts {
                        buffer.write(&fourcc(ext))?;
                    }
                    Ok(())
                })?;
            }
            Property::FormatFlags {
                can_read,
                can_write,
                can_write_if_read,
                saves_image_resources,
            } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fmtf", |buffer| {
                    #[rustfmt::skip]
                    let flags = if can_write_if_read     { 1 << 3 } else { 0 } |
                                if can_write             { 1 << 4 } else { 0 } |
                                if can_read              { 1 << 5 } else { 0 } |
                                if saves_image_resources { 1 << 6 } else { 0 };
                    buffer.write_u8(flags)?;
                    buffer.write_u24::<ByteOrder>(0)
                })?;
            }
            Property::FormatMaxSize { width, height } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"mxsz", |buffer| {
                    buffer.write_u16::<ByteOrder>(width)?;
                    buffer.write_u16::<ByteOrder>(height)
                })?;
            }
            Property::FormatMaxChannels(max_channels) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"mxch", |buffer| {
                    for ch in max_channels {
                        buffer.write_u16::<ByteOrder>(*ch)?;
                    }
                    for _ in 0..padding_4(max_channels.len() as u32 * 2) as usize {
                        buffer.write_u8(0)?;
                    }
                    Ok(())
                })?;
            }
            //-------------------------------------------------------------------
            // Photoshop Parser PiPL properties
            //-------------------------------------------------------------------
            // NOTE: If you specify you want to filter type 'foo_' and you specify you can parse type 'foo_', you will never get a filter call.
            Property::ParsableTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psTY", |buffer| {
                    for type_ in types {
                        buffer.write(&fourcc(type_.0))?;
                        buffer.write(&fourcc(type_.1))?;
                    }
                    Ok(())
                })?;
            }
            Property::ParsableClipboardTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psCB", |buffer| {
                    for &type_ in types {
                        buffer.write(&fourcc(type_))?;
                    }
                    Ok(())
                })?;
            }
            // NOTE: If you want to filter type 'foo_' and you specify you can parse type 'foo_', you will never get a filter call.
            Property::FilteredParsableTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psTy", |buffer| {
                    for type_ in types {
                        buffer.write(&fourcc(type_.0))?;
                        buffer.write(&fourcc(type_.1))?;
                    }
                    Ok(())
                })?;
            }
            // Macintosh plug-ins can use Windows file extensions to determine read/write/parseability.
            // NOTE: If you want to filter extension '.foo' and you specify you can parse extension '.foo', you will never get a filter call.
            Property::ParsableExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psEX", |buffer| {
                    for &ext in exts {
                        buffer.write(&fourcc(ext))?;
                    }
                    Ok(())
                })?;
            }
            Property::FilteredParsableExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psEx", |buffer| {
                    for &ext in exts {
                        buffer.write(&fourcc(ext))?;
                    }
                    Ok(())
                })?;
            }
            Property::PickerID(id) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"pnme", |buffer| {
                    write_pstring(buffer, id)
                })?;
            }
            //-------------------------------------------------------------------
            // Photoshop Actions/Scripting PiPL properties (Photoshop 4.0 and later)
            //-------------------------------------------------------------------
            Property::HasTerminology {
                class_id,
                event_id,
                dictionary_resource_id,
                unique_scope_string,
            } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"hstm", |buffer| {
                    buffer.write_u32::<ByteOrder>(0)?; // Version.
                    buffer.write_u32::<ByteOrder>(class_id)?; // Class ID, always required.  Can be Suite ID.
                    buffer.write_u32::<ByteOrder>(event_id)?; // Event ID, or typeNULL if not Filter/Color Picker/Selection.
                    buffer.write_u16::<ByteOrder>(dictionary_resource_id)?; // Dictionary ('AETE') resource ID.
                    write_cstring(buffer, unique_scope_string)
                    // TODO: Padding?
                })?;
            }
            // If this property is present, then its on. No parameters are required:
            Property::Persistent => {
                write(&mut buffer, b"8BIM", b"prst", |buffer| {
                    buffer.write_u32::<ByteOrder>(1)
                })?;
            }
            //-------------------------------------------------------------------
            // After Effects and Premiere specific PiPL properties
            //-------------------------------------------------------------------
            Property::AE_PiPL_Version { major, minor } => {
                write(&mut buffer, b"8BIM", b"ePVR", |buffer| {
                    buffer.write_u16::<ByteOrder>(major)?;
                    buffer.write_u16::<ByteOrder>(minor)
                })?;
            }
            Property::AE_Effect_Spec_Version { major, minor } => {
                write(&mut buffer, b"8BIM", b"eSVR", |buffer| {
                    buffer.write_u16::<ByteOrder>(major)?;
                    buffer.write_u16::<ByteOrder>(minor)
                })?;
            }
            Property::AE_Effect_Version {
                version,
                subversion,
                bugversion,
                stage,
                build,
            } => {
                write(&mut buffer, b"8BIM", b"eVER", |buffer| {
                    buffer.write_u32::<ByteOrder>(pf_version(
                        version, subversion, bugversion, stage, build,
                    ))
                })?;
            }
            Property::AE_Effect_Match_Name(name) => {
                write(&mut buffer, b"8BIM", b"eMNA", |buffer| {
                    write_pstring(buffer, name)
                })?;
            }
            Property::AE_Effect_Support_URL(name) => {
                write(&mut buffer, b"8BIM", b"eURL", |buffer| {
                    write_pstring(buffer, name)
                })?;
            }
            Property::AE_Effect_Info_Flags(x) => {
                write(&mut buffer, b"8BIM", b"eINF", |buffer| {
                    // This shouldn't make a difference, but let's keep it consistent with the native tools
                    if cfg!(target_os = "windows") {
                        buffer.write_u32::<ByteOrder>(x)
                    } else {
                        buffer.write_u16::<ByteOrder>(x as u16)
                    }
                })?;
            }
            Property::AE_Effect_Global_OutFlags(x) => {
                write(&mut buffer, b"8BIM", b"eGLO", |buffer| {
                    buffer.write_u32::<ByteOrder>(x.bits())
                })?;
            }
            Property::AE_Effect_Global_OutFlags_2(x) => {
                write(&mut buffer, b"8BIM", b"eGL2", |buffer| {
                    buffer.write_u32::<ByteOrder>(x.bits())
                })?;
            }
            Property::AE_Reserved(x) => {
                write(&mut buffer, b"8BIM", b"aeRD", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            Property::AE_Reserved_Info(x) => {
                write(&mut buffer, b"8BIM", b"aeFL", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            //-------------------------------------------------------------------
            // After Effects Image Format Extension PiPL properties
            //-------------------------------------------------------------------
            Property::AE_ImageFormat_Extension_Info {
                major_version,
                minor_version,
                has_options,
                sequential_only,
                must_interact,
                has_interact_put,
                has_interact_get,
                has_time,
                has_video,
                still,
                has_file,
                output,
                input,
                signature,
            } => {
                write(&mut buffer, b"8BIM", b"FXMF", |buffer| {
                    buffer.write_u16::<ByteOrder>(major_version)?;
                    buffer.write_u16::<ByteOrder>(minor_version)?;
                    #[rustfmt::skip]
                    let flags: u32 = if input            { 1u32 << 0 } else { 0 } |
                                     if output           { 1u32 << 1 } else { 0 } |
                                     if has_file         { 1u32 << 2 } else { 0 } |
                                     if still            { 1u32 << 3 } else { 0 } |
                                     if has_video        { 1u32 << 4 } else { 0 } |
                                     if !has_time        { 1u32 << 5 } else { 0 } |
                                     if has_interact_get { 1u32 << 6 } else { 0 } |
                                     if has_interact_put { 1u32 << 7 } else { 0 } |
                                     if must_interact    { 1u32 << 8 } else { 0 } |
                                     if !sequential_only { 1u32 << 9 } else { 0 } |
                                     if !has_options     { 1u32 << 10 } else { 0 };
                    buffer.write_u32::<ByteOrder>(flags)?;
                    buffer.write_u32::<ByteOrder>(0)?; // Reserved.
                    buffer.write(&fourcc(&signature))?;
                    Ok(())
                })?;
            }
            //-------------------------------------------------------------------
            // After Effects and Premiere ANIM PiPL properties
            //-------------------------------------------------------------------
            Property::ANIM_FilterInfo {
                spec_version_major,
                spec_version_minor,
                filter_params_version,
                unity_pixel_aspec_tratio,
                any_pixel_aspect_ratio,
                drive_me,
                needs_dialog,
                params_pointer,
                params_handle,
                params_mac_handle,
                dialog_in_render,
                params_in_globals,
                bg_animatable,
                fg_animatable,
                geometric,
                randomness,
                number_of_parameters,
                match_name,
            } => {
                write(&mut buffer, b"8BIM", b"aFLT", |buffer| {
                    buffer.write_u32::<ByteOrder>(spec_version_major)?;
                    buffer.write_u32::<ByteOrder>(spec_version_minor)?;
                    buffer.write_u32::<ByteOrder>(filter_params_version)?;
                    #[rustfmt::skip]
                    let flags: u32 = if randomness               { 1u32 << 0 } else { 0 } |  // ANIM_FF_HAS_RANDOMNESS (AE only)
                                     if !geometric               { 1u32 << 1 } else { 0 } |  // ANIM_FF_NON_GEOMETRIC (AE only)
                                     if fg_animatable            { 1u32 << 2 } else { 0 } |  // ANIM_FF_FG_ANIMATABLE (AE only)
                                     if bg_animatable            { 1u32 << 3 } else { 0 } |  // ANIM_FF_BG_ANIMATABLE (AE only)
                                     if params_in_globals        { 1u32 << 4 } else { 0 } |  // ANIM_FF_PARAMS_IN_GLOBALS (AE only)
                                     if dialog_in_render         { 1u32 << 5 } else { 0 } |  // ANIM_FF_DIALOG_IN_RENDER (AE only)
                                     if params_mac_handle        { 1u32 << 6 } else { 0 } |  // ANIM_FF_PARAMS_ARE_MAC_HANDLE (AE only)
                                     if params_handle            { 1u32 << 7 } else { 0 } |  // ANIM_FF_PARAMS_ARE_HANDLE (AE only)
                                     if params_pointer           { 1u32 << 8 } else { 0 } |  // ANIM_FF_PARAMS_ARE PTR (AE only)
                                     if !needs_dialog            { 1u32 << 9 } else { 0 } |  // ANIM_FF_DOESNT_NEED_DLOG (AE only)
                                     if !drive_me                { 1u32 << 10 } else { 0 } | // ANIM_FF_DONT_DRIVE_ME (AE only)
                                     if false                    { 1u32 << 11 } else { 0 } | // ANIM_FF_RESERVED0 (AE only)
                                     if false                    { 1u32 << 12 } else { 0 } | // ANIM_FF_RESERVED1 (AE only)
                                     if false                    { 1u32 << 13 } else { 0 } | // ANIM_FF_RESERVED2 (spare)
                                     if false                    { 1u32 << 14 } else { 0 } | // ANIM_FF_RESERVED3 (spare)
                                     if false                    { 1u32 << 15 } else { 0 } | // ANIM_FF_RESERVED4 (spare)
                                     if any_pixel_aspect_ratio   { 1u32 << 16 } else { 0 } | // ANIM_FF_ANY_PAR
                                     if unity_pixel_aspec_tratio { 1u32 << 17 } else { 0 }; // ANIM_FF_UNITY_PAR

                    buffer.write_u32::<ByteOrder>(flags)?;
                    buffer.write_u32::<ByteOrder>(number_of_parameters)?;

                    let match_name_buf = match_name.as_bytes();
                    assert!(match_name_buf.len() < 32);
                    buffer.extend(match_name_buf);
                    for _ in 0..(32 - match_name_buf.len()) {
                        buffer.push(0);
                    }

                    buffer.write_u32::<ByteOrder>(0)?; // Operates in place - not currently implemented
                    buffer.write_u32::<ByteOrder>(0)?; // reserved
                    buffer.write_u32::<ByteOrder>(0)?; // reserved
                    buffer.write_u32::<ByteOrder>(0) // reserved
                })?;
            }
            Property::ANIM_ParamAtom {
                external_name,
                match_id,
                data_type,
                ui_type,
                valid_min,
                valid_max,
                ui_min,
                ui_max,
                scale_ui_range,
                animate_param,
                restrict_bounds,
                space_is_relative,
                res_dependant,
                property_size,
            } => {
                write(&mut buffer, b"8BIM", b"aPAR", |buffer| {
                    // TODO: MUST SPECIFY THE FIRST 0 u32 - buffer[4..8]

                    let external_name = external_name.as_bytes();
                    assert!(external_name.len() < 32);
                    buffer.extend(external_name);
                    for _ in 0..(32 - external_name.len()) {
                        buffer.push(0);
                    }
                    buffer.write_u32::<ByteOrder>(match_id)?;
                    buffer.write_u32::<ByteOrder>(data_type as u32)?; // obsolete, don't use OPAQUE with Premiere
                    buffer.write_u32::<ByteOrder>(ui_type as u32)?; // UI types are only used by AE
                    buffer.write_f64::<ByteOrder>(valid_min)?; // used for UI type slider - AE only
                    buffer.write_f64::<ByteOrder>(valid_max)?; // used for UI type slider - AE only
                    buffer.write_f64::<ByteOrder>(ui_min)?; // used for UI type slider - AE only
                    buffer.write_f64::<ByteOrder>(ui_max)?; // used for UI type slider - AE only
                    #[rustfmt::skip]
                    let flags: u32 = if res_dependant     { 1u32 << 0 } else { 0 } |
                                     if space_is_relative { 1u32 << 1 } else { 0 } |
                                     if restrict_bounds   { 1u32 << 2 } else { 0 } |
                                     if animate_param     { 1u32 << 3 } else { 0 } |
                                     if scale_ui_range    { 1u32 << 4 } else { 0 };
                    buffer.write_u32::<ByteOrder>(flags)?;

                    buffer.write_u32::<ByteOrder>(property_size)?; // size of property described in bytes (short = 2, long = 4, etc.)

                    buffer.write_u32::<ByteOrder>(0)?; // reserved0
                    buffer.write_u32::<ByteOrder>(0)?; // reserved1
                    buffer.write_u32::<ByteOrder>(0)?; // reserved2
                    buffer.write_u32::<ByteOrder>(0) // reserved3
                })?;
            }
            //-------------------------------------------------------------------
            // Premiere Transition Effect PiPL properties
            //-------------------------------------------------------------------
            Property::Pr_Effect_Info {
                version,
                valid_corners_mask,
                initial_corners,
                exclusive_dialog,
                needs_callbacks_at_setup,
                direct_comp_data,
                want_initial_setup_call,
                treat_as_transition,
                has_custom_dialog,
                highlight_opposite_corners,
                exclusive,
                reversible,
                have_edges,
                have_start_point,
                have_end_point,
                more_flags,
            } => {
                write(&mut buffer, b"PrMr", b"pOPT", |buffer| {
                    buffer.write_u32::<ByteOrder>(version)?;

                    // Valid corners mask and initial corners (lsb to msb):
                    // bitTop | bitRight | bitBottom | bitLeft | bitUpperRight | bitLowerRight | bitLowerLeft | bitUpperLeft
                    buffer.write_u8(valid_corners_mask as u8)?;
                    buffer.write_u8(initial_corners as u8)?;
                    #[rustfmt::skip]
                    let flags: u8 = if highlight_opposite_corners { 1 << 0 } else { 0 } |
                                    if has_custom_dialog          { 1 << 1 } else { 0 } |
                                    if !treat_as_transition       { 1 << 2 } else { 0 } |
                                    if !want_initial_setup_call   { 1 << 3 } else { 0 } |
                                    if direct_comp_data           { 1 << 4 } else { 0 } |
                                    if needs_callbacks_at_setup   { 1 << 5 } else { 0 } |
                                    if exclusive_dialog           { 1 << 6 } else { 0 };
                    buffer.write_u8(flags)?;
                    buffer.write_u8(exclusive as u8)?;
                    buffer.write_u8(reversible as u8)?;
                    buffer.write_u8(have_edges as u8)?;
                    buffer.write_u8(have_start_point as u8)?;
                    buffer.write_u8(have_end_point as u8)?;

                    buffer.write_u32::<ByteOrder>(more_flags)
                })?;
            }
            // The text description of the transition.
            Property::Pr_Effect_Description(desc) => {
                write(&mut buffer, b"PrMr", b"TEXT", |buffer| {
                    write_pstring(buffer, desc)
                })?;
            }
            //-------------------------------------------------------------------
            // Illustrator/SweetPea PiPL properties
            //-------------------------------------------------------------------
            Property::InterfaceVersion(x) => {
                write(&mut buffer, b"ADBE", b"ivrs", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            Property::AdapterVersion(x) => {
                write(&mut buffer, b"ADBE", b"adpt", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            Property::SP_STSP(x) => {
                write(&mut buffer, b"ADBE", b"STSP", |buffer| {
                    buffer.write_u32::<ByteOrder>(x)
                })?;
            }
            Property::InternalName(name) => {
                write(&mut buffer, b"ADBE", b"pinm", |buffer| {
                    write_cstring(buffer, name)
                })?;
            }
            Property::Imports(imports) => {
                write(&mut buffer, b"ADBE", b"impt", |buffer| {
                    buffer.write_u32::<ByteOrder>(imports.len() as u32)?;
                    for import in imports {
                        let len = buffer.len();

                        buffer.write_u32::<ByteOrder>(0)?;
                        write_cstring(buffer, import.0)?;
                        buffer.write_u32::<ByteOrder>(import.1)?; // Suite version.

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len + 4].clone_from_slice(&u32_bytes(new_len));
                    }
                    Ok(())
                })?;
            }
            Property::Exports(exports) => {
                write(&mut buffer, b"ADBE", b"expt", |buffer| {
                    buffer.write_u32::<ByteOrder>(exports.len() as u32)?;
                    for export in exports {
                        let len = buffer.len();

                        buffer.write_u32::<ByteOrder>(0)?;
                        write_cstring(buffer, export.0)?;
                        buffer.write_u32::<ByteOrder>(export.1)?; // Suite version.

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len + 4].clone_from_slice(&u32_bytes(new_len));
                    }
                    Ok(())
                })?;
            }
            Property::Description(desc) => {
                write(&mut buffer, b"ADBE", b"desc", |buffer| {
                    write_cstring(buffer, desc)
                })?;
            }
            Property::Keywords(keywords) => {
                write(&mut buffer, b"ADBE", b"keyw", |buffer| {
                    buffer.write_u32::<ByteOrder>(keywords.len() as u32)?;
                    for keyword in keywords {
                        let len = buffer.len();

                        buffer.write_u32::<ByteOrder>(0)?;
                        write_cstring(buffer, keyword)?;

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len + 4].clone_from_slice(&u32_bytes(new_len));
                    }
                    Ok(())
                })?;
            }
            Property::Title(title) => {
                write(&mut buffer, b"ADBE", b"titl", |buffer| {
                    write_cstring(buffer, title)
                })?;
            }
            Property::Messages {
                startup_required,
                purge_cache,
                shutdown_required,
                accept_property,
            } => {
                write(&mut buffer, b"ADBE", b"AcpM", |buffer| {
                    #[rustfmt::skip]
                    let flags: u32 = if accept_property   { 1u32 << 0 } else { 0 } |
                                     if shutdown_required { 1u32 << 1 } else { 0 } | // Default is to give shutdown msg.
                                     if purge_cache       { 1u32 << 2 } else { 0 } |
                                     if startup_required  { 1u32 << 3 } else { 0 };
                    buffer.write_u32::<ByteOrder>(flags)
                })?;
            }
            //-------------------------------------------------------------------
            // PhotoDeluxe PiPL properties
            //-------------------------------------------------------------------
            Property::ButtonIcon {
                version,
                mac_icon_type,
                win_icon_type,
                resource_id,
                icon_name,
            } => {
                write(&mut buffer, b"8BIM", b"btni", |buffer| {
                    buffer.write_u32::<ByteOrder>(version)?; // version
                    match mac_icon_type {
                        ButtonIconType::None => buffer.write_u32::<ByteOrder>(0)?,
                        ButtonIconType::MacCICN => buffer.write_u32::<ByteOrder>(1)?,
                        _ => {}
                    }
                    match win_icon_type {
                        ButtonIconType::None => buffer.write_u32::<ByteOrder>(0)?,
                        ButtonIconType::WindowsICON => buffer.write_u32::<ByteOrder>(1)?,
                        _ => {}
                    }
                    buffer.write_u32::<ByteOrder>(resource_id)?;
                    write_cstring(buffer, icon_name)
                })?;
            }
            //-------------------------------------------------------------------
            // PhotoDeluxe extension to Import plug-in PiPL properties
            //-------------------------------------------------------------------
            Property::Class { version, class } => {
                write(&mut buffer, b"8BIM", b"clas", |buffer| {
                    buffer.write_u32::<ByteOrder>(version)?; // version
                    buffer.write_u32::<ByteOrder>(class as u32)
                })?;
            }
            Property::PreviewFile { version, filename } => {
                write(&mut buffer, b"8BIM", b"prvw", |buffer| {
                    buffer.write_u32::<ByteOrder>(version)?; // version
                    write_cstring(buffer, filename)
                })?;
            }
        }
    }

    Ok(buffer)
}

pub fn plugin_build(properties: Vec<Property>) {
    for prop in properties.iter() {
        match prop {
            Property::Kind(x) => {
                println!(
                    "cargo:rustc-env=PIPL_KIND={}",
                    u32::from_le_bytes(x.as_bytes())
                );
            }
            Property::Name(x) => {
                println!("cargo:rustc-env=PIPL_NAME={x}");
            }
            Property::Category(x) => {
                println!("cargo:rustc-env=PIPL_CATEGORY={x}");
            }
            Property::AE_Effect_Match_Name(x) => {
                println!("cargo:rustc-env=PIPL_MATCH_NAME={x}");
            }
            Property::AE_Effect_Support_URL(x) => {
                println!("cargo:rustc-env=PIPL_SUPPORT_URL={x}");
            }
            Property::CodeWin64X86(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            }
            Property::CodeMacIntel64(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            }
            Property::CodeMacARM64(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            }
            Property::AE_Effect_Spec_Version { major, minor } => {
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MAJOR={major}");
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MINOR={minor}");
            }
            Property::AE_Reserved_Info(x) => {
                println!("cargo:rustc-env=PIPL_AE_RESERVED={}", x);
            }
            Property::AE_Effect_Version {
                version,
                subversion,
                bugversion,
                stage,
                build,
            } => {
                println!(
                    "cargo:rustc-env=PIPL_VERSION={}",
                    pf_version(*version, *subversion, *bugversion, *stage, *build)
                );
            }
            Property::AE_Effect_Global_OutFlags(x) => {
                if x.contains(OutFlags::IDoDialog) {
                    println!("cargo:rustc-cfg=does_dialog");
                }
                if x.contains(OutFlags::IUseAudio) || x.contains(OutFlags::AudioEffectToo) || x.contains(OutFlags::AudioEffectOnly) {
                    println!("cargo:rustc-cfg=uses_audio");
                }
                if x.contains(OutFlags::SendUpdateParamsUI) {
                    println!("cargo:rustc-cfg=sends_update_params_ui");
                }
                println!("cargo:rustc-env=PIPL_OUTFLAGS={}", x.bits());
            }
            Property::AE_Effect_Global_OutFlags_2(x) => {
                if x.contains(OutFlags2::SupportsGpuRenderF32) {
                    println!("cargo:rustc-cfg=gpu_render");
                }
                if x.contains(OutFlags2::SupportsSmartRender) {
                    println!("cargo:rustc-cfg=smart_render");
                }
                if x.contains(OutFlags2::SupportsThreadedRendering) {
                    println!("cargo:rustc-cfg=threaded_rendering");
                }
                println!("cargo:rustc-env=PIPL_OUTFLAGS2={}", x.bits());
            }
            _ => {}
        }
    }
    let pipl = build_pipl(properties).unwrap();

    resource::produce_resource(
        &pipl,
        Some(&format!(
            "{}/../../../{}.rsrc",
            std::env::var("OUT_DIR").unwrap(),
            std::env::var("CARGO_PKG_NAME").unwrap()
        )),
    );
}
