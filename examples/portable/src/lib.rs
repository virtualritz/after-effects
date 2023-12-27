use ae::register_plugin;
use after_effects as ae;
use after_effects_sys as ae_sys;

#[derive(Eq, PartialEq, Hash)]
enum MyParams {
    Slider,
}

#[derive(Default)]
struct PortablePlugin {}

#[derive(Default)]
struct PortableInstance {}

register_plugin!(PortablePlugin, PortableInstance, MyParams);

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
                    if let Ok(_) = ae::pf::IterateFloatSuite::new() {
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

impl AdobePluginGlobal for PortablePlugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<MyParams>) -> Result<(), Error> {
        params.add_param(
            MyParams::Slider,
            "Mix channels",
            *ae::FloatSliderDef::new()
                .set_valid_min(0.0)
                .set_slider_min(0.0)
                .set_valid_max(200.0)
                .set_slider_max(200.0)
                .set_value(10.0)
                .set_default(10.0)
                .precision(1)
                .display_flags(ae::ValueDisplayFlag::PERCENT),
        );
        Ok(())
    }

    fn handle_command(
        &self,
        cmd: ae::Command,
        in_data: ae::InData,
        mut out_data: ae::OutData,
    ) -> Result<(), ae::Error> {
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

impl AdobePluginInstance for PortableInstance {
    fn flatten(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }
    fn unflatten(_bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {})
    }

    fn render(
        &self,
        in_data: ae::InData,
        in_layer: &Layer,
        out_layer: &mut Layer,
        params: &ae::Parameters<MyParams>,
    ) -> Result<(), ae::Error> {
        let slider_value =
            if let Some(ae::Param::FloatSlider(slider)) = params.get(MyParams::Slider) {
                slider.value()
            } else {
                0.0
            };

        // If the slider is 0 just make a direct copy.
        if slider_value < 0.001 {
            out_layer.copy_from(in_layer, None, None)?;
        } else {
            let extent_hint = in_data.extent_hint();
            let out_extent_hint = out_layer.extent_hint();
            // clear all pixels outside extent_hint.
            if extent_hint.left != out_extent_hint.left
                || extent_hint.top != out_extent_hint.top
                || extent_hint.right != out_extent_hint.right
                || extent_hint.bottom != out_extent_hint.bottom
            {
                out_layer.fill(Pixel::default(), Some(out_extent_hint))?;
            }

            // iterate over image data.
            let progress_height = extent_hint.top - extent_hint.bottom;
            #[rustfmt::skip]
            in_layer.iterate(out_layer, 0, progress_height, extent_hint, |_x: i32, _y: i32, pixel: ae::Pixel| -> Result<ae::Pixel, Error> {
                // Mix the values. The higher the slider, the more we blend the channel with the average of all channels
                let average = (pixel.red as f64 + pixel.green as f64 + pixel.blue as f64) / 3.0;
                // let midway_calc = (slider_value * average) + (200.0 - slider_value) * pixel.red as f64;

                Ok(ae::Pixel {
                    alpha: pixel.alpha,
                    red:   (((slider_value * average) + (100.0 - slider_value) * pixel.red   as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8,
                    green: (((slider_value * average) + (100.0 - slider_value) * pixel.green as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8,
                    blue:  (((slider_value * average) + (100.0 - slider_value) * pixel.blue  as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8,
                })
            })?;
        }

        Ok(())
    }

    fn handle_command(
        &mut self,
        _cmd: ae::Command,
        _in_data: ae::InData,
        _out_data: ae::OutData,
    ) -> Result<(), ae::Error> {
        Ok(())
    }
}
