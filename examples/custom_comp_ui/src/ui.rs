use super::*;

// Calculates the points for an oval bounded by oval_frame
fn calculate_oval(oval_frame: &ae::Rect) -> [ae::drawbot::PointF32; OVAL_PTS] {
    let oval_width  = (oval_frame.right  - oval_frame.left) as f32 / 2.0;
    let oval_height = (oval_frame.bottom - oval_frame.top) as f32  / 2.0;

    // Calculate center point for oval
    let center_point = ae::drawbot::PointF32 {
        x: oval_frame.left as f32 + oval_width,
        y: oval_frame.top  as f32 + oval_height,
    };

    let mut poly_oval = [ae::drawbot::PointF32 { x: 0.0, y: 0.0 }; OVAL_PTS];

    // Plot out the oval with OVAL_PTS number of points
    for i in 0..OVAL_PTS {
        let rad = (2.0 * std::f32::consts::PI * i as f32) / OVAL_PTS as f32;

        poly_oval[i].x = rad.sin();
        poly_oval[i].y = rad.cos();

        // Transform the point on a unit circle to the corresponding point on the oval
        poly_oval[i].x = poly_oval[i].x * oval_width  + center_point.x;
        poly_oval[i].y = poly_oval[i].y * oval_height + center_point.y;
    }

    poly_oval
}

// Draws an oval
fn draw_oval(in_data: &ae::InData, oval_frame: &ae::Rect, drawbot: &drawbot::Drawbot) -> Result<(), ae::Error> {
    let poly_oval = calculate_oval(oval_frame);

    let mut path = drawbot.supplier()?.new_path()?;
    path.move_to(poly_oval[0].x, poly_oval[0].y)?;

    for i in 0..OVAL_PTS {
        path.line_to(poly_oval[i].x, poly_oval[i].y)?;
    }
    path.line_to(poly_oval[0].x, poly_oval[0].y)?;

    // Currently, EffectCustomUIOverlayThemeSuite is unsupported in Premiere Pro/Elements
    if in_data.application_id() != *b"PrMr" {
        ae::pf::suites::EffectCustomUIOverlayTheme::new()?.stroke_path(drawbot, &path, false)?;
    } else {
        let foreground_color = ae::drawbot::ColorRgba {
            red:   0.9,
            green: 0.9,
            blue:  0.9,
            alpha: 1.0,
        };
        let pen = drawbot.supplier()?.new_pen(&foreground_color, 1.0)?;
        drawbot.surface()?.stroke_path(&pen, &path)?;
    }

    Ok(())
}

// Calculates the frame in fixed coordinates based on the params passed in
fn fixed_frame_from_params(in_data: &ae::InData, params: &ae::Parameters<Params>) -> Result<ae_sys::PF_FixedRect, Error> {
    let x_rad   = params.get(Params::XRadius)?.as_float_slider()?.value() as f32;
    let y_rad   = params.get(Params::YRadius)?.as_float_slider()?.value() as f32;
    let center  = params.get(Params::Point)?.as_point()?.value();
    let par_inv = f32::from(in_data.pixel_aspect_ratio().inv());

    Ok(ae_sys::PF_FixedRect {
        top   : ae::Fixed::from(center.1 - y_rad          ).as_fixed(),
        bottom: ae::Fixed::from(center.1 + y_rad          ).as_fixed(),
        left  : ae::Fixed::from(center.0 - x_rad * par_inv).as_fixed(),
        right : ae::Fixed::from(center.0 + x_rad * par_inv).as_fixed(),
    })
}

fn frame_from_params(in_data: &ae::InData, params: &ae::Parameters<Params>) -> Result<ae::Rect, Error> {
    let x_rad   = params.get(Params::XRadius)?.as_float_slider()?.value() as f32;
    let y_rad   = params.get(Params::YRadius)?.as_float_slider()?.value() as f32;
    let center  = params.get(Params::Point)?.as_point()?.value();
    let par_inv = f32::from(in_data.pixel_aspect_ratio().inv());

    Ok(ae::Rect {
        top   : (center.1 - y_rad          ).round() as _,
        bottom: (center.1 + y_rad          ).round() as _,
        left  : (center.0 - x_rad * par_inv).round() as _,
        right : (center.0 + x_rad * par_inv).round() as _,
    })
}

