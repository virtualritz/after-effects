use after_effects as ae;

mod ui;

const UI_GRID_WIDTH : u16 = 203;
const UI_GRID_HEIGHT: u16 = UI_GRID_WIDTH;
const ARB_REFCON    : u64 = 0xDEADBEEFDEADBEEF;

const BOXES_ACROSS  : usize = 3;
const BOXES_DOWN    : usize = 3;
const BOXES_PER_GRID: usize = BOXES_ACROSS * BOXES_DOWN; // Ones based
const CG_ARBDATA_ELEMENTS: usize = BOXES_PER_GRID;

#[derive(Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct FloatPixel {
    alpha: f32,
    red: f32,
    green: f32,
    blue: f32,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
struct ArbData {
    colors: [FloatPixel; CG_ARBDATA_ELEMENTS],
    string: String
}
impl ArbData {
    fn interp_pixel(intrp_amt: f32, lpix: &FloatPixel, rpix: &FloatPixel) -> FloatPixel {
        let mut head = FloatPixel::default();
        head.alpha = 1.0;

        if lpix.blue > rpix.blue {
            head.blue = (lpix.blue - rpix.blue) * intrp_amt;
            head.blue = lpix.blue - head.blue;
        } else if lpix.blue < rpix.blue {
            head.blue = (rpix.blue - lpix.blue) * intrp_amt;
            head.blue += lpix.blue;
        } else {
            head.blue = lpix.blue;
        }

        if lpix.green > rpix.green {
            head.green = (lpix.green - rpix.green) * intrp_amt;
            head.green = lpix.green - head.green;
        } else if lpix.green < rpix.green {
            head.green = (rpix.green - lpix.green) * intrp_amt;
            head.green += lpix.green;
        } else {
            head.green = lpix.green;
        }

        if lpix.red > rpix.red {
            head.red = (lpix.red - rpix.red) * intrp_amt;
            head.red = lpix.red - head.red;
        } else if lpix.red < rpix.red {
            head.red = (rpix.red - lpix.red) * intrp_amt;
            head.red += lpix.red;
        } else {
            head.red = lpix.red;
        }
        head
    }
}

