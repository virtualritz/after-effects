use std::ffi::c_void;

use after_effects_sys::{A_Boolean, A_FpLong, A_Time, A_long, AEGP_MemHandle, PF_PixelFloat};

use crate::aegp::*;
use crate::*;

define_suite!(
    /// Plug-ins have read and write access to persistent data in After Effects' preferences. AEGPs may add and manage their own persistent data using the following suite. The data entries are accessed by (section key, value key) pairs. It is recommended that plug-ins use their matchname as their section key, or as a prefix if using multiple section keys.
    /// The available data types are A_long, A_FpLong, strings, and void*. A_FpLongs are stored with 6 decimal places of precision. There is no provision for specifying a different precision. String data supports the full 8-bit space. Only 0x00 is reserved for string ending. This makes them ideal for storing UTF-8 encoded strings, ISO 8859-1, and plain ASCII. Both section keys and value keys are of this type. For data types not represented by the simple data types provided, use data handles containing your custom data. void* unstructured data allows you to store any kind of data. You must pass in a size in bytes along with the data.
    /// When calling any of the functions to retrieve the value of a key, if a given key is not found, the default value is both written to the blob and returned as the value; if no default is provided, a blank value will be written and returned.
    ///
    /// Note that this data is stored in the application's preferences, not in the project. As of 6.5, there is no way to store opaque AEGP-generated data in an After Effects project.
    /// After Effects can handle plug-ins which change the preferences during their application; it checks the in-RAM copy of the prefs before acting upon pref-able settings, rather than relying on the saved prefs. It's like we *planned* this, or something!
    ///
    /// Note:
    /// For getters and setters, If a given key is not found, the default value is both written to the blob and returned as the value; if no default is provided, a blank value will be written and returned.
    PersistentDataSuite,
    AEGP_PersistentDataSuite4,
    kAEGPPersistentDataSuite,
    kAEGPPersistentDataSuiteVersion4
);

define_enum! {
    ae_sys::AEGP_PersistentType,
    PersistentType {
        MachineIndependent            = ae_sys::AEGP_PersistentType_MACHINE_INDEPENDENT,
        MachineIndependentRender      = ae_sys::AEGP_PersistentType_MACHINE_INDEPENDENT_RENDER,
        MachineIndependentOutput      = ae_sys::AEGP_PersistentType_MACHINE_INDEPENDENT_OUTPUT,
        MachineIndependentComposition = ae_sys::AEGP_PersistentType_MACHINE_INDEPENDENT_COMPOSITION,
        MachineSpecific               = ae_sys::AEGP_PersistentType_MACHINE_SPECIFIC,
        MachineSpecificText           = ae_sys::AEGP_PersistentType_MACHINE_SPECIFIC_TEXT,
        MachineSpecificPaint          = ae_sys::AEGP_PersistentType_MACHINE_SPECIFIC_PAINT,
    }
}

register_handle!(AEGP_PersistentBlobH);
define_handle_wrapper!(PersistentBlobHandle, AEGP_PersistentBlobH);

impl PersistentDataSuite {
    pub fn new() -> Result<Self, Error> { Suite::new() }

