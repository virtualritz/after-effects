use after_effects_sys::{
    AEGP_AboutRefcon, AEGP_AboutStringRefcon, AEGP_CommandRefcon, AEGP_DeathRefcon,
    AEGP_GlobalRefcon, AEGP_UpdateMenuRefcon, AEGP_VersionRefcon,
};

use crate::*;
use std::ffi::CString;
use std::os::raw::c_void;

define_suite!(
    RegisterSuite,
    AEGP_RegisterSuite5,
    kAEGPRegisterSuite,
    kAEGPRegisterSuiteVersion5
);

define_enum! {
    ae_sys::AEGP_HookPriority,
    HookPriority {
        BeforeAE = ae_sys::AEGP_HP_BeforeAE,
        AfterAE = ae_sys::AEGP_HP_AfterAE,
    }
}

define_enum! {
    ae_sys::AEGP_WindowType,
    WindowType {
        None = ae_sys::AEGP_WindType_NONE,
        Project = ae_sys::AEGP_WindType_PROJECT,
        Comp = ae_sys::AEGP_WindType_COMP,
        TimeLayout = ae_sys::AEGP_WindType_TIME_LAYOUT,
        Layer = ae_sys::AEGP_WindType_LAYER,
        Footage = ae_sys::AEGP_WindType_FOOTAGE,
        RenderQueue = ae_sys::AEGP_WindType_RENDER_QUEUE,
        QuickTime = ae_sys::AEGP_WindType_QT,
        Dialog = ae_sys::AEGP_WindType_DIALOG,
        Flowchart = ae_sys::AEGP_WindType_FLOWCHART,
        Effect = ae_sys::AEGP_WindType_EFFECT,
        Other = ae_sys::AEGP_WindType_OTHER,
    }
}

