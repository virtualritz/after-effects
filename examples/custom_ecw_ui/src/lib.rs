use after_effects as ae;
use after_effects_sys as ae_sys;

mod events;

const CUSTOM_UI_STRING: &str = "Whoopee! A Custom UI!!!\nClick and drag here to\nchange the background color.\nHold down shift or cmd/ctrl\nfor different cursors!";
const STR_FRANK       : &str = "Frank";

const UI_BOX_WIDTH : u16 = 150;
const UI_BOX_HEIGHT: u16 = 150;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Color,
}

struct Plugin {
    _number: i32,
    _name: String,
}

#[derive(Default)]
struct Instance { _unused: u8 }

ae::define_plugin!(Plugin, Instance, Params);

impl Default for Plugin {
    fn default() -> Self {
        Self {
            _number: 93,
            _name: STR_FRANK.to_owned(),
        }
    }
}

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: InData, _: OutData) -> Result<(), Error> {
        let param_cb = |param: &mut ae::ParamDef| {
            param.set_flags(ae::ParamFlag::SUPERVISE);
            param.set_ui_flags(ae::ParamUIFlags::CONTROL);
            param.set_ui_width(UI_BOX_WIDTH);
            param.set_ui_height(UI_BOX_HEIGHT);
            -1
        };

        // Premiere Pro/Elements does not support a standard parameter type with custom UI (bug #1235407). Use an arbitrary or null parameter instead.
        if in_data.application_id() != *b"PrMr" {
            params.add_customized(Params::Color, "Fill Color", ae::ColorDef::new(), param_cb)?;
        } else {
            params.add_customized(Params::Color, "Fill Color", ae::ArbitraryDef::new(), param_cb)?;
        }

        in_data.interact().register_ui(
            CustomUIInfo::new()
                .events(ae::CustomEventFlags::EFFECT)
        )?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: InData, mut out_data: OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                let personal_info = ae::suites::App::new()?.personal_info()?;

                out_data.set_return_msg(&format!("Custom_ECW_UI, v3.2,\r{}\r{}\rExample using CustomUI in the effect control window.\rCopyright 2007-2023 Adobe Inc.", personal_info.name, personal_info.serial_str));
            }
            ae::Command::Event { mut extra } => {
                match extra.event() {
                    ae::Event::Click(_) => { events::click(&in_data, &mut extra)?; }
                    ae::Event::Drag(_)  => { events::drag(params, &mut extra)?; }
                    ae::Event::Draw(_)  => { events::draw(&in_data, params, &mut extra)?; }
                    ae::Event::AdjustCursor(_) => { events::change_cursor(&mut extra)?; }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl AdobePluginInstance for Instance {
    fn flatten(&self) -> Result<(u16, Vec<u8>), Error> {
        Ok((1, Vec::new()))
    }
    fn unflatten(_version: u16, _bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self { _unused: 0 })
    }

    fn user_changed_param(&mut self, _: &mut PluginState, _: Params) -> Result<(), ae::Error> { Ok(()) }

    fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error> {
        let color: ae::Pixel8 = if plugin.in_data.application_id() != *b"PrMr" {
            plugin.params.get(Params::Color)?.as_color()?.value()
        } else {
            unsafe { std::mem::zeroed() }//plugin.params.get_arbitrary(Params::Color).unwrap().value()
        };

        let extent_hint = plugin.in_data.extent_hint();

        // iterate over image data.
        #[rustfmt::skip]
        in_layer.iterate_with(out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
            match (pixel, out_pixel) {
                (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                    out_pixel.alpha = pixel.alpha as _;
                    out_pixel.red   = ((pixel.red   as u16 + color.red   as u16) >> 1) as u8;
                    out_pixel.green = ((pixel.green as u16 + color.green as u16) >> 1) as u8;
                    out_pixel.blue  = ((pixel.blue  as u16 + color.blue  as u16) >> 1) as u8;
                }
                (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                    fn convert_8_to_16(x: u8) -> u16 { (((x as u32 * ae_sys::PF_MAX_CHAN16) + ae_sys::PF_HALF_CHAN8) / ae_sys::PF_MAX_CHAN8) as u16 }
                    out_pixel.alpha = pixel.alpha as _;
                    out_pixel.red   = (pixel.red   + convert_8_to_16(color.red))   >> 1;
                    out_pixel.green = (pixel.green + convert_8_to_16(color.green)) >> 1;
                    out_pixel.blue  = (pixel.blue  + convert_8_to_16(color.blue))  >> 1;
                }
                _ => return Err(Error::BadCallbackParameter)
            }
            Ok(())
        })?;

        Ok(())
    }

    fn handle_command(&mut self, _plugin: &mut PluginState, _cmd: ae::Command) -> Result<(), ae::Error> {
        Ok(())
    }
}
