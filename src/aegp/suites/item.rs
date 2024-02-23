use crate::*;
use crate::aegp::*;

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
    pub fn first_proj_item(&self, project_handle: ProjectHandle) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetFirstProjItem -> ae_sys::AEGP_ItemH, project_handle.as_ptr())?
        ))
    }

    /// Retrieves the next project item; Result will be `None` after the last item.
    pub fn next_proj_item(&self, project_handle: ProjectHandle, item_handle: ItemHandle) -> Result<Option<ItemHandle>, Error> {
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
    pub fn is_item_selected(&self, item_handle: ItemHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsItemSelected -> ae_sys::A_Boolean, item_handle.as_ptr())? != 0)
    }

    /// Toggles the selection state of the item, and (depending on `deselect_others`) can deselect other items.
    /// This call selects items in the Project panel.
    ///
    /// To make selections in the Composition panel, use [`suites::Comp:set_selection()`](aegp::suites::Comp::set_selection).
    pub fn select_item(&self, item_handle: ItemHandle, select: bool, deselect_others: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SelectItem, item_handle.as_ptr(), select.into(), deselect_others.into())
    }

    /// Gets type of an item. Note: solids don't appear in the project, but can be the source to a layer.
    pub fn item_type(&self, item_handle: ItemHandle) -> Result<ItemType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemType -> ae_sys::AEGP_ItemType, item_handle.as_ptr())?.into())
    }

    /// Get name of type. (name length up to `32`).
    pub fn type_name(&self, item_type: ItemType) -> Result<String, Error> {
        let mut buffer = [0i8; ae_sys::AEGP_MAX_TYPE_NAME_SIZE as _];
        call_suite_fn!(self, AEGP_GetTypeName, item_type.into(), buffer.as_mut_ptr() as *mut _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Get item name.
    pub fn item_name(&self, plugin_id: PluginID, item_handle: ItemHandle) -> Result<String, Error> {
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
    pub fn set_item_name(&self, item_handle: ItemHandle, name: &str) -> Result<(), Error> {
        let name = U16CString::from_str(name).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetItemName, item_handle.as_ptr(), name.as_ptr())
    }

    /// Returns the item's unique ID, which persists across saves and loads of the project.
    pub fn item_id(&self, item_handle: ItemHandle) -> Result<ItemID, Error> {
        call_suite_fn_single!(self, AEGP_GetItemID -> ItemID, item_handle.as_ptr())
    }

    /// Get properties of an item.
    ///
    /// Unlike the [`ItemFlags::HAS_AUDIO`] flag, this bit flag will set only if the comp has at least one layer where audio is actually on.
    pub fn item_flags(&self, item_handle: ItemHandle) -> Result<ItemFlags, Error> {
        Ok(ItemFlags::from_bits_truncate(
            call_suite_fn_single!(self, AEGP_GetItemFlags -> ae_sys::A_long, item_handle.as_ptr())? as _
        ))
    }

    /// Toggle item's proxy usage. Undoable.
    pub fn set_item_use_proxy(&self, item_handle: ItemHandle, use_proxy: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemUseProxy, item_handle.as_ptr(), use_proxy.into())
    }

    /// Get folder containing item.
    pub fn item_parent_folder(&self, item_handle: ItemHandle) -> Result<Option<ItemHandle>, Error> {
        let parent_folder = call_suite_fn_single!(self, AEGP_GetItemParentFolder -> ae_sys::AEGP_ItemH, item_handle.as_ptr())?;
        if parent_folder.is_null() {
            Ok(None)
        } else {
            Ok(Some(ItemHandle::from_raw(parent_folder)))
        }
    }

    /// Sets an item's parent folder. Undoable.
    pub fn set_item_parent_folder(&self, item_handle: ItemHandle, parent_folder: ItemHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemParentFolder, item_handle.as_ptr(), parent_folder.as_ptr())
    }

    /// Get duration of item, in seconds.
    pub fn item_duration(&self, item_handle: ItemHandle) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemDuration -> ae_sys::A_Time, item_handle.as_ptr())?.into())
    }

    /// Get current time within item. Not updated while rendering.
    pub fn item_current_time(&self, item_handle: ItemHandle) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemCurrentTime -> ae_sys::A_Time, item_handle.as_ptr())?.into())
    }

    /// Get width and height of item.
    pub fn item_dimensions(&self, item_handle: ItemHandle) -> Result<(u32, u32), Error> {
        let (width, height) = call_suite_fn_double!(self, AEGP_GetItemDimensions -> ae_sys::A_long, ae_sys:: A_long, item_handle.as_ptr())?;
        Ok((
            width as _,
            height as _
        ))
    }

    /// Get the width of a pixel, assuming its height is 1.0, as numerator over denominator.
    pub fn item_pixel_aspect_ratio(&self, item_handle: ItemHandle) -> Result<Ratio, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemPixelAspectRatio -> ae_sys::A_Ratio, item_handle.as_ptr())?.into())
    }

    /// Removes item from all compositions. Undo-able.
    /// Do not use the [`ItemHandle`] after calling this function.
    pub fn delete_item(&self, item_handle: ItemHandle) -> Result<(), Error> {
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
                parent_folder.map_or(std::ptr::null_mut(), |f| f.as_ptr())
            )?
        ))
    }

    /// Sets the current time within a given [`ItemHandle`].
    pub fn set_item_current_time(&self, item_handle: ItemHandle, new_time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemCurrentTime, item_handle.as_ptr(), &new_time.into())
    }

    /// Retrieves the [`ItemHandle`]'s comment.
    pub fn item_comment(&self, item_handle: ItemHandle) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetItemComment -> ae_sys::AEGP_MemHandle, item_handle.as_ptr())?;
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Sets the [`ItemHandle`]'s comment.
    pub fn set_item_comment(&self, item_handle: ItemHandle, comment: &str) -> Result<(), Error> {
        let comment = U16CString::from_str(comment).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SetItemComment, item_handle.as_ptr(), comment.as_ptr())
    }

    /// Retrieves an item's label.
    pub fn item_label(&self, item_handle: ItemHandle) -> Result<LabelId, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemLabel -> ae_sys::AEGP_LabelID, item_handle.as_ptr())?.into())
    }

    /// Sets an item's label.
    pub fn set_item_label(&self, item_handle: ItemHandle, label: LabelId) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetItemLabel, item_handle.as_ptr(), label.into())
    }

    /// Gets an item's most recently used view.
    ///
    /// The view can be used with two calls in the [`suites::ColorSettings`](aegp::suites::ColorSettings), to perform a color transform on a pixel buffer from working to view color space.
    pub fn item_mru_view(&self, item_handle: ItemHandle) -> Result<ae_sys::AEGP_ItemViewP, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetItemMRUView -> ae_sys::AEGP_ItemViewP, item_handle.as_ptr())?.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_handle_wrapper!(ItemHandle, AEGP_ItemH);
define_handle_wrapper!(ProjectHandle, AEGP_ProjectH);

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
