use after_effects_sys::{
    AEGP_AboutRefcon, AEGP_AboutStringRefcon, AEGP_CommandRefcon, AEGP_DeathRefcon,
    AEGP_GlobalRefcon, AEGP_UpdateMenuRefcon, AEGP_VersionRefcon,
};

use crate::*;
use std::ffi::CString;

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

#[derive(Debug, Clone)]
pub enum CommandHookStatus {
    Handled,
    Unhandled,
}

pub type UpdateMenuHook<G, R> =
    Box<dyn Fn(Option<&mut G>, &mut R, WindowType) -> Result<(), Error>>;

pub type CommandHook<G, R> = Box<
    dyn FnMut(
        Option<&mut G>,
        &mut R,
        ae_sys::AEGP_Command,
        HookPriority,
        bool,
    ) -> Result<CommandHookStatus, Error>,
>;

pub type DeathHook<G, R> = Box<dyn FnMut(Option<&mut G>, &mut R) -> Result<(), Error>>;

pub type VersionHook<G, R> = Box<dyn FnMut(Option<&mut G>, &mut R, &mut u32) -> Result<(), Error>>;

pub type AboutStringHook<G, R> =
    Box<dyn FnMut(Option<&mut G>, &mut R, &mut [u8]) -> Result<(), Error>>;

pub type AboutHook<G, R> = Box<dyn FnMut(Option<&mut G>, &mut R) -> Result<(), Error>>;

pub type IdleHook<G, R> = Box<dyn FnMut(Option<&mut G>, &mut R, &mut i32) -> Result<(), Error>>;

