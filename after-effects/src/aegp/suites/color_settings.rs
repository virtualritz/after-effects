use crate::*;
use crate::aegp::*;
use pr::RenderContextHandle;

define_suite!(
    /// We've provided a function so AEGPs can obtain information on After Effects' current color management settings.
    ColorSettingsSuite,
    AEGP_ColorSettingsSuite6,
    kAEGPColorSettingsSuite,
    kAEGPColorSettingsSuiteVersion6
);

impl ColorSettingsSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the current opaque `PF_EffectBlendingTables`, for use with `AEGP_TransferRect`.
    pub fn blending_tables(&self, render_context: RenderContextHandle) -> Result<ae_sys::PF_EffectBlendingTables, Error> {
        call_suite_fn_single!(self, AEGP_GetBlendingTables -> ae_sys::PF_EffectBlendingTables, render_context.as_ptr())
    }

    /// Returns whether there is a colorspace transform applied to the current item view.
    pub fn does_view_have_color_space_xform(&self, view: ItemViewHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_DoesViewHaveColorSpaceXform -> ae_sys::A_Boolean, view.as_ptr())? != 0)
    }

    /// Changes the view colorspace of the source to be the working colorspace of the destination.
    /// Source and destination can be the same.
    pub fn xform_working_to_view_color_space(&self, view: ItemViewHandle, src: WorldHandle, dst: &mut WorldHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_XformWorkingToViewColorSpace, view.as_ptr(), src.as_ptr(), dst.as_ptr())
    }

    /// Retrieves the opaque current working space ICC profile.
    /// The "New" in the name does not indicate that you're making up a new profile; rather, it's part of our function naming standard; anything with "New" in the name allocates something which the caller must dispose.
    pub fn new_working_space_color_profile(&self, plugin_id: PluginId, comp: CompHandle) -> Result<ColorProfileHandle, Error> {
        Ok(ColorProfileHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_GetNewWorkingSpaceColorProfile -> ae_sys::AEGP_ColorProfileP, plugin_id, comp.as_ptr())?
        ))
    }

    /// Retrieves a new [`ColorProfileHandle`] from After Effects, representing the specified ICC profile.
    pub fn new_color_profile_from_icc_profile(&self, plugin_id: PluginId, icc_size: i32, icc_data: *const std::ffi::c_void) -> Result<ColorProfileHandle, Error> {
        Ok(ColorProfileHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_GetNewColorProfileFromICCProfile -> ae_sys::AEGP_ColorProfileP, plugin_id, icc_size, icc_data)?
        ))
    }

    /// Retrieves a new ICC profile representing the specified color profile.
    ///
    /// Use [`MemHandle::to_bytes()`] to convert into `Vec<u8>`.
    pub fn new_icc_profile_from_color_profile(&self, plugin_id: PluginId, color_profile: ConstColorProfileHandle) -> Result<MemHandle<'_, u8>, Error> {
        let handle = call_suite_fn_single!(self, AEGP_GetNewICCProfileFromColorProfile -> ae_sys::AEGP_MemHandle, plugin_id, color_profile.as_ptr())?;
        Ok(MemHandle::from_raw(handle)?)
    }

    /// Returns a textual description of the specified color profile.
    pub fn new_color_profile_description(&self, plugin_id: PluginId, color_profile: ConstColorProfileHandle) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetNewColorProfileDescription -> ae_sys::AEGP_MemHandle, plugin_id, color_profile.as_ptr())?;
        // SAFETY: Constructing UTF-16 string from After Effects SDK memory handle.
        // Detailed explanation: (1) mem_handle is a valid AEGP_MemHandle returned by AE SDK, (2) MemHandle::lock() guarantees exclusive access during the lifetime of the lock guard, (3) the pointer from as_ptr() is valid for the duration of the lock, (4) U16CString::from_ptr_str reads until null terminator, which AE SDK guarantees for string data.
        // Would be UB if: the memory handle contained non-null-terminated UTF-16 data, or if the pointer was accessed after MemHandle was dropped, or if the memory was not properly allocated by the AE SDK.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Disposes of a color profile, obtained using other functions in this suite.
    pub fn dispose_color_profile(&self, color_profile: ae_sys::AEGP_ColorProfileP) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeColorProfile, color_profile)
    }

    /// Returns a floating point number approximating the gamma setting used by the specified color profile.
    pub fn color_profile_approximate_gamma(&self, color_profile: ConstColorProfileHandle) -> Result<f32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetColorProfileApproximateGamma -> ae_sys::A_FpShort, color_profile.as_ptr())?)
    }

    /// Returns whether the specified color profile is RGB.
    pub fn is_rgb_color_profile(&self, color_profile: ConstColorProfileHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsRGBColorProfile -> ae_sys::A_Boolean, color_profile.as_ptr())? != 0)
    }

    /// Sets the working space to the passed color profile.
    pub fn set_working_color_space(&self, plugin_id: PluginId, comp: CompHandle, color_profile: ConstColorProfileHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetWorkingColorSpace, plugin_id, comp.as_ptr(), color_profile.as_ptr())
    }

    /// Check if the current project is using the OCIO color engine or not.
    /// Returns true if current project uses OCIO color managed mode.
    pub fn is_ocio_color_management_used(&self, plugin_id: PluginId) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_IsOCIOColorManagementUsed -> ae_sys::A_Boolean, plugin_id)? != 0)
    }

    /// Returns the OCIO configuration file used by the project.
    ///
    /// Returned string is the OCIO Configuration file.
    pub fn ocio_configuration_file(&self, plugin_id: PluginId) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetOCIOConfigurationFile -> ae_sys::AEGP_MemHandle, plugin_id)?;
        // SAFETY: Constructing UTF-16 string from After Effects SDK memory handle containing OCIO configuration filename.
        // Detailed explanation: (1) mem_handle is a valid AEGP_MemHandle returned by AE SDK, (2) MemHandle::lock() provides exclusive access to the memory during the lock's lifetime, (3) as_ptr() yields a valid pointer for the duration of the lock guard, (4) U16CString::from_ptr_str safely reads null-terminated UTF-16 data guaranteed by the AE SDK.
        // Would be UB if: the AE SDK returned non-null-terminated UTF-16 data, or if the pointer outlived the lock guard, or if the memory handle was invalid.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Returns the absolute file path to the OCIO configuration used by the project
    ///
    /// The returned string is an absolute path to OCIO Configuration file.
    pub fn ocio_configuration_file_path(&self, plugin_id: PluginId) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetOCIOConfigurationFilePath -> ae_sys::AEGP_MemHandle, plugin_id)?;
        // SAFETY: Constructing UTF-16 string from After Effects SDK memory handle containing OCIO configuration file path.
        // Detailed explanation: (1) mem_handle is a valid AEGP_MemHandle returned by AE SDK for path data, (2) MemHandle::lock() ensures exclusive access and prevents data races during read, (3) as_ptr() returns a valid pointer tied to the lock guard's lifetime, (4) U16CString::from_ptr_str reads null-terminated UTF-16 data which the AE SDK guarantees for path strings.
        // Would be UB if: the memory handle contained non-null-terminated data, or if the pointer was dereferenced after the lock guard was dropped, or if the AE SDK returned an invalid memory handle.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Returns the working color space of the project in OCIO mode.
    ///
    /// The returned string specifies the working color space.
    pub fn ocio_working_color_space(&self, plugin_id: PluginId) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGPD_GetOCIOWorkingColorSpace -> ae_sys::AEGP_MemHandle, plugin_id)?;
        // SAFETY: Constructing UTF-16 string from After Effects SDK memory handle containing OCIO working color space name.
        // Detailed explanation: (1) mem_handle is a valid AEGP_MemHandle obtained from the AE SDK, (2) MemHandle::lock() acquires exclusive access for the lock's lifetime, (3) as_ptr() provides a valid pointer bound to the lock guard's lifetime, (4) U16CString::from_ptr_str reads null-terminated UTF-16 data as guaranteed by the AE SDK for string values.
        // Would be UB if: the AE SDK returned non-null-terminated UTF-16 data, or if the pointer was used after the lock guard expired, or if the memory handle was corrupt.
        Ok(unsafe {
            U16CString::from_ptr_str(
                MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Returns the Display and View transforms used by the project.
    ///
    /// The returned strings specify the Display and View transforms used at project level.
    pub fn ocio_display_color_space(&self, plugin_id: PluginId) -> Result<(String, String), Error> {
        let (display, view) = call_suite_fn_double!(self, AEGPD_GetOCIODisplayColorSpace -> ae_sys::AEGP_MemHandle, ae_sys::AEGP_MemHandle, plugin_id)?;
        // SAFETY: Constructing two UTF-16 strings from After Effects SDK memory handles for display and view transforms.
        // Detailed explanation: (1) both display and view are valid AEGP_MemHandles returned by AE SDK, (2) MemHandle::lock() for each handle provides exclusive access during each lock's lifetime, (3) as_ptr() for each yields valid pointers bound to their respective lock guards, (4) U16CString::from_ptr_str reads null-terminated UTF-16 data from each handle as guaranteed by the AE SDK, (5) both operations complete before their respective lock guards are dropped.
        // Would be UB if: either memory handle contained non-null-terminated data, or if pointers were accessed after their lock guards dropped, or if the AE SDK returned invalid memory handles for either value.
        Ok(unsafe {(
            U16CString::from_ptr_str(MemHandle::<u16>::from_raw(display)?.lock()?.as_ptr()).to_string_lossy(),
            U16CString::from_ptr_str(MemHandle::<u16>::from_raw(view)   ?.lock()?.as_ptr()).to_string_lossy()
        )})
    }
    pub fn is_color_space_aware_effects_enabled(&self, plugin_id: PluginId) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGPD_IsColorSpaceAwareEffectsEnabled -> ae_sys::A_Boolean, plugin_id)? != 0)
    }
    pub fn lut_interpolation_method(&self, plugin_id: PluginId) -> Result<u16, Error> {
        Ok(call_suite_fn_single!(self, AEGPD_GetLUTInterpolationMethod -> ae_sys::A_u_short, plugin_id)?)
    }
    pub fn graphics_white_luminance(&self, plugin_id: PluginId) -> Result<u16, Error> {
        Ok(call_suite_fn_single!(self, AEGPD_GetGraphicsWhiteLuminance -> ae_sys::A_u_short, plugin_id)?)
    }
    pub fn working_color_space_id(&self, plugin_id: PluginId) -> Result<ae_sys::AEGP_GuidP, Error> {
        // SAFETY: Zero-initializing AEGP_GuidP pointer for use as out-parameter.
        // Detailed explanation: (1) AEGP_GuidP is a pointer type that can be safely zero-initialized to null, (2) the value will be populated by the FFI call below, (3) zero-initialization is safe for all pointer types as it creates a null pointer.
        // Would be UB if: AEGP_GuidP required non-zero initialization or had invariants that zero-initialization violated (not the case for pointer types).
        let val: ae_sys::AEGP_GuidP = unsafe { std::mem::zeroed() };
        // SAFETY: Calling After Effects SDK suite function through FFI.
        // Detailed explanation: (1) self.suite_ptr is a valid pointer to the ColorSettingsSuite function table obtained during suite initialization, (2) ae_get_suite_fn! macro safely retrieves the function pointer from the suite, (3) AEGPD_GetWorkingColorSpaceId is a valid function in the suite that accepts plugin_id and an out-parameter pointer, (4) val is a valid pointer that will be written to by the AE SDK.
        // Would be UB if: self.suite_ptr was null or invalid, or if the function pointer was not correctly typed, or if val was not a valid writable pointer, or if the AE SDK function was called after suite disposal.
        let err = unsafe { ae_get_suite_fn!(self.suite_ptr, AEGPD_GetWorkingColorSpaceId)(plugin_id, val) };
        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_handle_wrapper!(ItemViewHandle, AEGP_ItemViewP);
define_handle_wrapper!(ConstColorProfileHandle, AEGP_ConstColorProfileP);

define_owned_handle_wrapper!(ColorProfileHandle, AEGP_ColorProfileP);
impl Drop for ColorProfileHandle {
    fn drop(&mut self) {
        if let Ok(suite) = ColorSettingsSuite::new() {
            if suite.dispose_color_profile(self.as_ptr()).is_ok() {
                self.0 = std::ptr::null_mut();
            }
        }
    }
}
