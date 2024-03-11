use after_effects as ae;

mod events;

// This example uses ArbitraryData in Premiere, because it doesn't support custom UI with a Color parameter

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

ae::define_plugin!(Plugin, (), Params);

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
        if !in_data.is_premiere() {
            params.add_customized(Params::Color, "Fill Color", ae::ColorDef::new(), param_cb)?;
        } else {
            params.add_customized(Params::Color, "Fill Color", ae::ArbitraryDef::setup(|f| {
                f.set_default::<ArbColor>(ArbColor::default()).unwrap();
            }), param_cb)?;
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
                    ae::Event::Drag(_)  => { events::drag(&in_data, params, &mut extra)?; }
                    ae::Event::Draw(_)  => { events::draw(&in_data, params, &mut extra)?; }
                    ae::Event::AdjustCursor(_) => { events::change_cursor(&mut extra)?; }
                    _ => {}
                }
            }
            ae::Command::ArbitraryCallback { mut extra } => {
                extra.dispatch::<ArbColor, Params>(Params::Color)?;
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let color: ae::Pixel8 = if !in_data.is_premiere() {
                    params.get(Params::Color)?.as_color()?.value()
                } else {
                    *params.get(Params::Color)?.as_arbitrary()?.value::<ae::Pixel8>()?
                };
                let color16 = ae::pixel8_to_16(color);

                let extent_hint = in_data.extent_hint();

                // iterate over image data.
                in_layer.iterate_with(&mut out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                    match (pixel, out_pixel) {
                        (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                            out_pixel.alpha = pixel.alpha as _;
                            out_pixel.red   = ((pixel.red   as u16 + color.red   as u16) >> 1) as u8;
                            out_pixel.green = ((pixel.green as u16 + color.green as u16) >> 1) as u8;
                            out_pixel.blue  = ((pixel.blue  as u16 + color.blue  as u16) >> 1) as u8;
                        }
                        (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                            out_pixel.alpha = pixel.alpha as _;
                            out_pixel.red   = (pixel.red   + color16.red)   >> 1;
                            out_pixel.green = (pixel.green + color16.green) >> 1;
                            out_pixel.blue  = (pixel.blue  + color16.blue)  >> 1;
                        }
                        _ => return Err(Error::BadCallbackParameter)
                    }
                    Ok(())
                })?;
            }
            _ => { }
        }
        Ok(())
    }
}

// ―――――――――――― Arbitrary data holding a single RGBA color  ――――――――――――

#[derive(serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Copy, Clone)]
#[repr(C)]
struct ArbColor { alpha: u8, red: u8, green: u8, blue: u8 }
impl ae::ArbitraryData<ArbColor> for ArbColor {
    fn default() -> Self {
        Self { red: 255, green: 0, blue: 0, alpha: 255 }
    }
    fn interpolate(&self, other: &Self, v: f64) -> Self {
        let interp = |c1: u8, c2: u8| -> u8 { ((1.0 - v) * c1 as f64 + v * c2 as f64).round() as u8 };
        Self {
            red:   interp(self.red,   other.red),
            green: interp(self.green, other.green),
            blue:  interp(self.blue,  other.blue),
            alpha: interp(self.alpha, other.alpha),
        }
    }
}
const _: () = assert!(std::mem::size_of::<ArbColor>()  == std::mem::size_of::<ae::Pixel8>());
const _: () = assert!(std::mem::align_of::<ArbColor>() == std::mem::align_of::<ae::Pixel8>());
