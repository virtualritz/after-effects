use after_effects as ae;
use after_effects_sys as ae_sys;

mod ui;

const OVAL_PTS: usize = 64;
const CCU_SLOP: i32 = 16;

const CCU_RADIUS_BIG_MAX: f32 = 4000.0;
const CCU_RADIUS_MAX    : f32 = 250.0;
const CCU_RADIUS_MIN    : f32 = 0.0;
const CCU_RADIUS_DFLT   : f64 = 50.0;
const CCU_PTX_DFLT      : f32 = 50.0;
const CCU_PTY_DFLT      : f32 = 50.0;
const SLIDER_PRECISION  : i16 = 1;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    XRadius,
    YRadius,
    Point
}

enum Ccu {
    None,
    Handles
}

#[derive(Default)]
struct Plugin { }

ae::define_plugin!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::XRadius, "Horizontal Radius", ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(CCU_RADIUS_MIN);
            f.set_valid_max(CCU_RADIUS_BIG_MAX);
            f.set_slider_min(CCU_RADIUS_MIN);
            f.set_slider_max(CCU_RADIUS_MAX);
            f.set_default(CCU_RADIUS_DFLT);
            f.set_precision(SLIDER_PRECISION);
        }))?;

        params.add(Params::YRadius, "Vertical Radius", ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(CCU_RADIUS_MIN);
            f.set_valid_max(CCU_RADIUS_BIG_MAX);
            f.set_slider_min(CCU_RADIUS_MIN);
            f.set_slider_max(CCU_RADIUS_MAX);
            f.set_default(CCU_RADIUS_DFLT);
            f.set_precision(SLIDER_PRECISION);
        }))?;

        params.add(Params::Point, "Center", ae::PointDef::setup(|f| {
            f.set_x_value(CCU_PTX_DFLT);
            f.set_y_value(CCU_PTY_DFLT);
            f.set_default_x(f.x_value());
            f.set_default_y(f.y_value());
            f.set_restrict_bounds(false);
        }))?;

        in_data.interact().register_ui(
            CustomUIInfo::new()
                .events(ae::CustomEventFlags::LAYER | ae::CustomEventFlags::COMP)
        )?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
        log::set_max_level(log::LevelFilter::Debug);
        log_panics::init();

        log::info!("handle_command: {:?}, thread: {:?}, ptr: {:?}", ae::RawCommand::from(cmd.as_raw()), std::thread::current().id(), self as *const _);

        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Custom Comp UI, v3.3,\rManages a custom Comp (and Layer) window UI.\rCopyright 1994-2023\rAdobe Inc.");
            }
            ae::Command::Event { mut extra } => {
                match extra.event() {
                    ae::Event::Click(_) => {
                        log::info!("event click, send_drag: {}", extra.send_drag());
                        if extra.send_drag() {
                            ui::drag(&in_data, params, &mut extra)?;
                        } else {
                            ui::click(&in_data, params, &mut extra)?;
                        }
                    }
                    ae::Event::Drag(_) => { ui::drag(&in_data, params, &mut extra)?; }
                    ae::Event::Draw(_) => { ui::draw(&in_data, params, &mut extra)?; log::info!("drawn"); }
                    _ => {}
                }
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let out_extent = out_layer.extent_hint();
                let min_x      = out_extent.left;
                let max_x      = out_extent.right;
                let min_y      = out_extent.top;
                let max_y      = out_extent.bottom;
                let rad_x      = params.get(Params::XRadius)?.as_float_slider()?.value() as f32;
                let rad_y      = params.get(Params::YRadius)?.as_float_slider()?.value() as f32;
                let rad_x_sqr  = rad_x.powi(2);
                let rad_y_sqr  = rad_y.powi(2);
                let center = params.get(Params::Point)?.as_point()?.value();

                let par = f32::from(in_data.pixel_aspect_ratio());
                let downsample_x_inv = f32::from(in_data.downsample_x().inv());
                let downsample_y_inv = f32::from(in_data.downsample_y().inv());

                // If either width or height is 0, just copy the entire frame from input to output

                if rad_x == 0.0 || rad_y == 0.0 {
                    // Premiere Pro/Elements doesn't support WorldTransformSuite1, but it does support many of the callbacks in utils
                    if in_data.quality() == ae::Quality::Hi && in_data.application_id() != *b"PrMr" {
                        ae::pf::suites::WorldTransform::new()?.copy_hq(in_data.effect_ref(), in_layer, out_layer, None, None)?;
                    } else if in_data.application_id() != *b"PrMr" {
                        ae::pf::suites::WorldTransform::new()?.copy(in_data.effect_ref(), in_layer, out_layer, None, None)?;
                    } else {
                        out_layer.copy_from(&in_layer, None, None)?;
                    }
                } else {
                    // iterate over image data.
                    in_layer.iterate_with(&mut out_layer, 0, out_extent.height(), Some(out_extent), |x: i32, y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                        let in_ellipse = {
                            let dy = (y as f32 - center.1) * downsample_y_inv;
                            if dy > rad_y || dy < -rad_y || y < min_y || y > max_y {
                                false
                            } else {
                                let dx = (x as f32 - center.0) * downsample_x_inv * par;
                                // If the pixel is out of the visible x range covered by the circle
                                if dx > rad_x || dx < -rad_x || x < min_x || x > max_x {
                                    false
                                } else {
                                    // An ellipse centered at (0,0) has the equation:
                                    // x^2 / a^2 + y^2 / b^2 = 1, where a is the width and b is the height of the ellipse
                                    !(dx.powi(2) / rad_x_sqr + dy.powi(2) / rad_y_sqr >= 1.0)
                                }
                            }
                        };
                        match (pixel, out_pixel) {
                            (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                if in_ellipse {
                                    *out_pixel = ae::Pixel8 { alpha: pixel.alpha, red: 0, green: ae_sys::PF_MAX_CHAN8 as u8, blue: 0 };
                                } else {
                                    *out_pixel = *pixel;
                                }
                            }
                            (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                if in_ellipse {
                                    *out_pixel = ae::Pixel16 { alpha: pixel.alpha, red: 0, green: ae_sys::PF_MAX_CHAN16 as u16, blue: 0 };
                                } else {
                                    *out_pixel = *pixel;
                                }
                            }
                            _ => return Err(Error::BadCallbackParameter)
                        }
                        Ok(())
                    })?;

                    // Alternatively, you can loop manually through the data
                    /*unsafe {
                        let in_gutter  =  (in_layer.row_bytes() / std::mem::size_of::<ae::Pixel8>() as isize) - in_layer.width() as isize;
                        let out_gutter = (out_layer.row_bytes() / std::mem::size_of::<ae::Pixel8>() as isize) - out_layer.width() as isize;
                        let mut bop_out = out_layer.data_ptr_mut() as *mut ae::Pixel8;
                        let mut bop_in  = in_layer.data_ptr() as *mut ae::Pixel8;

                        for y in 0..out_layer.height() as i32 {
                            let mut dy  = y as f32 - center.1;
                            dy *= downsample_y_inv;
                            let dy_sqr = dy * dy;

                            // If the row will not be touched by the effect, copy the row from input to output
                            if dy > rad_y || dy < -rad_y || y < min_y || y > max_y {
                                let mut x = out_layer.width();
                                while x > 0 {
                                    *bop_out = *bop_in;
                                    bop_in  = bop_in.add(1);
                                    bop_out = bop_out.add(1);
                                    x -= 1;
                                }
                            } else {
                                for x in 0..in_layer.width() as i32 {
                                    let mut dx = x as f32 - center.0;
                                    dx *= downsample_x_inv;
                                    dx *= par;
                                    let dx_sqr = dx * dx;

                                    // If the pixel is out of the visible x range covered by the circle
                                    if dx > rad_x || dx < -rad_x || x < min_x || x > max_x {
                                        *bop_out = *bop_in;
                                        bop_in  = bop_in.add(1);
                                        bop_out = bop_out.add(1);
                                    } else {
                                        // An ellipse centered at (0,0) has the equation:
                                        // x^2 / a^2 + y^2 / b^2 = 1, where a is the width and b is the height of the ellipse
                                        // If the pixel is outside the ellipse's radius, just copy the source
                                        if dx_sqr / rad_x_sqr + dy_sqr / rad_y_sqr >= 1.0 {
                                            *bop_out = *bop_in;
                                            bop_in = bop_in.add(1);
                                            bop_out = bop_out.add(1);
                                        } else {
                                            (*bop_out).alpha = (*bop_in).alpha;
                                            (*bop_out).green = ae_sys::PF_MAX_CHAN8 as u8;
                                            (*bop_out).blue  = 0;
                                            (*bop_out).red   = 0;
                                            bop_in  = bop_in.add(1);
                                            bop_out = bop_out.add(1);
                                        }
                                    }
                                }
                            }

                            // At the end of each row, account for the gutter (this number can vary by platform and for other reasons)
                            if y >= 0 && y < in_layer.height() as i32 {
                                bop_in = bop_in.offset(in_gutter);
                            }

                            bop_out = bop_out.offset(out_gutter);

                            // Check for interrupt!
                            in_data.interact().progress(y + 1, out_layer.height() as i32)?;
                        }
                    }*/
                }
            }
            _ => {}
        }
        Ok(())
    }
}
