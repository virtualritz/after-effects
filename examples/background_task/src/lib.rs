use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Option,
}

#[derive(Default)]
struct Plugin {}

static PLUGIN_ID: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(
        &self,
        params: &mut ae::Parameters<Params>,
        _: ae::InData,
        _: ae::OutData,
    ) -> Result<(), Error> {
        params.add(
            Params::Option,
            "Check The logs",
            ae::PopupDef::setup(|f| {
                f.set_default(0);
                f.set_options(&["check", "the", "logs"]);
            }),
        )
    }

    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        _: ae::OutData,
        _: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::GlobalSetup => {
                let utility = ae::aegp::suites::Utility::new()?;

                // You can also use all the other useful tools in the register suite!
                // not just the idle hook!
                let register = ae::aegp::suites::RegisterNonAegp::new().unwrap();

                // We don't currently support registering a global refcon -
                // Send messages in channels instead
                let plugin_id = utility.register_with_aegp(None, "BackgroundTask")?;

                PLUGIN_ID.set(plugin_id).unwrap();

                // the register suite can be used on the *main* thread in effect plugins
                // - using it elsewhere will result in tragedy.
                register.register_idle_hook(
                    plugin_id,
                    Box::new(|_, _min_time| {
                        log::info!("Background Task plugin called - running useless task on main thread");
                        Ok(())
                    }),
                    (),
                ).unwrap();
            }
            ae::Command::Render {
                in_layer,
                mut out_layer,
            } => {
                let extent_hint = in_data.extent_hint();

                in_layer.iterate_with(
                    &mut out_layer,
                    0,
                    extent_hint.height(),
                    Some(extent_hint),
                    |_x: i32,
                     _y: i32,
                     pixel: ae::GenericPixel,
                     out_pixel: ae::GenericPixelMut|
                     -> Result<(), Error> {
                        match (pixel, out_pixel) {
                            (
                                ae::GenericPixel::Pixel8(pixel),
                                ae::GenericPixelMut::Pixel8(out_pixel),
                            ) => {
                                *out_pixel = *pixel;
                            }
                            (
                                ae::GenericPixel::Pixel16(pixel),
                                ae::GenericPixelMut::Pixel16(out_pixel),
                            ) => {
                                *out_pixel = *pixel;
                            }
                            _ => return Err(Error::BadCallbackParameter),
                        }
                        Ok(())
                    },
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}
