use after_effects::{
    aegp::{
        suites::{Command, Item, Register, Render, RenderOptions, Utility},
        CommandHookStatus, HookPriority, ItemType, MenuOrder,
    },
    define_general_plugin,
    sys::{AEGP_PluginID, PF_InData, SPBasicSuite},
    AegpPlugin, Error, InData, Layer, PicaBasicSuite, Time,
};

mod img_proc;
mod window_handle;

define_general_plugin!(Grabber);

#[derive(Clone, Debug)]
struct Grabber {}

impl AegpPlugin for Grabber {
    fn entry_point(
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: AEGP_PluginID,
    ) -> Result<Self, after_effects::Error> {
        log::debug!(
            "Aegp Demo Entry Point: v{major_version}.{minor_version} - id : {aegp_plugin_id}"
        );

        let res: Result<(), Error> = (|| {
            let command_suite = Command::new()?;
            let register_suite = Register::new()?;
            let grabba_cmd = command_suite.get_unique_command()?;

            command_suite.insert_command(
                "Grabber",
                grabba_cmd,
                after_effects::aegp::MenuId::Export,
                MenuOrder::Sorted,
            )?;

            register_suite.register_command_hook::<Grabber, _>(
                aegp_plugin_id,
                HookPriority::BeforeAE,
                grabba_cmd,
                Box::new(move |_, _, _, _, _| {
                    let item_suite = Item::new()?;
                    let render_options_suite = RenderOptions::new()?;
                    let render_suite = Render::new()?;

                    if let Ok(Some(active_item)) = item_suite.active_item() {
                        let render_options =
                            render_options_suite.new_from_item(active_item, aegp_plugin_id)?;

                        let time = Time { value: 0, scale: 1 };
                        render_options_suite.set_time(&render_options, time)?;

                        let world_type = render_options_suite.world_type(&render_options)?;
                        log::debug!("World type: {:?}", world_type);

                        if let Ok(receipt) = render_suite
                            .render_and_checkout_frame(&render_options, Some(Box::new(|| false)))
                        {
                            if let Ok(frame) = render_suite.receipt_world(receipt) {
                                let mut dialog = rfd::FileDialog::new();

                                if cfg!(target_os = "windows") {
                                    let parent =
                                        window_handle::WindowAndDisplayHandle::try_get_main_handles().map_err(|e| Error::Generic)?;
                                    dialog = dialog.set_parent(&parent);
                                }

                                let home_dir = match homedir::get_my_home() {
                                    Ok(Some(home)) => home,
                                    _ => "/".into(),
                                };

                                let Some(file_path) = dialog
                                    .set_directory(home_dir)
                                    .add_filter("PNG", &["png"])
                                    .set_file_name("image.png")
                                    .save_file()
                                else {
                                    log::warn!("Cancelled writing file!");
                                    render_suite.checkin_frame(receipt)?;
                                    return Ok(CommandHookStatus::Handled);
                                };

                                let dummy : PF_InData = unsafe { std::mem::zeroed() };
                                let layer = Layer::from_aegp_world(&dummy as *const _, frame)?;
                                let width = layer.width() as u32;
                                let height = layer.height() as u32;
                                let stride = layer.buffer_stride() as u32;
                                let data = layer.buffer();
                                let bit_depth = layer.bit_depth();

                                log::info!("Frame dimensions: {}x{}, bit depth: {}", width, height, bit_depth);

                                if let Err(e) = img_proc::save_frame_as_png(
                                    data,
                                    width,
                                    height,
                                    stride,
                                    bit_depth.into(),
                                    &file_path
                                ) {
                                    log::error!("Failed to save image: {:?}", e);
                                }

                            }

                            render_suite.checkin_frame(receipt)?;
                        }
                    }

                    Ok(CommandHookStatus::Handled)
                }),
                (),
            )?;

            register_suite.register_update_menu_hook::<Grabber, _>(
                aegp_plugin_id,
                Box::new(move |_, _, _| {
                    let command_suite = Command::new()?;
                    let item_suite = Item::new()?;

                    if let Ok(Some(active_item)) = item_suite.active_item() {
                        let item_type = item_suite.item_type(active_item)?;

                        if item_type == ItemType::Comp || item_type == ItemType::Footage {
                            command_suite.enable_command(grabba_cmd)?;
                        } else {
                            command_suite.disable_command(grabba_cmd)?;
                        }
                    } else {
                        command_suite.disable_command(grabba_cmd)?;
                    }

                    Ok(())
                }),
                (),
            )?;

            Ok(())
        })();

        match res {
            Ok(_) => {}
            Err(e) => {
                let util_suite = Utility::new()?;
                util_suite.report_info_unicode(
                    aegp_plugin_id,
                    &format!("Error while loading AegpDemo {e:?}"),
                )?;
            }
        }

        Ok(Grabber {})
    }
}
