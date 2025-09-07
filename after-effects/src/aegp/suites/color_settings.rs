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
        let val: ae_sys::AEGP_GuidP = unsafe { std::mem::zeroed() };
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
