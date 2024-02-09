use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params { Opacity }

#[derive(Default)]
struct Plugin { }

#[derive(Default)]
struct Instance { }

ae::define_plugin!(Plugin, Instance, Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(&self, params: &mut ae::Parameters<Params>) -> Result<(), Error> {
        params.add_param(Params::Opacity, "Opacity", ae::FloatSliderDef::new()
            .set_slider_min(0.0)
            .set_slider_max(100.0)
            .set_valid_min(0.0)
            .set_valid_max(100.0)
        );
        Ok(())
    }
    fn handle_command(&mut self, _: ae::Command, _: ae::InData, _: ae::OutData) -> Result<(), ae::Error> { Ok(()) }
}

impl AdobePluginInstance for Instance {
    fn flatten(&self) -> Result<(u16, Vec<u8>), Error> { Ok((1, Vec::new())) }
    fn unflatten(_version: u16, _bytes: &[u8]) -> Result<Self, Error> { Ok(Self {}) }

    fn user_changed_param(&mut self, _: &mut PluginState, _: Params) -> Result<(), ae::Error> { Ok(()) }

    fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error> {
        let slider_value = plugin.params.get_float_slider(Params::Opacity, None, None, None).unwrap().value();

        let extent_hint = plugin.in_data.extent_hint();

        in_layer.iterate(out_layer, 0, extent_hint.height(), extent_hint, |_x: i32, _y: i32, mut pixel: ae::Pixel| -> Result<ae::Pixel, Error> {
            pixel.alpha = (pixel.alpha as f64 * slider_value / 100.0) as u8;

            Ok(pixel)
        })
    }
    fn handle_command(&mut self, _: &mut PluginState, _: ae::Command) -> Result<(), ae::Error> { Ok(()) }
}
