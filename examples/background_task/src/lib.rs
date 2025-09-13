use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Option,
}

#[derive(Default)]
struct Plugin;

#[derive(Debug)]
struct Global {
    counter: usize,
}

// If you plan on storing your plugin state in the after effects global
// bless it with this marker trait. do this *once and only once*
// in any given plugin.
unsafe impl AegpSeal for Global {}

static PLUGIN_ID: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
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

                let plugin_id = if cfg!(feature = "global_refcon") {
                    utility.register_with_aegp_refcon(Global { counter: 0 }, "BackgroundTask")?
                } else {
                    utility.register_with_aegp("BackgroundTask")?
                };

                PLUGIN_ID.set(plugin_id).unwrap();

                // the register suite can be used on the *main* thread in effect plugins
                // - using it elsewhere will result in tragedy.
                // You can also use all the other useful tools in the register suite!
                // not just the idle hook!
                if cfg!(feature = "global_refcon") {
                    // If you want access to global refcons stored for static lifetimes by AE, use the
                    // register suite.
                    let register = ae::aegp::suites::Register::new().unwrap();
                    register
                        .register_idle_hook::<Global, _>(
                            plugin_id,
                            Box::new(|global, _, _min_time| {
                                // Global is guaranteed to be `Some` unless you implemented `AegpSeal` without calling
                                // `RegisterSuite::register_with_aegp_store_global`
                                log::info!(
                                    "Background Task plugin called - Global counter: {}",
                                    global.as_ref().unwrap().counter
                                );

                                global.unwrap().counter += 1;

                                Ok(())
                            }),
                            (),
                        )
                        .unwrap();
                } else {
                    // If you don't care about storing global data use the RegisterNonAegp suite.
                    let register = ae::aegp::suites::RegisterNonAegp::new().unwrap();
                    register.register_idle_hook(
                        plugin_id,
                        Box::new(|_, _min_time| {
                            log::info!("Background Task plugin called - running useless task on main thread");
                            Ok(())
                        }),
                        (),
                    ).unwrap();
                };
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
