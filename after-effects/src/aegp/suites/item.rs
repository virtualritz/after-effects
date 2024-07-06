use crate::*;
use crate::aegp::*;
use ae_sys::AEGP_ItemH;

define_suite!(
    /// Accesses and modifies items within a project or composition.
    ///
    /// Anything in the project bin is an `AEGP_Item`. Note that cameras have no source, and thus have no [`ItemHandle`].
    ///
    /// Unless more specificity is required for the function(s) you're using, remain as abstract as possible; AEGP_Comps are passed into and returned from most functions as AEGP_Items.
    ItemSuite,
    AEGP_ItemSuite9,
    kAEGPItemSuite,
    kAEGPItemSuiteVersion9
);

impl ItemSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the first item in a given project.
    pub fn first_proj_item(&self, project_handle: &ProjectHandle) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetFirstProjItem -> ae_sys::AEGP_ItemH, project_handle.as_ptr())?
        ))
    }

    /// Retrieves the next project item; Result will be `None` after the last item.
    pub fn next_proj_item(&self, project_handle: &ProjectHandle, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Option<ItemHandle>, Error> {
        let next_item = call_suite_fn_single!(self, AEGP_GetNextProjItem -> ae_sys::AEGP_ItemH, project_handle.as_ptr(), item_handle.as_ptr())?;
        if next_item.is_null() {
            Ok(None)
        } else {
            Ok(Some(ItemHandle::from_raw(next_item)))
        }
    }

    /// If the Project window is active, the active item is the selected item (if only one item is selected).
    /// If a Composition, Timeline, or Footage window is active, returns the parent of the layer associated with the front-most tab in the window.
    ///
    /// Returns `None` if no item is active.
    pub fn active_item(&self) -> Result<Option<ItemHandle>, Error> {
        let item_handle = call_suite_fn_single!(self, AEGP_GetActiveItem -> ae_sys::AEGP_ItemH)?;
        if item_handle.is_null() {
            Ok(None)
        } else {
            Ok(Some(ItemHandle::from_raw(item_handle)))
        }
    }

    /// Returns true if the Project window is active and the item is selected.
    pub fn is_item_selected(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsItemSelected -> ae_sys::A_Boolean, item_handle.as_ptr())? != 0)
    }

    /// Toggles the selection state of the item, and (depending on `deselect_others`) can deselect other items.
    /// This call selects items in the Project panel.
    ///
    /// To make selections in the Composition panel, use [`suites::Comp:set_selection()`](aegp::suites::Comp::set_selection).
    pub fn select_item(&self, item_handle: impl AsPtr<AEGP_ItemH>, select: bool, deselect_others: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SelectItem, item_handle.as_ptr(), select.into(), deselect_others.into())
    }

    /// Gets type of an item. Note: solids don't appear in the project, but can be the source to a layer.
    pub fn item_type(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<ItemType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemType -> ae_sys::AEGP_ItemType, item_handle.as_ptr())?.into())
    }

    /// Get name of type. (name length up to `32`).
    pub fn type_name(&self, item_type: ItemType) -> Result<String, Error> {
        let mut buffer = [0i8; ae_sys::AEGP_MAX_TYPE_NAME_SIZE as _];
        call_suite_fn!(self, AEGP_GetTypeName, item_type.into(), buffer.as_mut_ptr() as *mut _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Get item name.
    pub fn item_name(&self, item_handle: impl AsPtr<AEGP_ItemH>, plugin_id: PluginId) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetItemName -> ae_sys::AEGP_MemHandle, plugin_id, item_handle.as_ptr())?;
        // Create a mem handle each and lock it.
        // When the lock goes out of scope it unlocks and when the handle goes out of scope it gives the memory back to Ae.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Specifies the name of the [`ItemHandle`].
    pub fn set_item_name(&self, item_handle: impl AsPtr<AEGP_ItemH>, name: &str) -> Result<(), Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetItemName, item_handle.as_ptr(), name.as_ptr())
    }

    /// Returns the item's unique ID, which persists across saves and loads of the project.
    pub fn item_id(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<ItemId, Error> {
        call_suite_fn_single!(self, AEGP_GetItemID -> ItemId, item_handle.as_ptr())
    }

    /// Get properties of an item.
    ///
    /// Unlike the [`ItemFlags::HAS_AUDIO`] flag, this bit flag will set only if the comp has at least one layer where audio is actually on.
    pub fn item_flags(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<ItemFlags, Error> {
        Ok(ItemFlags::from_bits_truncate(
            call_suite_fn_single!(self, AEGP_GetItemFlags -> ae_sys::A_long, item_handle.as_ptr())? as _
        ))
    }

    /// Toggle item's proxy usage. Undoable.
    pub fn set_item_use_proxy(&self, item_handle: impl AsPtr<AEGP_ItemH>, use_proxy: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemUseProxy, item_handle.as_ptr(), use_proxy.into())
    }

    /// Get folder containing item.
    pub fn item_parent_folder(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Option<ItemHandle>, Error> {
        let parent_folder = call_suite_fn_single!(self, AEGP_GetItemParentFolder -> ae_sys::AEGP_ItemH, item_handle.as_ptr())?;
        if parent_folder.is_null() {
            Ok(None)
        } else {
            Ok(Some(ItemHandle::from_raw(parent_folder)))
        }
    }

    /// Sets an item's parent folder. Undoable.
    pub fn set_item_parent_folder(&self, item_handle: impl AsPtr<AEGP_ItemH>, parent_folder: &ItemHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemParentFolder, item_handle.as_ptr(), parent_folder.as_ptr())
    }

    /// Get duration of item, in seconds.
    pub fn item_duration(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemDuration -> ae_sys::A_Time, item_handle.as_ptr())?.into())
    }

    /// Get current time within item. Not updated while rendering.
    pub fn item_current_time(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemCurrentTime -> ae_sys::A_Time, item_handle.as_ptr())?.into())
    }

    /// Get width and height of item.
    pub fn item_dimensions(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<(u32, u32), Error> {
        let (width, height) = call_suite_fn_double!(self, AEGP_GetItemDimensions -> ae_sys::A_long, ae_sys:: A_long, item_handle.as_ptr())?;
        Ok((
            width as _,
            height as _
        ))
    }

    /// Get the width of a pixel, assuming its height is 1.0, as numerator over denominator.
    pub fn item_pixel_aspect_ratio(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<Ratio, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemPixelAspectRatio -> ae_sys::A_Ratio, item_handle.as_ptr())?.into())
    }

    /// Removes item from all compositions. Undo-able.
    /// Do not use the [`ItemHandle`] after calling this function.
    pub fn delete_item(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteItem, item_handle.as_ptr())
    }

    /// Creates a new folder in the project. The newly created folder is allocated and owned by After Effects.
    ///
    /// Passing `None` for `parent_folder` creates the folder at the project's root.
    pub fn create_new_folder(&self, name: &str, parent_folder: Option<ItemHandle>) -> Result<ItemHandle, Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self,
                AEGP_CreateNewFolder -> ae_sys::AEGP_ItemH,
                name.as_ptr(),
                parent_folder.as_ref().map_or(std::ptr::null_mut(), |f| f.as_ptr())
            )?
        ))
    }

    /// Sets the current time within a given [`ItemHandle`].
    pub fn set_item_current_time(&self, item_handle: impl AsPtr<AEGP_ItemH>, new_time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemCurrentTime, item_handle.as_ptr(), &new_time.into())
    }

    /// Retrieves the [`ItemHandle`]'s comment.
    pub fn item_comment(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetItemComment -> ae_sys::AEGP_MemHandle, item_handle.as_ptr())?;
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Sets the [`ItemHandle`]'s comment.
    pub fn set_item_comment(&self, item_handle: impl AsPtr<AEGP_ItemH>, comment: &str) -> Result<(), Error> {
        let comment = U16CString::from_str(comment).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetItemComment, item_handle.as_ptr(), comment.as_ptr())
    }

    /// Retrieves an item's label.
    pub fn item_label(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<LabelId, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemLabel -> ae_sys::AEGP_LabelID, item_handle.as_ptr())?.into())
    }

    /// Sets an item's label.
    pub fn set_item_label(&self, item_handle: impl AsPtr<AEGP_ItemH>, label: LabelId) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemLabel, item_handle.as_ptr(), label.into())
    }

    /// Gets an item's most recently used view.
    ///
    /// The view can be used with two calls in the [`suites::ColorSettings`](aegp::suites::ColorSettings), to perform a color transform on a pixel buffer from working to view color space.
    pub fn item_mru_view(&self, item_handle: impl AsPtr<AEGP_ItemH>) -> Result<ae_sys::AEGP_ItemViewP, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemMRUView -> ae_sys::AEGP_ItemViewP, item_handle.as_ptr())?.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――
