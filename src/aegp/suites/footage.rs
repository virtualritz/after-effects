use crate::*;
use crate::aegp::*;
use ae_sys::{ AEGP_ItemH, AEGP_FootageH };

define_suite!(
    /// Provides information about footage, or items in a project or composition. When getting and setting footage's interpretation, it is possible to specify incompatible options.
    ///
    /// If you encounter warnings and errors during development, be sure to make all related changes atomically, and reassess the logic of the operation you're performing.
    ///
    /// For example, changing the pull-down interpretation of footage won't work unless there's a difference between it's native and conformed frame rate.
    ///
    /// Depending on what you're trying to accomplish, it may make sense to abort all of your operations at that point, inform the user of the problem encountered.
    FootageSuite,
    AEGP_FootageSuite5,
    kAEGPFootageSuite,
    kAEGPFootageSuiteVersion5
);

impl FootageSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns an error if item isn't a footage item.
    /// Used to convert an item handle to a footage handle.
    pub fn main_footage_from_item(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<FootageHandle, Error> {
        Ok(FootageHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetMainFootageFromItem -> ae_sys::AEGP_FootageH, item_handle.as_ptr())?
        ))
    }

    /// Returns an error if item has no proxy. Returns the proxy footage handle.
    /// Note: a composition can have a proxy.
    pub fn proxy_footage_from_item(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<FootageHandle, Error> {
        Ok(FootageHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetProxyFootageFromItem -> ae_sys::AEGP_FootageH, item_handle.as_ptr())?
        ))
    }

    /// Returns the number of data (RGBA or audio) files, and the number of files per frame (may be greater than one if the footage has auxiliary channels).
    pub fn footage_num_files(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<(usize, usize), Error> {
        let (num_main_files, files_per_frame) = call_suite_fn_double!(self, AEGP_GetFootageNumFiles -> ae_sys::A_long, ae_sys::A_long, footage_handle.as_ptr())?;
        Ok((
            num_main_files  as usize,
            files_per_frame as usize
        ))
    }


    /// Get fully realized path to footage source file.
    ///
    /// Retrieves the footage path for a piece of footage (or for the specified frame of a footage sequence).
    /// `frame_num` ranges from `0 to num_main_files`, as obtained using [`footage_num_files()`](Self::footage_num_files).
    ///
    /// [`ae_sys::AEGP_FOOTAGE_MAIN_FILE_INDEX`](after_effects_sys::AEGP_FOOTAGE_MAIN_FILE_INDEX) is the main file.
    pub fn footage_path(&self, footage_handle: impl AsPtr<AEGP_FootageH>, frame_num: usize, file_index: usize) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetFootagePath -> ae_sys::AEGP_MemHandle, footage_handle.as_ptr(), frame_num as i32, file_index as i32)?;

        // Create a mem handle each and lock it.
        // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Retrieves the footage signature of specified footage.
    pub fn footage_signature(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<FootageSignature, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetFootageSignature -> ae_sys::AEGP_FootageSignature, footage_handle.as_ptr())?.into())
    }

    /// Creates a new footage item. The file path is a NULL-terminated UTF-16 string with platform separators.
    /// Note that footage filenames with colons are not allowed, since colons are used as path separators in the HFS+ file system.
    /// Note the optional params. If `allow_interpretation_dialog` is `false`, After Effects will guess the alpha interpretation.
    pub fn new_footage(&self, plugin_id: PluginId, path: &str, layer_info: Option<ae_sys::AEGP_FootageLayerKey>, sequence_options: Option<ae_sys::AEGP_FileSequenceImportOptions>,interp_style: InterpretationStyle) -> Result<FootageHandle, Error> {
        let path = widestring::U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;

        Ok(FootageHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_NewFootage -> ae_sys::AEGP_FootageH,
                plugin_id,
                path.as_ptr(),
                layer_info.map_or(std::ptr::null(), |li| &li as *const _),
                sequence_options.map_or(std::ptr::null(), |so| &so as *const _),
                interp_style.into(),
                std::ptr::null_mut()
            )?
        ))
    }

    /// Adds a footage item to a project. Footage will be adopted by the project, and may be added only once.
    /// This is Undo-able; do not dispose of the returned added item if it's undone.
    pub fn add_footage_to_project(&self, footage_handle: impl AsPtr<AEGP_FootageH>, folder_handle: &ItemHandle) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_AddFootageToProject -> ae_sys::AEGP_ItemH, footage_handle.as_ptr(), folder_handle.as_ptr())?
        ))
    }

    /// Sets footage as the proxy for an item. Will be adopted by the project.
    /// This is Undo-able; do not dispose of the returned added item if it's undone.
    pub fn set_item_proxy_footage(&self, item_handle: impl AsPtr<AEGP_ItemH>, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemProxyFootage, footage_handle.as_ptr(), item_handle.as_ptr())
    }

    /// Replaces footage for an item. The item will replace the main footage for this item.
    /// This is Undo-able; do not dispose of the returned added item if it's undone.
    pub fn replace_item_main_footage(&self, item_handle: impl AsPtr<AEGP_ItemH>, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReplaceItemMainFootage, footage_handle.as_ptr(), item_handle.as_ptr())
    }

    /// Deletes a footage item. Do not dispose of footage you did not create, or that has been added to the project.
    pub fn dispose_footage(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeFootage, footage_handle.as_ptr())
    }

    /// Populates an [`AEGP_FootageInterp`](after_effects_sys::AEGP_FootageInterp) describing the settings of the [`FootageHandle`].
    /// There is no way to create a valid [`AEGP_FootageInterp`](after_effects_sys::AEGP_FootageInterp) other than by using this function.
    /// If `proxy` is `true`, the proxy footage's settings are retrieved.
    pub fn footage_interpretation(&self, item_handle: impl AsPtr<AEGP_ItemH>, proxy: bool) -> Result<ae_sys::AEGP_FootageInterp, Error> {
        call_suite_fn_single!(self, AEGP_GetFootageInterpretation -> ae_sys::AEGP_FootageInterp, item_handle.as_ptr(), proxy.into())
    }

    /// Apply the settings in the [`AEGP_FootageInterp`](after_effects_sys::AEGP_FootageInterp) to the `item_handle`. Undo-able.
    /// If `proxy` is `true`, the proxy footage's settings are modified.
    pub fn set_footage_interpretation(&self, item_handle: impl AsPtr<AEGP_ItemH>, proxy: bool, interp: &ae_sys::AEGP_FootageInterp) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetFootageInterpretation, item_handle.as_ptr(), proxy.into(), interp)
    }

    /// Returns an [`AEGP_FootageLayerKey`](after_effects_sys::AEGP_FootageLayerKey) describing the footage.
    pub fn footage_layer_key(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<ae_sys::AEGP_FootageLayerKey, Error> {
        call_suite_fn_single!(self, AEGP_GetFootageLayerKey -> ae_sys::AEGP_FootageLayerKey, footage_handle.as_ptr())
    }

    // /// Deprecated. Adds a new placeholder footage item to the project.
    // /// Using this function for missing footage will cause the user to search for each individual missing file, regardless of whether or not they're all in the same directory.
    // /// Undo-able.
    // pub fn new_placeholder_footage(&self, plugin_id: PluginId, name: &str, width: i32, height: i32, duration: Option<Time>) -> Result<FootageHandle, Error> {
    //     let name = std::ffi::CString::new(name).map_err(|_| Error::InvalidParms)?;
    //     Ok(FootageHandle::from_raw(
    //         call_suite_fn_single!(self, AEGP_NewPlaceholderFootage -> ae_sys::AEGP_FootageH, plugin_id, name.as_ptr(), width, height, duration.map(Into::into).as_ref().map_or(std::ptr::null(), |d| d))?
    //     ))
    // }

    /// This is the hip new way to add references to footage that can't be found right this moment.
    /// The file path is a string with platform separators.
    ///
    /// In CS6 and earlier, file_type was ignored and we previously recommendedsetting it to [`FileType::None`](crate::aeio::FileType::None).
    /// Starting in CC, [`FileType::None`](crate::aeio::FileType::None) is now a warning condition.
    /// If you pass [`FileType::Any`](crate::aeio::FileType::Any), then path MUST exist.
    /// If the path may not exist, pass [`FileType::Dir`](crate::aeio::FileType::Dir) for folder, or [`FileType::Generic`](crate::aeio::FileType::Generic) for a file.
    pub fn new_placeholder_footage_with_path(&self, plugin_id: PluginId, path: &str, path_platform: Platform, file_type: crate::aeio::FileType, width: i32, height: i32, duration: Option<Time>) -> Result<FootageHandle, Error> {
        let path = widestring::U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;

        Ok(FootageHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_NewPlaceholderFootageWithPath -> ae_sys::AEGP_FootageH,
                plugin_id,
                path.as_ptr(),
                path_platform.into(),
                file_type as _,
                width,
                height,
                duration.map(Into::into).as_ref().map_or(std::ptr::null(), |d| d)
            )?
        ))
    }

    /// This is the way to add a solid.
    /// Until the footage is added to the project, the caller owns the [`FootageHandle`]
    /// (and must dispose of it if, and only if, it isn't added to the project).
    pub fn new_solid_footage(&self, name: &str, width: i32, height: i32, color: &ae_sys::AEGP_ColorVal) -> Result<FootageHandle, Error> {
        let name = std::ffi::CString::new(name).map_err(|_| Error::InvalidParms)?;

        Ok(FootageHandle::from_raw(
            call_suite_fn_single!(self, AEGP_NewSolidFootage -> ae_sys::AEGP_FootageH, name.as_ptr(), width, height, color as *const _)?
        ))
    }

    /// Returns the color of a given solid. Returns an error if the [`ItemHandle`] is not a solid.
    /// If `proxy` is `true`, the proxy solid's color is retrieved.
    pub fn solid_footage_color(&self, item_handle: impl AsPtr<AEGP_ItemH>, proxy: bool) -> Result<ae_sys::AEGP_ColorVal, Error> {
        call_suite_fn_single!(self, AEGP_GetSolidFootageColor -> ae_sys::AEGP_ColorVal, item_handle.as_ptr(), proxy.into())
    }

    /// Sets the color of a solid. Undo-able.
    /// If `proxy` is `true`, the proxy solid's color is set.
    pub fn set_solid_footage_color(&self, item_handle: impl AsPtr<AEGP_ItemH>, proxy: bool, color: &ae_sys::AEGP_ColorVal) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetSolidFootageColor, item_handle.as_ptr(), proxy.into(), color)
    }

    /// Sets the dimensions of a solid. Undo-able.
    /// If `proxy` is `true`, the proxy solid's dimensions are modified. Returns an error if the item isn't a solid.
    pub fn set_solid_footage_dimensions(&self, item_handle: impl AsPtr<AEGP_ItemH>, proxy: bool, width: i32, height: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetSolidFootageDimensions, item_handle.as_ptr(), proxy.into(), width, height)
    }

    /// Retrieves information about the audio data in the footage item (by populating the `AEGP_SoundDataFormat` you passed in).
    pub fn footage_sound_data_format(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<ae_sys::AEGP_SoundDataFormat, Error> {
        call_suite_fn_single!(self, AEGP_GetFootageSoundDataFormat -> ae_sys::AEGP_SoundDataFormat, footage_handle.as_ptr())
    }

    /// Populates and returns a `AEGP_FileSequenceImportOptions` describing the given `AEGP_FootageH`.
    pub fn footage_sequence_import_options(&self, footage_handle: impl AsPtr<AEGP_FootageH>) -> Result<ae_sys::AEGP_FileSequenceImportOptions, Error> {
        call_suite_fn_single!(self, AEGP_GetFootageSequenceImportOptions -> ae_sys::AEGP_FileSequenceImportOptions, footage_handle.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_FootageH);
define_handle_wrapper!(FootageHandle, AEGP_FootageH);

define_enum! {
    ae_sys::AEGP_FootageSignature,
    FootageSignature {
        None    = ae_sys::AEGP_FootageSignature_NONE,
        Missing = ae_sys::AEGP_FootageSignature_MISSING,
        Solid   = ae_sys::AEGP_FootageSignature_SOLID,
    }
}
define_enum! {
    ae_sys::AEGP_InterpretationStyle,
    InterpretationStyle {
        /// Will guess alpha interpretation even if file contains unknown alpha interpretation and user pref says to ask user.
        NoDialogGuess = ae_sys::AEGP_InterpretationStyle_NO_DIALOG_GUESS,
        /// Optionally can show a dialog.
        DialogOk = ae_sys::AEGP_InterpretationStyle_DIALOG_OK,
        /// Used for replace footage implementation.
        NoDialogNoGuess = ae_sys::AEGP_InterpretationStyle_NO_DIALOG_NO_GUESS,
    }
}
define_enum! {
    ae_sys::AEGP_Platform,
    Platform {
        Mac = ae_sys::AEGP_Platform_MAC,
        Win = ae_sys::AEGP_Platform_WIN,
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_FootageH, FootageHandle,
    suite: FootageSuite,
    /// Footage represents a piece of media, such as a video or audio file, or a still image, or a solid.
    Footage {
        dispose: ;

        /// Returns the number of data (RGBA or audio) files, and the number of files per frame (may be greater than one if the footage has auxiliary channels).
        num_files() -> (usize, usize) => suite.footage_num_files,

        /// Get fully realized path to footage source file.
        ///
        /// Retrieves the footage path for a piece of footage (or for the specified frame of a footage sequence).
        /// `frame_num` ranges from `0 to num_main_files`, as obtained using [`num_files()`](Self::num_files).
        ///
        /// [`ae_sys::AEGP_FOOTAGE_MAIN_FILE_INDEX`](after_effects_sys::AEGP_FOOTAGE_MAIN_FILE_INDEX) is the main file.
        path(frame_num: usize, file_index: usize) -> String => suite.footage_path,

        /// Retrieves the footage signature of specified footage.
        signature() -> FootageSignature => suite.footage_signature,

        /// Adds a footage item to a project. Footage will be adopted by the project, and may be added only once.
        /// This is Undo-able; do not dispose of the returned added item if it's undone.
        add_to_project(folder_handle: &ItemHandle) -> ItemHandle => suite.add_footage_to_project,

        /// Deletes a footage item. Do not dispose of footage you did not create, or that has been added to the project.
        dispose() -> () => suite.dispose_footage,

        /// Returns an [`ae_sys::AEGP_FootageLayerKey`](after_effects_sys::AEGP_FootageLayerKey) describing the footage.
        layer_key() -> ae_sys::AEGP_FootageLayerKey => suite.footage_layer_key,

        /// Retrieves information about the audio data in the footage item (by populating the `AEGP_SoundDataFormat` you passed in).
        sound_data_format() -> ae_sys::AEGP_SoundDataFormat => suite.footage_sound_data_format,

        /// Populates and returns a `AEGP_FileSequenceImportOptions` describing the given `AEGP_FootageH`.
        sequence_import_options() -> ae_sys::AEGP_FileSequenceImportOptions => suite.footage_sequence_import_options,
    }
);

impl Footage {
    /// Creates a new footage item. The file path is a string with platform separators.
    /// Note that footage filenames with colons are not allowed, since colons are used as path separators in the HFS+ file system.
    /// Note the optional params. If `allow_interpretation_dialog` is `false`, After Effects will guess the alpha interpretation.
    pub fn create(plugin_id: PluginId, path: &str, layer_info: Option<ae_sys::AEGP_FootageLayerKey>, sequence_options: Option<ae_sys::AEGP_FileSequenceImportOptions>, interp_style: InterpretationStyle) -> Result<Footage, Error> {
        let footage_suite = FootageSuite::new()?;
        Ok(Footage::from_handle(
            footage_suite.new_footage(plugin_id, path, layer_info, sequence_options, interp_style)?,
            false
        ))
    }

    /// This is the hip new way to add references to footage that can't be found right this moment.
    /// The file path is a string with platform separators.
    ///
    /// In CS6 and earlier, file_type was ignored and we previously recommendedsetting it to [`FileType::None`](crate::aeio::FileType::None).
    /// Starting in CC, [`FileType::None`](crate::aeio::FileType::None) is now a warning condition.
    /// If you pass [`FileType::Any`](crate::aeio::FileType::Any), then path MUST exist.
    /// If the path may not exist, pass [`FileType::Dir`](crate::aeio::FileType::Dir) for folder, or [`FileType::Generic`](crate::aeio::FileType::Generic) for a file.
    pub fn new_placeholder_with_path(plugin_id: PluginId, path: &str, path_platform: Platform, file_type: crate::aeio::FileType, width: i32, height: i32, duration: Option<Time>) -> Result<Footage, Error> {
        let footage_suite = FootageSuite::new()?;
        Ok(Footage::from_handle(
            footage_suite.new_placeholder_footage_with_path(plugin_id, path, path_platform, file_type, width, height, duration)?,
            false
        ))
    }

    /// This is the way to add a solid.
    /// Until the footage is added to the project, the caller owns the [`FootageHandle`]
    /// (and must dispose of it if, and only if, it isn't added to the project).
    pub fn new_solid(name: &str, width: i32, height: i32, color: &ae_sys::AEGP_ColorVal) -> Result<Footage, Error> {
        let footage_suite = FootageSuite::new()?;
        Ok(Footage::from_handle(
            footage_suite.new_solid_footage(name, width, height, color)?,
            false
        ))
    }
}