/// Note: functions in this suite take a `Global` Paramater, for AEGPs this must be the same as your global `AegpPlugin` type, for all
/// other plugins this should likely be the type you registered with the [UtilitySuite::register_aegp_plugin] function.
impl RegisterSuite {
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// Register a hook (command handler) function with After Effects.
    /// If you are replacing a function which After Effects also handles, `AEGP_HookPriority` determines whether your plug-in gets run first.
    pub fn register_command_hook<Global: AegpSeal, RefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        hook_priority: HookPriority,
        command: ae_sys::AEGP_Command,
        command_hook_func: CommandHook<Global, RefCon>,
        command_refcon: RefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn command_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_CommandRefcon,
            command: ae_sys::AEGP_Command,
            hook_priority: ae_sys::AEGP_HookPriority,
            already_handled: ae_sys::A_Boolean,
            handled_p: *mut ae_sys::A_Boolean,
        ) -> ae_sys::A_Err {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_command_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid CommandHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original RefCon type.
            let Some((callback, refcon)) = (unsafe { (refcon as *mut (CommandHook<P, T>, T)).as_mut() }) else {
                return Error::Generic.into();
            };

            let already_handled_bool = already_handled != 0;

            let hook_priority_enum = HookPriority::from(hook_priority);

            let res = callback(
                global,
                refcon,
                command,
                hook_priority_enum,
                already_handled_bool,
            );

            match res {
                Ok(CommandHookStatus::Handled) => {
                    // SAFETY: Write handled status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (1) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 1; }
                    Error::None
                }
                Ok(CommandHookStatus::Unhandled) => {
                    // SAFETY: Write unhandled status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (0) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 0; }
                    Error::None
                }
                Err(e) => {
                    // SAFETY: Write error status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (0) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 0; }
                    e
                }
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((command_hook_func, command_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterCommandHook,
            plugin_id,
            hook_priority.into(),
            command,
            Some(command_hook_wrapper::<Global, RefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your menu update function (which determines whether or not items are active),
    /// called every time any menu is to be drawn.
    pub fn register_update_menu_hook<Global: AegpSeal, UpdateMenuRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        update_menu_hook_func: UpdateMenuHook<Global, UpdateMenuRefCon>,
        update_menu_refcon: UpdateMenuRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn update_menu_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_UpdateMenuRefcon,
            window_type: ae_sys::AEGP_WindowType,
        ) -> sys::PF_Err {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_update_menu_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid UpdateMenuHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original UpdateMenuRefCon type.
            let Some((callback, refcon)) = (unsafe { (refcon as *mut (UpdateMenuHook<P, T>, T)).as_mut() })
            else {
                return Error::Generic.into();
            };

            match callback(global, refcon, window_type.into()) {
                Ok(_) => Error::None,
                Err(e) => e.into(),
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((update_menu_hook_func, update_menu_refcon));

        call_suite_fn!(
            self,
            AEGP_RegisterUpdateMenuHook,
            plugin_id,
            Some(update_menu_hook_wrapper::<Global, UpdateMenuRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your termination function. Called when the application quits.
    pub fn register_death_hook<Global: AegpSeal, DeathRefcon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        death_hook_func: DeathHook<Global, DeathRefcon>,
        death_refcon: DeathRefcon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn death_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_DeathRefcon,
        ) -> sys::PF_Err {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_death_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid DeathHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original DeathRefcon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (DeathHook<P, T>, T)) };
            match cb(global, refcon) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((death_hook_func, death_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterDeathHook,
            plugin_id,
            Some(death_hook_wrapper::<Global, DeathRefcon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_version_hook<Global: AegpSeal, VersionRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        version_hook_func: VersionHook<Global, VersionRefCon>,
        version_refcon: VersionRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn version_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_VersionRefcon,
            pf_version_p: *mut ae_sys::A_u_long,
        ) -> ae_sys::A_Err {
            log::error!(
                "The after effects documentation said version hook should never be called!"
            );
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_version_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid VersionHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original VersionRefCon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (VersionHook<P, T>, T)) };
            // SAFETY: Dereference and cast pf_version_p to mutable reference to u32.
            // Detailed explanation: (1) pf_version_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for reads/writes as provided by AE caller, (3) A_u_long and u32 have compatible representations on target platforms.
            // Would be UB if: pf_version_p was null, pointed to invalid memory, or A_u_long size differs from u32 on the platform.
            let pf_version = unsafe { &mut (*pf_version_p as u32) };

            match cb(global, refcon, pf_version) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_version_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((version_hook_func, version_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterVersionHook,
            plugin_id,
            Some(version_hook_wrapper::<Global, VersionRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_string_hook<Global: AegpSeal, AboutString>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_string_hook_func: AboutStringHook<Global, AboutString>,
        about_string_refcon: AboutString,
    ) -> Result<(), Error> {
        unsafe extern "C" fn about_string_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_AboutStringRefcon,
            // We have 0 documentation about this pointer
            // besides that it is never constructed, so it's treated as null
            _about_z: *mut ae_sys::A_char,
        ) -> ae_sys::A_Err {
            log::error!(
                "The after effects documentation said about string hook should never be called!"
            );
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_about_string_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid AboutStringHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original AboutString type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (AboutStringHook<P, T>, T)) };

            match cb(global, refcon, &mut []) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_about_string_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_string_hook_func, about_string_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutStringHook,
            plugin_id,
            Some(about_string_hook_wrapper::<Global, AboutString>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_hook<Global: AegpSeal, About>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_hook_func: AboutHook<Global, About>,
        about_refcon: About,
    ) -> Result<(), Error> {
        unsafe extern "C" fn about_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: AEGP_AboutRefcon,
        ) -> ae_sys::A_Err {
            log::error!("The after effects documentation said about hook should never be called!");
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_about_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid AboutHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original About type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (AboutHook<P, T>, T)) };

            match cb(global, refcon) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_about_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_hook_func, about_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutHook,
            plugin_id,
            Some(about_hook_wrapper::<Global, About>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your IdleHook function. After Effects will call the function sporadically,
    /// while the user makes difficult artistic decisions (or while they're getting more coffee).
    pub fn register_idle_hook<Global: AegpSeal, IdleRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        idle_hook_func: IdleHook<Global, IdleRefCon>,
        idle_refcon: IdleRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn idle_hook_wrapper<P, T>(
            plugin_refcon: AEGP_GlobalRefcon,
            refcon: ae_sys::AEGP_IdleRefcon,
            max_sleep_p: *mut ae_sys::A_long,
        ) -> ae_sys::A_Err {
            let global = if plugin_refcon.is_null() {
                None
            } else {
                // SAFETY: Cast plugin_refcon to mutable reference to Global type.
                // Detailed explanation: (1) plugin_refcon is non-null as checked above, (2) pointer was originally created from valid P instance by AE SDK, (3) lifetime is bounded by this callback invocation which AE guarantees doesn't outlive the plugin.
                // Would be UB if: plugin_refcon pointed to freed memory, was misaligned, or was accessed concurrently from another thread.
                Some(unsafe { &mut *(plugin_refcon as *mut P) })
            };

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_idle_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid IdleHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original IdleRefCon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (IdleHook<P, T>, T)) };
            // SAFETY: Dereference max_sleep_p to mutable reference to A_long.
            // Detailed explanation: (1) max_sleep_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for reads/writes as provided by AE caller, (3) lifetime is bounded by this callback invocation.
            // Would be UB if: max_sleep_p was null, pointed to invalid memory, or was already freed.
            let max_sleep = unsafe { &mut (*max_sleep_p) };

            match cb(global, refcon, max_sleep) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((idle_hook_func, idle_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterIdleHook,
            plugin_id,
            Some(idle_hook_wrapper::<Global, IdleRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
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

pub type NonAegpUpdateMenuHook<R> = Box<dyn Fn(&mut R, WindowType) -> Result<(), Error>>;

pub type NonAegpCommandHook<R> = Box<
    dyn FnMut(&mut R, ae_sys::AEGP_Command, HookPriority, bool) -> Result<CommandHookStatus, Error>,
>;

pub type NonAegpDeathHook<R> = Box<dyn FnMut(&mut R) -> Result<(), Error>>;

pub type NonAegpVersionHook<R> = Box<dyn FnMut(&mut R, &mut u32) -> Result<(), Error>>;

pub type NonAegpAboutStringHook<R> = Box<dyn FnMut(&mut R, &mut [u8]) -> Result<(), Error>>;

pub type NonAegpAboutHook<R> = Box<dyn FnMut(&mut R) -> Result<(), Error>>;

pub type NonAegpIdleHook<R> = Box<dyn FnMut(&mut R, &mut i32) -> Result<(), Error>>;

define_suite!(
    RegisterNonAegpSuite,
    AEGP_RegisterSuite5,
    kAEGPRegisterSuite,
    kAEGPRegisterSuiteVersion5
);

/// Note: functions in this suite assume the plugin_refcon is always null.
/// This is appropriate for non-AEGP plugins.
impl RegisterNonAegpSuite {
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// Register a hook (command handler) function with After Effects.
    /// If you are replacing a function which After Effects also handles, `AEGP_HookPriority` determines whether your plug-in gets run first.
    pub fn register_command_hook<RefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        hook_priority: HookPriority,
        command: ae_sys::AEGP_Command,
        command_hook_func: NonAegpCommandHook<RefCon>,
        command_refcon: RefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn command_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_CommandRefcon,
            command: ae_sys::AEGP_Command,
            hook_priority: ae_sys::AEGP_HookPriority,
            already_handled: ae_sys::A_Boolean,
            handled_p: *mut ae_sys::A_Boolean,
        ) -> ae_sys::A_Err {
            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_command_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpCommandHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original RefCon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpCommandHook<T>, T)) };
            let already_handled_bool = already_handled != 0;

            let hook_priority_enum = HookPriority::from(hook_priority);

            let res = cb(refcon, command, hook_priority_enum, already_handled_bool);

            match res {
                Ok(CommandHookStatus::Handled) => {
                    // SAFETY: Write handled status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (1) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 1; }
                    Error::None
                }
                Ok(CommandHookStatus::Unhandled) => {
                    // SAFETY: Write unhandled status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (0) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 0; }
                    Error::None
                }
                Err(e) => {
                    // SAFETY: Write error status to AE output pointer.
                    // Detailed explanation: (1) handled_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for writes as provided by AE caller, (3) writing boolean value (0) is valid for A_Boolean type.
                    // Would be UB if: handled_p was null, pointed to read-only memory, or was already freed.
                    unsafe { *handled_p = 0; }
                    e
                }
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((command_hook_func, command_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterCommandHook,
            plugin_id,
            hook_priority.into(),
            command,
            Some(command_hook_wrapper::<RefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your menu update function (which determines whether or not items are active),
    /// called every time any menu is to be drawn.
    pub fn register_update_menu_hook<UpdateMenuRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        update_menu_hook_func: NonAegpUpdateMenuHook<UpdateMenuRefCon>,
        update_menu_refcon: UpdateMenuRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn update_menu_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_UpdateMenuRefcon,
            window_type: ae_sys::AEGP_WindowType,
        ) -> sys::PF_Err {
            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_update_menu_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpUpdateMenuHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original UpdateMenuRefCon type.
            let (callback, refcon) = unsafe { &mut *(refcon as *mut (NonAegpUpdateMenuHook<T>, T)) };

            match callback(refcon, window_type.into()) {
                Ok(_) => Error::None,
                Err(e) => e.into(),
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((update_menu_hook_func, update_menu_refcon));

        call_suite_fn!(
            self,
            AEGP_RegisterUpdateMenuHook,
            plugin_id,
            Some(update_menu_hook_wrapper::<UpdateMenuRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your termination function. Called when the application quits.
    pub fn register_death_hook<DeathRefcon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        death_hook_func: NonAegpDeathHook<DeathRefcon>,
        death_refcon: DeathRefcon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn death_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_DeathRefcon,
        ) -> sys::PF_Err {
            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_death_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpDeathHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original DeathRefcon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpDeathHook<T>, T)) };
            match cb(refcon) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((death_hook_func, death_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterDeathHook,
            plugin_id,
            Some(death_hook_wrapper::<DeathRefcon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_version_hook<VersionRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        version_hook_func: NonAegpVersionHook<VersionRefCon>,
        version_refcon: VersionRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn version_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_VersionRefcon,
            pf_version_p: *mut ae_sys::A_u_long,
        ) -> ae_sys::A_Err {
            log::error!(
                "The after effects documentation said version hook should never be called!"
            );

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_version_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpVersionHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original VersionRefCon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpVersionHook<T>, T)) };
            // SAFETY: Dereference and cast pf_version_p to mutable reference to u32.
            // Detailed explanation: (1) pf_version_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for reads/writes as provided by AE caller, (3) A_u_long and u32 have compatible representations on target platforms.
            // Would be UB if: pf_version_p was null, pointed to invalid memory, or A_u_long size differs from u32 on the platform.
            let pf_version = unsafe { &mut (*pf_version_p as u32) };

            match cb(refcon, pf_version) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_version_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((version_hook_func, version_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterVersionHook,
            plugin_id,
            Some(version_hook_wrapper::<VersionRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_string_hook<AboutString>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_string_hook_func: NonAegpAboutStringHook<AboutString>,
        about_string_refcon: AboutString,
    ) -> Result<(), Error> {
        unsafe extern "C" fn about_string_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_AboutStringRefcon,
            // We have 0 documentation about this pointer
            // besides that it is never constructed, so it's treated as null
            _about_z: *mut ae_sys::A_char,
        ) -> ae_sys::A_Err {
            log::error!(
                "The after effects documentation said about string hook should never be called!"
            );

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_about_string_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpAboutStringHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original AboutString type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpAboutStringHook<T>, T)) };

            match cb(refcon, &mut []) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_about_string_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_string_hook_func, about_string_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutStringHook,
            plugin_id,
            Some(about_string_hook_wrapper::<AboutString>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Currently not called.
    pub fn register_about_hook<About>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        about_hook_func: NonAegpAboutHook<About>,
        about_refcon: About,
    ) -> Result<(), Error> {
        unsafe extern "C" fn about_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: AEGP_AboutRefcon,
        ) -> ae_sys::A_Err {
            log::error!("The after effects documentation said about hook should never be called!");

            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_about_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpAboutHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original About type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpAboutHook<T>, T)) };

            match cb(refcon) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        log::warn!("Called `register_about_hook`, this does nothing!");

        let refcon_cb_tuple = Box::new((about_hook_func, about_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterAboutHook,
            plugin_id,
            Some(about_hook_wrapper::<About>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
    }

    /// Register your IdleHook function. After Effects will call the function sporadically,
    /// while the user makes difficult artistic decisions (or while they're getting more coffee).
    pub fn register_idle_hook<IdleRefCon>(
        &self,
        plugin_id: ae_sys::AEGP_PluginID,
        idle_hook_func: NonAegpIdleHook<IdleRefCon>,
        idle_refcon: IdleRefCon,
    ) -> Result<(), Error> {
        unsafe extern "C" fn idle_hook_wrapper<T>(
            _: AEGP_GlobalRefcon,
            refcon: ae_sys::AEGP_IdleRefcon,
            max_sleep_p: *mut ae_sys::A_long,
        ) -> ae_sys::A_Err {
            // SAFETY: Cast refcon to mutable reference to tuple containing callback and user data.
            // Detailed explanation: (1) refcon was created via Box::into_raw in register_idle_hook, (2) pointer ownership is transferred to AE which passes it back unmodified, (3) tuple contains valid NonAegpIdleHook and RefCon types.
            // Would be UB if: refcon was null, pointed to freed/invalid memory, or type T didn't match the original IdleRefCon type.
            let (cb, refcon) = unsafe { &mut *(refcon as *mut (NonAegpIdleHook<T>, T)) };
            // SAFETY: Dereference max_sleep_p to mutable reference to A_long.
            // Detailed explanation: (1) max_sleep_p is guaranteed non-null by AE SDK contract, (2) pointer is valid for reads/writes as provided by AE caller, (3) lifetime is bounded by this callback invocation.
            // Would be UB if: max_sleep_p was null, pointed to invalid memory, or was already freed.
            let max_sleep = unsafe { &mut (*max_sleep_p) };

            match cb(refcon, max_sleep) {
                Ok(_) => Error::None,
                Err(e) => e,
            }
            .into()
        }

        let refcon_cb_tuple = Box::new((idle_hook_func, idle_refcon));
        call_suite_fn!(
            self,
            AEGP_RegisterIdleHook,
            plugin_id,
            Some(idle_hook_wrapper::<IdleRefCon>),
            Box::into_raw(refcon_cb_tuple) as *mut _,
        )
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
