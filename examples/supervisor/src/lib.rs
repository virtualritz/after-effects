use after_effects as ae;

// This plug-in demonstrates parameter supervision.

// This plugin is not marked as Thread-Safe because it is writing to sequence data during PF_Cmd_UPDATE_PARAMS_UI.
// There is an exisiting issue that's preventing this from working during Multi-Frame rendering.

#[repr(i32)]
#[derive(PartialEq, Eq)]
enum Mode {
    Basic = 1,
    Advanced,
}

#[repr(i32)]
#[derive(PartialEq, Eq)]
#[allow(dead_code)]
enum Flavor {
    Chocolate = 1,
    Filler1,
    Strawberry,
    Filler2,
    Sherbet,
}
impl Into<Flavor> for i32 {
    fn into(self) -> Flavor {
        match self {
            1 => Flavor::Chocolate,
            3 => Flavor::Strawberry,
            5 => Flavor::Sherbet,
            _ => Flavor::Chocolate,
        }
    }
}

struct PreRenderData {
    color: ae::PixelF32
}
impl Default for PreRenderData {
    fn default() -> Self {
        Self {
            color: ae::PixelF32 { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    Mode,
    Flavor,
    Color,
    Slider,
    Checkbox,
}

fn get_preset_color_value(flavor: Flavor) -> ae::Pixel8 {
    match flavor {
        Flavor::Chocolate  => ae::Pixel8 { red: 136, green: 83,  blue: 51, alpha: 0 },
        Flavor::Strawberry => ae::Pixel8 { red: 232, green: 21,  blue: 84, alpha: 0 },
        Flavor::Sherbet    => ae::Pixel8 { red: 255, green: 128, blue: 0,  alpha: 0 },
        _                  => ae::Pixel8 { red: 0,   green: 0,   blue: 0,  alpha: 0 },
    }
}

#[derive(Default)]
struct Plugin {
    initialized: bool,
    my_id: ae::aegp::PluginId,
}

#[repr(transparent)]
struct State(ae::sys::PF_State);
impl Default for State {
    fn default() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Instance {
    #[serde(skip)]
    state: State,
    advanced_mode: bool,
}

ae::define_effect!(Plugin, Instance, Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _in_data: InData, _: OutData) -> Result<(), Error> {
        params.add_with_flags(Params::Mode, "Mode", ae::PopupDef::setup(|f| {
            f.set_options(&["Basic", "Advanced"]);
            f.set_default(1);
        }), ae::ParamFlag::SUPERVISE | ae::ParamFlag::CANNOT_TIME_VARY | ae::ParamFlag::CANNOT_INTERP, ae::ParamUIFlags::CONTROL_ONLY)?;

        params.add_with_flags(Params::Flavor, "Flavor", ae::PopupDef::setup(|f| {
            f.set_options(&["Chocolate", "(-", "Strawberry", "(-", "Sherbet"]);
            f.set_default(1);
        }), ae::ParamFlag::SUPERVISE | ae::ParamFlag::CANNOT_INTERP, ae::ParamUIFlags::empty())?;

        params.add_with_flags(Params::Color, "Color", ae::ColorDef::setup(|f| {
            f.set_default(get_preset_color_value(Flavor::Chocolate));
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;

        params.add_with_flags(Params::Slider, "Slider", ae::FloatSliderDef::setup(|f| {
            f.set_slider_min(0.0);
            f.set_slider_max(100.0);
            f.set_valid_min(0.0);
            f.set_valid_max(100.0);
            f.set_default(28.0);
            f.set_value(f.default());
            f.set_precision(1);
        }), ae::ParamFlag::EXCLUDE_FROM_HAVE_INPUTS_CHANGED, ae::ParamUIFlags::empty())?;

        params.add_with_flags(Params::Checkbox, "Checkbox", ae::CheckBoxDef::setup(|f| {
            f.set_default(false);
            f.set_label("Set slider to 50%");
        }), ae::ParamFlag::SUPERVISE | ae::ParamFlag::CANNOT_TIME_VARY, ae::ParamUIFlags::CONTROL_ONLY)?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, _in_data: InData, mut out_data: OutData, _params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                let personal_info = ae::suites::App::new()?.personal_info()?;

                out_data.set_return_msg(&format!("Supervisor, v5.10,\r{}\r{}\rDemonstrates parameter supervision. Also, dig the whizzy seperators in the 'Flavor' pop-up!\rCopyright 2007-2023\rAdobe Inc.", personal_info.name, personal_info.serial_str));
            }
            ae::Command::GlobalSetup => {
                if let Ok(suite) = ae::aegp::suites::Utility::new() {
                    self.my_id = suite.register_with_aegp("Supervisor")?;
                }
            }
            _ => { }
        }
        Ok(())
    }
}

impl AdobePluginInstance for Instance {
    fn flatten(&self) -> Result<(u16, Vec<u8>), Error> {
        Ok((1, bincode::serialize(self).unwrap()))
    }
    fn unflatten(_version: u16, bytes: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(bytes).unwrap_or_default())
    }

    fn render(&self, _: &mut PluginState, _: &Layer, _: &mut Layer) -> Result<(), ae::Error> { Ok(()) }

    fn handle_command(&mut self, plugin: &mut PluginState, cmd: ae::Command) -> Result<(), ae::Error> {
        let in_data = &plugin.in_data;

        match cmd {
            ae::Command::SequenceSetup => {
                if !in_data.is_premiere() {
                    let slider_index = plugin.params.index(Params::Slider).unwrap() as _;
                    self.state = State(in_data.effect().current_param_state(slider_index, None, None)?);
                }
                self.advanced_mode = false;
            },
            ae::Command::Render { in_layer, mut out_layer } => {
                let mode = plugin.params.get(Params::Mode)?.as_popup()?.value();

                let extent_hint = in_data.extent_hint();
                let out_extent_hint = out_layer.extent_hint();
                // clear all pixels outside extent_hint.
                if extent_hint != out_extent_hint {
                    out_layer.fill(None, Some(out_extent_hint))?;
                }

                // If we're in Basic mode, use a preset.
                // Otherwise, use the value of our color param.
                let scratch8 = if mode == Mode::Basic as i32 {
                    get_preset_color_value(plugin.params.get(Params::Flavor)?.as_popup()?.value().into())
                } else {
                    plugin.params.get(Params::Color)?.as_color()?.value()
                };

                let pixel_float = ae::PixelF32 {
                    red:   scratch8.red   as f32 / ae::MAX_CHANNEL8 as f32,
                    green: scratch8.green as f32 / ae::MAX_CHANNEL8 as f32,
                    blue:  scratch8.blue  as f32 / ae::MAX_CHANNEL8 as f32,
                    alpha: scratch8.alpha as f32 / ae::MAX_CHANNEL8 as f32,
                };

                let progress_final = out_extent_hint.bottom - out_extent_hint.top;

                // iterate over image data.
                #[rustfmt::skip]
                in_layer.iterate_with(&mut out_layer, 0, progress_final, Some(out_extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                    match (pixel, out_pixel) {
                        (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                            out_pixel.alpha = pixel.alpha;
                            out_pixel.red   = (pixel.red   as u16 + (pixel_float.red   * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                            out_pixel.green = (pixel.green as u16 + (pixel_float.green * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                            out_pixel.blue  = (pixel.blue  as u16 + (pixel_float.blue  * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                        }
                        (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                            out_pixel.alpha = pixel.alpha;
                            out_pixel.red   = (pixel.red   as u32 + (pixel_float.red   * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                            out_pixel.green = (pixel.green as u32 + (pixel_float.green * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                            out_pixel.blue  = (pixel.blue  as u32 + (pixel_float.blue  * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                        }
                        _ => return Err(Error::BadCallbackParameter)
                    }
                    Ok(())
                })?;
            }
            ae::Command::SmartPreRender { mut extra } => {
                let mut req = extra.output_request();
                req.preserve_rgb_of_zero_alpha = 0;

                // Let's investigate our input parameters, and save ourselves a few clues for rendering later.
                // Because pre-render gets called A LOT, it's best to put checkouts you'll always need in SmartRender().
                // Because smart pre-computing of what you'll actually NEED can save time, it's best to check conditionals here in pre-render.
                extra.set_pre_render_data::<PreRenderData>(Default::default());

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let _ = extra.union_result_rect(in_result.result_rect.into());
                    let _ = extra.union_max_result_rect(in_result.max_result_rect.into());
                }
            }
            ae::Command::SmartRender { mut extra } => {
                let cb = extra.callbacks();
                let Some(input_world) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };

                let pre_render_data = extra.pre_render_data_mut::<PreRenderData>().unwrap();
                if self.advanced_mode {
                    pre_render_data.color = plugin.params.get(Params::Color)?.as_color()?.float_value()?;
                    // color_param gets checked in here
                } else {
                    // Basic mode
                    let flavor_param = plugin.params.get(Params::Flavor)?;
                    let lo_rent_color = get_preset_color_value(flavor_param.as_popup()?.value().into());

                    // Rounding slop? Yes! But hey, whaddayawant, they're 0-255 presets...
                    pre_render_data.color.red   = lo_rent_color.red   as f32 / ae::MAX_CHANNEL8 as f32;
                    pre_render_data.color.green = lo_rent_color.green as f32 / ae::MAX_CHANNEL8 as f32;
                    pre_render_data.color.blue  = lo_rent_color.blue  as f32 / ae::MAX_CHANNEL8 as f32;
                    pre_render_data.color.alpha = lo_rent_color.alpha as f32 / ae::MAX_CHANNEL8 as f32;
                }

                let pixel_float = pre_render_data.color;

                if let Ok(Some(mut output_world)) = cb.checkout_output() {
                    let out_extent_hint = output_world.extent_hint();
                    let progress_final = output_world.height() as _;
                    // iterate over image data.
                    #[rustfmt::skip]
                    input_world.iterate_with(&mut output_world, 0, progress_final, Some(out_extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                        match (pixel, out_pixel) {
                            (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha;
                                out_pixel.red   = (pixel.red   as u16 + (pixel_float.red   * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                                out_pixel.green = (pixel.green as u16 + (pixel_float.green * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                                out_pixel.blue  = (pixel.blue  as u16 + (pixel_float.blue  * ae::MAX_CHANNEL8 as f32 / 2.0) as u16).min(ae::MAX_CHANNEL8 as _) as _;
                            }
                            (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha;
                                out_pixel.red   = (pixel.red   as u32 + (pixel_float.red   * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                                out_pixel.green = (pixel.green as u32 + (pixel_float.green * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                                out_pixel.blue  = (pixel.blue  as u32 + (pixel_float.blue  * ae::MAX_CHANNEL16 as f32 / 2.0) as u32).min(ae::MAX_CHANNEL16 as _) as _;
                            }
                            (ae::GenericPixel::PixelF32(pixel), ae::GenericPixelMut::PixelF32(out_pixel)) => {
                                out_pixel.alpha = pixel.alpha as _;
                                out_pixel.red   = (pixel.red   + pixel_float.red)   / 2.0;
                                out_pixel.green = (pixel.green + pixel_float.green) / 2.0;
                                out_pixel.blue  = (pixel.blue  + pixel_float.blue)  / 2.0;
                            }
                            _ => return Err(Error::BadCallbackParameter)
                        }
                        Ok(())
                    })?;
                }

                cb.checkin_layer_pixels(0)?;
            }
            ae::Command::UpdateParamsUi => {
                let params = &plugin.params;
                let mode = params.get(Params::Mode)?.as_popup()?.value();

                // Before we can change the enabled/disabled state of parameters, we need to make a copy (remember, parts of those passed into us are read-only).
                let mut params_copy = params.cloned();

                // Toggle enable/disabled state of flavor param
                if mode == (Mode::Basic as i32) {
                    let mut flavor = params_copy.get_mut(Params::Flavor)?;
                    flavor.set_ui_flag(ae::ParamUIFlags::DISABLED, false);
                    flavor.set_name("Flavor")?;
                    flavor.update_param_ui()?;
                } else if mode == Mode::Advanced as i32 && !params_copy.get(Params::Flavor)?.ui_flags().contains(ae::ParamUIFlags::DISABLED) {
                    let mut flavor = params_copy.get_mut(Params::Flavor)?;
                    flavor.set_ui_flag(ae::ParamUIFlags::DISABLED, true);
                    flavor.set_name("Flavor (disabled in Basic mode)")?;
                    flavor.update_param_ui()?;
                }

                let mut hide_them = false;
                if mode == (Mode::Advanced as i32) {
                    self.advanced_mode = true;
                } else {
                    self.advanced_mode = false;
                    hide_them = true;
                }

                if !in_data.is_premiere() {
                    let effect = in_data.effect();

                    // Twirl open the slider param
                    {
                        let mut slider = params_copy.get_mut(Params::Slider)?;
                        slider.set_flag(ae::ParamFlag::TWIRLY, false);
                        slider.update_param_ui()?;
                    }

                    // Changing visibility of params in AE is handled through stream suites
                    {
                        let plugin_id = plugin.global.my_id;
                        let me = effect.aegp_effect(plugin_id)?;
                        // let flavor_stream   = me.new_stream_by_index(plugin_id, plugin.params.index(Params::Flavor)  .unwrap() as _)?;
                        let color_stream    = me.new_stream_by_index(plugin_id, params.index(Params::Color)   .unwrap() as _)?;
                        let slider_stream   = me.new_stream_by_index(plugin_id, params.index(Params::Slider)  .unwrap() as _)?;
                        let checkbox_stream = me.new_stream_by_index(plugin_id, params.index(Params::Checkbox).unwrap() as _)?;

                        // Toggle visibility of parameters
                        color_stream   .set_dynamic_stream_flag(ae::aegp::DynamicStreamFlags::Hidden, false, hide_them)?;
                        slider_stream  .set_dynamic_stream_flag(ae::aegp::DynamicStreamFlags::Hidden, false, hide_them)?;
                        checkbox_stream.set_dynamic_stream_flag(ae::aegp::DynamicStreamFlags::Hidden, false, hide_them)?;

                        // Change popup menu items
                        let param_union = me.param_union_by_index(plugin_id, params.index(Params::Flavor).unwrap() as _)?;
                        if let ae::Param::Popup(mut popup) = param_union {
                            popup.set_options(&["Chocolate", "(-", "Strawberry", "(-", "And more!"]);
                        }
                    }

                    // Demonstrate using PF_AreStatesIdentical to check whether a parameter has changed
                    let new_state = effect.current_param_state(params.index(Params::Slider).unwrap() as _, None, None)?;
                    let something_changed = effect.are_param_states_identical(&self.state.0, &new_state)?;

                    if something_changed || !plugin.global.initialized {
                        // If something changed (or it's the first time we're being called), get the new state and store it in our sequence data
                        self.state = State(new_state);
                    }
                } else { // Premiere Pro doesn't support the stream suites, but uses a UI flag instead
                    // Test all parameters except layers for changes

                    // If the mode is currently Basic, hide the advanced-only params
                    if mode == (Mode::Basic as i32) {
                        for param in [Params::Color, Params::Slider, Params::Checkbox] {
                            let mut param = params_copy.get_mut(param)?;
                            param.set_ui_flag(ae::ParamUIFlags::INVISIBLE, true);
                            param.update_param_ui()?;
                        }
                    } else {
                        // Since we're in advanced mode, show the advanced-only params
                        for param in [Params::Color, Params::Slider, Params::Checkbox] {
                            let mut param = params_copy.get_mut(param)?;
                            param.set_ui_flag(ae::ParamUIFlags::INVISIBLE, false);
                            param.update_param_ui()?;
                        }
                    }
                }

                plugin.global.initialized = true;
                plugin.out_data.set_out_flag(ae::OutFlags::RefreshUi, true);
                plugin.out_data.set_out_flag(ae::OutFlags::ForceRerender, true);
            }
            ae::Command::UserChangedParam { param_index } => {
                let params = &mut plugin.params;
                let param = params.type_at(param_index);
                if param == Params::Checkbox {
                    // If checkbox is checked, change slider value to 50 and flip checkbox back off
                    if params.get(Params::Checkbox)?.as_checkbox()?.value() {
                        params.get_mut(Params::Slider)?.as_float_slider_mut()?.set_value(50.0);

                        let mut cb = params.get_mut(Params::Checkbox)?;
                        cb.as_checkbox_mut()?.set_value(false);
                        cb.update_param_ui()?;
                    }
                }
                if param == Params::Color || param == Params::Flavor {
                    // Force refresh all cached sequence_data instances
                    plugin.out_data.set_force_rerender();
                }
            }
            _ => { }
        }
        Ok(())
    }
}