/// Note: functions in this suite take a `Global` Paramater, for AEGPs this must be the same as your global `AegpPlugin` type, for all
/// other plugins this should likely be `()` - as you will always receive a `None` Global argument in your callbacks.
impl RegisterSuite {
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// Register a hook (command handler) function with After Effects.
    /// If you are replacing a function which After Effects also handles, `AEGP_HookPriority` determines whether your plug-in gets run first.
    pub fn register_command_hook<Global, Command, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        hook_priority: HookPriority,
        command: ae_sys::AEGP_Command,
        command_hook_func: F,
        command_refcon: Command,
    ) -> Result<(), Error>
    where
        F: FnMut(
            Option<&mut Global>,
            &mut Command,
            ae_sys::AEGP_Command,
            HookPriority,
            bool,
        ) -> (Error, bool),
    {
        unsafe extern "C" fn command_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_CommandRefcon,
            command: ae_sys::AEGP_Command,
            hook_priority: ae_sys::AEGP_HookPriority,
            already_handled: ae_sys::A_Boolean,
            handled_p: *mut ae_sys::A_Boolean,
        ) -> ae_sys::A_Err
        where
            F: FnMut(
                Option<&mut P>,
                &mut T,
                ae_sys::AEGP_Command,
                HookPriority,
                bool,
            ) -> (Error, bool),
        {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            let already_handled_bool = already_handled != 0;

            let hook_priority_enum = match hook_priority {
                ae_sys::AEGP_HP_BeforeAE => HookPriority::BeforeAE,
                ae_sys::AEGP_HP_AfterAE => HookPriority::AfterAE,
                _ => HookPriority::BeforeAE, // Default case
            };

            let (error, was_handled) = cb(
                global,
                refcon,
                command,
                hook_priority_enum,
                already_handled_bool,
            );

            if !handled_p.is_null() {
                *handled_p = if was_handled { 1 } else { 0 };
            }

            error.into()
        }

        let refcon_cb_tuple = Box::new((command_hook_func, command_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterCommandHook,
            plugin_id,
            hook_priority.into(),
            command,
            Some(command_hook_wrapper::<Global, Command, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your menu update function (which determines whether or not items are active),
    /// called every time any menu is to be drawn.
    pub fn register_update_menu_hook<Global, UpdateMenu, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        update_menu_hook_func: F,
        update_menu_refcon: UpdateMenu,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut UpdateMenu, WindowType) -> Error,
    {
        unsafe extern "C" fn update_menu_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_UpdateMenuRefcon,
            window_type: ae_sys::AEGP_WindowType,
        ) -> sys::PF_Err
        where
            F: FnMut(Option<&mut P>, &mut T, WindowType) -> Error,
        {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            cb(global, refcon, window_type.into()).into()
        }

        let refcon_cb_tuple = Box::new((update_menu_hook_func, update_menu_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterUpdateMenuHook,
            plugin_id,
            Some(update_menu_hook_wrapper::<Global, UpdateMenu, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your termination function. Called when the application quits.
    pub fn register_death_hook<Global, Death, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        death_hook_func: F,
        death_refcon: Death,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut Death) -> Error,
    {
        unsafe extern "C" fn death_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_DeathRefcon,
        ) -> sys::PF_Err
        where
            F: FnMut(Option<&mut P>, &mut T) -> Error,
        {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            cb(global, refcon).into()
        }

        let refcon_cb_tuple = Box::new((death_hook_func, death_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterDeathHook,
            plugin_id,
            Some(death_hook_wrapper::<Global, Death, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_version_hook<Global, Version, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        version_hook_func: F,
        version_refcon: Version,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut Version, &mut u32) -> Error,
    {
        unsafe extern "C" fn version_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_VersionRefcon,
            pf_version_p: *mut ae_sys::A_u_long,
        ) -> ae_sys::A_Err
        where
            F: FnMut(Option<&mut P>, &mut T, &mut u32) -> Error,
        {
            log::error!(
                "The after effects documentation said version hook should never be called!"
            );
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            let pf_version = &mut (*pf_version_p as u32);

            cb(global, refcon, pf_version).into()
        }

        log::warn!("Called `register_version_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((version_hook_func, version_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterVersionHook,
            plugin_id,
            Some(version_hook_wrapper::<Global, Version, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_string_hook<Global, AboutString, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_string_hook_func: F,
        about_string_refcon: AboutString,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut AboutString, &mut [u8]) -> Error,
    {
        unsafe extern "C" fn about_string_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_AboutStringRefcon,
            // We have 0 documentation about this pointer
            // besides that it is never constructed, so i'm treating it as null
            _about_z: *mut ae_sys::A_char,
        ) -> ae_sys::A_Err
        where
            F: FnMut(Option<&mut P>, &mut T, &mut [u8]) -> Error,
        {
            log::error!(
                "The after effects documentation said about string hook should never be called!"
            );
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            cb(global, refcon, &mut []).into()
        }

        log::warn!("Called `register_about_string_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_string_hook_func, about_string_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutStringHook,
            plugin_id,
            Some(about_string_hook_wrapper::<Global, AboutString, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_hook<Global, About, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_hook_func: F,
        about_refcon: About,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut About) -> Error,
    {
        unsafe extern "C" fn about_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_AboutRefcon,
        ) -> ae_sys::A_Err
        where
            F: FnMut(Option<&mut P>, &mut T) -> Error,
        {
            log::error!("The after effects documentation said about hook should never be called!");
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            cb(global, refcon).into()
        }

        log::warn!("Called `register_about_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_hook_func, about_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutHook,
            plugin_id,
            Some(about_hook_wrapper::<Global, About, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your Artisan. See Artisans for more details.
    pub fn register_artisan(
        &self,
        _api_version: ae_sys::A_Version,
        _artisan_version: ae_sys::A_Version,
        _plugin_id: i32,
        _refcon: *mut c_void,
        _match_name: &str,
        _artisan_name: &str,
        _entry_funcs: *mut ae_sys::PR_ArtisanEntryPoints,
    ) -> Result<(), Error> {
        todo!("Artisan plugins are not yet supported");
    }

    /// Register your AEIO plug-in. See AEIOs for more details.
    pub fn register_io(
        &self,
        _plugin_id: ae_sys::AEGP_PluginID,
        _refcon: ae_sys::AEGP_IORefcon,
        _io_info: *const ae_sys::AEIO_ModuleInfo,
        _aeio_fcn_block: *const ae_sys::AEIO_FunctionBlock4,
    ) -> Result<(), Error> {
        todo!("AEIO plugins are not yet supported");
    }

    /// Register your IdleHook function. After Effects will call the function sporadically,
    /// while the user makes difficult artistic decisions (or while they're getting more coffee).
    /// Register your IdleHook function. After Effects will call the function sporadically,
    /// while the user makes difficult artistic decisions (or while they're getting more coffee).
    pub fn register_idle_hook<Global, Idle, F>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        idle_hook_func: F,
        idle_refcon: Idle,
    ) -> Result<(), Error>
    where
        F: FnMut(Option<&mut Global>, &mut Idle, &mut i32) -> Error,
    {
        unsafe extern "C" fn idle_hook_wrapper<P, T, F>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: ae_sys::AEGP_IdleRefcon,
            max_sleep_p: *mut ae_sys::A_long,
        ) -> ae_sys::A_Err
        where
            F: FnMut(Option<&mut P>, &mut T, &mut i32) -> Error,
        {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                Some(&mut *(plugin_refcon as *mut P))
            };

            let (cb, refcon) = &mut *(refcon as *mut (F, T));
            let max_sleep = &mut (*max_sleep_p);

            cb(global, refcon, max_sleep).into()
        }

        let refcon_cb_tuple = Box::new((idle_hook_func, idle_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterIdleHook,
            plugin_id,
            Some(idle_hook_wrapper::<Global, Idle, F>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Registers your AEGP as an interactive artisan,
    /// for use in previewing and rendering all layers in a given composition.
    /// TODO: Artisan plugins are not yet supported
    pub fn register_interactive_artisan(
        &self,
        _api_version: ae_sys::A_Version,
        _artisan_version: ae_sys::A_Version,
        _plugin_id: ae_sys::AEGP_PluginID,
        _refcon: *mut c_void,
        _match_name: &str,
        _artisan_name: &str,
        _entry_funcs: *mut ae_sys::PR_ArtisanEntryPoints,
    ) -> Result<(), Error> {
        todo!("Artisan plugins are not yet supported");
    }

    /// Call this to register as many strings as you like for name-replacement when presets are loaded.
    /// Any time a Property name is found, or referred to in an expression, and it starts with an ASCII tab character ('t'),
    /// followed by one of the English names, it will be replaced with the localized name.
    /// (In English the tab character will simply be removed).
    pub fn register_preset_localization_string(
        &self,
        english_name: &str,
        localized_name: &str,
    ) -> Result<(), Error> {
        let english_name_c = CString::new(english_name).map_err(|_| Error::InvalidParms)?;
        let localized_name_c = CString::new(localized_name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(
            self,
            AEGP_RegisterPresetLocalizationString,
            english_name_c.as_ptr(),
            localized_name_c.as_ptr()
        )
    }
}
