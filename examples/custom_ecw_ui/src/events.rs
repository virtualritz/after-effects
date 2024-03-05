use super::*;

static mut PPRO_GRAY: f32 = 0.0;

pub fn draw(in_data: &ae::InData, params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
	// Premiere Pro/Elements does not support a standard parameter type with custom UI (bug #1235407), so we can't use the color values.
	// Use an static gray value instead.
	let drawbot_color = if in_data.application_id() != *b"PrMr" {
        let param_color = params.get(Params::Color)?.as_color()?.value();

        ae::drawbot::ColorRgba {
            red:   param_color.red   as f32 / 255.0,
            green: param_color.green as f32 / 255.0,
            blue:  param_color.blue  as f32 / 255.0,
            alpha: 1.0,
        }
	} else {
		// PPro only code. Won't affect Thread-Safety in the AE world
        let gray = unsafe { PPRO_GRAY };
        unsafe { PPRO_GRAY += 0.01 };
        ae::drawbot::ColorRgba {
            red:   gray % 1.0,
            green: gray % 1.0,
            blue:  gray % 1.0,
            alpha: 1.0,
        }
	};

    if event.effect_area() == ae::EffectArea::Control {
        let drawbot = event.context_handle().drawing_reference()?;
        let supplier = drawbot.supplier()?;
        let surface = drawbot.surface()?;

        let mut path = supplier.new_path()?;
        let brush = supplier.new_brush(&drawbot_color)?;

        let current_frame = event.current_frame();

		// Add the rectangle to path
        path.add_rect(&ae::drawbot::RectF32 {
            left:   current_frame.left   as f32 + 0.5, // Center of the pixel in new drawing model is (0.5, 0.5)
            top:    current_frame.top    as f32 + 0.5,
            width:  current_frame.right  as f32 - current_frame.left as f32,
            height: current_frame.bottom as f32 - current_frame.top  as f32,
        })?;

		// Fill the path with the brush created
        surface.fill_path(&brush, &path, ae::drawbot::FillType::Winding)?;

		// Get the default font size.
        let default_font_size = supplier.default_font_size()?;

		// Create default font with default size.  Note that you can provide a different font size.
        let font = supplier.new_default_font(default_font_size)?;

		// Draw string with white color
        let string_brush = supplier.new_brush(&ae::drawbot::ColorRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })?;

        let origin = ae::drawbot::PointF32 {
            x: current_frame.left as f32 + 5.0,
            y: current_frame.top  as f32 + 50.0,
        };

        surface.draw_string(&string_brush, &font, CUSTOM_UI_STRING, &origin, ae::drawbot::TextAlignment::Left, ae::drawbot::TextTruncation::None, 0.0)?;
    }

	event.event_out_flags(ae::EventOutFlags::HANDLED_EVENT);

    Ok(())
}

pub fn drag(params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    let ae::Event::Drag(drag) = event.event() else { return Err(ae::Error::InvalidParms) };
    let context = event.context_handle();

    if context.window_type() == ae::WindowType::Effect {
        if event.effect_area() == ae::EffectArea::Control {
            let mouse_down = drag.screen_point();
            event.set_continue_refcon(1, mouse_down.h as _);
            let mut param = params.get_mut(Params::Color)?;
            let mut color = param.as_color_mut()?;

            let current_color = color.value();
            color.set_value(ae::Pixel8 {
                red:   (((mouse_down.h as u16) << 8) / UI_BOX_WIDTH) as u8,
                blue:  (((mouse_down.v as u16) << 8) / UI_BOX_HEIGHT) as u8,
                green: current_color.red + current_color.blue,
                alpha: 0xFF,
            });
        }
    }

    Ok(())
}

pub fn click(in_data: &ae::InData, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
	// Premiere Pro/Elements does not support this suite
	if in_data.application_id() != *b"PrMr" {
        ae::suites::Helper2::new()?.set_current_extended_tool(ae::ExtendedSuiteTool::Magnify)?;
	} else {
		event.set_cursor(ae::CursorType::Magnify);
	}
    event.send_drag(true);
	event.event_out_flags(ae::EventOutFlags::HANDLED_EVENT);

    Ok(())
}

pub fn change_cursor(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.modifiers().contains(ae::Modifiers::SHIFT_KEY) {
        event.set_cursor(ae::CursorType::Eyedropper);
    } else if event.modifiers().contains(ae::Modifiers::CMD_CTRL_KEY) {
        event.set_cursor(ae::CursorType::Crosshairs);
    }
    Ok(())
}
