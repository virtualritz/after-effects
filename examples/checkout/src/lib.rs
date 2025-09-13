use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Frame,
    Layer
}
const CHECK_FRAME_MIN:  i32 = -100;
const CHECK_FRAME_MAX:  i32 = 100;
const CHECK_FRAME_DFLT: i32 = 0;

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: ae::InData, _: ae::OutData) -> Result<(), Error> {
        params.add(Params::Frame, "Frame offset", ae::SliderDef::setup(|f| {
            f.set_slider_min(CHECK_FRAME_MIN);
            f.set_slider_max(CHECK_FRAME_MAX);
            f.set_valid_min(CHECK_FRAME_MIN);
            f.set_valid_max(CHECK_FRAME_MAX);
            f.set_default(CHECK_FRAME_DFLT);
            f.set_value(f.default());
        }))?;

        params.add(Params::Layer, "Layer to checkout", ae::LayerDef::setup(|f| {
            f.set_default_to_this_layer();
        }))?;

        if !in_data.is_premiere() {
            ae::pf::suites::EffectUI::new()?
                .set_options_button_name(in_data.effect_ref(), "Whatever I want!")?;
        }

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: ae::InData, mut out_data: ae::OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Checkout, v2.6,\rChecks out layers at other times.\rCopyright 1994-2023\r\rAdobe Inc.");
            }
            ae::Command::DoDialog => {
                out_data.set_error_msg("This would be a fine place for\ra platform-specific options dialog.");
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                // Premiere Pro/Elements does not support this suite
                if !in_data.is_premiere() {
                    let cs = ae::pf::suites::Channel::new()?;
                    let num_channels = cs.layer_channel_count(in_data.effect_ref(), 0)?;
                    if num_channels > 0 {
                        if let Some((ref_, desc)) = cs.layer_channel_typed_ref_and_desc(in_data.effect_ref(), 0, ae::ChannelType::Depth)? {
                            let chunk = cs.checkout_layer_channel(in_data.effect_ref(), &ref_, in_data.current_time(), in_data.time_step(), in_data.time_scale(), desc.data_type.into())?;

                            // do interesting 3d stuff here;

                            cs.checkin_layer_channel(in_data.effect_ref(), &ref_, &chunk)?;
                        }
                    }
                }

                // set the checked-out rect to be the top half of the layer
                let mut halfsies = ae::Rect {
                    top:    0,
                    left:   0,
                    right:  out_layer.width() as _,
                    bottom: out_layer.height() as i32 / 2
                };
                let slider = params.get(Params::Frame)?.as_slider()?.value();

                let checkout = params.checkout_at(Params::Layer, Some(in_data.current_time() + slider * in_data.time_step()), None, None)?;
                let layer = checkout.as_layer()?;

                if let Some(layer) = layer.value() {
                    out_layer.copy_from(&layer, None, Some(halfsies))?;
                }  else  {
                    // no layer? Zero-alpha black.
                    out_layer.fill(None, None)?;
                }

                halfsies.top    = halfsies.bottom; // reset rect, copy.
                halfsies.bottom = out_layer.height() as _;
                out_layer.copy_from(&in_layer, None, Some(halfsies))?;
            }
            _ => { }
        }
        Ok(())
    }
}
