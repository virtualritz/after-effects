
use crate::*;
use ae_sys::*;

define_suite!(
    /// # Source Settings = Effect + Importer
    /// Source Settings for clips can now be implemented using effects that are tied to importers.
    /// This has the advantage of providing settings in the Effect Controls panel, rather than in a modal dialog.
    /// Editors can adjust Source Settings for multiple clips this way. These effects are used for the DPX source settings, CinemaDNG, etc.
    ///
    /// To implement this, an importer should set ``imImportInfoRec.hasSourceSettingsEffect`` to true.
    /// Then in imFileInfoRec8, it should set sourceSettingsMatchName to the match name of the effect to be used for the Source Settings.
    ///
    /// On the effects side, a new PF Source Settings Suite has been added to PrSDKAESupport.h, for effects using the After Effects API.
    /// This is how an effect registers a function to handle the Source Settings command.
    ///
    /// A source settings effect is used primarily for the parameter UI and management.
    /// A source settings effect doesn't provide the actual frames.
    /// In fact, the effect isn't even called with *PF_Cmd_RENDER*.
    /// The frames come directly from the importer, which provides frames based on the settings as passed to the importer via prefs data.
    ///
    /// When a clip is first imported, the effect is called with *PF_Cmd_SEQUENCE_SETUP*.
    /// It should call PerformSourceSettingsCommand() in the Source Settings Suite, to initialize the prefs.
    /// This causes the importer to get called with *imPerformSourceSettingsCommand*, where it can read the file and set the default prefs.
    /// param1 of that function is imFileAccessRec8*, and param2 is imSourceSettingsCommandRec*.
    ///
    /// When the source settings effect parameters are changed, the effect gets called with *PF_Cmd_TRANSLATE_PARAMS_TO_PREFS*. The function signature is:
    ///
    /// ```ignore
    ///   PF_Err TranslateParamsToPrefs(
    ///     PF_InData*                      in_data,
    ///     PF_OutData*                     out_data,
    ///     PF_ParamDef*                    params[],
    ///     PF_TranslateParamsToPrefsExtra  *extra)
    /// ```
    /// With the new prefs, the importer will be sent *imOpenFile8, imGetInfo8, imGetIndPixelFormat, imGetPreferredFrameSize, imGetSourceVideo*, etc.
    ///
    /// imSourceSettingsCommandRec and PF Source Settings Suite allow the effect to communicate directly with the importer, so that it can initialize its parameters properly,
    /// based on the source media. In the DPX source settings effect, for example, in *PF_Cmd_SEQUENCE_SETUP*, it calls PF_SourceSettingsSuite->PerformSourceSettingsCommand(),
    /// which calls through to the importer with the selector *imPerformSourceSettingsCommand*.
    /// Here, the importer opens the media, looks at the header and initializes the prefs based on the media. For
    ///
    /// DPX, the initial parameters and default prefs are based on the bit depth of the video.
    /// These default prefs are passed back to the effect, which sets the initial param values and stashes a copy of them in sequence_data to use again for future calls to *PF_Cmd_SEQUENCE_RESETUP*.
    SourceSettingsSuite,
    PF_SourceSettingsSuite,
    kPFSourceSettingsSuite,
    kPFSourceSettingsSuiteVersion
);

impl SourceSettingsSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn perform_source_settings_command(&self, effect_ref: impl AsPtr<PF_ProgPtr>, command_struct: *mut std::ffi::c_void, data_size: usize) -> Result<(), Error> {
        call_suite_fn!(self, PerformSourceSettingsCommand, effect_ref.as_ptr(), command_struct, data_size as _)
    }

    // pub fn set_is_source_settings_effect(&self, effect_ref: impl AsPtr<PF_ProgPtr>, value: bool) -> Result<(), Error> {
    //     call_suite_fn!(self, SetIsSourceSettingsEffect, effect_ref.as_ptr(), value as _)
    // }
}
