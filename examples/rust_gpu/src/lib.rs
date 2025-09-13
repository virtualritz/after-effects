use after_effects as ae;

// This demonstrates:
//  - Using Rust to write GPU shader code
//  - Using `wgpu` to run the shader
//
// This example is *not* gpu zero-copy (yet)
// To build the SPIR-V module from Rust code, cd to `spirv_builder` and run `cargo run --release`

mod wgpu_proc;
use wgpu_proc::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Red,
    Green,
    Blue,
    Mirror,
}

// This struct needs to be 16-byte aligned for proper usage on the GPU
#[repr(C)]
struct KernelParams {
    param_mirror: f32,
    param_r: f32,
    param_g: f32,
    param_b: f32,
}

impl KernelParams {
    fn from_params(params: &mut ae::Parameters<Params>) -> Result<Self, ae::Error> {
        Ok(Self {
            param_mirror: params.get(Params::Mirror)?.as_checkbox()?.value() as u8 as _,
            param_r: params.get(Params::Red)?  .as_float_slider()?.value() as f32 / 100.0,
            param_g: params.get(Params::Green)?.as_float_slider()?.value() as f32 / 100.0,
            param_b: params.get(Params::Blue)? .as_float_slider()?.value() as f32 / 100.0,
        })
    }
}

struct Plugin {
    wgpu: wgpu_proc::WgpuProcessing<KernelParams>
}
impl Default for Plugin {
    fn default() -> Self {
        Self {
            // wgpu: WgpuProcessing::new(ProcShaderSource::Wgsl(include_str!("../shader.wgsl")))
            wgpu: WgpuProcessing::new(ProcShaderSource::SpirV(include_bytes!("../shader.spv")))
        }
    }
}

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn params_setup(&self, params: &mut ae::Parameters<Params>, _: InData, _: OutData) -> Result<(), Error> {
        fn setup(f: &mut ae::FloatSliderDef) {
            f.set_slider_min(0.0);
            f.set_slider_max(100.0);
            f.set_valid_min(0.0);
            f.set_valid_max(100.0);
            f.set_default(0.0);
            f.set_precision(1);
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }
        params.add(Params::Red,   "Red",   ae::FloatSliderDef::setup(setup))?;
        params.add(Params::Green, "Green", ae::FloatSliderDef::setup(setup))?;
        params.add(Params::Blue,  "Blue",  ae::FloatSliderDef::setup(setup))?;
        params.add(Params::Mirror, "",     ae::CheckBoxDef::setup(|f| {
            f.set_label("Mirror");
            f.set_default(true);
        }))?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Rust GPU v0.1\rProcess pixels on the GPU using shader written in Rust and compiled to SPIR-V");
            }
            ae::Command::FrameSetup { .. } => {
                out_data.set_frame_data::<KernelParams>(KernelParams::from_params(params)?);
            },
            ae::Command::FrameSetdown { .. } => {
                in_data.destroy_frame_data::<KernelParams>();
            },
            ae::Command::Render { in_layer, mut out_layer } => {
                let in_size  = (in_layer.width()  as usize, in_layer.height()  as usize, in_layer.buffer_stride());
                let out_size = (out_layer.width() as usize, out_layer.height() as usize, out_layer.buffer_stride());

                let _time = std::time::Instant::now();

                let params = in_data.frame_data::<KernelParams>().unwrap();
                self.wgpu.run_compute(&params, in_size, out_size, in_layer.buffer(), out_layer.buffer_mut());

                log::warn!("Render time: {:.3} ms", _time.elapsed().as_micros() as f64 / 1000.0);

            },
            ae::Command::SmartPreRender { mut extra } => {
                let req = extra.output_request();

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let _ = extra.union_result_rect(in_result.result_rect.into());
                    let _ = extra.union_max_result_rect(in_result.max_result_rect.into());

                    extra.set_pre_render_data::<KernelParams>(KernelParams::from_params(params)?);
                }
            }
            ae::Command::SmartRender { extra } => {
                let cb = extra.callbacks();
                let Some(in_layer) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };

                if let Ok(Some(mut out_layer)) = cb.checkout_output() {
                    let in_size  = (in_layer.width()  as usize, in_layer.height()  as usize, in_layer.buffer_stride());
                    let out_size = (out_layer.width() as usize, out_layer.height() as usize, out_layer.buffer_stride());

                    let _time = std::time::Instant::now();

                    let params = extra.pre_render_data::<KernelParams>().unwrap();
                    self.wgpu.run_compute(&params, in_size, out_size, in_layer.buffer(), out_layer.buffer_mut());

                    log::warn!("Smart render time: {:.3} ms", _time.elapsed().as_micros() as f64 / 1000.0);
                }

                cb.checkin_layer_pixels(0)?;
            }
            _ => { }
        }
        Ok(())
    }
}
