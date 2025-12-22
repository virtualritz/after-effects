mod simulation;

use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    NumParticles,
    Seed,
    Radius,
    GravityPoint,
    GravityStrength,
    UseCache,
}

use simulation::CACHE_ID;

#[derive(Default)]
struct Plugin {}

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn params_setup(
        &self,
        params: &mut ae::Parameters<Params>,
        _: ae::InData,
        _: ae::OutData,
    ) -> Result<(), Error> {
        params.add_with_flags(
            Params::NumParticles,
            "Num Particles",
            ae::SliderDef::setup(|f| {
                f.set_slider_min(1);
                f.set_slider_max(100000);
                f.set_valid_min(1);
                f.set_valid_max(1000000);
                f.set_default(10000);
                f.set_value(10000);
            }),
            ae::ParamFlag::CANNOT_TIME_VARY,
            ae::ParamUIFlags::empty(),
        )?;
        params.add_with_flags(
            Params::Seed,
            "Seed",
            ae::SliderDef::setup(|f| {
                f.set_slider_min(0);
                f.set_slider_max(1000);
                f.set_valid_min(0);
                f.set_valid_max(10000);
                f.set_default(0);
                f.set_value(0);
            }),
            ae::ParamFlag::CANNOT_TIME_VARY,
            ae::ParamUIFlags::empty(),
        )?;
        params.add(
            Params::Radius,
            "Radius",
            ae::FloatSliderDef::setup(|f| {
                f.set_slider_min(1.0);
                f.set_slider_max(5.0);
                f.set_valid_min(1.0);
                f.set_valid_max(8.0);
                f.set_default(1.0);
                f.set_value(1.0);
            }),
        )?;
        params.add(
            Params::GravityPoint,
            "Gravity Point",
            ae::PointDef::setup(|f| {
                f.set_default((50.0, 50.0));
                f.set_value((50.0, 50.0));
            }),
        )?;
        params.add(
            Params::GravityStrength,
            "Gravity Strength",
            ae::FloatSliderDef::setup(|f| {
                f.set_slider_min(-10.0);
                f.set_slider_max(10.0);
                f.set_valid_min(-100.0);
                f.set_valid_max(100.0);
                f.set_default(1.5);
                f.set_value(1.5);
                f.set_precision(2);
            }),
        )?;
        params.add(
            Params::UseCache,
            "Use Cache",
            ae::CheckBoxDef::setup(|f| {
                f.set_default(true);
                f.set_value(true);
                f.set_label("Enabled");
            }),
        )
    }

    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        _: ae::OutData,
        params: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::GlobalSetup => {
                ae::aegp::suites::ComputeCache::new()?.register_class(
                    CACHE_ID,
                    simulation::SimStep::generate_key,
                    simulation::SimStep::compute,
                    simulation::SimStep::approx_size,
                    simulation::SimStep::delete,
                )?;
            }
            ae::Command::GlobalSetdown => {
                ae::aegp::suites::ComputeCache::new()?.unregister_class(CACHE_ID)?;
            }
            ae::Command::Render {
                in_layer,
                mut out_layer,
            } => {
                let num_particles = params.get(Params::NumParticles)?.as_slider()?.value();
                let seed = params.get(Params::Seed)?.as_slider()?.value();
                let radius = params.get(Params::Radius)?.as_float_slider()?.value() as f32;
                let use_cache = params.get(Params::UseCache)?.as_checkbox()?.value();
                let frame = in_data.current_frame().round() as i32;
                let time_step = in_data.time_step();
                let time_scale = in_data.time_scale();
                let dt = time_step as f32 / time_scale as f32;

                let (w, h) = (in_layer.width() as f32, in_layer.height() as f32);

                let get_gravity_at = |f: i32| -> Result<([f32; 2], f32), ae::Error> {
                    let time = f * time_step;
                    let gravity_point = params
                        .checkout_at(
                            Params::GravityPoint,
                            Some(time),
                            Some(time_step),
                            Some(time_scale),
                        )?
                        .as_point()?
                        .value();

                    let strength = params
                        .checkout_at(
                            Params::GravityStrength,
                            Some(time),
                            Some(time_step),
                            Some(time_scale),
                        )?
                        .as_float_slider()?
                        .value() as f32;
                    Ok((
                        [gravity_point.0 as f32 / w, gravity_point.1 as f32 / h],
                        strength,
                    ))
                };

                let step = if use_cache {
                    simulation::simulate_to_frame(frame, num_particles, seed, dt, &get_gravity_at)?
                } else {
                    simulation::simulate_to_frame_no_cache(
                        frame,
                        num_particles,
                        seed,
                        dt,
                        &get_gravity_at,
                    )?
                };

                out_layer.copy_from(&in_layer, None, None)?;
                simulation::blit_particles(&mut out_layer, &step.0, radius as i32);
            }
            _ => {}
        }
        Ok(())
    }
}
