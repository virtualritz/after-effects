use after_effects as ae;

const CONVO_AMOUNT_MIN:  f32 = 0.0;
const CONVO_AMOUNT_MAX:  f32 = 100.0;
const CONVO_AMOUNT_DFLT: f64 = 50.0;
const CURVE_TOLERANCE:   f32 = 0.05;
const KERNEL_SIZE:       i32 = 3;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Amount,
    BlendGroupStart,
    BlendColorAmount,
    BlendColor,
    BlendGroupEnd
}

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _in_data: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::Amount, "Convolve", ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(CONVO_AMOUNT_MIN);
            f.set_valid_max(CONVO_AMOUNT_MAX);
            f.set_slider_min(CONVO_AMOUNT_MIN);
            f.set_slider_max(CONVO_AMOUNT_MAX);
            f.set_default(CONVO_AMOUNT_DFLT);
            f.set_value(f.default());
            f.set_curve_tolerance(CURVE_TOLERANCE);
            f.set_precision(2);
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }))?;

        params.add_group(Params::BlendGroupStart, Params::BlendGroupEnd, "Blend Controls", |params| {
            params.add(Params::BlendColorAmount, "Blend percentage", ae::FloatSliderDef::setup(|f| {
                f.set_valid_min(CONVO_AMOUNT_MIN);
                f.set_valid_max(CONVO_AMOUNT_MAX);
                f.set_slider_min(CONVO_AMOUNT_MIN);
                f.set_slider_max(CONVO_AMOUNT_MAX);
                f.set_default(CONVO_AMOUNT_DFLT);
                f.set_value(f.default());
                f.set_curve_tolerance(CURVE_TOLERANCE);
                f.set_precision(2);
                f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
            }))?;

            params.add(Params::BlendColor, "Blend color", ae::ColorDef::setup(|f| {
                f.set_default(ae::Pixel8 {
                    red:   255,
                    green: 0,
                    blue:  255,
                    alpha: 255
                });
                f.set_value(f.default());
            }))?;
            Ok(())
        })?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Convolutrix, v3.2,\rDemonstrate our image processing callbacks.\rCopyright 2007-2023 Adobe Inc.");
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let sharpen   = params.get(Params::Amount)?.as_float_slider()?.value() as f32 / 16.0;
                let color_amt = params.get(Params::BlendColorAmount)?.as_float_slider()?.value() as f32 / 100.0;

                if sharpen > 0.0 { // we're doing some convolving...
                    let mut kernel_sum = 256.0 * 9.0;
                    let mut conv_kernel = [0i32; 9];
                    conv_kernel[4] = (sharpen * kernel_sum).trunc() as _;
                    kernel_sum    = (256.0 * 9.0 - conv_kernel[4] as f32) / 4.0;
                    let sum_long = kernel_sum.trunc() as _;
                    conv_kernel[1] = sum_long;
                    conv_kernel[3] = sum_long;
                    conv_kernel[5] = sum_long;
                    conv_kernel[7] = sum_long;
                    let kernel_ptr = conv_kernel.as_mut_ptr() as *mut _;

                    // Premiere Pro/Elements doesn't support WorldTransformSuite1, but it does support many of the callbacks in utils
                    if !in_data.is_premiere() {
                        ae::pf::suites::WorldTransform::new()?.convolve(
                            in_data.effect_ref(),
                            &in_layer,
                            Some(in_data.extent_hint()),
                            ae::KernelFlags::TWO_D | ae::KernelFlags::CLAMP,
                            KERNEL_SIZE,
                            kernel_ptr,
                            kernel_ptr,
                            kernel_ptr,
                            kernel_ptr,
                            &mut out_layer
                        )?;
                    } else {
                        in_data.utils().convolve(
                            &in_layer,
                            Some(in_data.extent_hint()),
                            ae::KernelFlags::TWO_D | ae::KernelFlags::CLAMP,
                            KERNEL_SIZE,
                            kernel_ptr,
                            kernel_ptr,
                            kernel_ptr,
                            kernel_ptr,
                            &mut out_layer
                        )?;
                    }

                    if color_amt > 0.0 { // we're blending in a color.
                        let color = params.get(Params::BlendColor)?.as_color()?.value();
                        // Allocate a world full of the color to blend.
                        let mut temp = in_data.utils().new_world(out_layer.width() as _, out_layer.height() as _, ae::NewWorldFlags::NONE)?;
                        in_data.utils().fill(&mut temp, Some(color), None)?;

                        in_data.utils().blend(out_layer.as_ptr(), temp, color_amt, out_layer)?;
                    }
                } else { // No matter what, we populate the output buffer.
                    if in_data.quality() == ae::Quality::Hi && !in_data.is_premiere() {
                        ae::pf::suites::WorldTransform::new()?.copy_hq(in_data.effect_ref(), in_layer, out_layer, None, None)?;
                    } else if !in_data.is_premiere() {
                        ae::pf::suites::WorldTransform::new()?.copy(in_data.effect_ref(), in_layer, out_layer, None, None)?;
                    } else {
                        out_layer.copy_from(&in_layer, None, None)?;
                    }
                }
            }
            ae::Command::GetExternalDependencies { mut extra } => {
                match extra.check_type() {
                    ae::DepCheckType::AllDependencies => {
                        extra.set_dependencies_str("All Dependencies requested.")?;
                    }
                    ae::DepCheckType::MissingDependencies => {
                        // one-ninth of the time, something's missing
                        if fastrand::u32(..) % 9 != 0 {
                            extra.set_dependencies_str("Missing Dependencies requested.")?;
                        }
                    }
                    _ => extra.set_dependencies_str("None")?
                }
            }
            _ => {}
        }
        Ok(())
    }
}
