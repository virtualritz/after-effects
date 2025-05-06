use after_effects::{
    aegp::suites::Utility,
    define_general_plugin,
    sys::{AEGP_PluginID, SPBasicSuite},
    AegpPlugin, Error, PicaBasicSuite,
};

define_general_plugin!(Grabber);

struct Grabber {}

impl AegpPlugin for Grabber {
    fn entry_point(
        basic_suite: after_effects::PicaBasicSuite,
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: AEGP_PluginID,
    ) -> Result<Self, after_effects::Error> {
        log::debug!(
            "Aegp Demo Entry Point: v{major_version}.{minor_version} - id : {aegp_plugin_id}"
        );

        let util_suite = Utility::new()?;
        util_suite.report_info_unicode(aegp_plugin_id, "Succesfully Loaded Test AEGP")?;
        Ok(Grabber {})
    }
}