impl Default for ArbData {
    fn default() -> Self {
        Self {
            colors: [
                FloatPixel { alpha: 1.0, blue: 0.0,    green: 0.5,    red: 1.0 },
                FloatPixel { alpha: 1.0, blue: 0.5,    green: 0.32,   red: 0.67 },
                FloatPixel { alpha: 1.0, blue: 0.6,    green: 0.27,   red: 0.34 },
                FloatPixel { alpha: 1.0, blue: 1.0,    green: 0.4039, red: 0.47 },
                FloatPixel { alpha: 1.0, blue: 0.47,   green: 0.54,   red: 0.603 },
                FloatPixel { alpha: 1.0, blue: 0.0603, green: 0.67,   red: 0.9 },
                FloatPixel { alpha: 1.0, blue: 0.737,  green: 0.81,   red: 0.89 },
                FloatPixel { alpha: 1.0, blue: 0.5,    green: 1.0,    red: 0.5 },
                FloatPixel { alpha: 1.0, blue: 0.5,    green: 0.5,    red: 0.9 },
            ],
            string: "Hello world".to_owned()
        }
    }
}
impl ae::ArbitraryData<ArbData> for ArbData {
    fn interpolate(&self, other: &Self, value: f64) -> Self {
        let mut ret = Self::default();
        for i in 0..CG_ARBDATA_ELEMENTS {
            ret.colors[i] = Self::interp_pixel(value as f32, &self.colors[i], &other.colors[i]);
        }
        ret
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    GridUI,
}

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: InData, _: OutData) -> Result<(), Error> {
        params.add_customized(Params::GridUI, "Color Grid", ae::ArbitraryDef::setup(|f| {
            f.set_default(ArbData::default()).unwrap();
            f.set_refcon(ARB_REFCON as _);
        }), |param| {
            param.set_ui_flags(ae::ParamUIFlags::CONTROL | ae::ParamUIFlags::DO_NOT_ERASE_CONTROL);
            param.set_ui_width(UI_GRID_WIDTH);
            param.set_ui_height(UI_GRID_HEIGHT);
            -1
        })?;

        in_data.interact().register_ui(
            CustomUIInfo::new()
                .events(ae::CustomEventFlags::EFFECT)
        )?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("ColorGrid v3.3\rCopyright 2007-2023 Adobe Inc.\rArbitrary data and Custom UI sample.");
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let param = params.get(Params::GridUI)?;
                let colors = param.as_arbitrary()?.value::<ArbData>()?;

                let mut current_color = 0;
                let origin = in_data.pre_effect_source_origin();
                let mut box_across = 0;
                let mut box_down = 0;
                let progress_final = BOXES_PER_GRID as i32;
                let mut progress_base = 0;

                // This section uses the pre-effect extent hint, since it wants to only be applied to the source layer material, and NOT to any
                // resized effect area. Example: User applies "Resizer" to a layer before using ColorGrid.
                // The effect makes the output area larger than the source footage. ColorGrid will look at the pre-effect
                // extent width and height to determine what the relative coordinates are for the source material inside the params[0] (the layer).

                for _ in 0..BOXES_PER_GRID {
                    if box_across == BOXES_ACROSS {
                        box_down += 1;
                        box_across = 0;
                    }
                    let current_rect = ui::colorgrid_get_box_in_grid(&origin,
                        (in_data.width()  as f32 * f32::from(in_data.downsample_x())).round() as _,
                        (in_data.height() as f32 * f32::from(in_data.downsample_y())).round() as _,
                        box_across,
                        box_down
                    );

                    let cur_color = &colors.colors[current_color];
                    let color8_r  = (cur_color.red   * ae::MAX_CHANNEL8 as f32) as u16;
                    let color8_g  = (cur_color.green * ae::MAX_CHANNEL8 as f32) as u16;
                    let color8_b  = (cur_color.blue  * ae::MAX_CHANNEL8 as f32) as u16;

                    progress_base += 1;

                    in_layer.iterate_with(&mut out_layer, progress_base, progress_final as _, Some(current_rect), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                        match (pixel, out_pixel) {
                            (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha as _;
                                out_pixel.red   = ((pixel.red   as u16 + color8_r) / 2) as u8;
                                out_pixel.green = ((pixel.green as u16 + color8_g) / 2) as u8;
                                out_pixel.blue  = ((pixel.blue  as u16 + color8_b) / 2) as u8;
                            }
                            _ => return Err(Error::BadCallbackParameter)
                        }
                        Ok(())
                    })?;

                    current_color += 1;
                    box_across += 1;
                }

            }
            ae::Command::SmartPreRender { mut extra } => {
                let req = extra.output_request();

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let _ = extra.union_result_rect(in_result.result_rect.into());
                    let _ = extra.union_max_result_rect(in_result.max_result_rect.into());
                }
            }
            ae::Command::SmartRender { extra } => {
                let mut origin = ae::Point::empty();
                let mut box_across    = 0;
                let mut box_down      = 0;
                let mut current_color = 0;

                let cb = extra.callbacks();
                let input_world = cb.checkout_layer_pixels(0)?;

                let param = params.get(Params::GridUI)?;
                let colors = param.as_arbitrary()?.value::<ArbData>()?;

                for _ in 0..BOXES_PER_GRID {
                    if box_across == BOXES_ACROSS {
                        box_down += 1;
                        box_across = 0;
                    }

                    let current_rect = ui::colorgrid_get_box_in_grid(&origin,
                        (in_data.width()  as f32 * f32::from(in_data.downsample_x())).round() as _,
                        (in_data.height() as f32 * f32::from(in_data.downsample_y())).round() as _,
                        box_across,
                        box_down
                    );

                    let color32 = &colors.colors[current_color];

                    let color8_r  = (color32.red   * ae::MAX_CHANNEL8  as f32) as u16;
                    let color8_g  = (color32.green * ae::MAX_CHANNEL8  as f32) as u16;
                    let color8_b  = (color32.blue  * ae::MAX_CHANNEL8  as f32) as u16;

                    let color16_r = (color32.red   * ae::MAX_CHANNEL16 as f32) as u32;
                    let color16_g = (color32.green * ae::MAX_CHANNEL16 as f32) as u32;
                    let color16_b = (color32.blue  * ae::MAX_CHANNEL16 as f32) as u32;

                    if let Ok(mut output_world) = cb.checkout_output() {
                        let progress_final = output_world.height() as _;

                        origin = in_data.output_origin();

                        input_world.iterate_with(&mut output_world, 0, progress_final, Some(current_rect), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                            match (pixel, out_pixel) {
                                (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = ((pixel.red   as u16 + color8_r) / 2) as u8;
                                    out_pixel.green = ((pixel.green as u16 + color8_g) / 2) as u8;
                                    out_pixel.blue  = ((pixel.blue  as u16 + color8_b) / 2) as u8;
                                }
                                (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = ((pixel.red   as u32 + color16_r) / 2) as u16;
                                    out_pixel.green = ((pixel.green as u32 + color16_g) / 2) as u16;
                                    out_pixel.blue  = ((pixel.blue  as u32 + color16_b) / 2) as u16;
                                }
                                (ae::GenericPixel::PixelF32(pixel), ae::GenericPixelMut::PixelF32(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = (pixel.red   + color32.red)   / 2.0;
                                    out_pixel.green = (pixel.green + color32.green) / 2.0;
                                    out_pixel.blue  = (pixel.blue  + color32.blue)  / 2.0;
                                }
                                _ => return Err(Error::BadCallbackParameter)
                            }
                            Ok(())
                        })?;
                    }
                    current_color += 1;
                    box_across += 1;
                }

                cb.checkin_layer_pixels(0)?;
            }
            ae::Command::ArbitraryCallback { mut extra } => {
                extra.dispatch::<ArbData, Params>(Params::GridUI)?;
            }
            ae::Command::Event { mut extra } => {
                match extra.event() {
                    ae::Event::Click(_) => { ui::click(&in_data, params, &mut extra)?; }
                    ae::Event::Draw(_)  => { ui::draw(&in_data, params, &mut extra)?; }
                    ae::Event::AdjustCursor(_) => { ui::change_cursor(&in_data, &mut extra)?; }
                    _ => {}
                }
            }
            _ => { }
        }
        Ok(())
    }
}
