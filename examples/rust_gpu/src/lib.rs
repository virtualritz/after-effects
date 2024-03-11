use after_effects as ae;
use parking_lot::RwLock;

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
}

#[repr(C)]
struct KernelParams {
    param_a: f32,
    param_r: f32,
    param_g: f32,
    param_b: f32,
}
impl KernelParams {
    fn from_params(params: &mut ae::Parameters<Params>) -> Result<Self, ae::Error> {
        Ok(Self {
            param_a: 0.0,
            param_r: params.get(Params::Red)?  .as_float_slider()?.value() as f32 / 100.0,
            param_g: params.get(Params::Green)?.as_float_slider()?.value() as f32 / 100.0,
            param_b: params.get(Params::Blue)? .as_float_slider()?.value() as f32 / 100.0,
        })
    }
}

#[derive(Default)]
struct Plugin { }

struct Instance {
    wgpu: RwLock<wgpu_proc::WgpuProcessing<KernelParams>>
}

ae::define_plugin!(Plugin, Instance, Params);

impl Default for Instance {
    fn default() -> Self {
        Self {
            //wgpu: RwLock::new(WgpuProcessing::new(ProcShaderSource::Wgsl(include_str!("../shader.wgsl"))))
            wgpu: RwLock::new(WgpuProcessing::new(ProcShaderSource::SpirV(include_bytes!("../shader.spv"))))
        }
    }
}

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _: InData, _: OutData) -> Result<(), Error> {
        fn setup(f: &mut ae::FloatSliderDef) {
            f.set_slider_min(0.0);
            f.set_slider_max(100.0);
            f.set_valid_min(0.0);
            f.set_valid_max(100.0);
            f.set_default(0.0);
            f.set_precision(1);
        }
        params.add(Params::Red,   "Red",   ae::FloatSliderDef::setup(setup))?;
        params.add(Params::Green, "Green", ae::FloatSliderDef::setup(setup))?;
        params.add(Params::Blue,  "Blue",  ae::FloatSliderDef::setup(setup))?;
        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, _in_data: InData, mut out_data: OutData, _params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Rust GPU v0.1\rProcess pixels on the GPU using shader written in Rust and compiled to SPIR-V");
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
        Ok(Self::default())
    }

    fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error> {
        let in_size  = (in_layer.width()  as usize, in_layer.height()  as usize, in_layer.buffer_stride());
        let out_size = (out_layer.width() as usize, out_layer.height() as usize, out_layer.buffer_stride());

        {
            let mut upgradable_read = self.wgpu.upgradable_read();
            if upgradable_read.state.as_ref().map(|x| x.in_size) != Some(in_size) || upgradable_read.state.as_ref().map(|x| x.out_size) != Some(out_size) {
                upgradable_read.with_upgraded(|x| x.setup_size(in_size, out_size));
            }
        }

        let _time = std::time::Instant::now();

        let params = plugin.in_data.frame_data::<KernelParams>().unwrap();
        self.wgpu.read().run_compute(&params, in_size, out_size, in_layer.buffer(), out_layer.buffer_mut());

        log::warn!("Render time: {:.3} ms", _time.elapsed().as_micros() as f64 / 1000.0);

        Ok(())
    }

    fn handle_command(&mut self, plugin: &mut PluginState, cmd: ae::Command) -> Result<(), ae::Error> {
        let in_data = &plugin.in_data;
        let out_data = &mut plugin.out_data;
        match cmd {
            ae::Command::FrameSetup { .. } => {
                out_data.set_frame_data::<KernelParams>(KernelParams::from_params(plugin.params)?);
            },
            ae::Command::FrameSetdown { .. } => {
                in_data.destroy_frame_data::<KernelParams>();
            },
            ae::Command::SmartPreRender { mut extra } => {
                let req = extra.output_request();

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let _ = extra.union_result_rect(in_result.result_rect.into());
                    let _ = extra.union_max_result_rect(in_result.max_result_rect.into());

                    extra.set_pre_render_data::<KernelParams>(KernelParams::from_params(plugin.params)?);
                }
            }
            ae::Command::SmartRender { extra } => {
                let cb = extra.callbacks();
                let in_layer = cb.checkout_layer_pixels(0)?;

                if let Ok(mut out_layer) = cb.checkout_output() {
                    let in_size  = (in_layer.width() as usize, in_layer.height() as usize, in_layer.buffer_stride());
                    let out_size = (out_layer.width() as usize, out_layer.height() as usize, out_layer.buffer_stride());

                    {
                        let mut upgradable_read = self.wgpu.upgradable_read();
                        if upgradable_read.state.as_ref().map(|x| x.in_size) != Some(in_size) || upgradable_read.state.as_ref().map(|x| x.out_size) != Some(out_size) {
                            upgradable_read.with_upgraded(|x| x.setup_size(in_size, out_size));
                        }
                    }

                    let _time = std::time::Instant::now();

                    let params = extra.pre_render_data::<KernelParams>().unwrap();
                    self.wgpu.read().run_compute(&params, in_size, out_size, in_layer.buffer(), out_layer.buffer_mut());

                    log::warn!("Smart render time: {:.3} ms", _time.elapsed().as_micros() as f64 / 1000.0);
                }

                cb.checkin_layer_pixels(0)?;
            }
            _ => { }
        }
        Ok(())
    }
}
