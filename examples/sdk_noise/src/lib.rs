use after_effects as ae;

// A simple noise algorithm. This example shows how to handle different pixel formats and how to iterate them using rayon

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Noise,
}

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

#[repr(C)] struct PixelBGRA8u { blue: u8, green: u8, red: u8, alpha: u8 }
#[repr(C)] struct PixelVUYA8u { pr: u8, pb: u8, luma: u8, alpha: u8 }
#[repr(C)] struct PixelBGRA32f { blue: f32, green: f32, red: f32, alpha: f32 }
#[repr(C)] struct PixelVUYA32f { pr: f32, pb: f32, luma: f32, alpha: f32 }

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _in_data: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::Noise, "Noise variation", ae::FloatSliderDef::setup(|f| {
            f.set_slider_min(0.0);
            f.set_slider_max(100.0);
            f.set_valid_min(0.0);
            f.set_valid_max(1000.0);
            f.set_default(10.0);
            f.set_precision(1);
            f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
        }))
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("SDK_Noise v5.6\rCopyright 2007-2023 Adobe Inc.\rSimple noise effect.");
            }
            ae::Command::GlobalSetup => {
                // For Premiere - declare supported pixel formats
                if in_data.is_premiere() {
                    let suite = ae::pf::suites::PixelFormat::new()?;

                    // Add the pixel formats we support in order of preference.
                    suite.clear_supported_pixel_formats(in_data.effect_ref())?;
                    let formats = [
                        ae::pr::PixelFormat::Vuya4444_32f,
                        ae::pr::PixelFormat::Bgra4444_32f,
                        ae::pr::PixelFormat::Vuya4444_8u,
                        ae::pr::PixelFormat::Bgra4444_8u
                    ];
                    for x in formats {
                        suite.add_supported_pixel_format(in_data.effect_ref(), x)?;
                    }
                }
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let value = params.get(Params::Noise)?.as_float_slider()?.value() as f32 / 100.0;

                let progress_final = out_layer.height() as _;

                if in_data.is_premiere() {
                    // Premiere doesn't support IterateFloat so let's use rayon
                    use rayon::prelude::*;

                    let out_pixel_format = out_layer.pr_pixel_format()?;
                    let bytes_per_pixel = match out_pixel_format {
                        pr::PixelFormat::Bgra4444_8u  => std::mem::size_of::<PixelBGRA8u>(),
                        pr::PixelFormat::Vuya4444_8u  => std::mem::size_of::<PixelVUYA8u>(),
                        pr::PixelFormat::Bgra4444_32f => std::mem::size_of::<PixelBGRA32f>(),
                        pr::PixelFormat::Vuya4444_32f => std::mem::size_of::<PixelVUYA32f>(),
                        _ => return Err(Error::InvalidParms)
                    };

                    log::info!("Iterating {out_pixel_format:?}, pixel size: {bytes_per_pixel}");

                    let in_stride  = in_layer.buffer_stride();
                    let in_data    = in_layer.buffer();
                    let out_stride = out_layer.buffer_stride();
                    let out_data   = out_layer.buffer_mut();
                    out_data.par_chunks_mut(out_stride).enumerate().for_each(|(y, row_bytes)| { // Parallel iterator over buffer rows
                        row_bytes.chunks_mut(bytes_per_pixel).enumerate().for_each(|(x, pix_chunk)| { // iterator over row pixels
                            match out_pixel_format {
                                pr::PixelFormat::Bgra4444_8u => {
                                    let pixel = unsafe { &mut *(in_data.as_ptr().add(y as usize * in_stride) as *mut PixelBGRA8u).add(x) };
                                    let out_pixel = unsafe { &mut *(pix_chunk as *mut _ as *mut PixelBGRA8u) };

                                    let temp = (fastrand::u8(..) as f32 * value) as u8;
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = (pixel.red  .saturating_add(temp)).min(ae::MAX_CHANNEL8 as u8);
                                    out_pixel.green = (pixel.green.saturating_add(temp)).min(ae::MAX_CHANNEL8 as u8);
                                    out_pixel.blue  = (pixel.blue .saturating_add(temp)).min(ae::MAX_CHANNEL8 as u8);
                                },
                                pr::PixelFormat::Vuya4444_8u => {
                                    let pixel = unsafe { &mut *(in_data.as_ptr().add(y as usize * in_stride) as *mut PixelVUYA8u).add(x) };
                                    let out_pixel = unsafe { &mut *(pix_chunk as *mut _ as *mut PixelVUYA8u) };

                                    let temp = (fastrand::u8(..) as f32 * value) as u8;
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.pb = pixel.pb as _;
                                    out_pixel.pr = pixel.pr as _;
                                    out_pixel.luma = (pixel.luma.saturating_add(temp)).min(ae::MAX_CHANNEL8 as u8);
                                },
                                pr::PixelFormat::Bgra4444_32f => {
                                    let pixel = unsafe { &mut *(in_data.as_ptr().add(y as usize * in_stride) as *mut PixelBGRA32f).add(x) };
                                    let out_pixel = unsafe { &mut *(pix_chunk as *mut _ as *mut PixelBGRA32f) };

                                    let temp = fastrand::f32() * value;
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = (pixel.red   + temp).min(1.0);
                                    out_pixel.green = (pixel.green + temp).min(1.0);
                                    out_pixel.blue  = (pixel.blue  + temp).min(1.0);
                                },
                                pr::PixelFormat::Vuya4444_32f => {
                                    let pixel = unsafe { &mut *(in_data.as_ptr().add(y as usize * in_stride) as *mut PixelVUYA32f).add(x) };
                                    let out_pixel = unsafe { &mut *(pix_chunk as *mut _ as *mut PixelVUYA32f) };

                                    let temp = fastrand::f32() * value;
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.pb = pixel.pb as _;
                                    out_pixel.pr = pixel.pr as _;
                                    out_pixel.luma = (pixel.luma + temp).min(1.0);
                                },
                                _ => { }
                            }
                        });
                    });
                } else if value == 0.0 {
                    out_layer.copy_from(&in_layer, None, None)?;
                } else {
                    in_layer.iterate_with(&mut out_layer, 0, progress_final, None, |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                        let temp = (fastrand::u8(..) as f32 * value).round() as u8;
                        match (pixel, out_pixel) {
                            (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha as _;
                                out_pixel.red   = (pixel.red   + temp).min(ae::MAX_CHANNEL8 as u8);
                                out_pixel.green = (pixel.green + temp).min(ae::MAX_CHANNEL8 as u8);
                                out_pixel.blue  = (pixel.blue  + temp).min(ae::MAX_CHANNEL8 as u8);
                            }
                            _ => return Err(Error::BadCallbackParameter)
                        }
                        Ok(())
                    })?;
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
                let cb = extra.callbacks();
                let Some(input_world) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };

                if let Ok(Some(mut output_world)) = cb.checkout_output() {
                    let progress_final = output_world.height() as _;

                    let value = params.get(Params::Noise)?.as_float_slider()?.value() as f32 / 100.0;

                    if value == 0.0 {
                        output_world.copy_from(&input_world, None, None)?;
                    } else {
                        input_world.iterate_with(&mut output_world, 0, progress_final, None, |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                            match (pixel, out_pixel) {
                                (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                    let temp8 = (fastrand::u8(..) as f32 * value).round() as u8;

                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = pixel.red  .saturating_add(temp8).min(ae::MAX_CHANNEL8 as u8);
                                    out_pixel.green = pixel.green.saturating_add(temp8).min(ae::MAX_CHANNEL8 as u8);
                                    out_pixel.blue  = pixel.blue .saturating_add(temp8).min(ae::MAX_CHANNEL8 as u8);
                                }
                                (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                    let temp16 = (fastrand::u16(..32768) as f32 * value).round() as u16;

                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = pixel.red  .saturating_add(temp16).min(ae::MAX_CHANNEL16 as u16);
                                    out_pixel.green = pixel.green.saturating_add(temp16).min(ae::MAX_CHANNEL16 as u16);
                                    out_pixel.blue  = pixel.blue .saturating_add(temp16).min(ae::MAX_CHANNEL16 as u16);
                                }
                                (ae::GenericPixel::PixelF32(pixel), ae::GenericPixelMut::PixelF32(out_pixel)) => {
                                    let temp32 = fastrand::f32() * value;

                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = (pixel.red   + temp32).min(1.0);
                                    out_pixel.green = (pixel.green + temp32).min(1.0);
                                    out_pixel.blue  = (pixel.blue  + temp32).min(1.0);
                                }
                                _ => return Err(Error::BadCallbackParameter)
                            }
                            Ok(())
                        })?;
                    }
                }

                cb.checkin_layer_pixels(0)?;
            }
            _ => {}
        }
        Ok(())
    }
}
