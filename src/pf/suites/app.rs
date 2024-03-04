use crate::*;
use std::ffi::{ CStr, CString };

define_suite!(
    /// Roughly 437 years ago, when we released After Effects 5.0, we published some useful utility callbacks in PF_AppSuite. They're as useful today as they were then. After Effects has user-controllable UI brightness.
    ///
    /// In addition to the [`EffectCustomUIOverlayThemeSuite`](pf::suites::EffectCustomUIOverlayTheme) for custom UI in effects, use these calls to integrate seamlessly into the After Effects UI.
    ///
    /// What better way to shame someone into purchasing a copy of your plug-in than by putting their personal information into a watermark, eh? Or set the cursor to add mask vertices, just to confuse people? Heh heh heh. But that would be wrong.
    AppSuite,
    PFAppSuite6,
    kPFAppSuite,
    kPFAppSuiteVersion6
);

fn str_from_c(slice: &[i8]) -> Result<String, Error> {
    let slice: &[u8] = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
    Ok(CStr::from_bytes_until_nul(slice).map_err(|_| Error::InvalidParms)?.to_string_lossy().into_owned())
}

impl AppSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the current background color.
    pub fn bg_color(&self) -> Result<ae_sys::PF_App_Color, Error> {
        call_suite_fn_single!(self, PF_AppGetBgColor -> ae_sys:: PF_App_Color)
    }

    /// Retrieves the color for the specified UI element. See [`AppColorType`] for a complete enumeration of available values.
    ///
    /// Basically any color in After Effects' UI can be retrieved.
    pub fn color(&self, color_type: AppColorType) -> Result<ae_sys::PF_App_Color, Error> {
        call_suite_fn_single!(self, PF_AppGetColor -> ae_sys:: PF_App_Color, color_type.into())
    }

    /// New in CC. Retrieves the active displayed language of AE UI so plug-in can match. Here are the possible language codes as of CC:
    ///
    /// - Chinese - `zh_CN`
    /// - English - `en_US`
    /// - French - `fr_FR`
    /// - German - `de_DE`
    /// - Italian - `it_IT`
    /// - Japanese - `ja_JP`
    /// - Korean - `ko_KR`
    /// - Spanish - `es_ES`
    pub fn language(&self) -> Result<String, Error> {
        let mut lang = [0i8; 16];
        call_suite_fn!(self, PF_AppGetLanguage, lang.as_mut_ptr())?;
        Ok(str_from_c(&lang)?)
    }

    /// Retrieves the user's registration information.
    pub fn personal_info(&self) -> Result<AppPersonalTextInfo, Error> {
        let info = call_suite_fn_single!(self, PF_GetPersonalInfo -> ae_sys::PF_AppPersonalTextInfo)?;
        Ok(AppPersonalTextInfo {
            name:       str_from_c(&info.name)?,
            org:        str_from_c(&info.org)?,
            serial_str: str_from_c(&info.serial_str)?,
        })
    }

    /// Retrieves font style sheet information for the fonts used in After Effects' UI.
    ///
    /// Trivia: The font used in After Effects' UI starting in 15.0 is Adobe Clean. Before that, it was Tahoma on Windows and Lucida Grande on macOS X.
    ///
    /// Returns a tuple containing: (font name, font number, size, style),
    pub fn font_style_sheet(&self, sheet: FontStyleSheet) -> Result<(String, i16, i16, i16), Error> {
        let mut font_name: ae_sys::PF_FontName = unsafe { std::mem::zeroed() };
        let mut font_num = 0;
        let mut size = 0;
        let mut style = 0;
        call_suite_fn!(self, PF_GetFontStyleSheet, sheet.into(), &mut font_name as *mut _, &mut font_num, &mut size, &mut style)?;
        Ok((
            str_from_c(&font_name.font_nameAC)?,
            font_num,
            size,
            style,
        ))
    }

    /// Sets the cursor to any of After Effects' cursors. See [`CursorType`]` for a complete enumeration.
    ///
    /// Set to:
    /// - [`CursorType::None`] to allow After Effects to set the cursor.
    /// - [`CursorType::Custom`] if you've used OS-specific calls to change the cursor (After Effects will honor your changes).
    pub fn set_cursor(&self, cursor: CursorType) -> Result<(), Error> {
        call_suite_fn!(self, PF_SetCursor, cursor.into())
    }

    /// Returns `true` if After Effects is running in watched folder mode, or is a render engine installation.
    pub fn is_render_engine(&self) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_IsRenderEngine -> ae_sys::PF_Boolean)? != 0)
    }

    /// Displays the After Effects color picker dialog (which may be the system color picker, depending on the user's preferences).
    ///
    /// Will return `Error::InterruptCancel` if user cancels dialog. Returned color is in the project's working color space.
    pub fn color_picker_dialog(&self, dialog_title: Option<&str>, sample_color: &pf::PixelF32, use_ws_to_monitor_xform: bool) -> Result<pf::PixelF32, Error> {
        let dialog_title = dialog_title.map(|s| CString::new(s).unwrap());
        call_suite_fn_single!(self, PF_AppColorPickerDialog -> ae_sys::PF_PixelFloat, dialog_title.map_or(std::ptr::null(), |x| x.as_ptr()), sample_color, use_ws_to_monitor_xform as _)
    }

    /// Returns the position of the mouse in the custom UI coordinate space.
    pub fn mouse_position(&self) -> Result<ae_sys::PF_Point, Error> {
        call_suite_fn_single!(self, PF_GetMouse -> ae_sys::PF_Point)
    }

    /// Queue up a redraw of a specific area of the custom UI for an effect.
    ///
    /// Only valid while handling a non-drawing event in the effect.
    ///
    /// Specify `None` to invalidate the entire window. The redraw will happen at the next available idle moment after returning from the event.
    ///
    /// Set the `PF_EO_UPDATE_NOW` event outflag to update the window immediately after the event returns.
    pub fn invalidate_rect(&self, context: impl AsPtr<ae_sys::PF_ContextH>, rect: Option<pf::Rect>) -> Result<(), Error> {
        call_suite_fn!(self, PF_InvalidateRect, context.as_ptr(), rect.map(Into::into).as_ref().map_or(std::ptr::null(), |x| x))
    }

    /// Converts from the custom UI coordinate system to global screen coordinates. Use only during custom UI event handling.
    pub fn convert_local_to_global(&self, local: &ae_sys::PF_Point) -> Result<ae_sys::PF_Point, Error> {
        call_suite_fn_single!(self, PF_ConvertLocalToGlobal -> ae_sys::PF_Point, local)
    }

    pub fn color_at_global_point(&self, global: &ae_sys::PF_Point, eye_size: i16, mode: EyeDropperSampleMode) -> Result<pf::PixelF32, Error> {
        call_suite_fn_single!(self, PF_GetColorAtGlobalPoint -> ae_sys::PF_PixelFloat, global, eye_size, mode.into())
    }

    /// Creates a new progress dialog. Use the `update()` method to update the progress.
    ///
    /// If `cancel_str` is `None`, the dialog will not have a cancel button.
    ///
    /// It won't open the dialog unless it detects a slow render. (2 seconds timeout).
    pub fn create_progress_dialog(&self, title: &str, cancel_str: Option<&str>, indeterminate: bool) -> Result<AppProgressDialog, Error> {
        let title = widestring::U16CString::from_str(title).unwrap();
        let cancel_str = cancel_str.map(|s| widestring::U16CString::from_str(s).unwrap());
        let mut ptr = std::ptr::null_mut();
        call_suite_fn!(self, PF_CreateNewAppProgressDialog, title.as_ptr(), cancel_str.map_or(std::ptr::null(), |x| x.as_ptr()), indeterminate as _, &mut ptr)?;
        Ok(AppProgressDialog {
            suite_ptr: self.suite_ptr,
            ptr
        })
    }
}