    /// Obtains the handle to all persistent application data. Modifying this will modify the application.
    /// The `AEGP_PersistentType` parameter is new in CC, and should be set to one of the following:
    ///
    /// - AEGP_PersistentType_MACHINE_SPECIFIC
    /// - AEGP_PersistentType_MACHINE_INDEPENDENT
    /// - AEGP_PersistentType_MACHINE_INDEPENDENT_RENDER
    /// - AEGP_PersistentType_MACHINE_INDEPENDENT_OUTPUT
    /// - AEGP_PersistentType_MACHINE_INDEPENDENT_COMPOSITION
    /// - AEGP_PersistentType_MACHINE_SPECIFIC_TEXT
    /// - AEGP_PersistentType_MACHINE_SPECIFIC_PAINT
    pub fn application_blob(
        &self,
        blob_type: PersistentType,
    ) -> Result<PersistentBlobHandle, Error> {
        Ok(PersistentBlobHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetApplicationBlob -> ae_sys::AEGP_PersistentBlobH, blob_type.into())?,
        ))
    }

    /// Obtains the number of sections in the application blob.
    pub fn num_sections(&self, blob_handle: PersistentBlobHandle) -> Result<i32, Error> {
        call_suite_fn_single!(self, AEGP_GetNumSections -> A_long, blob_handle.into())
    }

    /// Obtains the key at the given index.
    pub fn section_by_key_index(
        &self,
        blob_handle: PersistentBlobHandle,
        section_index: i32,
        max_section_size: usize,
    ) -> Result<String, Error> {
        let mut out_data = vec![0i8; max_section_size + 1];

        call_suite_fn!(
            self,
            AEGP_GetSectionKeyByIndex,
            blob_handle.into(),
            section_index,
            max_section_size as _,
            out_data.as_mut_ptr() as _
        )?;

        buffer_to_string(out_data)
    }

    /// Returns whether or not a given key/value pair exists with the blob.
    pub fn does_key_exist(
        &self,
        blob_handle: PersistentBlobHandle,
        section: &str,
        value: &str,
    ) -> Result<bool, Error> {
        let key = CString::new(section).map_err(|_| Error::InvalidParms)?;
        let value = CString::new(value).map_err(|_| Error::InvalidParms)?;
        call_suite_fn_single!(self, AEGP_DoesKeyExist -> A_Boolean, blob_handle.into(), key.as_ptr(), value.as_ptr()).map(|b| !b.is_zero())
    }

    /// Retrieves the number of value keys in the section.
    pub fn num_keys(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
    ) -> Result<i32, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        call_suite_fn_single!(self, AEGP_GetNumKeys -> A_long, blob_handle.into(), section_key.as_ptr())
    }

    /// Retrieves the value of the indexed key.
    pub fn value_by_key_index(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        key_index: i32,
        max_key_size: usize,
    ) -> Result<String, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let mut out_data = vec![0i8; max_key_size + 1];

        call_suite_fn!(
            self,
            AEGP_GetValueKeyByIndex,
            blob_handle.into(),
            section_key.as_ptr(),
            key_index,
            max_key_size as _,
            out_data.as_mut_ptr() as _,
        )?;

        buffer_to_string(out_data)
    }

    pub fn get_data_handle<'a, T>(
        &self,
        plugin_id: PluginId,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: MemHandle<'a, T>,
    ) -> Result<MemHandle<'_, T>, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        let handle = call_suite_fn_single!(self, AEGP_GetDataHandle -> ae_sys::AEGP_MemHandle,
                                           plugin_id,
                                           blob_handle.as_ptr(),
                                           section_key.as_ptr(),
                                           value_key.as_ptr(),
                                           default.as_raw())?;

        MemHandle::from_raw(handle)
    }

    /// Obtains the data located at a given section's value.
    pub fn get_data(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        data_size: u32,
        default: *const c_void,
    ) -> Result<*mut c_void, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        let ptr = std::ptr::null_mut();

        call_suite_fn!(
            self,
            AEGP_GetData,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            data_size,
            default,
            ptr
        )?;

        Ok(ptr)
    }

    /// Obtains the string for a given section key's value
    ///
    /// Note: This interperets all stored strings as UTF-8. This is safe if you are retrieving
    /// strings stored from rust, which must be UTF-8.
    pub fn get_string(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: &str,
        max_result_length: usize,
    ) -> Result<String, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;
        let default_value = CString::new(default).map_err(|_| Error::InvalidParms)?;
        let default_len = default_value.as_bytes_with_nul().len();

        let mut out_data = vec![0i8; max_result_length + 1];
        let mut out_len = 0;

        call_suite_fn!(
            self,
            AEGP_GetString,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            default_value.as_ptr(),
            default_len as _,
            out_data.as_mut_ptr() as _,
            &mut out_len
        )?;

        buffer_to_string(out_data)
    }

    pub fn get_long(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: i32,
    ) -> Result<i32, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn_single!(
            self,
            AEGP_GetLong -> A_long,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            default
        )
    }

    pub fn get_fp_long(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: f64,
    ) -> Result<f64, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn_single!(
            self,
            AEGP_GetFpLong -> A_FpLong,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            default
        )
    }

    pub fn get_time(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: Time,
    ) -> Result<Time, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        let default = A_Time {
            value: default.value,
            scale: default.scale,
        };

        call_suite_fn_single!(
            self,
            AEGP_GetTime -> A_Time,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            &default
        )
        .map(|t| t.into())
    }

    pub fn get_argb(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        default: PF_PixelFloat,
    ) -> Result<PF_PixelFloat, Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn_single!(
            self,
            AEGP_GetARGB -> PF_PixelFloat,
            blob_handle.as_ptr(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            &default
        )
    }

    // Sets the given section key's value to the handle passed in.
    pub fn set_data_handle<'a, T>(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: MemHandle<'a, T>,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetDataHandle,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            value.as_raw(),
        )
    }

    // Sets the given section key's value to value.
    pub fn set_data(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        data: *const c_void,
        data_size: u32,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetData,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            data_size,
            data
        )
    }

    // Sets the given section key's value to value.
    pub fn set_string(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: &str,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;
        let value = CString::new(value).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetString,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            value.as_ptr(),
        )
    }

    // Sets the given section key's value to value.
    pub fn set_long(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: i32,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetLong,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            value
        )
    }

    // Sets the given section key's value to value.
    pub fn set_fp_long(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: f64,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        call_suite_fn!(
            self,
            AEGP_SetFpLong,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            value
        )
    }

    // Sets the given section key's value to value.
    pub fn set_time(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: &Time,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;

        let raw_time = A_Time {
            value: value.value,
            scale: value.scale,
        };

        call_suite_fn!(
            self,
            AEGP_SetTime,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            &raw_time as _
        )
    }

    // New in CC. Sets the given section key's value to valueP.
    pub fn set_argb(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
        value: &PF_PixelFloat,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(
            self,
            AEGP_SetARGB,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr(),
            value
        )
    }

    // Removes the given section's value from the blob.
    pub fn delete_entry(
        &self,
        blob_handle: PersistentBlobHandle,
        section_key: &str,
        value_key: &str,
    ) -> Result<(), Error> {
        let section_key = CString::new(section_key).map_err(|_| Error::InvalidParms)?;
        let value_key = CString::new(value_key).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(
            self,
            AEGP_DeleteEntry,
            blob_handle.into(),
            section_key.as_ptr(),
            value_key.as_ptr()
        )
    }

    // Get the path to the folder containing After Effects' preference file.
    pub fn prefs_directory(&self) -> Result<String, Error> {
        let wide_path = call_suite_fn_single!(
            self,
            AEGP_GetPrefsDirectory -> AEGP_MemHandle
        )?;

        Ok(unsafe {
            U16CString::from_ptr_str(MemHandle::<u16>::from_raw(wide_path)?.lock()?.as_ptr())
                .to_string_lossy()
        })
    }
}

fn buffer_to_string(mut buffer: Vec<i8>) -> Result<String, Error> {
    if let Some(null_pos) = buffer.iter().position(|b| *b == 0) {
        buffer.truncate(null_pos + 1);
    } else {
        buffer.push(0);
    }

    let buffer = buffer.into_iter().map(|i| i as u8).collect();

    let c_str = CString::from_vec_with_nul(buffer).map_err(|_| Error::InternalStructDamaged)?;

    Ok(c_str.to_string_lossy().into_owned())
}
