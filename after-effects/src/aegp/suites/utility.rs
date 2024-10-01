use crate::*;
use crate::aegp::*;

pub struct ErrReportState((ae_sys::AEGP_ErrReportState, bool));
impl Drop for ErrReportState {
    fn drop(&mut self) {
        if let Ok(util) = UtilitySuite::new() {
            let _ = util.end_quiet_errors(self.0.0, self.0.1);
        }
    }
}
define_suite!(
    /// The Utility suite supplies error message handling, AEGP version checking and access to the undo stack.
    ///
    /// Everything you need to keep After Effects and your plug-in tidy.
    UtilitySuite,
    AEGP_UtilitySuite6,
    kAEGPUtilitySuite,
    kAEGPUtilitySuiteVersion6
);

impl UtilitySuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Displays dialog with name of the AEGP followed by the string passed.
    pub fn report_info(&self, plugin_id: PluginId, info_string: &str) -> Result<(), Error> {
        let info_string = CString::new(info_string).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_ReportInfo, plugin_id, info_string.as_ptr())
    }

    /// New in CC. Displays dialog with name of the AEGP followed by the unicode string passed.
    pub fn report_info_unicode(&self, plugin_id: PluginId, info_string: &str) -> Result<(), Error> {
        let info_string = U16CString::from_str(info_string).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_ReportInfoUnicode, plugin_id, info_string.as_ptr())
    }

    /// Silences errors until the `ErrReportState` is dropped.
    pub fn start_quiet_errors(&self, report_quieted_errors_on_drop: bool) -> Result<ErrReportState, Error> {
        Ok(ErrReportState((call_suite_fn_single!(self, AEGP_StartQuietErrors -> ae_sys::AEGP_ErrReportState)?, report_quieted_errors_on_drop)))
    }

    /// Re-enables errors.
    pub fn end_quiet_errors(&self, err_state: ae_sys::AEGP_ErrReportState, report_quieted_errors: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_EndQuietErrors, report_quieted_errors as _, &err_state as *const _ as *mut _)
    }

    /// Add action(s) to the undo queue. The user may undo any actions between this and [`Self::end_undo_group`].
    /// The `undo_name` will appear in the edit menu.
    pub fn start_undo_group(&self, undo_name: &str) -> Result<(), Error> {
        let undo_name = CString::new(undo_name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_StartUndoGroup, undo_name.as_ptr())
    }

    /// Ends the undo list.
    pub fn end_undo_group(&self) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_EndUndoGroup,)
    }

    /// Returns an [`PluginId`], which effect plug-ins can then use in calls to many functions throughout the AEGP API.
    /// Effects should only call this function once, during [`Command::GlobalSetup`], and save the [`PluginId`] for later use.
    /// The first parameter can be any value, and the second parameter should be the plug-in's match name.
    pub fn register_with_aegp(&self, global_refcon: Option<*mut std::ffi::c_void>, plugin_name: &str) -> Result<PluginId, Error> {
        let plugin_name = CString::new(plugin_name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn_single!(self, AEGP_RegisterWithAEGP -> ae_sys::AEGP_PluginID, global_refcon.unwrap_or(std::ptr::null_mut()) as _, plugin_name.as_ptr())
    }

    /// Retrieves After Effects' HWND; useful when displaying your own dialog on Windows.
    /// If you don't use After Effects' HWND, your modal dialog will not prevent interaction with the windows behind, and pain will ensue.
    pub fn main_hwnd(&self) -> Result<*mut std::ffi::c_void, Error> {
        let mut hwnd = std::ptr::null_mut();
        call_suite_fn!(self, AEGP_GetMainHWND, &mut hwnd as *mut _ as *mut _)?;
        Ok(hwnd)
    }

    /// Toggles whether or not floating palettes are displayed.
    ///
    /// Use this with care; users get twitchy when you unexpectedly change the UI on them.
    pub fn show_hide_all_floaters(&self, include_tool_pal: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ShowHideAllFloaters, include_tool_pal as _)
    }

    /// Retrieves the foreground color from the paint palette.
    pub fn paint_palette_foreground_color(&self) -> Result<pf::PixelF64, Error> {
        let color = call_suite_fn_single!(self, AEGP_PaintPalGetForeColor -> ae_sys::AEGP_ColorVal)?;
        Ok(unsafe { std::mem::transmute(color) })
    }

    /// Retrieves the background color from the paint palette.
    pub fn paint_palette_background_color(&self) -> Result<pf::PixelF64, Error> {
        let color = call_suite_fn_single!(self, AEGP_PaintPalGetBackColor -> ae_sys::AEGP_ColorVal)?;
        Ok(unsafe { std::mem::transmute(color) })
    }

    /// Sets the foreground color in the paint palette.
    pub fn paint_palette_set_foreground_color(&self, color: pf::PixelF64) -> Result<(), Error> {
        let color = unsafe { std::mem::transmute(color) };
        call_suite_fn!(self, AEGP_PaintPalSetForeColor, &color as *const _)
    }

    /// Sets the background color in the paint palette.
    pub fn paint_palette_set_background_color(&self, color: pf::PixelF64) -> Result<(), Error> {
        let color = unsafe { std::mem::transmute(color) };
        call_suite_fn!(self, AEGP_PaintPalSetBackColor, &color as *const _)
    }

    /// Retrieves the fill color from the character palette.
    ///
    /// Returns tuple containing (is_defined, color)
    pub fn character_palette_fill_color(&self) -> Result<(bool, pf::PixelF64), Error> {
        let (is_defined, color) = call_suite_fn_double!(self, AEGP_CharPalGetFillColor -> ae_sys::A_Boolean, ae_sys::AEGP_ColorVal)?;
        Ok((is_defined != 0, unsafe { std::mem::transmute(color) }))
    }

    /// Retrieves the stroke color from the character palette.
    ///
    /// Returns tuple containing (is_defined, color)
    pub fn character_palette_stroke_color(&self) -> Result<(bool, pf::PixelF64), Error> {
        let (is_defined, color) = call_suite_fn_double!(self, AEGP_CharPalGetStrokeColor -> ae_sys::A_Boolean, ae_sys::AEGP_ColorVal)?;
        Ok((is_defined != 0, unsafe { std::mem::transmute(color) }))
    }

    /// Sets the fill color in the character palette.
    pub fn character_palette_set_fill_color(&self, color: pf::PixelF64) -> Result<(), Error> {
        let color = unsafe { std::mem::transmute(color) };
        call_suite_fn!(self, AEGP_CharPalSetFillColor, &color as *const _)
    }

    /// Sets the stroke color in the character palette.
    pub fn character_palette_set_stroke_color(&self, color: pf::PixelF64) -> Result<(), Error> {
        let color = unsafe { std::mem::transmute(color) };
        call_suite_fn!(self, AEGP_CharPalSetStrokeColor, &color as *const _)
    }

    /// Returns whether or not the fill color is frontmost. If it isn't, the stroke color is frontmost.
    pub fn character_palette_is_fill_color_ui_frontmost(&self) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_CharPalIsFillColorUIFrontmost -> ae_sys::A_Boolean)? != 0)
    }

    /// Returns an [`Ratio`] interpretation of the given `f64`. Useful for horizontal scale factor interpretation.
    pub fn convert_fp_long_to_hsf_ratio(&self, number: f64) -> Result<Ratio, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ConvertFpLongToHSFRatio -> ae_sys::A_Ratio, number)?.into())
    }

    /// Returns an `f64` interpretation of the given [`Ratio`]. Useful for horizontal scale factor interpretation.
    pub fn convert_hsf_ratio_to_fp_long(&self, ratio: Ratio) -> Result<f64, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ConvertHSFRatioToFpLong -> f64, ratio.into())?)
    }

    /// This routine is safe to call from threads other than the main thread.
    /// It is asynchronous and will return before the idle handler is called.
    ///
    /// The suite functions to get this function pointer are not thread safe; save it off in the main thread for use by the child thread.
    pub fn cause_idle_routines_to_be_called(&self) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_CauseIdleRoutinesToBeCalled,)
    }

    /// Returns whether After Effects is running without a user interface.
    pub fn suppress_interactive_ui(&self) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetSuppressInteractiveUI -> ae_sys::A_Boolean)? != 0)
    }

    /// Sends a string to the OS console.
    pub fn write_to_os_console(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_WriteToOSConsole, text.as_ptr())
    }

    /// Writes a message to the debug log, or to the OS command line if After Effects was launched with the "-debug" option.
    pub fn write_to_debug_log(&self, subsystem: &str, event_type: &str, info: &str) -> Result<(), Error> {
        let subsystem = CString::new(subsystem).map_err(|_| Error::InvalidParms)?;
        let event_type = CString::new(event_type).map_err(|_| Error::InvalidParms)?;
        let info = CString::new(info).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_WriteToDebugLog, subsystem.as_ptr(), event_type.as_ptr(), info.as_ptr())
    }

    /// Retrieves the last error message displayed to the user, and its associated error number.
    /// Pass in the size of the character buffer to be returned.
    pub fn last_error_message(&self) -> Result<(String, Error), Error> {
        let mut buffer = vec![0u8; 512];
        let err = call_suite_fn_single!(self, AEGP_GetLastErrorMessage -> ae_sys::A_Err, buffer.len() as _, buffer.as_mut_ptr() as *mut _)?;
        Ok((String::from_utf8_lossy(&buffer).to_string(), Error::from(err)))
    }

    /// Returns `true` if scripting is available to the plug-in.
    pub fn is_scripting_available(&self) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsScriptingAvailable -> ae_sys::A_Boolean)? != 0)
    }

    /// Have After Effects execute a script.
    ///
    /// The script passed in can be in either UTF-8 or the current application encoding (if `platform_encoding` is passed in as `true`).
    ///
    /// Returns a tuple containing `(result, error_string)`. The result is the value of the last line of the script.
    pub fn execute_script(&self, plugin_id: PluginId, script: &str, platform_encoding: bool) -> Result<(String, String), Error> {
        let script = CString::new(script).map_err(|_| Error::InvalidParms)?;
        let (result, error_string) = call_suite_fn_double!(self,
            AEGP_ExecuteScript -> ae_sys::AEGP_MemHandle, ae_sys::AEGP_MemHandle,
            plugin_id,
            script.as_ptr(),
            platform_encoding as _
        )?;

        let result       = MemHandle::<u8>::from_raw(result)?;
        let error_string = MemHandle::<u8>::from_raw(error_string)?;
        let result_len       = result.len()?;
        let error_string_len = error_string.len()?;
        let result       = unsafe { std::slice::from_raw_parts(result      .lock()?.as_ptr(), result_len) };
        let error_string = unsafe { std::slice::from_raw_parts(error_string.lock()?.as_ptr(), error_string_len) };

        Ok((
            std::str::from_utf8(result)      .map_err(|_| Error::InvalidParms)?.trim_end_matches('\0').to_owned(),
            std::str::from_utf8(error_string).map_err(|_| Error::InvalidParms)?.trim_end_matches('\0').to_owned()
        ))
    }

    /// Returns `true` if the user has successfully activated After Effects.
    pub fn host_is_activated(&self) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_HostIsActivated -> ae_sys::A_Boolean)? != 0)
    }

    /// On macOS, returns a `CFBundleRef` to your Mach-O plug-in, or `NULL` for a CFM plug-in.
    /// Always returns `NULL` on Windows (you can use an OS-specific entry point to capture your DLLInstance).
    pub fn plugin_platform_ref(&self, plugin_id: PluginId) -> Result<*mut std::ffi::c_void, Error> {
        let mut plat_ref = std::ptr::null_mut();
        call_suite_fn!(self, AEGP_GetPluginPlatformRef, plugin_id, &mut plat_ref as *mut _ as *mut _)?;
        Ok(plat_ref)
    }

    /// Rescans the system font list.
    pub fn update_font_list(&self) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_UpdateFontList,)
    }

    /// Returns a particular path associated with the plug-in:
    ///
    /// - [`GetPathTypes::Plugin`] - (Not Implemented) The path to the location of the plug-in itself.
    /// - [`GetPathTypes::UserPlugin`] - The suite specific location of user specific plug-ins.
    /// - [`GetPathTypes::AllUserPlugin`] - The suite specific location of plug-ins shared by all users.
    /// - [`GetPathTypes::App`] - The After Effects .exe or .app location. Not plug-in specific.
    pub fn plugin_paths(&self, plugin_id: PluginId, path_type: GetPathTypes) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetPluginPaths -> ae_sys::AEGP_MemHandle, plugin_id, path_type.into())?;
        // Create a mem handle each and lock it.
        // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_GetPathTypes,
    GetPathTypes {
        /// (Not Implemented) The path to the location of the plug-in itself.
        Plugin        = ae_sys::AEGP_GetPathTypes_PLUGIN,
        /// The suite specific location of user specific plug-ins.
        UserPlugin    = ae_sys::AEGP_GetPathTypes_USER_PLUGIN,
        /// The suite specific location of plug-ins shared by all users.
        AllUserPlugin = ae_sys::AEGP_GetPathTypes_ALLUSER_PLUGIN,
        /// The After Effects .exe or .app location. Not plug-in specific.
        App           = ae_sys::AEGP_GetPathTypes_APP,
    }
}
