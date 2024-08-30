use after_effects as ae;

// This sample exercises some of After Effects' image processing callback functions.

const XFORM_AMOUNT_MIN: f32       = 0.0;
const XFORM_AMOUNT_MAX: f32       = 100.0;
const XFORM_COLOR_BLEND_DFLT: f64 = 50.0;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    ColorBlend,
    Color,
    GroupStart,
    LayerBlend,
    Layer,
    GroupEnd,
}

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _in_data: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::ColorBlend, "Color Blend Ratio", ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(XFORM_AMOUNT_MIN);
            f.set_valid_max(XFORM_AMOUNT_MAX);
            f.set_slider_min(XFORM_AMOUNT_MIN);
            f.set_slider_max(XFORM_AMOUNT_MAX);
            f.set_default(XFORM_COLOR_BLEND_DFLT);
            f.set_precision(2);
            f.set_value(f.default());
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }))?;

        params.add(Params::Color, "Color", ae::ColorDef::setup(|f| {
            f.set_default(ae::Pixel8 {
                red: 128,
                green: 255,
                blue: 255,
                alpha: 255
            });
            f.set_value(f.default());
        }))?;

        // def.flags = PF_ParamFlag_START_COLLAPSED;
        params.add_group(Params::GroupStart, Params::GroupEnd, "Layer Controls", false, |params| {
            params.add(Params::LayerBlend, "Layer Opacity", ae::FloatSliderDef::setup(|f| {
                f.set_valid_min(XFORM_AMOUNT_MIN);
                f.set_valid_max(XFORM_AMOUNT_MAX);
                f.set_slider_min(XFORM_AMOUNT_MIN);
                f.set_slider_max(XFORM_AMOUNT_MAX);
                f.set_default(XFORM_COLOR_BLEND_DFLT);
                f.set_value(f.default());
                f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
            }))?;

            params.add(Params::Layer, "Layer Blend Ratio", ae::LayerDef::setup(|f| {
                f.set_default_to_this_layer();
            }))?;
            Ok(())
        })?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Resizer, v2.2,\rDemonstrate image processing callbacks.\nCopyright 2007-2023 Adobe Inc.");
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let in_extent = in_layer.extent_hint();
                let out_extent = out_layer.extent_hint();

                // We're going to blend the input with the color. If the user has picked an additional layer, we'll blend it in too, giving it a 'special' look along the way.
                if in_data.quality() == ae::Quality::Hi && !in_data.is_premiere() {
                    ae::pf::suites::WorldTransform::new()?.copy_hq(in_data.effect_ref(), &in_layer, &mut out_layer, Some(in_extent), Some(out_extent))?;
                } else if !in_data.is_premiere() {
                    ae::pf::suites::WorldTransform::new()?.copy(in_data.effect_ref(), &in_layer, &mut out_layer, Some(in_extent), Some(out_extent))?;
                } else {
                    out_layer.copy_from(&in_layer, Some(in_extent), Some(out_extent))?;
                }

                in_data.interact().abort()?;

                // Make an offscreen world. If the user has selected an extra input layer, we'll size to that (so we can use the blend callback,
                // which requires input PF_Worlds of the same dimensions), of the appropriate pixel depth.
                let mut pixel_format = ae::PixelFormat::Argb32;
                if out_layer.bit_depth() == 16 {
                    pixel_format = ae::PixelFormat::Argb64;
                }

                let mut width = out_layer.width();
                let mut height = out_layer.height();
                let temp_param = params.get(Params::Layer)?.as_layer()?.value();
                if let Some(ref temp_param) = temp_param {
                    width = temp_param.width();
                    height = temp_param.height();
                }

                in_data.interact().abort()?;

                let mut color_world = ae::pf::suites::World::new()?
                    .new_world(in_data.clone(), width as _, height as _, true, pixel_format)?;

                in_data.interact().abort()?;

                let color = params.get(Params::Color)?.as_color()?.value();
                if out_layer.bit_depth() == 16 {
                    color_world.fill16(Some(ae::pixel8_to_16(color)), None)?;
                } else {
                    color_world.fill(Some(color), None)?;
                }

                in_data.interact().abort()?;

                let layer_blend = params.get(Params::LayerBlend)?.as_float_slider()?.value() as f32 / 100.0;
                let color_blend = params.get(Params::ColorBlend)?.as_float_slider()?.value() as f32 / 100.0;

                if let Some(ref temp_param) = temp_param {
                    ae::pf::suites::WorldTransform::new()?
                        .blend(in_data.effect_ref(), temp_param, color_world.as_ptr(), layer_blend, &mut color_world)?;
                } else {
                    ae::pf::suites::WorldTransform::new()?
                        .blend(in_data.effect_ref(), &in_layer, &color_world, color_blend, &mut out_layer)?;
                }

                in_data.interact().abort()?;

                // Blend the color layer () with the output

                let mode = ae::CompositeMode {
                    xfer:       ae::TransferMode::Difference,
                    opacity:    (layer_blend * ae::MAX_CHANNEL8  as f32).round() as _,
                    opacity_su: (layer_blend * ae::MAX_CHANNEL16 as f32).round() as _,
                    rgb_only:   true,
                    rand_seed:  0
                };
                let destx = (out_extent.right  - color_world.extent_hint().right).abs() / 2;
                let desty = (out_extent.bottom - color_world.extent_hint().bottom).abs() / 2;
                ae::pf::suites::WorldTransform::new()?.transfer_rect(
                    in_data.effect_ref(),
                    in_data.quality(),
                    ae::ModeFlags::AlphaStraight,
                    in_data.field(),
                    Some(color_world.extent_hint()),
                    color_world,
                    mode,
                    None,
                    destx,
                    desty,
                    &mut out_layer
                )?;

                in_data.interact().abort()?;
            }
            _ => {}
        }
        Ok(())
    }
}
