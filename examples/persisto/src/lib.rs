use after_effects::{
    AegpPlugin, Error,
    aegp::{
        CommandHookStatus, HookPriority, MenuId, MenuOrder, PersistentType,
        suites::{Command, PersistentData, Register, Utility},
    },
    define_general_plugin,
    sys::{AEGP_Command, AEGP_PluginID},
};

define_general_plugin!(Persisto);

const SECTION_KEY: &str = "Persisto Section";
const VALUE_KEY_1: &str = "Fuzziness";
const VALUE_KEY_2: &str = "Description";
const DEFAULT_STRING: &str = "Default Description";
const MENU_ITEM: &str = "Persisto Demo...";

const DEFAULT_FUZZINESS: f64 = 42.0;

#[derive(Clone, Debug)]
struct Persisto {
    _plugin_id: AEGP_PluginID,
    _persisto_cmd: AEGP_Command,
}

impl AegpPlugin for Persisto {
    fn entry_point(
        _major_version: i32,
        _minor_version: i32,
        aegp_plugin_id: AEGP_PluginID,
    ) -> Result<Self, Error> {
        log::debug!("Persisto Entry Point - id: {}", aegp_plugin_id);

        let command_suite = Command::new()?;
        let register_suite = Register::new()?;

        // Get unique command ID
        let persisto_cmd = command_suite.get_unique_command()?;

        // Insert menu command
        command_suite.insert_command(MENU_ITEM, persisto_cmd, MenuId::File, MenuOrder::Sorted)?;

        // Register command hook
        register_suite.register_command_hook::<Persisto, _>(
            aegp_plugin_id,
            HookPriority::BeforeAE,
            persisto_cmd,
            Box::new(move |_, _, command, _, _| {
                if command == persisto_cmd {
                    if let Err(e) = handle_persisto_command(aegp_plugin_id) {
                        log::error!("Error in Persisto command: {:?}", e);
                        if let Ok(util_suite) = Utility::new() {
                            let _ = util_suite.report_info_unicode(
                                aegp_plugin_id,
                                &format!("Persisto Error: {:?}", e),
                            );
                        }
                    }
                    return Ok(CommandHookStatus::Handled);
                }
                Ok(CommandHookStatus::Unhandled)
            }),
            (),
        )?;

        // Register update menu hook
        register_suite.register_update_menu_hook::<Persisto, _>(
            aegp_plugin_id,
            Box::new(move |_, _, _| {
                // Always enable the command (similar to C++ version)
                if let Ok(command_suite) = Command::new() {
                    let _ = command_suite.enable_command(persisto_cmd);
                }
                Ok(())
            }),
            (),
        )?;

        Ok(Persisto {
            _plugin_id: aegp_plugin_id,
            _persisto_cmd: persisto_cmd,
        })
    }
}

fn handle_persisto_command(plugin_id: AEGP_PluginID) -> Result<(), Error> {
    let persistent_suite = PersistentData::new()?;
    let util_suite = Utility::new()?;

    let blob = persistent_suite.application_blob(PersistentType::MachineSpecific)?;

    //  Check/modify alpha preferen
    // (This demonstrates reading/writing system preferences)
    let pref_section = "Main Pref Section";
    let pref_key = "Pref_DEFAULT_UNLABELED_ALPHA";

    let ask_exists = persistent_suite.does_key_exist(blob, pref_section, pref_key)?;

    if !ask_exists {
        // Set the preference to false (0) if it doesn't exist
        let default_value: i32 = 0;
        persistent_suite.set_long(blob, pref_section, pref_key, default_value)?;
    }

    // Check if our value already exists
    let found_my_stuff = persistent_suite.does_key_exist(blob, SECTION_KEY, VALUE_KEY_1)?;

    // Get or set the float value
    let value = persistent_suite.fp_long(blob, SECTION_KEY, VALUE_KEY_1, DEFAULT_FUZZINESS)?;

    // Report status to user
    let message = if found_my_stuff && (value - DEFAULT_FUZZINESS).abs() < 0.0001 {
        "Value existed with default value"
    } else if found_my_stuff {
        &format!("Different value set: {}", value)
    } else {
        "New value added"
    };

    util_suite.report_info_unicode(plugin_id, message)?;

    let buffer_size = 256;
    let string_value =
        persistent_suite.string(blob, SECTION_KEY, VALUE_KEY_2, DEFAULT_STRING, buffer_size)?;

    log::debug!("String value: {}", string_value);

    // Demonstrate other persistence operations
    demonstrate_persistence_features(plugin_id, blob)?;

    Ok(())
}

fn demonstrate_persistence_features(
    plugin_id: AEGP_PluginID,
    blob: after_effects::aegp::PersistentBlobHandle,
) -> Result<(), Error> {
    let persistent_suite = PersistentData::new()?;
    let util_suite = Utility::new()?;

    // Demonstrate setting various types of data
    let demo_section = "Persisto Demo Section";

    // Set a long value
    persistent_suite.set_long(blob, demo_section, "counter", 100)?;

    // Set a float value
    persistent_suite.set_fp_long(blob, demo_section, "precision", 3.14159)?;
    let float = persistent_suite.set_fp_long(blob, demo_section, "precision", 3.50)?;

    log::debug!("Inserted float with value `3.14159` retrieved from `precision`: {float:?}");

    // Set a string value
    persistent_suite.set_string(blob, demo_section, "name", "Persisto Demo")?;
    persistent_suite.set_string(blob, demo_section, "utf-8", "utf-8 牛奶")?;
    persistent_suite.set_string(blob, demo_section, "whitespace", "one \n two \n three \n")?;

    // Get String
    let string =
        persistent_suite.string(blob, demo_section, "name", "Error - Key should exist", 256)?;

    log::debug!("Inserted string `Persisto Demo` into `name` --- retrieved: {string}");

    let string =
        persistent_suite.string(blob, demo_section, "utf-8", "Error - Key should exist", 256)?;

    log::debug!("Inserted string `utf-8 牛奶` into key `utf-8` --- retrieved: {string}");

    let string = persistent_suite.string(
        blob,
        demo_section,
        "whitespace",
        "Error - Key should exist",
        256,
    )?;

    log::debug!(
        "Inserted string `one \n two \n three \n` into key `whitespace` --- retrieved : {string}"
    );

    // Set time value
    let time = after_effects::Time {
        value: 1000,
        scale: 30,
    };
    persistent_suite.set_time(blob, demo_section, "timestamp", &time)?;

    // Set ARGB color value
    let color = after_effects::sys::PF_PixelFloat {
        alpha: 1.0,
        red: 0.5,
        green: 0.75,
        blue: 0.25,
    };
    persistent_suite.set_argb(blob, demo_section, "theme_color", &color)?;

    // Count sections and keys
    let num_sections = persistent_suite.num_sections(blob)?;
    let num_keys = persistent_suite.num_keys(blob, demo_section)?;

    util_suite.report_info_unicode(
        plugin_id,
        &format!(
            "Persisto: {} sections, {} keys in demo section",
            num_sections, num_keys
        ),
    )?;

    // List all keys in our demo section
    log::debug!("Keys in {}: ", demo_section);
    for i in 0..num_keys {
        if let Ok(key_name) = persistent_suite.value_by_key_index(blob, demo_section, i, 256) {
            log::debug!("  [{}]: {}", i, key_name);
        }
    }

    // Get preferences directory path
    if let Ok(prefs_dir) = persistent_suite.prefs_directory() {
        log::debug!("Preferences directory: {}", prefs_dir);
    }

    Ok(())
}
