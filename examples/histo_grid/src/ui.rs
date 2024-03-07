use super::*;

pub fn histogrid_get_box_in_grid(origin: &ae::Point, grid_width: usize, grid_height: usize, box_across: usize, box_down: usize) -> ae::Rect {
    let box_width  = (grid_width  / BOXES_ACROSS) as usize;
    let box_height = (grid_height / BOXES_DOWN)   as usize;

    // Given the grid h+w and the box coord (0,0 through BOXES_ACROSS,BOXES_DOWN) return the rect of the box

    let left = (box_width  * box_across) as i32 + origin.h;
    let top  = (box_height * box_down)   as i32 + origin.v;

    ae::Rect {
        left,
        top,
        right:  left + box_width as i32,
        bottom: top  + box_height as i32,
    }
}

pub fn pf_to_drawbot_rect(in_rect: &ae::Rect) -> ae::drawbot::RectF32 {
    ae::drawbot::RectF32 {
        left:   in_rect.left as f32 + 0.5,
        top:    in_rect.top  as f32 + 0.5,
        width:  (in_rect.right - in_rect.left) as f32,
        height: (in_rect.bottom - in_rect.top) as f32
    }
}

pub fn qd_to_drawbot_color(c: &ae_sys::PF_App_Color) -> ae::drawbot::ColorRgba {
    const MAX_SHORT_COLOR: f32 = 65535.0;
    let inv_sixty_five_k = 1.0 / MAX_SHORT_COLOR;

    ae::drawbot::ColorRgba {
        red:   c.red   as f32 * inv_sixty_five_k,
        green: c.green as f32 * inv_sixty_five_k,
        blue:  c.blue  as f32 * inv_sixty_five_k,
        alpha: 1.0,
    }
}

pub fn acquire_background_color() -> Result<ae::drawbot::ColorRgba, ae::Error> {
    Ok(qd_to_drawbot_color(
        &ae::pf::suites::App::new()?
            .bg_color()?
    ))
}

// EXAMPLE: This requests the upstream input frame for lightweight preview purposes, but highly downsampled for speed
// The frame may not be immediately available. PF_Event_DRAW will get triggered again when async render completes
pub fn request_async_frame_for_preview(in_data: &ae::InData, event: &ae::EventExtra) -> Result<ae_sys::AEGP_FrameReceiptH, ae::Error> {
    let aegp_plugin_id = unsafe { super::AEGP_PLUGIN_ID };

    // get render options description of upstream input frame to effect
    let effect = aegp::Effect::new(in_data.effect_ref(), aegp_plugin_id)?;
    let layer_rops = effect.layer_render_options(aegp_plugin_id)?;

    // make the preview render request fast by downsampling a lot. This could be more intelligent
    layer_rops.set_downsample_factor(16, 16)?;

    // ask the async manager to checkout the desired frame. if this isn't in cache, this will fail but the async manager will schedule an asynchronous render.
    // when that render completes, PF_Event_DRAW will be called again

    let async_mgr = ae::pf::suites::EffectCustomUI::new()?.context_async_manager(in_data, event)?;

    const PURPOSE1: u32 = 1; // unique ID for effect helps hints auto cancelation of rendering by async manager when using multiple render requests

    Ok(async_mgr.checkout_or_render_layer_frame_async_manager(PURPOSE1, layer_rops)?)
}

