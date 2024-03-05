use after_effects as ae;
use after_effects_sys as ae_sys;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    MixChannels,
}

#[derive(Default)]
struct Plugin {}

#[derive(Default)]
struct Instance { _unused: u8 }

ae::define_plugin!(Plugin, Instance, Params);

fn detect_host(in_data: ae::InData) -> String {
    use ae_sys::*;
    let v = (in_data.version().0 as u32, in_data.version().1 as u32);

    let app = match &in_data.application_id() {
        #[rustfmt::skip]
        b"FXTC" => {
            if v.0 >= 12 {
                     if v.0 == PF_AE234_PLUG_IN_VERSION && v.1 >= PF_AE234_PLUG_IN_SUBVERS { "After Effects 2023 (23.4) or later." }
                else if v.0 == PF_AE220_PLUG_IN_VERSION && v.1 == PF_AE220_PLUG_IN_SUBVERS { "After Effects 2022 (22.0)." }
                else if v.0 == PF_AE184_PLUG_IN_VERSION && v.1 == PF_AE184_PLUG_IN_SUBVERS { "After Effects 2021 (18.4)." }
                else if v.0 == PF_AE182_PLUG_IN_VERSION && v.1 == PF_AE182_PLUG_IN_SUBVERS { "After Effects 2021 (18.2)." }
                else if v.0 == PF_AE180_PLUG_IN_VERSION && v.1 == PF_AE180_PLUG_IN_SUBVERS { "After Effects 2021 (18.0)." }
                else if v.0 == PF_AE177_PLUG_IN_VERSION && v.1 == PF_AE177_PLUG_IN_SUBVERS { "After Effects 2020 (17.7)." }
                else if v.0 == PF_AE176_PLUG_IN_VERSION && v.1 == PF_AE176_PLUG_IN_SUBVERS { "After Effects 2020 (17.6)." }
                else if v.0 == PF_AE175_PLUG_IN_VERSION && v.1 == PF_AE175_PLUG_IN_SUBVERS { "After Effects 2020 (17.5)." }
                else if v.0 == PF_AE171_PLUG_IN_VERSION && v.1 >= PF_AE171_PLUG_IN_SUBVERS { "After Effects 2020 (17.1)." }
                else if v.0 == PF_AE170_PLUG_IN_VERSION && v.1 == PF_AE170_PLUG_IN_SUBVERS { "After Effects 2020 (17.0)." }
                else if v.0 == PF_AE161_PLUG_IN_VERSION && v.1 == PF_AE161_PLUG_IN_SUBVERS { "After Effects 2019 (16.1)." }
                else if v.0 == PF_AE160_PLUG_IN_VERSION && v.1 >= PF_AE160_PLUG_IN_SUBVERS { "After Effects CC 2019 (16.0)." }
                else if v.0 == PF_AE151_PLUG_IN_VERSION && v.1 >= PF_AE151_PLUG_IN_SUBVERS { "After Effects CC 2018 (15.1)." }
                else if v.0 == PF_AE150_PLUG_IN_VERSION && v.1 >= PF_AE150_PLUG_IN_SUBVERS { "After Effects CC 2017 (15.0)." }
                else if v.0 == PF_AE140_PLUG_IN_VERSION && v.1 >= PF_AE140_PLUG_IN_SUBVERS { "After Effects CC 2017 (14.0)." }
                else if v.0 == PF_AE138_PLUG_IN_VERSION && v.1 == PF_AE138_PLUG_IN_SUBVERS { "After Effects CC 2015.3 (13.8)." }
                else if v.0 == PF_AE136_PLUG_IN_VERSION && v.1 == PF_AE136_PLUG_IN_SUBVERS { "After Effects CC 2015.1 or 2015.2 (13.6 or 13.7)."  }
                else if v.0 == PF_AE135_PLUG_IN_VERSION && v.1 == PF_AE135_PLUG_IN_SUBVERS { "After Effects CC 2015 (13.5)." }
                else if v.0 == PF_AE130_PLUG_IN_VERSION && v.1 == PF_AE130_PLUG_IN_SUBVERS { "After Effects CC 2014 (13.0 - 13.2)." }
                else if v.0 == PF_AE122_PLUG_IN_VERSION && v.1 == PF_AE122_PLUG_IN_SUBVERS { "After Effects CC (12.2)." }
                else if v.0 == PF_AE121_PLUG_IN_VERSION && v.1 == PF_AE121_PLUG_IN_SUBVERS { "After Effects CC (12.1)." }
                else if v.0 == PF_AE120_PLUG_IN_VERSION && v.1 == PF_AE120_PLUG_IN_SUBVERS { "After Effects CC (12.0)." }
                else if v.0 == PF_AE1101_PLUG_IN_VERSION && v.1 == PF_AE1101_PLUG_IN_SUBVERS { "After Effects CS6.0.1 or CS6.0.2." }
                else if v.0 == PF_AE110_PLUG_IN_VERSION && v.1 == PF_AE110_PLUG_IN_SUBVERS { "After Effects CS6.0." }
                else {
                    // Q. How can I tell the difference between versions where the API version is the same, such as AE 6.5 and 7.0?
                    // A. The effect API didn't change the only way to differentiate between them is to check for the presence of a version of a suite new in 7.0.
                    // Say, something 32bpc-ish. To avoid AEGP_SuiteHandler throwing if the suite isn't present, we'll acquire it the old-school way.
                    if pf::suites::IterateFloat::new().is_ok() {
                        "After Effects between 7.0 and CS4."
                    } else {
                        "After Effects 6.5 or earlier."
                    }
                }
            } else { // Wow, an antique!
                "some unknown version of After Effects!"
            }
        }
        b"PrMr" => {
            // let pixel_format = ae::pf::PixelFormatSuite::new().unwrap();
            // pixel_format.clear_supported_pixel_formats(in_data.effect_ref()).unwrap();
            // pixel_format.add_supported_pixel_format(in_data.effect_ref(), ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f).unwrap();

            // The major/minor versions provide basic differentiation.
            // If you need finer granularity, e.g. differentiating between
            // PPro CC 2015.3 and CC 2017, then use the App Info Suite from/ the Premiere Pro SDK
            if v.0 == 13 && v.1 >= 4 {
                "Premiere Pro CC, CC 2014, or later!"
            } else if v.0 == 13 && v.1 == 2 {
                "Premiere Pro CS6!"
            } else {
                "some unknown version of Premiere!"
            }
        }
        _ => "some oddball host.",
    };

    log::info!("Running in {app}");

    format!("Running in {app}")
}

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _in_data: InData, _out_data: OutData) -> Result<(), Error> {
        params.add_param(Params::MixChannels, "Mix channels", ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(0.0);
            f.set_slider_min(0.0);
            f.set_valid_max(200.0);
            f.set_slider_max(200.0);
            f.set_value(10.0);
            f.set_default(10.0);
            f.set_precision(1);
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }));
        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: ae::InData, mut out_data: ae::OutData, _params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Portable, v3.3\rThis example shows how to detect and respond to different hosts.\rCopyright 2007-2023 Adobe Inc.");
            }
            ae::Command::SequenceSetup => {
                out_data.set_return_msg(&detect_host(in_data));
            }
            _ => {}
        }
        Ok(())
    }
}

