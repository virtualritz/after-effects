use after_effects as ae;
use after_effects_sys as ae_sys;

// This demonstrates:
//  - using sequence data
//  - iterating over the image pixels
//  - a Float Slider control
//  - the Extent Hint rectangle (from the InData structure)

const GAMMA_MIN:     f32 = 0.0;
const GAMMA_MAX:     f32 = 2.0;
const GAMMA_BIG_MAX: f32 = 2.0;
const GAMMA_DFLT:    f64 = 1.0;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Gamma, // gamma correction factor
}

#[derive(Default)]
struct Plugin { }

struct GammaTable {
    gamma_val: f32,
    lut: [u8; 256],
}

ae::define_plugin!(Plugin, GammaTable, Params);

impl Default for GammaTable {
    fn default() -> Self {
        let mut lut = [0u8; 256];
        // generate base table
        for i in 0..=ae_sys::PF_MAX_CHAN8 {
            lut[i as usize] = i as u8;
        }
        Self {
            gamma_val: 1.0,
            lut,
        }
    }
}

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::Gamma, "Gamma", ae::FloatSliderDef::setup(|f| {
            f.set_slider_min(GAMMA_MIN);
            f.set_slider_max(GAMMA_MAX);
            f.set_valid_min(GAMMA_MIN);
            f.set_valid_max(GAMMA_BIG_MAX);
            f.set_default(GAMMA_DFLT);
            f.set_precision(1);
        }))
    }

    fn handle_command(&mut self, cmd: ae::Command, _in_data: InData, mut out_data: OutData, _params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Gamma_Table v2.1\rPerform simple image gamma correction. Copyright 1994-2023 Adobe Inc.");
            }
            _ => {}
        }
        Ok(())
    }
}

impl AdobePluginInstance for GammaTable {
    fn flatten(&self) -> Result<(u16, Vec<u8>), Error> {
        let mut data = self.lut.to_vec();
        data.extend_from_slice(&f32::to_le_bytes(self.gamma_val));
        Ok((1, data))
    }
    fn unflatten(_version: u16, bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 256 + 4 {
            return Ok(Self::default());
        }
        Ok(Self {
            gamma_val: f32::from_le_bytes([bytes[256], bytes[257], bytes[258], bytes[259]]),
            lut: {
                let mut lut = [0u8; 256];
                lut.copy_from_slice(&bytes[0..256]);
                lut
            },
        })
    }

    fn user_changed_param(&mut self, _: &mut PluginState, _: Params) -> Result<(), ae::Error> { Ok(()) }
    fn render(&self, _: &mut PluginState, _: &Layer, _: &mut Layer) -> Result<(), ae::Error> { Ok(()) }

    fn handle_command(&mut self, plugin: &mut PluginState, cmd: ae::Command) -> Result<(), ae::Error> {
        log::info!("sequence command: {:?}, thread: {:?}, ptr: {:?}", cmd, std::thread::current().id(), self as *const _);

        let in_data = &plugin.in_data;

        match cmd {
            ae::Command::Render { in_layer, mut out_layer } => {
                let gamma = plugin.params.get(Params::Gamma)?.as_float_slider()?.value() as f32;

                // If the gamma factor is exactly 1.0 just make a direct copy.
                if gamma == 1.0 {
                    out_layer.copy_from(&in_layer, None, None)?;
                } else {
                    let extent_hint = in_data.extent_hint();
                    let out_extent_hint = out_layer.extent_hint();
                    // clear all pixels outside extent_hint.
                    if extent_hint != out_extent_hint {
                        out_layer.fill(None, Some(out_extent_hint))?;
                    }

                    // if the table values are bad, regenerate table contents.
                    if self.gamma_val != gamma {
                        self.gamma_val = gamma;
                        let gamma = 1.0 / gamma;
                        for x in 0..=ae_sys::PF_MAX_CHAN8 {
                            self.lut[x as usize] = ((x as f32 / 255.0).powf(gamma) * 255.0) as u8;
                        }
                    }

                    // iterate over image data.
                    #[rustfmt::skip]
                    in_layer.iterate_with(&mut out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                        match (pixel, out_pixel) {
                            (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha;
                                out_pixel.red   = self.lut[pixel.red   as usize];
                                out_pixel.green = self.lut[pixel.green as usize];
                                out_pixel.blue  = self.lut[pixel.blue  as usize];
                            }
                            (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                let px8 = ae::pixel16_to_8(*pixel);
                                out_pixel.alpha = pixel.alpha;
                                out_pixel.red   = self.lut[px8.red   as usize] as u16 * 128;
                                out_pixel.green = self.lut[px8.green as usize] as u16 * 128;
                                out_pixel.blue  = self.lut[px8.blue  as usize] as u16 * 128;
                            }
                            _ => return Err(Error::BadCallbackParameter)
                        }

                        Ok(())
                    })?;
                }
            }
            _ => { }
        }
        Ok(())
    }
}