pub fn draw(seq: &mut Instance, in_data: &ae::InData, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    let mut origin = ae::Point { v: 0, h: 0 };
    let mut grid_width  = 0;
    let mut grid_height = 0;
    let mut box_across  = 0;
    let mut box_down    = 0;

    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;

    let background_color = acquire_background_color()?;

    if event.effect_area() == ae::EffectArea::Control {
        let current_frame = event.current_frame();
        // Use to fill background with AE's BG color
        let onscreen_rect = ae::drawbot::RectF32 {
            left:   current_frame.left   as f32,
            top:    current_frame.top    as f32,
            width:  current_frame.right  as f32 - current_frame.left as f32,
            height: current_frame.bottom as f32 - current_frame.top  as f32 + 1.0,
        };
        origin = ae::Point {
            v: onscreen_rect.top as _,
            h: onscreen_rect.left as _,
        };

        // Calculate the space taken up by the grid
        // Allow the width to scale horizontally, but not too much
        grid_width  = onscreen_rect.width as _;
        grid_height = onscreen_rect.height as _;
        if (grid_width as f32) < UI_GRID_WIDTH as f32 / 1.5 {
            grid_width = (UI_GRID_WIDTH as f32 / 1.5) as _;
        } else if grid_width as f32 > UI_GRID_WIDTH as f32 * 1.5 {
            grid_width = (UI_GRID_WIDTH as f32 * 1.5) as _;
        }
        surface.paint_rect(&background_color, &onscreen_rect)?
    }

    // EXAMPLE: If this is the effect pane, then request the upstream frame for preview purposes and do any lightweight preview computation on it.
    // If the have a frame, update our sequence data color cache with the new computation
    // If the frame is not be immediately available, we draw our cached (or blank) state PF_Event_DRAW will get called again later when the frame render completes and then we'll try again
    if event.window_type() == ae::WindowType::Effect {
        let frame_receipt = request_async_frame_for_preview(in_data, event)?;

        if !frame_receipt.is_null() {
            let r_suite = ae::aegp::suites::Render::new()?;

            let world = r_suite.receipt_world(frame_receipt)?;
            if !world.is_null() { // receipt could be valid but empty
                seq.color_cache.compute_color_grid_from_frame(&ae::Layer::from_aegp_world(in_data, world)?)?;
                seq.valid = true;
            }
            r_suite.checkin_frame(frame_receipt)?;
        }
    }

    // The below will draw the grid with the color we have.
    // Might be fresh computed, previously cached (to reduce flicker), or just blank because it isn't valid yet (on sequence data setup)

    // Draw the color grid using the cached color preview (if valid)
    if seq.valid {
        let mut pixel_colr: [ae::drawbot::ColorRgba; CACHE_ELEMENTS] = unsafe { std::mem::zeroed() };
        let colors = &seq.color_cache.color;

        for i in 0..CACHE_ELEMENTS {
            pixel_colr[i].red   = colors[i].red;
            pixel_colr[i].green = colors[i].green;
            pixel_colr[i].blue  = colors[i].blue;
            pixel_colr[i].alpha = 1.0;
        }

        for i in 0..BOXES_PER_GRID {
            // Fill in our grid
            let box_rect = histogrid_get_box_in_grid(&origin, grid_width, grid_height, box_across, box_down);

            let mut path = supplier.new_path()?;
            path.add_rect(&pf_to_drawbot_rect(&box_rect))?;

            let brush = supplier.new_brush(&pixel_colr[i])?;
            surface.fill_path(&brush, &path, ae::drawbot::FillType::Winding)?;

            box_across += 1;

            if box_across == BOXES_ACROSS {
                box_across = 0;
                box_down += 1;
            }
        }
    }

    let black_color = ae::drawbot::ColorRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
    let pen = supplier.new_pen(&black_color, 1.0)?;

    // Draw the grid - outline
    for box_down in 0..BOXES_DOWN {
        for box_across in 0..BOXES_ACROSS {
            let box_rect = histogrid_get_box_in_grid(&origin, grid_width, grid_height, box_across, box_down);

            let mut path = supplier.new_path()?;
            path.add_rect(&pf_to_drawbot_rect(&box_rect))?;

            surface.stroke_path(&pen, &path)?;
        }
    }

    event.event_out_flags(ae::EventOutFlags::HANDLED_EVENT);

    Ok(())
}

// pub fn histogrid_point_in_rect(point: &ae::Point, rect: &ae::Rect) -> bool {
//     point.h > rect.left && point.h <= rect.right && point.v > rect.top && point.v <= rect.bottom
// }
// pub fn deactivate(in_data: &ae::InData) -> Result<(), ae::Error> {
//     // Premiere Pro/Elements does not support this suite
//     if in_data.application_id() != *b"PrMr" {
//         ae::suites::AdvApp::new()?.info_draw_text("HistoGrid - Deactivate Event", "Adobe Inc")?;
//     }
//     Ok(())
// }