impl AdobePluginInstance for Instance {
    fn flatten(&self) -> Result<(u16, Vec<u8>), Error> {
        Ok((1, Vec::new()))
    }
    fn unflatten(_version: u16, _bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self { _unused: 0 })
    }

    fn user_changed_param(&mut self, _: &mut PluginState, _: Params) -> Result<(), ae::Error> { Ok(()) }

    fn render(
        &self,
        plugin: &mut PluginState,
        in_layer: &Layer,
        out_layer: &mut Layer,
    ) -> Result<(), ae::Error> {
        let slider_value = plugin.params.get(Params::MixChannels)?.as_float_slider()?.value() as f32;

        // If the slider is 0 just make a direct copy.
        if slider_value < 0.001 {
            out_layer.copy_from(in_layer, None, None)?;
        } else {
            let extent_hint = plugin.in_data.extent_hint();
            let out_extent_hint = out_layer.extent_hint();
            // clear all pixels outside extent_hint.
            if extent_hint != out_extent_hint {
                out_layer.fill(None, Some(out_extent_hint))?;
            }

            // iterate over image data.
            #[rustfmt::skip]
            in_layer.iterate_with(out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                let pixel = pixel.as_f32();

                // Mix the values. The higher the slider, the more we blend the channel with the average of all channels
                let average = (pixel.red + pixel.green + pixel.blue) / 3.0;
                // let midway_calc = (slider_value * average) + (200.0 - slider_value) * pixel.red;

                let r = ((slider_value * average) + (100.0 - slider_value) * pixel.red  ) / 100.0;
                let g = ((slider_value * average) + (100.0 - slider_value) * pixel.green) / 100.0;
                let b = ((slider_value * average) + (100.0 - slider_value) * pixel.blue ) / 100.0;

                match out_pixel {
                    ae::GenericPixelMut::Pixel8(out_pixel) => {
                        out_pixel.alpha = pixel.alpha as _;
                        out_pixel.red   = r.min(ae_sys::PF_MAX_CHAN8 as f32) as _;
                        out_pixel.green = g.min(ae_sys::PF_MAX_CHAN8 as f32) as _;
                        out_pixel.blue  = b.min(ae_sys::PF_MAX_CHAN8 as f32) as _;
                    }
                    ae::GenericPixelMut::Pixel16(out_pixel) => {
                        out_pixel.alpha = pixel.alpha as _;
                        out_pixel.red   = r.min(ae_sys::PF_MAX_CHAN16 as f32) as _;
                        out_pixel.green = g.min(ae_sys::PF_MAX_CHAN16 as f32) as _;
                        out_pixel.blue  = b.min(ae_sys::PF_MAX_CHAN16 as f32) as _;
                    }
                    _ => return Err(Error::BadCallbackParameter)
                }
                Ok(())
            })?;
        }

        Ok(())
    }

    fn handle_command(
        &mut self,
        _plugin: &mut PluginState,
        _cmd: ae::Command
    ) -> Result<(), ae::Error> {
        Ok(())
    }
}