register_handle!(AEGP_ItemH);
define_handle_wrapper!(ItemHandle, AEGP_ItemH);

define_enum! {
    ae_sys::AEGP_ItemType,
    ItemType {
        None    = ae_sys::AEGP_ItemType_NONE,
        Folder  = ae_sys::AEGP_ItemType_FOLDER,
        Comp    = ae_sys::AEGP_ItemType_COMP,
        /// as of AE6, solids are now just [`ItemType::Footage`] with `AEGP_FootageSignature_SOLID`
        Solid   = ae_sys::AEGP_ItemType_SOLID_defunct,
        Footage = ae_sys::AEGP_ItemType_FOOTAGE,
    }
}

define_enum! {
    ae_sys::AEGP_LabelID,
    LabelId {
        None = ae_sys::AEGP_Label_NONE,
        NoLabel = ae_sys::AEGP_Label_NO_LABEL,
        Label1 = ae_sys::AEGP_Label_1,
        Label2 = ae_sys::AEGP_Label_2,
        Label3 = ae_sys::AEGP_Label_3,
        Label4 = ae_sys::AEGP_Label_4,
        Label5 = ae_sys::AEGP_Label_5,
        Label6 = ae_sys::AEGP_Label_6,
        Label7 = ae_sys::AEGP_Label_7,
        Label8 = ae_sys::AEGP_Label_8,
        Label9 = ae_sys::AEGP_Label_9,
        Label10 = ae_sys::AEGP_Label_10,
        Label11 = ae_sys::AEGP_Label_11,
        Label12 = ae_sys::AEGP_Label_12,
        Label13 = ae_sys::AEGP_Label_13,
        Label14 = ae_sys::AEGP_Label_14,
        Label15 = ae_sys::AEGP_Label_15,
        Label16 = ae_sys::AEGP_Label_16,
    }
}