define_suite!(
    /// [`AdvAppSuite`] was originally designed for some pretty nefarious purposes; an external application was pretending to be an After Effects plug-in, and required ways to notify After Effects of the changes it had made to the project. Our API impurity is your gain.
    AdvAppSuite,
    PF_AdvAppSuite2,
    kPFAdvAppSuite,
    kPFAdvAppSuiteVersion2
);

impl AdvAppSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Tells After Effects that the project has been changed since it was last saved.
    pub fn set_project_dirty(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_SetProjectDirty,)
    }

    /// Saves the project to the current path. To save the project elsewhere, use [`aegp::suites::Project::save_project_to_path`].
    pub fn save_project(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_SaveProject,)
    }

    /// Stores the background state (After Effects' position in the stacking order of open applications and windows).
    pub fn save_background_state(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_SaveBackgroundState,)
    }

    /// Brings After Effects to the front of all currently open applications and windows.
    pub fn force_foreground(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_ForceForeground,)
    }

    /// Puts After Effects back where it was, in relation to other applications and windows.
    pub fn restore_background_state(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_RestoreBackgroundState,)
    }

    /// Forces all After Effects windows to update.
    ///
    /// Note that although the Composition panel will be refreshed, this does not guarantee a new frame will be sent to External Monitor Preview plug-ins.
    pub fn refresh_all_windows(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_RefreshAllWindows,)
    }

    /// Writes text into the After Effects info palette.
    pub fn info_draw_text(&self, line1: &str, line2: &str) -> Result<(), Error> {
        let line1 = CString::new(line1).unwrap();
        let line2 = CString::new(line2).unwrap();
        call_suite_fn!(self, PF_InfoDrawText, line1.as_ptr(), line2.as_ptr())
    }

    /// Draws the specified color in the After Effects info palette (alpha is ignored).
    pub fn info_draw_color(&self, color: pf::Pixel8) -> Result<(), Error> {
        call_suite_fn!(self, PF_InfoDrawColor, color)
    }

    /// Writes three lines of text into the After Effects info palette.
    pub fn info_draw_text3(&self, line1: &str, line2: &str, line3: &str) -> Result<(), Error> {
        let line1 = CString::new(line1).unwrap();
        let line2 = CString::new(line2).unwrap();
        let line3 = CString::new(line3).unwrap();
        call_suite_fn!(self, PF_InfoDrawText3, line1.as_ptr(), line2.as_ptr(), line3.as_ptr())
    }

    /// Writes three lines of text into the After Effects info palette, with portions of the second and third lines left and right justified.
    pub fn info_draw_text3_plus(&self, line1: &str, line2_jr: &str, line2_jl: &str, line3_jr: &str, line3_jl: &str) -> Result<(), Error> {
        let line1 = CString::new(line1).unwrap();
        let line2_jr = CString::new(line2_jr).unwrap();
        let line2_jl = CString::new(line2_jl).unwrap();
        let line3_jr = CString::new(line3_jr).unwrap();
        let line3_jl = CString::new(line3_jl).unwrap();
        call_suite_fn!(self, PF_InfoDrawText3Plus, line1.as_ptr(), line2_jr.as_ptr(), line2_jl.as_ptr(), line3_jr.as_ptr(), line3_jl.as_ptr())
    }

    /// Appends characters to the currently-displayed info text.
    pub fn append_info_text(&self, append: &str) -> Result<(), Error> {
        let append = CString::new(append).unwrap();
        call_suite_fn!(self, PF_AppendInfoText, append.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::PF_App_ColorType,
    AppColorType {
        None                        = ae_sys::PF_App_Color_NONE,
        Frame                       = ae_sys::PF_App_Color_FRAME,
        Fill                        = ae_sys::PF_App_Color_FILL,
        Text                        = ae_sys::PF_App_Color_TEXT,
        LightTinge                  = ae_sys::PF_App_Color_LIGHT_TINGE,
        DarkTinge                   = ae_sys::PF_App_Color_DARK_TINGE,
        Hilite                      = ae_sys::PF_App_Color_HILITE,
        Shadow                      = ae_sys::PF_App_Color_SHADOW,
        ButtonFrame                 = ae_sys::PF_App_Color_BUTTON_FRAME,
        ButtonFill                  = ae_sys::PF_App_Color_BUTTON_FILL,
        ButtonText                  = ae_sys::PF_App_Color_BUTTON_TEXT,
        ButtonLightTinge            = ae_sys::PF_App_Color_BUTTON_LIGHT_TINGE,
        ButtonDarkTinge             = ae_sys::PF_App_Color_BUTTON_DARK_TINGE,
        ButtonHilite                = ae_sys::PF_App_Color_BUTTON_HILITE,
        ButtonShadow                = ae_sys::PF_App_Color_BUTTON_SHADOW,
        ButtonPressedFrame          = ae_sys::PF_App_Color_BUTTON_PRESSED_FRAME,
        ButtonPressedFill           = ae_sys::PF_App_Color_BUTTON_PRESSED_FILL,
        ButtonPressedText           = ae_sys::PF_App_Color_BUTTON_PRESSED_TEXT,
        ButtonPressedLightTinge     = ae_sys::PF_App_Color_BUTTON_PRESSED_LIGHT_TINGE,
        ButtonPressedDarkTinge      = ae_sys::PF_App_Color_BUTTON_PRESSED_DARK_TINGE,
        ButtonPressedHilite         = ae_sys::PF_App_Color_BUTTON_PRESSED_HILITE,
        ButtonPressedShadow         = ae_sys::PF_App_Color_BUTTON_PRESSED_SHADOW,
        FrameDisabled               = ae_sys::PF_App_Color_FRAME_DISABLED,
        FillDisabled                = ae_sys::PF_App_Color_FILL_DISABLED,
        TextDisabled                = ae_sys::PF_App_Color_TEXT_DISABLED,
        LightTingeDisabled          = ae_sys::PF_App_Color_LIGHT_TINGE_DISABLED,
        DarkTingeDisabled           = ae_sys::PF_App_Color_DARK_TINGE_DISABLED,
        HiliteDisabled              = ae_sys::PF_App_Color_HILITE_DISABLED,
        ShadowDisabled              = ae_sys::PF_App_Color_SHADOW_DISABLED,
        ButtonFrameDisabled         = ae_sys::PF_App_Color_BUTTON_FRAME_DISABLED,
        ButtonFillDisabled          = ae_sys::PF_App_Color_BUTTON_FILL_DISABLED,
        ButtonTextDisabled          = ae_sys::PF_App_Color_BUTTON_TEXT_DISABLED,
        ButtonLightTingeDisabled    = ae_sys::PF_App_Color_BUTTON_LIGHT_TINGE_DISABLED,
        ButtonDarkTingeDisabled     = ae_sys::PF_App_Color_BUTTON_DARK_TINGE_DISABLED,
        ButtonHiliteDisabled        = ae_sys::PF_App_Color_BUTTON_HILITE_DISABLED,
        ButtonShadowDisabled        = ae_sys::PF_App_Color_BUTTON_SHADOW_DISABLED,
        ButtonPressedFrameDisabled  = ae_sys::PF_App_Color_BUTTON_PRESSED_FRAME_DISABLED,
        ButtonPressedFillDisabled   = ae_sys::PF_App_Color_BUTTON_PRESSED_FILL_DISABLED,
        ButtonPressedTextDisabled   = ae_sys::PF_App_Color_BUTTON_PRESSED_TEXT_DISABLED,
        ButtonPressedLightTingeDisabled = ae_sys::PF_App_Color_BUTTON_PRESSED_LIGHT_TINGE_DISABLED,
        ButtonPressedDarkTingeDisabled  = ae_sys::PF_App_Color_BUTTON_PRESSED_DARK_TINGE_DISABLED,
        ButtonPressedHiliteDisabled = ae_sys::PF_App_Color_BUTTON_PRESSED_HILITE_DISABLED,
        ButtonPressedShadowDisabled = ae_sys::PF_App_Color_BUTTON_PRESSED_SHADOW_DISABLED,
        Black                     = ae_sys::PF_App_Color_BLACK,
        White                     = ae_sys::PF_App_Color_WHITE,
        Gray                      = ae_sys::PF_App_Color_GRAY,
        Red                       = ae_sys::PF_App_Color_RED,
        Yellow                    = ae_sys::PF_App_Color_YELLOW,
        Green                     = ae_sys::PF_App_Color_GREEN,
        Cyan                      = ae_sys::PF_App_Color_CYAN,
        TlwNeedleCurrentTime        = ae_sys::PF_App_Color_TLW_NEEDLE_CURRENT_TIME,
        TlwNeedlePreviewTime        = ae_sys::PF_App_Color_TLW_NEEDLE_PREVIEW_TIME,
        TlwCacheMarkMem             = ae_sys::PF_App_Color_TLW_CACHE_MARK_MEM,
        TlwCacheMarkDisk            = ae_sys::PF_App_Color_TLW_CACHE_MARK_DISK,
        TlwCacheMarkMix             = ae_sys::PF_App_Color_TLW_CACHE_MARK_MIX,
        FillLight                   = ae_sys::PF_App_Color_FILL_LIGHT,
        HotText                     = ae_sys::PF_App_Color_HOT_TEXT,
        HotTextDisabled             = ae_sys::PF_App_Color_HOT_TEXT_DISABLED,
        Label0                      = ae_sys::PF_App_Color_LABEL_0,
        Label1                      = ae_sys::PF_App_Color_LABEL_1,
        Label2                      = ae_sys::PF_App_Color_LABEL_2,
        Label3                      = ae_sys::PF_App_Color_LABEL_3,
        Label4                      = ae_sys::PF_App_Color_LABEL_4,
        Label5                      = ae_sys::PF_App_Color_LABEL_5,
        Label6                      = ae_sys::PF_App_Color_LABEL_6,
        Label7                      = ae_sys::PF_App_Color_LABEL_7,
        Label8                      = ae_sys::PF_App_Color_LABEL_8,
        Label9                      = ae_sys::PF_App_Color_LABEL_9,
        Label10                     = ae_sys::PF_App_Color_LABEL_10,
        Label11                     = ae_sys::PF_App_Color_LABEL_11,
        Label12                     = ae_sys::PF_App_Color_LABEL_12,
        Label13                     = ae_sys::PF_App_Color_LABEL_13,
        Label14                     = ae_sys::PF_App_Color_LABEL_14,
        Label15                     = ae_sys::PF_App_Color_LABEL_15,
        Label16                     = ae_sys::PF_App_Color_LABEL_16,
        TlwCacheMarkMemDubious      = ae_sys::PF_App_Color_TLW_CACHE_MARK_MEM_DUBIOUS,
        TlwCacheMarkDiskDubious     = ae_sys::PF_App_Color_TLW_CACHE_MARK_DISK_DUBIOUS,
        TlwCacheMarkMixDubious      = ae_sys::PF_App_Color_TLW_CACHE_MARK_MIX_DUBIOUS,
        HotTextPressed              = ae_sys::PF_App_Color_HOT_TEXT_PRESSED,
        HotTextWarning              = ae_sys::PF_App_Color_HOT_TEXT_WARNING,
        PureBlack                   = ae_sys::PF_App_Color_PURE_BLACK,
        PureWhite                   = ae_sys::PF_App_Color_PURE_WHITE,
        PanelBackground             = ae_sys::PF_App_Color_PANEL_BACKGROUND,
        ListBoxFill                 = ae_sys::PF_App_Color_LIST_BOX_FILL,
        DarkCaptionFill             = ae_sys::PF_App_Color_DARK_CAPTION_FILL,
        DarkCaptionText             = ae_sys::PF_App_Color_DARK_CAPTION_TEXT,
        TextOnLighterBg             = ae_sys::PF_App_Color_TEXT_ON_LIGHTER_BG,
    }
}

define_enum! {
    ae_sys::PF_FontStyleSheet,
    FontStyleSheet {
        None        = ae_sys::PF_FontStyle_NONE,
        Sys         = ae_sys::PF_FontStyle_SYS,
        Small       = ae_sys::PF_FontStyle_SMALL,
        SmallBold   = ae_sys::PF_FontStyle_SMALL_BOLD,
        SmallItalic = ae_sys::PF_FontStyle_SMALL_ITALIC,
        Med         = ae_sys::PF_FontStyle_MED,
        MedBold     = ae_sys::PF_FontStyle_MED_BOLD,
        App         = ae_sys::PF_FontStyle_APP,
        AppBold     = ae_sys::PF_FontStyle_APP_BOLD,
        AppItalic   = ae_sys::PF_FontStyle_APP_ITALIC,
    }
}

define_enum! {
    ae_sys::PF_CursorType,
    CursorType {
        None                     = ae_sys::PF_Cursor_NONE,
        Custom                   = ae_sys::PF_Cursor_CUSTOM,
        Arrow                    = ae_sys::PF_Cursor_ARROW,
        HollowArrow              = ae_sys::PF_Cursor_HOLLOW_ARROW,
        WatchNWait               = ae_sys::PF_Cursor_WATCH_N_WAIT,
        Magnify                  = ae_sys::PF_Cursor_MAGNIFY,
        MagnifyPlus              = ae_sys::PF_Cursor_MAGNIFY_PLUS,
        MagnifyMinus             = ae_sys::PF_Cursor_MAGNIFY_MINUS,
        Crosshairs               = ae_sys::PF_Cursor_CROSSHAIRS,
        CrossRect                = ae_sys::PF_Cursor_CROSS_RECT,
        CrossOval                = ae_sys::PF_Cursor_CROSS_OVAL,
        CrossRotate              = ae_sys::PF_Cursor_CROSS_ROTATE,
        Pan                      = ae_sys::PF_Cursor_PAN,
        Eyedropper               = ae_sys::PF_Cursor_EYEDROPPER,
        Hand                     = ae_sys::PF_Cursor_HAND,
        Pen                      = ae_sys::PF_Cursor_PEN,
        PenAdd                   = ae_sys::PF_Cursor_PEN_ADD,
        PenDelete                = ae_sys::PF_Cursor_PEN_DELETE,
        PenClose                 = ae_sys::PF_Cursor_PEN_CLOSE,
        PenDrag                  = ae_sys::PF_Cursor_PEN_DRAG,
        PenCorner                = ae_sys::PF_Cursor_PEN_CORNER,
        ResizeVertical           = ae_sys::PF_Cursor_RESIZE_VERTICAL,
        ResizeHorizontal         = ae_sys::PF_Cursor_RESIZE_HORIZONTAL,
        FingerPointer            = ae_sys::PF_Cursor_FINGER_POINTER,
        ScaleHoriz               = ae_sys::PF_Cursor_SCALE_HORIZ,
        ScaleDiagLr              = ae_sys::PF_Cursor_SCALE_DIAG_LR,
        ScaleVert                = ae_sys::PF_Cursor_SCALE_VERT,
        ScaleDiagUr              = ae_sys::PF_Cursor_SCALE_DIAG_UR,
        RotTop                   = ae_sys::PF_Cursor_ROT_TOP,
        RotTopRight              = ae_sys::PF_Cursor_ROT_TOP_RIGHT,
        RotRight                 = ae_sys::PF_Cursor_ROT_RIGHT,
        RotBotRight              = ae_sys::PF_Cursor_ROT_BOT_RIGHT,
        RotBottom                = ae_sys::PF_Cursor_ROT_BOTTOM,
        RotBotLeft               = ae_sys::PF_Cursor_ROT_BOT_LEFT,
        RotLeft                  = ae_sys::PF_Cursor_ROT_LEFT,
        RotTopLeft               = ae_sys::PF_Cursor_ROT_TOP_LEFT,
        DragCenter               = ae_sys::PF_Cursor_DRAG_CENTER,
        Copy                     = ae_sys::PF_Cursor_COPY,
        Alias                    = ae_sys::PF_Cursor_ALIAS,
        Context                  = ae_sys::PF_Cursor_CONTEXT,
        SlipEdit                 = ae_sys::PF_Cursor_SLIP_EDIT,
        CameraOrbitCamera        = ae_sys::PF_Cursor_CAMERA_ORBIT_CAMERA,
        CameraPanCamera          = ae_sys::PF_Cursor_CAMERA_PAN_CAMERA,
        CameraDollyCamera        = ae_sys::PF_Cursor_CAMERA_DOLLY_CAMERA,
        RotateX                  = ae_sys::PF_Cursor_ROTATE_X,
        RotateY                  = ae_sys::PF_Cursor_ROTATE_Y,
        RotateZ                  = ae_sys::PF_Cursor_ROTATE_Z,
        ArrowX                   = ae_sys::PF_Cursor_ARROW_X,
        ArrowY                   = ae_sys::PF_Cursor_ARROW_Y,
        ArrowZ                   = ae_sys::PF_Cursor_ARROW_Z,
        Scissors                 = ae_sys::PF_Cursor_SCISSORS,
        FatEyedropper            = ae_sys::PF_Cursor_FAT_EYEDROPPER,
        FingerPointerScrub       = ae_sys::PF_Cursor_FINGER_POINTER_SCRUB,
        HorzIBeam                = ae_sys::PF_Cursor_HORZ_I_BEAM,
        VertIBeam                = ae_sys::PF_Cursor_VERT_I_BEAM,
        HorzBoxIBeam             = ae_sys::PF_Cursor_HORZ_BOX_I_BEAM,
        VertBoxIBeam             = ae_sys::PF_Cursor_VERT_BOX_I_BEAM,
        IBeam0                   = ae_sys::PF_Cursor_I_BEAM_0,
        IBeam11_25               = ae_sys::PF_Cursor_I_BEAM_11_25,
        IBeam22_5                = ae_sys::PF_Cursor_I_BEAM_22_5,
        IBeam33_75               = ae_sys::PF_Cursor_I_BEAM_33_75,
        IBeam45                  = ae_sys::PF_Cursor_I_BEAM_45,
        IBeam56_25               = ae_sys::PF_Cursor_I_BEAM_56_25,
        IBeam67_5                = ae_sys::PF_Cursor_I_BEAM_67_5,
        IBeam78_75               = ae_sys::PF_Cursor_I_BEAM_78_75,
        IBeam90                  = ae_sys::PF_Cursor_I_BEAM_90,
        IBeam101_25              = ae_sys::PF_Cursor_I_BEAM_101_25,
        IBeam112_5               = ae_sys::PF_Cursor_I_BEAM_112_5,
        IBeam123_75              = ae_sys::PF_Cursor_I_BEAM_123_75,
        IBeam135                 = ae_sys::PF_Cursor_I_BEAM_135,
        IBeam146_25              = ae_sys::PF_Cursor_I_BEAM_146_25,
        IBeam157_5               = ae_sys::PF_Cursor_I_BEAM_157_5,
        IBeam168_75              = ae_sys::PF_Cursor_I_BEAM_168_75,
        CrosshairsPickup         = ae_sys::PF_Cursor_CROSSHAIRS_PICKUP,
        ArrowSelector            = ae_sys::PF_Cursor_ARROW_SELECTOR,
        LayerMove                = ae_sys::PF_Cursor_LAYER_MOVE,
        MoveStartMargin          = ae_sys::PF_Cursor_MOVE_START_MARGIN,
        MoveEndMargin            = ae_sys::PF_Cursor_MOVE_END_MARGIN,
        SolidArrow               = ae_sys::PF_Cursor_SOLID_ARROW,
        HollowArrowPlus          = ae_sys::PF_Cursor_HOLLOW_ARROW_PLUS,
        BrushCenter              = ae_sys::PF_Cursor_BRUSH_CENTER,
        CloneSource              = ae_sys::PF_Cursor_CLONE_SOURCE,
        CloneSourceOffset        = ae_sys::PF_Cursor_CLONE_SOURCE_OFFSET,
        HollowLayerMove          = ae_sys::PF_Cursor_HOLLOW_LAYER_MOVE,
        MoveTrackSearchRegion    = ae_sys::PF_Cursor_MOVE_TRACK_SEARCH_REGION,
        MoveTrackAttachPoint     = ae_sys::PF_Cursor_MOVE_TRACK_ATTACH_POINT,
        ColorCubeCrossSection    = ae_sys::PF_Cursor_COLOR_CUBE_CROSS_SECTION,
        PenCornerRotobezTension  = ae_sys::PF_Cursor_PEN_CORNER_ROTOBEZ_TENSION,
        Pin                      = ae_sys::PF_Cursor_PIN,
        PinAdd                   = ae_sys::PF_Cursor_PIN_ADD,
        MeshAdd                  = ae_sys::PF_Cursor_MESH_ADD,
        Marquee                  = ae_sys::PF_Cursor_MARQUEE,
        CrossRoundedRect         = ae_sys::PF_Cursor_CROSS_ROUNDED_RECT,
        CrossPolygon             = ae_sys::PF_Cursor_CROSS_POLYGON,
        CrossStar                = ae_sys::PF_Cursor_CROSS_STAR,
        PinStarch                = ae_sys::PF_Cursor_PIN_STARCH,
        PinOverlap               = ae_sys::PF_Cursor_PIN_OVERLAP,
        Stopwatch                = ae_sys::PF_Cursor_STOPWATCH,
        DragDot                  = ae_sys::PF_Cursor_DRAG_DOT,
        DragCircle               = ae_sys::PF_Cursor_DRAG_CIRCLE,
        DirectSelect             = ae_sys::PF_Cursor_DIRECT_SELECT,
        DragCopyMove             = ae_sys::PF_Cursor_DRAG_COPY_MOVE,
        DragCopyRotate           = ae_sys::PF_Cursor_DRAG_COPY_ROTATE,
        CameraMaya               = ae_sys::PF_Cursor_CAMERA_MAYA,
        ResizeHorizontalLeft     = ae_sys::PF_Cursor_RESIZE_HORIZONTAL_LEFT,
        ResizeHorizontalRight    = ae_sys::PF_Cursor_RESIZE_HORIZONTAL_RIGHT,
        Feather                  = ae_sys::PF_Cursor_FEATHER,
        FeatherAdd               = ae_sys::PF_Cursor_FEATHER_ADD,
        FeatherDelete            = ae_sys::PF_Cursor_FEATHER_DELETE,
        FeatherMove              = ae_sys::PF_Cursor_FEATHER_MOVE,
        FeatherTension           = ae_sys::PF_Cursor_FEATHER_TENSION,
        FeatherMarquee           = ae_sys::PF_Cursor_FEATHER_MARQUEE,
        LassoArrow               = ae_sys::PF_Cursor_LASSO_ARROW,
        DragNoDrop               = ae_sys::PF_Cursor_DRAG_NO_DROP,
        DragCopy                 = ae_sys::PF_Cursor_DRAG_COPY,
        DragLink                 = ae_sys::PF_Cursor_DRAG_LINK,
        PinBend                  = ae_sys::PF_Cursor_PIN_BEND,
        PinAdvanced              = ae_sys::PF_Cursor_PIN_ADVANCED,
        CameraOrbitCursor        = ae_sys::PF_Cursor_CAMERA_ORBIT_CURSOR,
        CameraOrbitScene         = ae_sys::PF_Cursor_CAMERA_ORBIT_SCENE,
        CameraPanCursor          = ae_sys::PF_Cursor_CAMERA_PAN_CURSOR,
        CameraDollyTowardsCursor = ae_sys::PF_Cursor_CAMERA_DOLLY_TOWARDS_CURSOR,
        CameraDollyToCursor      = ae_sys::PF_Cursor_CAMERA_DOLLY_TO_CURSOR,
    }
}

define_enum! {
    ae_sys::PF_EyeDropperSampleMode,
    EyeDropperSampleMode {
        Default  = ae_sys::PF_EyeDropperSampleMode_DEFAULT,
        Straight = ae_sys::PF_EyeDropperSampleMode_STRAIGHT,
        Premul   = ae_sys::PF_EyeDropperSampleMode_PREMUL,
    }
}

#[derive(Debug, Clone)]
pub struct AppPersonalTextInfo {
    pub name: String,
    pub org: String,
    pub serial_str: String
}

pub struct AppProgressDialog {
    suite_ptr: *const ae_sys::PFAppSuite6,
    ptr: ae_sys::PF_AppProgressDialogP,
}
impl AppProgressDialog {
    /// Updates the progress dialog.
    pub fn update(&self, count: i32, total: i32) -> Result<(), Error> {
        call_suite_fn!(self, PF_AppProgressDialogUpdate, self.ptr, count as _, total as _)
    }
}
impl Drop for AppProgressDialog {
    fn drop(&mut self) {
        call_suite_fn!(self, PF_DisposeAppProgressDialog, self.ptr).unwrap();
    }
}
