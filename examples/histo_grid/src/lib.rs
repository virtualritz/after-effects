use after_effects as ae;

mod ui;

// A simple example of how the UI can request asynchronous rendered upstream frames for lightweight processing in AE 13.5 and later.
// This effect calculates a sampled 10x10 color grid from the upstream frame, calcs and displays a preview of that color grid.
// In render, a higher quality grid is calculated and used to modify the output image creating a blend of a color grid + the original image.
//
// BACKGROUND
//
// As of 13.5, UI and Render thread introduction means lightweight histograms must asynchronously request the frame needed inside of DRAW.
// This replaces the pattern of calculation of histograms inside of render, storing in sequence data, and then passing back REFRESH_UI outflag.
// That method no longer works in 13.5 because the render sequence data is not in the same database as the UI and must not be written to.
//
// This example shows:
//     1. how to use the new RenderAsyncManager inside of PF_Event_DRAW to manage asynchronous frame requests for CUSTOM UI
//     2. how to request the "upstream" frame of the effect for lightweight preview processing for display in CUSTOM UI.
//         Note how the requested render size is downsampled to keep preview render fast.
//         Note how code shared by preview and by render is factored into a common function
//         that can be called by either without introducing UI dependencies in render or vice versa.
//     3. Storing a cached preview to prevent flicker in asynchronous requests
//
// The emphasis is on "lightweight". Heavy processing on a frame for preview on AE UI thread can interfere with user interactivity and ram preview playback in 13.5 onward.
// Heavier processing would need to be done on another thread.
//
// Search for "EXAMPLE" in comments for locations modified in this source code to make that work.

const UI_GRID_WIDTH : u16 = 203;
const UI_GRID_HEIGHT: u16 = UI_GRID_WIDTH;

const BOXES_ACROSS  : usize = 10;
const BOXES_DOWN    : usize = 10;
const BOXES_PER_GRID: usize = BOXES_ACROSS * BOXES_DOWN; // Ones based
const CACHE_ELEMENTS: usize = BOXES_PER_GRID;

const CA_MAGIC: i32 = i32::from_be_bytes(*b"CAsd");

static mut AEGP_PLUGIN_ID: ae::aegp::PluginId = 0;

struct ColorCache {
    color: [ae::PixelF32; CACHE_ELEMENTS]
}

impl ColorCache {
    pub fn new() -> Self {
        Self {
            color: unsafe { std::mem::zeroed() }
        }
    }
    pub fn assign_grid_cell_color8(px: &ae::Pixel8) -> ae::PixelF32 {
        ae::PixelF32 {
            alpha: px.alpha as f32 / ae::MAX_CHANNEL8 as f32,
            red:   px.red   as f32 / ae::MAX_CHANNEL8 as f32,
            green: px.green as f32 / ae::MAX_CHANNEL8 as f32,
            blue:  px.blue  as f32 / ae::MAX_CHANNEL8 as f32,
        }
    }
    pub fn assign_grid_cell_color16(px: &ae::Pixel16) -> ae::PixelF32 {
        ae::PixelF32 {
            alpha: px.alpha as f32 / ae::MAX_CHANNEL16 as f32,
            red:   px.red   as f32 / ae::MAX_CHANNEL16 as f32,
            green: px.green as f32 / ae::MAX_CHANNEL16 as f32,
            blue:  px.blue  as f32 / ae::MAX_CHANNEL16 as f32,
        }
    }