bitflags::bitflags! {
    pub struct ItemFlags: ae_sys::A_long {
        const MISSING           = ae_sys::AEGP_ItemFlag_MISSING          as ae_sys::A_long;
        const HAS_PROXY         = ae_sys::AEGP_ItemFlag_HAS_PROXY        as ae_sys::A_long;
        const USING_PROXY       = ae_sys::AEGP_ItemFlag_USING_PROXY      as ae_sys::A_long;
        const MISSING_PROXY     = ae_sys::AEGP_ItemFlag_MISSING_PROXY    as ae_sys::A_long;
        const HAS_VIDEO         = ae_sys::AEGP_ItemFlag_HAS_VIDEO        as ae_sys::A_long;
        const HAS_AUDIO         = ae_sys::AEGP_ItemFlag_HAS_AUDIO        as ae_sys::A_long;
        const STILL             = ae_sys::AEGP_ItemFlag_STILL            as ae_sys::A_long;
        const HAS_ACTIVE_AUDIO  = ae_sys::AEGP_ItemFlag_HAS_ACTIVE_AUDIO as ae_sys::A_long;
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_ItemH, ItemHandle,
    suite: ItemSuite,
    footage: aegp::suites::Footage,
    comp: aegp::suites::Comp,
    /// Item can be a folder, a composition, or a footage
    Item {
        dispose: ;

        /// Returns true if the Project window is active and the item is selected.
        is_selected() -> bool => suite.is_item_selected,

        /// Toggles the selection state of the item, and (depending on `deselect_others`) can deselect other items.
        /// This call selects items in the Project panel.
        ///
        /// To make selections in the Composition panel, use [`suites::Comp:set_selection()`](aegp::suites::Comp::set_selection).
        select(select: bool, deselect_others: bool) -> () => suite.select_item,

        /// Gets type of an item. Note: solids don't appear in the project, but can be the source to a layer.
        item_type() -> ItemType => suite.item_type,

        /// Get item name.
        name(plugin_id: PluginId) -> String => suite.item_name,

        /// Specifies the name of this item.
        set_name(name: &str) -> () => suite.set_item_name,

        /// Returns the item's unique ID, which persists across saves and loads of the project.
        id() -> ItemId => suite.item_id,

        /// Get properties of an item.
        ///
        /// Unlike the [`ItemFlags::HAS_AUDIO`] flag, this bit flag will set only if the comp has at least one layer where audio is actually on.
        flags() -> ItemFlags => suite.item_flags,

        /// Toggle item's proxy usage. Undoable.
        set_use_proxy(use_proxy: bool) -> () => suite.set_item_use_proxy,

        /// Get folder containing item.
        parent_folder() -> Option<ItemHandle> => suite.item_parent_folder,

        /// Sets an item's parent folder. Undoable.
        set_parent_folder(parent_folder: &ItemHandle) -> () => suite.set_item_parent_folder,

        /// Get duration of item, in seconds.
        duration() -> Time => suite.item_duration,

        /// Get current time within item. Not updated while rendering.
        current_time() -> Time => suite.item_current_time,

        /// Get width and height of item.
        dimensions() -> (u32, u32) => suite.item_dimensions,

        /// Get the width of a pixel, assuming its height is 1.0, as numerator over denominator.
        pixel_aspect_ratio() -> Ratio => suite.item_pixel_aspect_ratio,

        /// Removes item from all compositions. Undo-able.
        /// Do not use the [`ItemHandle`] after calling this function.
        delete() -> () => suite.delete_item,

        /// Sets the current time.
        set_current_time(new_time: Time) -> () => suite.set_item_current_time,

        /// Retrieves the comment.
        comment() -> String => suite.item_comment,

        /// Sets the comment.
        set_comment(comment: &str) -> () => suite.set_item_comment,

        /// Retrieves an item's label.
        label() -> LabelId => suite.item_label,

        /// Sets an item's label.
        set_label(label: LabelId) -> () => suite.set_item_label,

        /// Gets an item's most recently used view.
        ///
        /// The view can be used with two calls in the [`suites::ColorSettings`](aegp::suites::ColorSettings), to perform a color transform on a pixel buffer from working to view color space.
        mru_view() -> ae_sys::AEGP_ItemViewP => suite.item_mru_view,

        // ―――――――――――――――――――――――――――― Footage suite functions ――――――――――――――――――――――――――――

        /// Returns an error if item isn't a footage item.
        /// Used to convert an item handle to a footage handle.
        main_footage() -> Footage => footage.main_footage_from_item,

        /// Returns an error if item has no proxy. Returns the proxy footage handle.
        /// Note: a composition can have a proxy.
        proxy_footage() -> Footage => footage.proxy_footage_from_item,

        /// Sets footage as the proxy for an item. Will be adopted by the project.
        /// This is Undo-able; do not dispose of the returned added item if it's undone.
        set_proxy_footage(footage: impl AsPtr<ae_sys::AEGP_FootageH>) -> () => footage.set_item_proxy_footage,

        /// Replaces footage for an item. The item will replace the main footage for this item.
        /// This is Undo-able; do not dispose of the returned added item if it's undone.
        replace_main_footage(footage: impl AsPtr<ae_sys::AEGP_FootageH>) -> () => footage.replace_item_main_footage,

        /// Populates an AEGP_FootageInterp describing the settings of the [`FootageHandle`].
        /// There is no way to create a valid `AEGP_FootageInterp` other than by using this function.
        /// If `proxy` is `true`, the proxy footage's settings are retrieved.
        footage_interpretation(proxy: bool) -> ae_sys::AEGP_FootageInterp => footage.footage_interpretation,

        /// Apply the settings in the `AEGP_FootageInterp` to the `AEGP_FootageH`. Undo-able.
        /// If `proxy` is `true`, the proxy footage's settings are modified.
        set_footage_interpretation(proxy: bool, interp: &ae_sys::AEGP_FootageInterp) -> () => footage.set_footage_interpretation,

        /// Returns the color of a given solid. Returns an error if the [`ItemHandle`] is not a solid.
        /// If `proxy` is `true`, the proxy solid's color is retrieved.
        solid_footage_color(proxy: bool) -> ae_sys::AEGP_ColorVal => footage.solid_footage_color,

        /// Sets the color of a solid. Undo-able.
        /// If `proxy` is `true`, the proxy solid's color is set.
        set_solid_footage_color(proxy: bool, color: &ae_sys::AEGP_ColorVal) -> () => footage.set_solid_footage_color,

        /// Sets the dimensions of a solid. Undo-able.
        /// If `proxy` is `true`, the proxy solid's dimensions are modified. Returns an error if the item isn't a solid.
        set_solid_footage_dimensions(proxy: bool, width: i32, height: i32) -> () => footage.set_solid_footage_dimensions,
    }
);

impl Item {
    /// Get name of type. (name length up to `32`).
    pub fn type_name(&self) -> Result<String, Error> {
        let Ok(ref suite) = *self.suite else { return Err(Error::MissingSuite); };
        suite.type_name(self.item_type()?)
    }

    /// Creates a new folder in the project, inside the current item.
    /// The newly created folder is allocated and owned by After Effects.
    pub fn create_folder_inside(&self, name: &str) -> Result<Item, Error> {
        let Ok(ref suite) = *self.suite else { return Err(Error::MissingSuite); };
        Ok(Item::from_handle(
            suite.create_new_folder(name, Some(self.handle))?,
            false
        ))
    }

    /// Returns the composition for this item. Returns an error if the item isn't a composition.
    pub fn composition(&self) -> Result<Composition, Error> {
        if self.item_type()? == ItemType::Comp {
            let Ok(ref comp) = *self.comp else { return Err(Error::MissingSuite); };
            Ok(comp.comp_from_item(self.as_ptr())?.unwrap().into())
        } else {
            Err(Error::Parameter)
        }
    }

    /// Creates a new [`aegp::RenderOptions`] from this layer.
    pub fn render_options(&self, plugin_id: PluginId) -> Result<aegp::RenderOptions, Error> {
        aegp::RenderOptions::from_item(self.handle.as_ptr(), plugin_id)
    }
}
