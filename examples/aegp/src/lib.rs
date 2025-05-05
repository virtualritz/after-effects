use after_effects::{
    aegp::suites::Utility, define_general_plugin, sys::SPBasicSuite, AegpPlugin, Error,
    PicaBasicSuite,
};

define_general_plugin!(Grabber);

struct Grabber {}

impl AegpPlugin for Grabber {
    fn entry_point(
        basic_suite: after_effects::PicaBasicSuite,
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: i32,
    ) -> Result<Self, after_effects::Error> {
        let util_suite = Utility::new()?;
        util_suite.report_info_unicode(aegp_plugin_id, "Succesfully Loaded Test AEGP")?;
        Ok(Grabber {})
    }
}
