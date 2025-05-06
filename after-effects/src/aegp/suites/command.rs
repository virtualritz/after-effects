use std::ffi::CString;

use after_effects_sys::AEGP_Command;

use crate::*;

define_suite!(
    CommandSuite,
    AEGP_CommandSuite1,
    kAEGPCommandSuite,
    kAEGPCommandSuiteVersion1
);

define_enum! {
    ae_sys::AEGP_MenuID,
    MenuId {
        None = ae_sys::AEGP_Menu_NONE,
        Apple = ae_sys::AEGP_Menu_APPLE,
        File = ae_sys::AEGP_Menu_FILE,
        Edit = ae_sys::AEGP_Menu_EDIT,
        Composition = ae_sys::AEGP_Menu_COMPOSITION,
        Layer = ae_sys::AEGP_Menu_LAYER,
        Effect = ae_sys::AEGP_Menu_EFFECT,
        Window = ae_sys::AEGP_Menu_WINDOW,
        Floaters = ae_sys::AEGP_Menu_FLOATERS,
        KfAssist = ae_sys::AEGP_Menu_KF_ASSIST,
        Import = ae_sys::AEGP_Menu_IMPORT,
        SaveFrameAs = ae_sys::AEGP_Menu_SAVE_FRAME_AS,
        Prefs = ae_sys::AEGP_Menu_PREFS,
        Export = ae_sys::AEGP_Menu_EXPORT,
        Animation = ae_sys::AEGP_Menu_ANIMATION,
        Purge = ae_sys::AEGP_Menu_PURGE,
        New = ae_sys::AEGP_Menu_NEW,
    }
}

impl CommandSuite {
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// Obtain a unique command identifier. Use the Register Suite to register a handler for the command.
    ///
    /// Note: On occasion After Effects will send command 0 (zero), so don't use that as part of your command handling logic.
    pub fn get_unique_command(&self) -> Result<AEGP_Command, Error> {
        call_suite_fn_single!(self, AEGP_GetUniqueCommand -> ae_sys::AEGP_Command)
    }

    /// Set menu name of a command.
    pub fn set_command_name(&self, command_name: &str, command: AEGP_Command) -> Result<(), Error> {
        let command_name = CString::new(command_name).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetMenuCommandName,
            command,
            command_name.as_ptr()
        )
    }

    /// Inserts a command into a menu
    pub fn insert_command(
        &self,
        command_name: &str,
        command_id: AEGP_Command,
        menu_id: MenuId,
        after_item: i32,
    ) -> Result<(), Error> {
        let command_name = CString::new(command_name).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_InsertMenuCommand,
            command_id,
            command_name.as_ptr(),
            menu_id.into(),
            after_item,
        )
    }

    /// Removes a command from After Effects.
    pub fn remove_command(&self, command_id: AEGP_Command) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveMenuCommand, command_id)
    }

    /// Enable a menu command.
    pub fn enable_command(&self, command: AEGP_Command) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_EnableCommand, command)
    }

    /// Disable a menu command.
    pub fn disable_command(&self, command: AEGP_Command) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisableCommand, command)
    }

    /// After Effects will draw a check mark next to the menu command.
    pub fn check_mark_menu_command(&self, command: AEGP_Command, check: bool) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_CheckMarkMenuCommand,
            command,
            check as ae_sys::A_Boolean
        )
    }

    /// Call the handler for a specified menu command. Every After Effects menu item has an associated command.
    /// Note that we make no guarantees that command IDs will be consistent from version to version.
    pub fn do_command(&self, command: AEGP_Command) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DoCommand, command)
    }
}
