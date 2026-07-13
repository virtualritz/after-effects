//! QueueBert - Rust port of Adobe's QueueBert AEGP sample
//!
//! This plugin demonstrates the use of Render Queue and Output Module suites.
//! It adds a menu item under File that, when clicked, manipulates the render queue
//! by adding compositions, modifying output module settings, and more.

use after_effects::{
    AegpPlugin, Error,
    aegp::{
        CommandHookStatus, EmbeddingType, HookPriority, LogType, MenuOrder, OutputTypes,
        PostRenderAction, StretchQuality, VideoChannels,
        suites::{Command, OutputModule, Register, RenderQueue, RenderQueueItem, Utility},
    },
    define_general_plugin,
    sys::AEGP_PluginID,
};

define_general_plugin!(QueueBert);

#[derive(Clone, Debug)]
struct QueueBert;

impl AegpPlugin for QueueBert {
    fn entry_point(
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: AEGP_PluginID,
    ) -> Result<Self, Error> {
        log::info!(
            "QueueBert Entry Point: v{major_version}.{minor_version} - id: {aegp_plugin_id}"
        );

        let res: Result<(), Error> = (|| {
            let command_suite = Command::new()?;
            let register_suite = Register::new()?;
            let queuebert_cmd = command_suite.unique_command()?;

            command_suite.insert_command(
                "QueueBert",
                queuebert_cmd,
                after_effects::aegp::MenuId::File,
                MenuOrder::Sorted,
            )?;

            // Register command hook - this is called when the menu item is clicked
            register_suite.register_command_hook::<QueueBert, _>(
                aegp_plugin_id,
                HookPriority::BeforeAE,
                queuebert_cmd,
                Box::new(move |_, _, command, _, _| {
                    if command != queuebert_cmd {
                        return Ok(CommandHookStatus::Unhandled);
                    }

                    if let Err(e) = handle_queuebert_command() {
                        log::error!("QueueBert command failed: {:?}", e);
                    }

                    Ok(CommandHookStatus::Handled)
                }),
                (),
            )?;

            // Register update menu hook - enable/disable menu based on render queue state
            register_suite.register_update_menu_hook::<QueueBert, _>(
                aegp_plugin_id,
                Box::new(move |_, _, _| {
                    let command_suite = Command::new()?;
                    let rq_item_suite = RenderQueueItem::new()?;

                    let num_rq_items = rq_item_suite.num_items()?;

                    if num_rq_items > 0 {
                        command_suite.enable_command(queuebert_cmd)?;
                    } else {
                        command_suite.disable_command(queuebert_cmd)?;
                    }

                    Ok(())
                }),
                (),
            )?;

            Ok(())
        })();

        if let Err(e) = res {
            let util_suite = Utility::new()?;
            util_suite.report_info_unicode(
                aegp_plugin_id,
                &format!(
                    "QueueBert: Problems encountered during initialization: {:?}",
                    e
                ),
            )?;
        }

        Ok(QueueBert)
    }
}

