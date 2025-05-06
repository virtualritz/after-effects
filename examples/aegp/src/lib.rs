use after_effects::{
    aegp::{
        suites::{Command, Register, Utility},
        CommandHookStatus, HookPriority, MenuOrder,
    },
    define_general_plugin,
    sys::{AEGP_PluginID, SPBasicSuite},
    AegpPlugin, Error, PicaBasicSuite,
};

define_general_plugin!(Grabber);

#[derive(Clone, Debug)]
struct Grabber {}

impl AegpPlugin for Grabber {
    fn entry_point(
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: AEGP_PluginID,
    ) -> Result<Self, after_effects::Error> {
        log::debug!(
            "Aegp Demo Entry Point: v{major_version}.{minor_version} - id : {aegp_plugin_id}"
        );

        let res: Result<(), Error> = (|| {
            let command_suite = Command::new()?;
            let register_suite = Register::new()?;
            let grabba_cmd = command_suite.get_unique_command()?;

            command_suite.insert_command(
                "Grabber",
                grabba_cmd,
                after_effects::aegp::MenuId::Export,
                MenuOrder::Sorted,
            )?;

            register_suite.register_command_hook::<Grabber, _>(
                aegp_plugin_id,
                HookPriority::BeforeAE,
                grabba_cmd,
                Box::new(|_, _, _, _, _| {
                    // todo: print screen here
                    Ok(CommandHookStatus::Handled)
                }),
                (),
            )?;

            register_suite.register_update_menu_hook::<Grabber, _>(
                aegp_plugin_id,
                Box::new(move |_, _, _| {
                    let command_suite = Command::new().unwrap();
                    command_suite.enable_command(grabba_cmd)?;
                    Ok(())
                }),
                (),
            )?;

            Ok(())
        })();

        match res {
            Ok(_) => {}
            Err(e) => {
                let util_suite = Utility::new()?;
                util_suite.report_info_unicode(
                    aegp_plugin_id,
                    &format!("Error while loading AegpDemo {e:?}"),
                )?;
            }
        }

        Ok(Grabber {})
    }
}