    // for each grid cell get the central cell color for preview or render usage
    pub fn compute_grid_cell_colors<const DEPTH: usize>(&mut self, input: &ae::Layer) {
        let cell_width = input.width() / BOXES_ACROSS; // ignoring remainder for example
        let cell_height = input.height() / BOXES_DOWN;

        if cell_width > 0 && cell_height > 0 {
            for row_box in 0..BOXES_DOWN {
                let y = row_box * cell_height + cell_height / 2;
                for col_box in 0..BOXES_ACROSS {
                    // input
                    let x = col_box * cell_width + cell_width / 2;

                    // output
                    let position = row_box * BOXES_ACROSS + col_box;
                    match DEPTH {
                        8  => self.color[position] = Self::assign_grid_cell_color8(input.as_pixel8(x, y)),
                        16 => self.color[position] = Self::assign_grid_cell_color16(input.as_pixel16(x, y)),
                        32 => self.color[position] = *input.as_pixel32(x, y),
                        _ => { }
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..CACHE_ELEMENTS {
            self.color[i] = ae::PixelF32 { alpha: 0.0, red: 0.0, green: 0.0, blue: 0.0 };
        }
    }

    // EXAMPLE
    // This calculates color grid values on an input frame and is used in both render and custom UI preview.
    // To keep this operation lightweight on the UI, we'll pass a highly downsampled frame.
    // On the UI thread the result is cached in sequence data to prevent flicker if we don't have a completed computation yet
    pub fn compute_color_grid_from_frame(&mut self, world: &ae::Layer) -> Result<(), ae::Error> {
        self.clear();

        match world.pixel_format()? {
            ae::PixelFormat::Argb128 => self.compute_grid_cell_colors::<32>(world),
            ae::PixelFormat::Argb64  => self.compute_grid_cell_colors::<16>(world),
            ae::PixelFormat::Argb32  => self.compute_grid_cell_colors::<8> (world),
            _ => return Err(ae::Error::BadCallbackParameter)
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    GridUI,
}

#[derive(Default)]
struct Plugin { }

struct Instance {
    _magic: i32,
    color_cache: ColorCache,
    valid: bool, // whether cached color data for DRAW is valid, otherwise draw blank
}

ae::define_plugin!(Plugin, Instance, Params);

impl Default for Instance {
    fn default() -> Self {
        Self {
            _magic: CA_MAGIC,
            color_cache: ColorCache::new(),
            valid: false, // EXAMPLE cached preview is not yet valid
        }
    }
}

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, in_data: InData, _: OutData) -> Result<(), Error> {
        // EXAMPLE. the NULL is being used to reserve an area in the custom UI for drawing the preview
        // An example of using an ARB for this for storing persistant data is in the ColorGrid example
        params.add_customized(Params::GridUI, "Preview", ae::NullDef::new(), |param| {
            param.set_flags(ae::ParamFlag::SUPERVISE);
            param.set_ui_flags(ae::ParamUIFlags::CONTROL | ae::ParamUIFlags::DO_NOT_ERASE_CONTROL);
            param.set_ui_width(UI_GRID_WIDTH);
            param.set_ui_height(UI_GRID_HEIGHT);
            -1
        })?;

        in_data.interact().register_ui(
            CustomUIInfo::new()
                .events(ae::CustomEventFlags::EFFECT)
        )?;

        Ok(())
    }

    fn handle_command(&mut self, cmd: ae::Command, _in_data: InData, mut out_data: OutData, _params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("\rCopyright 2015-2023 Adobe Inc.\rHistoGrid sample.");
            }
            ae::Command::GlobalSetup => {
                if let Ok(suite) = ae::aegp::suites::Utility::new() {
                    unsafe {
                        AEGP_PLUGIN_ID = suite.register_with_aegp(None, "HistoGrid")?;
                    }
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
        Ok(Self::default())
    }

    fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error> {
        let mut box_across     = 0;
        let mut box_down       = 0;
        let mut progress_base  = 0;
        let progress_final     = BOXES_PER_GRID;
        let in_data = &plugin.in_data;

        let origin = in_data.pre_effect_source_origin();

        // EXAMPLE compute the colors we will use for render. high quality
        let mut color_cache = ColorCache::new();
        color_cache.compute_color_grid_from_frame(&in_layer)?;

        let mut current_color = 0;

        // This section uses the pre-effect extent hint, since it wants to only be applied to the source layer material, and NOT to any resized effect area.
        // Example: User applies "Resizer" to a layer before using HistoGrid.
        // The effect makes the output area larger than the source footage.
        // HistoGrid will look at the pre-effect extent width and height to determine what the relative coordinates are for the source material inside the params[0] (the layer).
        for _ in 0..BOXES_PER_GRID {
            if box_across == BOXES_ACROSS {
                box_down += 1;
                box_across = 0;
            }

            let current_rect = ui::histogrid_get_box_in_grid(&origin,
                (in_data.width()  as f32 * f32::from(in_data.downsample_x())).round() as usize,
                (in_data.height() as f32 * f32::from(in_data.downsample_y())).round() as usize,
                box_across,
                box_down
            );

            let cur_color = &color_cache.color[current_color];
            let color8_r  = (cur_color.red   * ae::MAX_CHANNEL8 as f32) as u16;
            let color8_g  = (cur_color.green * ae::MAX_CHANNEL8 as f32) as u16;
            let color8_b  = (cur_color.blue  * ae::MAX_CHANNEL8 as f32) as u16;

            progress_base += 1;

            in_layer.iterate_with(out_layer, progress_base, progress_final as _, Some(current_rect), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                match (pixel, out_pixel) {
                    (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                        out_pixel.alpha = pixel.alpha as _;
                        out_pixel.red   = ((pixel.red   as u16 + color8_r) / 2) as u8;
                        out_pixel.green = ((pixel.green as u16 + color8_g) / 2) as u8;
                        out_pixel.blue  = ((pixel.blue  as u16 + color8_b) / 2) as u8;
                    }
                    _ => return Err(Error::BadCallbackParameter)
                }
                Ok(())
            })?;

            current_color += 1;
            box_across += 1;
        }

        Ok(())
    }

    fn handle_command(&mut self, plugin: &mut PluginState, cmd: ae::Command) -> Result<(), ae::Error> {
        let in_data = &plugin.in_data;

        match cmd {
            ae::Command::SmartPreRender { mut extra } => {
                let req = extra.output_request();

                if let Ok(in_result) = extra.callbacks().checkout_layer(0, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
                    let _ = extra.union_result_rect(in_result.result_rect.into());
                    let _ = extra.union_max_result_rect(in_result.max_result_rect.into());
                }
            }
            ae::Command::SmartRender { extra } => {
                let mut origin = ae::Point { h: 0, v: 0 };
                let mut box_across    = 0;
                let mut box_down      = 0;
                let mut current_color = 0;

                let cb = extra.callbacks();
                let input_world = cb.checkout_layer_pixels(0)?;

                let mut color_cache = ColorCache::new();
                color_cache.compute_color_grid_from_frame(&input_world)?;

                for _ in 0..BOXES_PER_GRID {
                    if box_across == BOXES_ACROSS {
                        box_down += 1;
                        box_across = 0;
                    }

                    let current_rect = ui::histogrid_get_box_in_grid(&origin,
                        (in_data.width()  as f32 * f32::from(in_data.downsample_x())).round() as usize,
                        (in_data.height() as f32 * f32::from(in_data.downsample_y())).round() as usize,
                        box_across,
                        box_down
                    );

                    let color32 = &color_cache.color[current_color];

                    let color8_r  = (color32.red   * ae::MAX_CHANNEL8  as f32) as u16;
                    let color8_g  = (color32.green * ae::MAX_CHANNEL8  as f32) as u16;
                    let color8_b  = (color32.blue  * ae::MAX_CHANNEL8  as f32) as u16;

                    let color16_r = (color32.red   * ae::MAX_CHANNEL16 as f32) as u32;
                    let color16_g = (color32.green * ae::MAX_CHANNEL16 as f32) as u32;
                    let color16_b = (color32.blue  * ae::MAX_CHANNEL16 as f32) as u32;

                    if let Ok(mut output_world) = cb.checkout_output() {
                        let progress_final = output_world.height() as _;

                        origin = in_data.output_origin();

                        input_world.iterate_with(&mut output_world, 0, progress_final, Some(current_rect), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                            match (pixel, out_pixel) {
                                (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = ((pixel.red   as u16 + color8_r) / 2) as u8;
                                    out_pixel.green = ((pixel.green as u16 + color8_g) / 2) as u8;
                                    out_pixel.blue  = ((pixel.blue  as u16 + color8_b) / 2) as u8;
                                }
                                (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = ((pixel.red   as u32 + color16_r) / 2) as u16;
                                    out_pixel.green = ((pixel.green as u32 + color16_g) / 2) as u16;
                                    out_pixel.blue  = ((pixel.blue  as u32 + color16_b) / 2) as u16;
                                }
                                (ae::GenericPixel::PixelF32(pixel), ae::GenericPixelMut::PixelF32(out_pixel)) => {
                                    out_pixel.alpha = pixel.alpha as _;
                                    out_pixel.red   = (pixel.red   + color32.red)   / 2.0;
                                    out_pixel.green = (pixel.green + color32.green) / 2.0;
                                    out_pixel.blue  = (pixel.blue  + color32.blue)  / 2.0;
                                }
                                _ => return Err(Error::BadCallbackParameter)
                            }
                            Ok(())
                        })?;
                    }
                    current_color += 1;
                    box_across += 1;
                }

                cb.checkin_layer_pixels(0)?;
            }
            ae::Command::Event { mut extra } => {
                if let ae::Event::Draw(_) = extra.event() {
                    ui::draw(self, &in_data, &mut extra)?;
                }
            }
            _ => { }
        }
        Ok(())
    }
}