fn source_to_frame_rect(in_data: &ae::InData, event: &mut ae::EventExtra, fx_frame: &mut ae_sys::PF_FixedRect) -> Result<[ae_sys::PF_FixedPoint; 4], Error> {
    let mut bounding_box = [
        ae_sys::PF_FixedPoint { x: fx_frame.left,  y: fx_frame.top },
        ae_sys::PF_FixedPoint { x: fx_frame.right, y: fx_frame.top },
        ae_sys::PF_FixedPoint { x: fx_frame.right, y: fx_frame.bottom },
        ae_sys::PF_FixedPoint { x: fx_frame.left,  y: fx_frame.bottom },
    ];

    if event.window_type() == ae::WindowType::Comp {
        for i in 0..4 {
            event.callbacks().layer_to_comp(in_data.current_time(), in_data.time_scale(), &mut bounding_box[i])?;
        }
    }
    for j in 0..4 {
        event.callbacks().source_to_frame(&mut bounding_box[j])?;
    }

    fx_frame.left   = bounding_box[0].x;
    fx_frame.top    = bounding_box[0].y;
    fx_frame.right  = bounding_box[1].x;
    fx_frame.bottom = bounding_box[2].y;

    Ok(bounding_box)
}

fn comp_frame_to_layer(in_data: &ae::InData, event: &mut ae::EventExtra, frame_pt: ae::Point, lyr_pt: &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {
    let mut fix_lyr = ae_sys::PF_FixedPoint {
        x: ae::Fixed::from_int(frame_pt.h).as_fixed(),
        y: ae::Fixed::from_int(frame_pt.v).as_fixed()
    };

    event.callbacks().frame_to_source(&mut fix_lyr)?;

    // Now back into layer space
    event.callbacks().comp_to_layer(in_data.current_time(), in_data.time_scale(), &mut fix_lyr)?;

    lyr_pt.h = fix_lyr.x;
    lyr_pt.v = fix_lyr.y;

    Ok(fix_lyr)
}

fn layer_to_comp_frame(in_data: &ae::InData, event: &mut ae::EventExtra, layer_pt: ae::Point, frame_pt: &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {
    let mut fix_frame = ae_sys::PF_FixedPoint {
        x: ae::Fixed::from_int(layer_pt.h).as_fixed(),
        y: ae::Fixed::from_int(layer_pt.v).as_fixed()
    };

    event.callbacks().layer_to_comp(in_data.current_time(), in_data.time_scale(), &mut fix_frame)?;
    event.callbacks().source_to_frame(&mut fix_frame)?;

    frame_pt.h = ae::Fixed::from_fixed(fix_frame.x).to_int();
    frame_pt.v = ae::Fixed::from_fixed(fix_frame.y).to_int();

    Ok(fix_frame)
}

fn layer_frame_to_layer(_: &ae::InData, event: &mut ae::EventExtra, frame_pt: ae::Point, lyr_pt: &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {
    let mut fix_lyr = ae_sys::PF_FixedPoint {
        x: ae::Fixed::from_int(frame_pt.h).to_int(),
        y: ae::Fixed::from_int(frame_pt.v).to_int()
    };

    event.callbacks().frame_to_source(&mut fix_lyr)?;

    lyr_pt.h = ae::Fixed::from_fixed(fix_lyr.x).to_int();
    lyr_pt.v = ae::Fixed::from_fixed(fix_lyr.y).to_int();

    Ok(fix_lyr)
}

fn layer_to_layer_frame(_: &ae::InData, event: &mut ae::EventExtra, layer_pt: ae::Point, frame_pt: &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {
    let mut fix_frame = ae_sys::PF_FixedPoint {
        x: ae::Fixed::from_int(layer_pt.h).as_fixed(),
        y: ae::Fixed::from_int(layer_pt.v).as_fixed()
    };

    event.callbacks().source_to_frame(&mut fix_frame)?;

    frame_pt.h = ae::Fixed::from_fixed(fix_frame.x).to_int();
    frame_pt.v = ae::Fixed::from_fixed(fix_frame.y).to_int();

    Ok(fix_frame)
}

pub fn draw(in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.window_type() == ae::WindowType::Layer || event.window_type() == ae::WindowType::Comp {
        let drawbot = event.context_handle().drawing_reference()?;
        let surface = drawbot.surface()?;

        let mut fx_frame = fixed_frame_from_params(in_data, params)?;
        let _points = source_to_frame_rect(in_data, event, &mut fx_frame);

        let frame = ae::Rect {
            top   : ae::Fixed::from_fixed(fx_frame.top).to_int_rounded(),
            bottom: ae::Fixed::from_fixed(fx_frame.bottom).to_int_rounded(),
            left  : ae::Fixed::from_fixed(fx_frame.left).to_int_rounded(),
            right : ae::Fixed::from_fixed(fx_frame.right).to_int_rounded(),
        };

        // Currently, EffectCustomUIOverlayThemeSuite is unsupported in Premiere Pro/Elements
        let foreground_color = if in_data.application_id() != *b"PrMr" {
            ae::pf::suites::EffectCustomUIOverlayTheme::new()?.preferred_foreground_color()?
        } else {
            drawbot::ColorRgba {
                alpha: 1.0,
                blue:  0.9,
                green: 0.9,
                red:   0.9,
            }
        };

        draw_oval(in_data, &frame, &drawbot)?;

        let mut bbox = ae::drawbot::RectF32 {
            left: 0.0,
            top:  0.0,
            width: 6.0,
            height: 6.0,
        };

        bbox.top  = frame.top  as f32 - 3.0;
        bbox.left = frame.left as f32 - 3.0;
        surface.paint_rect(&foreground_color, &bbox)?;

        bbox.top  = frame.top   as f32 - 3.0;
        bbox.left = frame.right as f32 - 3.0;
        surface.paint_rect(&foreground_color, &bbox)?;

        bbox.top  = frame.bottom as f32 - 3.0;
        bbox.left = frame.left   as f32 - 3.0;
        surface.paint_rect(&foreground_color, &bbox)?;

        bbox.top  = frame.bottom as f32 - 3.0;
        bbox.left = frame.right  as f32 - 3.0;
        surface.paint_rect(&foreground_color, &bbox)?;

        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    }

    Ok(())
}

fn do_click_handles<F>(in_data: &ae::InData, frame: &ae::Rect, frame_func: F, event: &mut ae::EventExtra) -> Result<bool, ae::Error>
where F: Fn(&ae::InData, &mut ae::EventExtra, ae::Point, &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {

    let mut done = false;
    let mouse_down_pt = event.screen_point();
    let mut corners = [
        ae::Point { h: frame.left,  v: frame.top },
        ae::Point { h: frame.right, v: frame.top },
        ae::Point { h: frame.right, v: frame.bottom },
        ae::Point { h: frame.left,  v: frame.bottom },
    ];

    // let mut hit = -1;

    for i in 0..4 {
        // Convert corners to comp frame
        let mouse_layer = frame_func(in_data, event, corners[i], &mut corners[i])?;

        let mut slop = (corners[i].h - mouse_down_pt.h).abs();
        slop        += (corners[i].v - mouse_down_pt.v).abs();

        if slop < CCU_SLOP {
            // hit = i as isize;
            done = true;
            event.set_send_drag(true);
            event.set_continue_refcon(0, Ccu::Handles as i64);
            event.set_continue_refcon(1, mouse_layer.x as i64);
            event.set_continue_refcon(2, mouse_layer.y as i64);
            event.set_continue_refcon(3, false as i64);
            break;
        }
    }

    Ok(done)
}

pub fn click(in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    let frame = frame_from_params(in_data, params)?;

    if event.window_type() == ae::WindowType::Layer {
        if do_click_handles(in_data, &frame, layer_to_layer_frame, event)? {
            event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
        }
    } else if event.window_type() == ae::WindowType::Comp {
        if do_click_handles(in_data, &frame, layer_to_comp_frame, event)? {
            event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
        }
    }

    Ok(())
}

fn do_drag_handles<F>(in_data: &ae::InData, params: &mut ae::Parameters<Params>, frame_func: F, event: &mut ae::EventExtra) -> Result<(), ae::Error>
where F: Fn(&ae::InData, &mut ae::EventExtra, ae::Point, &mut ae::Point) -> Result<ae_sys::PF_FixedPoint, Error> {
    let mut mouse_down_pt = event.screen_point();

    // if event.in_flags().contains(ae::EventInFlags::DONT_DRAW) {
    //     draw = false;
    // }

    let mouse_layer = frame_func(in_data, event, mouse_down_pt, &mut mouse_down_pt)?;

    // let old_center = ae_sys::PF_FixedPoint {
    //     x: event.continue_refcon(1) as _,
    //     y: event.continue_refcon(2) as _,
    // };

    let center = params.get(Params::Point)?.as_point()?.value();
    let par = f32::from(in_data.pixel_aspect_ratio());

    // Calculate new radius
    let new_x = (center.0 - ae::Fixed::from_fixed(mouse_layer.x).as_f32()) * par;
    let new_y =  center.1 - ae::Fixed::from_fixed(mouse_layer.y).as_f32();

    params.get_mut(Params::XRadius)?.as_float_slider_mut()?.set_value(new_x.abs() as _);
    params.get_mut(Params::YRadius)?.as_float_slider_mut()?.set_value(new_y.abs() as _);

    event.set_send_drag(true);
    event.set_continue_refcon(0, Ccu::Handles as i64);
    event.set_continue_refcon(1, mouse_layer.x as i64);
    event.set_continue_refcon(2, mouse_layer.y as i64);
    event.set_continue_refcon(3, true as i64);

    if event.last_time() {
        event.set_continue_refcon(0, Ccu::None as i64);
        event.set_send_drag(false);
    }

    Ok(())
}

pub fn drag(in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    // let mut frame = frame_from_params(in_data, params)?;

    if event.continue_refcon(0) == Ccu::Handles as i64 {
        if event.window_type() == ae::WindowType::Layer {
            do_drag_handles(in_data, params, layer_frame_to_layer, event)?;
        } else if event.window_type() == ae::WindowType::Comp {
            do_drag_handles(in_data, params, comp_frame_to_layer, event)?;
        }
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    }

    Ok(())
}