/// Main command handler - demonstrates Render Queue and Output Module suite functionality
fn handle_queuebert_command() -> Result<(), Error> {
    let rq_suite = RenderQueue::new()?;
    let rq_item_suite = RenderQueueItem::new()?;
    let output_module_suite = OutputModule::new()?;

    let rq_item_count = rq_item_suite.num_items()?;
    log::info!("QueueBert: Found {} render queue items", rq_item_count);

    if rq_item_count == 0 {
        log::info!("QueueBert: No items in render queue, nothing to do");
        return Ok(());
    }

    // Get the composition from the first render queue item
    let first_item = rq_item_suite.item_by_index(0)?;
    let comp = rq_item_suite.comp(&first_item)?;

    // Add the composition to the render queue multiple times (like the original sample)
    for i in 0..6 {
        // Note: The original sample uses a hardcoded path - we use a platform-appropriate one
        #[cfg(target_os = "windows")]
        let output_path = format!("C:\\temp\\queuebert_output_{}.mov", i);
        #[cfg(not(target_os = "windows"))]
        let output_path = format!("/tmp/queuebert_output_{}.mov", i);

        if let Err(e) = rq_suite.add_comp_to_render_queue(&comp, &output_path) {
            log::warn!("Failed to add comp to render queue: {:?}", e);
        }
    }

    // Re-fetch the item count after adding
    let rq_item_count = rq_item_suite.num_items()?;

    // Now work with the first render queue item
    if rq_item_count > 0 {
        let rq_item = rq_item_suite.item_by_index(0)?;
        let outmod_count = rq_item_suite.num_output_modules(&rq_item)?;

        log::info!("QueueBert: First item has {} output modules", outmod_count);

        if outmod_count > 0 {
            // Get and set log type
            let log_type = rq_item_suite.log_type(&rq_item)?;
            log::info!("QueueBert: Current log type: {:?}", log_type);
            rq_item_suite.set_log_type(&rq_item, LogType::PlusSettings)?;

            // Get and set render state
            let status = rq_item_suite.render_state(&rq_item)?;
            log::info!("QueueBert: Current render state: {:?}", status);

            // Get timing info
            let started_time = rq_item_suite.started_time(&rq_item)?;
            let elapsed_time = rq_item_suite.elapsed_time(&rq_item)?;
            log::info!(
                "QueueBert: Started: {:?}, Elapsed: {:?}",
                started_time,
                elapsed_time
            );

            // Now work with the first output module
            let output_module = output_module_suite.output_module_by_index(&rq_item, 0)?;

            // Enable both video and audio output
            let current_outputs = output_module_suite.enabled_outputs(&rq_item, &output_module)?;
            log::info!("QueueBert: Current enabled outputs: {:?}", current_outputs);

            let new_outputs = OutputTypes::VIDEO | OutputTypes::AUDIO;
            output_module_suite.set_enabled_outputs(&rq_item, &output_module, new_outputs)?;

            // Set output channels to RGBA
            let vid_channels = output_module_suite.output_channels(&rq_item, &output_module)?;
            log::info!("QueueBert: Current video channels: {:?}", vid_channels);
            output_module_suite.set_output_channels(
                &rq_item,
                &output_module,
                VideoChannels::Rgba,
            )?;

            // Get and set stretch info
            let (stretch_enabled, stretch_qual, locked) =
                output_module_suite.stretch_info(&rq_item, &output_module)?;
            log::info!(
                "QueueBert: Stretch - enabled: {}, quality: {:?}, locked: {}",
                stretch_enabled,
                stretch_qual,
                locked
            );
            output_module_suite.set_stretch_info(
                &rq_item,
                &output_module,
                true,
                StretchQuality::High,
            )?;

            // Get and set crop info
            let (crop_enabled, crop_rect) =
                output_module_suite.crop_info(&rq_item, &output_module)?;
            log::info!(
                "QueueBert: Crop - enabled: {}, rect: {:?}",
                crop_enabled,
                crop_rect
            );

            // Set a crop rectangle
            let new_crop_rect = after_effects::sys::A_Rect {
                left: 0,
                top: 0,
                right: 200,
                bottom: 100,
            };
            output_module_suite.set_crop_info(&rq_item, &output_module, true, new_crop_rect)?;

            // Get and set sound format info
            let (sound_format, audio_enabled) =
                output_module_suite.sound_format_info(&rq_item, &output_module)?;
            log::info!(
                "QueueBert: Audio - enabled: {}, sample_rate: {}, channels: {}, bytes_per_sample: {}",
                audio_enabled,
                sound_format.sample_rateF,
                sound_format.num_channelsL,
                sound_format.bytes_per_sampleL
            );

            // If audio is not configured, set up default audio settings
            if !audio_enabled || sound_format.sample_rateF == 0.0 {
                let new_sound_format = after_effects::sys::AEGP_SoundDataFormat {
                    sample_rateF: 44100.0,
                    encoding: after_effects::sys::AEGP_SoundEncoding_FLOAT as _,
                    bytes_per_sampleL: 4,
                    num_channelsL: 1,
                };
                output_module_suite.set_sound_format_info(
                    &rq_item,
                    &output_module,
                    new_sound_format,
                    true,
                )?;
            }

            // Get and set embed options
            let embed_type = output_module_suite.embed_options(&rq_item, &output_module)?;
            log::info!("QueueBert: Embed type: {:?}", embed_type);
            output_module_suite.set_embed_options(
                &rq_item,
                &output_module,
                EmbeddingType::LinkAndCopy,
            )?;

            // Get and set post-render action
            let post_action = output_module_suite.post_render_action(&rq_item, &output_module)?;
            log::info!("QueueBert: Post-render action: {:?}", post_action);
            output_module_suite.set_post_render_action(
                &rq_item,
                &output_module,
                PostRenderAction::ImportAndReplace,
            )?;

            // Add a new default output module
            let new_outmod = output_module_suite.add_default_output_module(&rq_item)?;
            let new_outmod_count = rq_item_suite.num_output_modules(&rq_item)?;
            log::info!(
                "QueueBert: Added new output module, now have {} modules",
                new_outmod_count
            );

            // Set output file path on the new module
            #[cfg(target_os = "windows")]
            let output_path = "C:\\temp\\queuebert_new_output.mov";
            #[cfg(not(target_os = "windows"))]
            let output_path = "/tmp/queuebert_new_output.mov";

            output_module_suite.set_output_file_path(&rq_item, &new_outmod, output_path)?;

            // Get extra output module info
            if let Ok((format, info, is_sequence, multi_frame)) =
                output_module_suite.extra_output_module_info(&rq_item, &output_module)
            {
                log::info!(
                    "QueueBert: Format: {}, Info: {}, Sequence: {}, Multi-frame: {}",
                    format,
                    info,
                    is_sequence,
                    multi_frame
                );
            }

            // Set a comment on the render queue item
            rq_item_suite.set_comment(&rq_item, "That's pronounced cue-BARE!")?;

            // Read the comment back
            let comment = rq_item_suite.comment(&rq_item)?;
            log::info!("QueueBert: Item comment: {}", comment);
        }
    }

    // Log the current render queue state
    let rq_state = rq_suite.render_queue_state()?;
    log::info!("QueueBert: Render queue state: {:?}", rq_state);

    log::info!("QueueBert: Command completed successfully");
    Ok(())
}
