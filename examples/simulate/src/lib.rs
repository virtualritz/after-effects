mod simulation;

use after_effects as ae;
use fastrand::Rng;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    NumParticles,
    Seed,
    Radius,
    GravityPoint,
}

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
        params.add(
            Params::NumParticles,
            "Num Particles",
            ae::SliderDef::setup(|f| {
                f.set_slider_min(1);
                f.set_slider_max(10000);
                f.set_valid_min(1);
                f.set_valid_max(100000);
                f.set_default(100);
                f.set_value(100);
            }),
        )?;
        params.add(
            Params::Seed,
            "Seed",
            ae::SliderDef::setup(|f| {
                f.set_slider_min(0);
                f.set_slider_max(10000);
                f.set_valid_min(0);
                f.set_valid_max(i32::MAX);
                f.set_default(0);
                f.set_value(0);
            }),
        )?;

        params.add(
            Params::Radius,
            "Radius",
            ae::FloatSliderDef::setup(|f| {
                f.set_slider_min(1.0);
                f.set_slider_max(100.0);
                f.set_valid_min(0.5);
                f.set_valid_max(500.0);
                f.set_default(5.0);
                f.set_value(5.0);
            }),
        )?;

        params.add(
            Params::GravityPoint,
            "Gravity Point",
            ae::PointDef::setup(|f| {
                f.set_default((50.0, 50.0));
                f.set_value((50.0, 50.0));
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
            ae::Command::Render {
                in_layer,
                mut out_layer,
            } => {
                let num_particles = params.get(Params::NumParticles)?.as_slider()?.value();
                let seed = params.get(Params::Seed)?.as_slider()?.value();

                let rng = Rng::with_seed(seed as u64);

                let extent_hint = in_data.extent_hint();

                // TODO: Use rng and num_particles for particle simulation

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

                let _ = (rng, num_particles); // suppress unused warnings for now
            }
            _ => {}
        }
        Ok(())
    }
}
