use super::*;

pub fn colorgrid_get_box_in_grid(origin: &ae::Point, grid_width: usize, grid_height: usize, box_across: usize, box_down: usize) -> ae::Rect {
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

pub fn colorgrid_point_in_rect(point: &ae::Point, rect: &ae::Rect) -> bool {
    point.h > rect.left && point.h <= rect.right && point.v > rect.top && point.v <= rect.bottom
}

pub fn colorgrid_get_new_color(in_data: &ae::InData, hpos: usize, vpos: usize, arb_data: &mut ArbData) -> Result<(), Error> {
    let pos = vpos * BOXES_ACROSS + hpos;

    if hpos < BOXES_ACROSS && vpos < BOXES_DOWN {
        let box_color = &mut arb_data.colors[pos as usize];

        // Premiere Pro/Elements don't support PF_AppColorPickerDialog
        if in_data.application_id() != *b"PrMr" {
            let color = pf::PixelF32 { alpha: box_color.alpha, red: box_color.red, green: box_color.green, blue: box_color.blue };
            let new_color = ae::pf::suites::App::new()?.color_picker_dialog(Some("ColorGrid!"), &color, true)?;
            box_color.red   = new_color.red;
            box_color.green = new_color.green;
            box_color.blue  = new_color.blue;
        } else {
            box_color.red   = fastrand::u8(..) as f32 / 255.0;
            box_color.green = fastrand::u8(..) as f32 / 255.0;
            box_color.blue  = fastrand::u8(..) as f32 / 255.0;
        }
    }
    Ok(())
}

pub fn click(in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    let mouse_pt = event.screen_point();
    let origin = ae::Point {
        h: event.current_frame().left,
        v: event.current_frame().top,
    };
    let grid_rect = ae::Rect {
        left:   origin.h,
        top:    origin.v,
        right:  origin.h + UI_GRID_WIDTH as i32,
        bottom: origin.v + UI_GRID_HEIGHT as i32,
    };

    if event.effect_area() == ae::EffectArea::Control {
        // Is the click in the grid?
        if colorgrid_point_in_rect(&mouse_pt, &grid_rect) {
            let mut arb_param = params.get_mut(Params::GridUI)?;
            let mut arb_param = arb_param.as_arbitrary_mut()?;
            let mut arb_data = arb_param.value::<ArbData>()?;

            let mut box_across = 0;
            let mut box_down = 0;

            for _ in 0..CG_ARBDATA_ELEMENTS {
                let box_rect = colorgrid_get_box_in_grid(&origin, UI_GRID_WIDTH as _, UI_GRID_HEIGHT as _, box_across, box_down);

                if colorgrid_point_in_rect(&mouse_pt, &box_rect) {
                    break;
                } else {
                    box_across += 1;
                    if box_across == BOXES_ACROSS {
                        box_down += 1;
                        box_across = 0;
                    }
                }
            }

            colorgrid_get_new_color(in_data, box_across, box_down, &mut arb_data)?;
            arb_param.set_value_changed();

            // Specify the area to redraw
            let inval = event.current_frame();
            ae::pf::suites::App::new()?.invalidate_rect(event.context_handle(), Some(inval))?;

            event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT | ae::EventOutFlags::UPDATE_NOW);
        }
    }

    // Premiere Pro/Elements does not support this suite
    if in_data.application_id() != *b"PrMr" {
        ae::suites::AdvApp::new()?.info_draw_text("ColorGrid - DoClick Event", "Adobe Inc")?;
    }

    Ok(())
}

pub fn change_cursor(in_data: &ae::InData, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    // Determine where the mouse is, and sent the info window the appropriate color and change the cursor only when over the ColorGrid
    let mouse_pt = event.screen_point();
    let origin = ae::Point {
        h: event.current_frame().left,
        v: event.current_frame().top,
    };
    let grid_rect = ae::Rect {
        left:   origin.h,
        top:    origin.v,
        right:  origin.h + UI_GRID_WIDTH as i32,
        bottom: origin.v + UI_GRID_HEIGHT as i32,
    };

    if event.effect_area() == ae::EffectArea::Control {
        // Is the click in the grid?
        if colorgrid_point_in_rect(&mouse_pt, &grid_rect) {
            event.set_cursor(ae::CursorType::Eyedropper);
        }
    }

    // Premiere Pro/Elements does not support this suite
    if in_data.application_id() != *b"PrMr" {
        ae::suites::AdvApp::new()?.info_draw_text("ColorGrid - ChangeCursor Event", "Adobe Inc")?;
    }

    Ok(())
}

pub fn draw(_in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
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
        surface.paint_rect(&background_color, &onscreen_rect)?;
    }

    // Get the arb data to fill out the grid colors
    let arb_param = params.get(Params::GridUI)?;
    let arb_param = arb_param.as_arbitrary()?;
    let arb_data = arb_param.value::<ArbData>()?;

    if true {
        let mut pixel_colr: [ae::drawbot::ColorRgba; CG_ARBDATA_ELEMENTS] = unsafe { std::mem::zeroed() };
        for i in 0..CG_ARBDATA_ELEMENTS {
            pixel_colr[i].red   = arb_data.colors[i].red;
            pixel_colr[i].green = arb_data.colors[i].green;
            pixel_colr[i].blue  = arb_data.colors[i].blue;
            pixel_colr[i].alpha = 1.0;
        }

        for i in 0..BOXES_PER_GRID {
            // Fill in our grid
            let box_rect = colorgrid_get_box_in_grid(&origin, grid_width, grid_height, box_across, box_down);

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
            let box_rect = colorgrid_get_box_in_grid(&origin, grid_width, grid_height, box_across, box_down);

            let mut path = supplier.new_path()?;
            path.add_rect(&pf_to_drawbot_rect(&box_rect))?;

            surface.stroke_path(&pen, &path)?;
        }
    }

    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);

    Ok(())
}

// pub fn deactivate(in_data: &ae::InData) -> Result<(), ae::Error> {
//     // Premiere Pro/Elements does not support this suite
//     if in_data.application_id() != *b"PrMr" {
//         ae::suites::AdvApp::new()?.info_draw_text("HistoGrid - Deactivate Event", "Adobe Inc")?;
//     }
//     Ok(())
// }
