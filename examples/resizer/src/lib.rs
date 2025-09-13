use after_effects as ae;

// This sample shows how to resize the output buffer to prevent clipping the edges of an effect (or just to scale the source).
// The resizing occurs during the response to PF_Cmd_FRAME_SETUP.

const RESIZE_AMOUNT_MIN:  i32 = 0;
const RESIZE_AMOUNT_MAX:  i32 = 100;
const RESIZE_AMOUNT_DFLT: i32 = 50;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Amount,
    Color,
    Downsample,
    Use3D,
}

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: InData, _: OutData) -> Result<(), Error> {
        params.add(Params::Amount, "Resizer", ae::SliderDef::setup(|f| {
            f.set_valid_min(RESIZE_AMOUNT_MIN);
            f.set_valid_max(RESIZE_AMOUNT_MAX);
            f.set_slider_min(RESIZE_AMOUNT_MIN);
            f.set_slider_max(RESIZE_AMOUNT_MAX);
            f.set_default(RESIZE_AMOUNT_DFLT);
            f.set_value(f.default());
        }))?;

        params.add(Params::Color, "Resized Area Color", ae::ColorDef::setup(|f| {
            f.set_default(ae::Pixel8 {
                red:   128,
                green: 255,
                blue:  255,
                alpha: 255
            });
            f.set_value(f.default());
        }))?;

        params.add_with_flags(Params::Downsample, "Use Downsample Factors", ae::CheckBoxDef::setup(|f| {
            f.set_default(true);
            f.set_label("Correct at all resolutions");
            f.set_value(false); // value for legacy projects which did not have this param
        }), ae::ParamFlag::USE_VALUE_FOR_OLD_PROJECTS, ae::ParamUIFlags::empty())?;

        // Don't expose 3D capabilities in PPro, since they are unsupported
        if !in_data.is_premiere() {
            params.add(Params::Use3D, "Use lights and cameras", ae::CheckBoxDef::setup(|f| {
                f.set_default(false);
                f.set_label("(new in 5.0!)");
                f.set_value(false);
            }))?;
        }

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("Resizer, v2.4,\rDemonstrate Output Buffer Resizing.\rCopyright 1994-2023 Adobe Inc.");
            }
            ae::Command::FrameSetup { in_layer, .. } => {
                // Output buffer resizing may only occur during PF_Cmd_FRAME_SETUP.
                let border = params.get(Params::Amount)?.as_slider()?.value() as f32;

                let (border_x, border_y) = if params.get(Params::Downsample)?.as_checkbox()?.value() {
                    // shrink the border to accomodate decreased resolutions.
                    (
                        border * f32::from(in_data.downsample_x()),
                        border * f32::from(in_data.downsample_y())
                    )
                } else {
                    (border, border)
                };

                // add 2 times the border width and height to the input width and height to get the output size.
                out_data.set_width( (2.0 * border_x + in_layer.width()  as f32).round() as _);
                out_data.set_height((2.0 * border_y + in_layer.height() as f32).round() as _);

                // The origin of the input buffer corresponds to the (border_x, border_y) pixel in the output buffer.
                out_data.set_origin(ae::Point {
                    h: border_x as _,
                    v: border_y as _
                });
            },
            ae::Command::Render { in_layer, mut out_layer } => {
                let border = params.get(Params::Amount)?.as_slider()?.value() as f32;

                let origin_pt = in_data.output_origin();
                // let iter_rect = ae::Rect { left: 0, top: 0, right: 1, bottom: out_layer.height() as _ };

                let color = params.get(Params::Color)?.as_color()?.value();

                if !in_data.is_premiere() && params.get(Params::Use3D)?.as_checkbox()?.value() {
                    // if we're paying attention to the camera
                    let effect = in_data.effect();
                    let comp_time = effect.comp_time(in_data.current_time(), in_data.time_scale())?;

                    let camera_layer = effect.camera(comp_time)?;
                    if let Some(camera_layer) = camera_layer {
                        log::info!("camera matrix: {:?}", camera_layer.to_world_xform(comp_time));

                        if let Ok(stream_val) = camera_layer.layer_stream_value(aegp::LayerStream::Zoom, aegp::TimeMode::CompTime, comp_time, false) {
                            let focal_length: f64 = stream_val.try_into().unwrap();
                            log::info!("focal_length: {focal_length}");
                        }
                        if let Ok(aperture) = camera_layer.layer_stream_value(aegp::LayerStream::Aperture, aegp::TimeMode::CompTime, comp_time, false) {
                            log::info!("aperture: {aperture:?}");
                        }
                        if let Ok(effect_layer) = effect.layer() {
                            let comp = effect_layer.parent_comp()?;

                            log::info!("comp_dimensions: {:?}", comp.item()?.dimensions()?);

                            for i in 0..comp.num_layers()? {
                                let layer = comp.layer_by_index(i)?;
                                if layer.object_type()? == aegp::ObjectType::Light {
                                    // At this point, you have the camera, the transform,
                                    // the layer to which the effect is applied, and details about the camera. Have fun!
                                    log::info!("light_type: {:?}", layer.light_type()?);
                                }
                            }
                        }
                    }
                }

                // For PPro, since effects operate on the sequence rectangle, not the clip rectangle, we need to take care to color the proper area
                let (border_x, border_y) = if params.get(Params::Downsample)?.as_checkbox()?.value() {
                    // shrink the border to accomodate decreased resolutions.
                    (
                        border * f32::from(in_data.downsample_x()),
                        border * f32::from(in_data.downsample_y())
                    )
                } else {
                    (border, border)
                };

                let pre_effect_origin = in_data.pre_effect_source_origin();

                let rect = if in_data.is_premiere() {
                    Some(ae::Rect {
                        left:   pre_effect_origin.h,
                        top:    pre_effect_origin.v,
                        right:  pre_effect_origin.h + in_data.width()  + 2 * border_x as i32,
                        bottom: pre_effect_origin.v + in_data.height() + 2 * border_y as i32
                    })
                } else {
                    None
                };

                match out_layer.bit_depth() {
                    8  => out_layer.fill(Some(color), rect)?,
                    16 => out_layer.fill16(Some(ae::pixel8_to_16(color)), rect)?,
                    _ => {}
                }

                // Now, copy the input frame
                let dst_rect = ae::Rect {
                    left:   origin_pt.h,
                    top:    origin_pt.v,
                    right:  origin_pt.h + in_layer.width()  as i32,
                    bottom: origin_pt.v + in_layer.height() as i32
                };

                // The suite functions do not automatically detect the requested output quality. Call different functions based on the current quality state.
                if in_data.quality() == ae::Quality::Hi && !in_data.is_premiere() {
                    ae::pf::suites::WorldTransform::new()?.copy_hq(in_data.effect_ref(), in_layer, out_layer, None, Some(dst_rect))?;
                } else if !in_data.is_premiere() {
                    ae::pf::suites::WorldTransform::new()?.copy(in_data.effect_ref(), in_layer, out_layer, None, Some(dst_rect))?;
                } else {
                    let src_rect = ae::Rect {
                        left:   pre_effect_origin.h,
                        top:    pre_effect_origin.v,
                        right:  pre_effect_origin.h + in_data.width() ,
                        bottom: pre_effect_origin.v + in_data.height()
                    };

                    let dst_rect = ae::Rect {
                        left:   pre_effect_origin.h + border_x as i32,
                        top:    pre_effect_origin.v + border_y as i32,
                        right:  pre_effect_origin.h + in_data.width()  + border_x as i32,
                        bottom: pre_effect_origin.v + in_data.height() + border_y as i32
                    };

                    out_layer.copy_from(&in_layer, Some(src_rect), Some(dst_rect))?;
                }
            }
            ae::Command::GetExternalDependencies { mut extra } => {
                match extra.check_type() {
                    ae::DepCheckType::AllDependencies => {
                        extra.set_dependencies_str("All Dependencies requested.")?;
                    }
                    ae::DepCheckType::MissingDependencies => {
                        extra.set_dependencies_str("Missing Dependencies requested.")?;
                    }
                    _ => {
                        extra.set_dependencies_str("None")?;
                    }
                }
            }
            ae::Command::QueryDynamicFlags => {
                // The parameter array passed with PF_Cmd_QUERY_DYNAMIC_FLAGS contains invalid values; use PF_CHECKOUT_PARAM() to obtain valid values.
                let use_3d = params.checkout(Params::Use3D)?.as_checkbox()?.value();

                out_data.set_out_flag2(OutFlags2::IUse3DLights, use_3d);
                out_data.set_out_flag2(OutFlags2::IUse3DCamera, use_3d);
            }

            ae::Command::SmartPreRender { mut extra } => {
                let req = extra.output_request();

                // Output buffer resizing may only occur during PF_Cmd_FRAME_SETUP.
                let border = params.get(Params::Amount)?.as_slider()?.value() as f32;

                let (border_x, border_y) = if params.get(Params::Downsample)?.as_checkbox()?.value() {
                    // shrink the border to accomodate decreased resolutions.
                    (
                        border * f32::from(in_data.downsample_x()),
                        border * f32::from(in_data.downsample_y())
                    )
                } else {
                    (border, border)
                };

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let mut res_rect = ae::Rect::from(in_result.result_rect);
                    res_rect.set_origin(ae::Point {
                        h: -border_x as _,
                        v: -border_y as _
                    });

                    res_rect.set_width( (border_x + res_rect.width()  as f32).round() as _);
                    res_rect.set_height((border_y + res_rect.height() as f32).round() as _);

                    let _ = extra.union_result_rect(res_rect);
                    let _ = extra.union_max_result_rect(res_rect);

                    extra.set_returns_extra_pixels(true);
                }
            }
            ae::Command::SmartRender { extra } => {
                let cb = extra.callbacks();
                let Some(in_layer) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };

                if let Ok(Some(mut out_layer)) = cb.checkout_output() {
                    let border = params.get(Params::Amount)?.as_slider()?.value() as f32;

                    let color = params.get(Params::Color)?.as_color()?.value();

                    // For PPro, since effects operate on the sequence rectangle, not the clip rectangle, we need to take care to color the proper area
                    let (border_x, border_y) = if params.get(Params::Downsample)?.as_checkbox()?.value() {
                        // shrink the border to accomodate decreased resolutions.
                        (
                            border * f32::from(in_data.downsample_x()),
                            border * f32::from(in_data.downsample_y())
                        )
                    } else {
                        (border, border)
                    };

                    let origin_pt = ae::Point {
                        h: border_x as _,
                        v: border_y as _
                    };

                    let pre_effect_origin = in_data.pre_effect_source_origin();

                    let rect = if in_data.is_premiere() {
                        Some(ae::Rect {
                            left:   pre_effect_origin.h,
                            top:    pre_effect_origin.v,
                            right:  pre_effect_origin.h + in_data.width()  + 2 * border_x as i32,
                            bottom: pre_effect_origin.v + in_data.height() + 2 * border_y as i32
                        })
                    } else {
                        None
                    };

                    match out_layer.bit_depth() {
                        8  => out_layer.fill(Some(color), rect)?,
                        16 => out_layer.fill16(Some(ae::pixel8_to_16(color)), rect)?,
                        _ => {}
                    }

                    // Now, copy the input frame
                    let dst_rect = ae::Rect {
                        left:   origin_pt.h,
                        top:    origin_pt.v,
                        right:  origin_pt.h + in_layer.width()  as i32,
                        bottom: origin_pt.v + in_layer.height() as i32
                    };

                    // The suite functions do not automatically detect the requested output quality. Call different functions based on the current quality state.
                    if in_data.quality() == ae::Quality::Hi && !in_data.is_premiere() {
                        ae::pf::suites::WorldTransform::new()?.copy_hq(in_data.effect_ref(), in_layer, out_layer, None, Some(dst_rect))?;
                    } else if !in_data.is_premiere() {
                        ae::pf::suites::WorldTransform::new()?.copy(in_data.effect_ref(), in_layer, out_layer, None, Some(dst_rect))?;
                    } else {
                        let src_rect = ae::Rect {
                            left:   pre_effect_origin.h,
                            top:    pre_effect_origin.v,
                            right:  pre_effect_origin.h + in_data.width() ,
                            bottom: pre_effect_origin.v + in_data.height()
                        };

                        let dst_rect = ae::Rect {
                            left:   pre_effect_origin.h + border_x as i32,
                            top:    pre_effect_origin.v + border_y as i32,
                            right:  pre_effect_origin.h + in_data.width()  + border_x as i32,
                            bottom: pre_effect_origin.v + in_data.height() + border_y as i32
                        };

                        out_layer.copy_from(&in_layer, Some(src_rect), Some(dst_rect))?;
                    }
                }

                cb.checkin_layer_pixels(0)?;
            }
            _ => {}
        }
        Ok(())
    }
}
