use crate::*;
use widestring::U16CString;
use ae_sys::AEIO_InSpecH;

define_suite!(
    /// These functions manage an input specification, After Effects' internal representation of data gathered from any source.
    ///
    /// Any image or audio data in After Effects (except solids) is obtained from an input specification handle, or [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    IOInSuite,
    AEGP_IOInSuite5,
    kAEGPIOInSuite,
    kAEGPIOInSuiteVersion5
);

impl IOInSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the options data (created by your AEIO) for the given [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_options_handle(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<aeio::Handle, Error> {
        Ok(aeio::Handle::from_raw(
            call_suite_fn_single!(self, AEGP_GetInSpecOptionsHandle -> *mut std::ffi::c_void, in_spec_handle.as_ptr())? as _
        ))
    }

    /// Sets the options data for the given [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    ///
    /// Must be allocated using the [`suites::Memory`](aegp::suites::Memory).
    ///
    /// Returns the old options handle.
    pub fn set_in_spec_options_handle(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, options: &aeio::Handle) -> Result<aeio::Handle, Error> {
        Ok(aeio::Handle::from_raw(
            call_suite_fn_single!(self, AEGP_SetInSpecOptionsHandle -> *mut std::ffi::c_void, in_spec_handle.as_ptr(), options.as_ptr() as *mut _)? as _
        ))
    }

    /// Retrieves the file path for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_file_path(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetInSpecFilePath -> ae_sys::AEGP_MemHandle, in_spec_handle.as_ptr())?;
        Ok(unsafe {
            U16CString::from_ptr_str(
                aegp::MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr(),
            ).to_string_lossy()
        })
    }

    /// Retrieves the frame rate of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_native_fps(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecNativeFPS -> ae_sys::A_Fixed, in_spec_handle.as_ptr())? as i32)
    }

    /// Sets the frame rate of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_native_fps(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, native_fps: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecNativeFPS, in_spec_handle.as_ptr(), native_fps as _)
    }

    /// Retrieves the bit depth of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_depth(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<i16, Error> {
        call_suite_fn_single!(self, AEGP_GetInSpecDepth -> i16, in_spec_handle.as_ptr())
    }

    /// Indicates to After Effects the bit depth of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_depth(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, depth: i16) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecDepth, in_spec_handle.as_ptr(), depth)
    }

    /// Retrieves the size (in bytes) of the data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_size(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<u64, Error> {
        call_suite_fn_single!(self, AEGP_GetInSpecSize -> u64, in_spec_handle.as_ptr())
    }

    /// Indicates to After Effects the size (in bytes) of the data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_size(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, size: u64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecSize, in_spec_handle.as_ptr(), size)
    }

    /// Retrieves field information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_interlace_label(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<ae_sys::FIEL_Label, Error> {
        call_suite_fn_single!(self, AEGP_GetInSpecInterlaceLabel -> ae_sys::FIEL_Label, in_spec_handle.as_ptr())
    }

    /// Specifies field information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_interlace_label(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, interlace_label: &ae_sys::FIEL_Label) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecInterlaceLabel, in_spec_handle.as_ptr(), interlace_label)
    }

    /// Retrieves alpha channel interpretation information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_alpha_label(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<ae_sys::AEIO_AlphaLabel, Error> {
        call_suite_fn_single!(self, AEGP_GetInSpecAlphaLabel -> ae_sys::AEIO_AlphaLabel, in_spec_handle.as_ptr())
    }

    /// Sets alpha channel interpretation information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_alpha_label(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, alpha_label: &ae_sys::AEIO_AlphaLabel) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecAlphaLabel, in_spec_handle.as_ptr(), alpha_label)
    }

    /// Retrieves the duration of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_duration(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecDuration -> ae_sys::A_Time, in_spec_handle.as_ptr())?.into())
    }

    /// Sets the duration of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    ///
    /// NOTE: As of 5.5, this must be called, even for frame-based file formats.
    /// If you don't set the `A_Time.scale` to something other than zero, your file(s) will not import.
    ///
    /// This will be fixed in future versions.
    pub fn set_in_spec_duration(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, duration: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecDuration, in_spec_handle.as_ptr(), &duration.into() as *const _)
    }

    /// Retrieves the width and height of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_dimensions(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<(i32, i32), Error> {
        let (width, height) = call_suite_fn_double!(self, AEGP_GetInSpecDimensions -> ae_sys::A_long, ae_sys::A_long, in_spec_handle.as_ptr())?;
        Ok((
            width as i32,
            height as i32
        ))
    }

    /// Indicates to After Effects the width and height of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_dimensions(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, width: i32, height: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecDimensions, in_spec_handle.as_ptr(), width, height)
    }

    /// Retrieves the width, height, bounding rect, and scaling factor applied to an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_get_rational_dimensions(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<(ae_sys::AEIO_RationalScale, i32, i32, Rect), Error> {
        let mut rs: ae_sys::AEIO_RationalScale = unsafe { std::mem::zeroed() };
        let mut width = 0;
        let mut height = 0;
        let mut rect: ae_sys::A_Rect = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, AEGP_InSpecGetRationalDimensions, in_spec_handle.as_ptr(), &mut rs, &mut width, &mut height, &mut rect)?;
        Ok((rs, width, height, rect.into()))
    }

    /// Retrieves the horizontal scaling factor applied to an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_hsf(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<Ratio, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecHSF -> ae_sys::A_Ratio, in_spec_handle.as_ptr())?.into())
    }

    /// Sets the horizontal scaling factor of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_hsf(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, hsf: Ratio) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecHSF, in_spec_handle.as_ptr(), &hsf.into() as *const _)
    }

    /// Obtains the sampling rate (in samples per second) for the audio data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_sound_rate(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetInSpecSoundRate -> f64, in_spec_handle.as_ptr())
    }

    /// Sets the sampling rate (in samples per second) for the audio data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_sound_rate(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, rate: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecSoundRate, in_spec_handle.as_ptr(), rate)
    }

    /// Obtains the encoding method (signed PCM, unsigned PCM, or floating point) from an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_sound_encoding(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<aeio::SoundEncoding, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecSoundEncoding -> ae_sys::AEIO_SndEncoding, in_spec_handle.as_ptr())?.into())
    }

    /// Sets the encoding method of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_sound_encoding(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, encoding: aeio::SoundEncoding) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecSoundEncoding, in_spec_handle.as_ptr(), encoding.into())
    }

    /// Retrieves the bytes-per-sample (1,2, or 4) from an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn in_spec_sound_sample_size(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<aeio::SoundSampleSize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecSoundSampleSize -> ae_sys::AEIO_SndSampleSize, in_spec_handle.as_ptr())?.into())
    }

    /// Set the bytes per sample of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    pub fn set_in_spec_sound_sample_size(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, bytes_per_sample: aeio::SoundSampleSize) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecSoundSampleSize, in_spec_handle.as_ptr(), bytes_per_sample.into())
    }

    /// Determines whether the audio in the [`aeio::SoundChannels`] is mono or stereo.
    pub fn in_spec_sound_channels(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<aeio::SoundChannels, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecSoundChannels -> ae_sys::AEIO_SndChannels, in_spec_handle.as_ptr())?.into())
    }

    /// Sets the audio in an [`aeio::SoundChannels`] to mono or stereo.
    pub fn set_in_spec_sound_channels(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, num_channels: aeio::SoundChannels) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecSoundChannels, in_spec_handle.as_ptr(), num_channels.into())
    }

    /// If your file format has auxiliary files which you want to prevent users from opening directly,
    /// pass it's extension, file type and creator to this function to keep it from appearing in input dialogs.
    pub fn add_aux_ext_map(&self, extension: &str, file_type: aeio::FileType, creator: i32) -> Result<(), Error> {
        let extension = std::ffi::CString::new(extension).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_AddAuxExtMap, extension.as_ptr(), file_type as _, creator)
    }

    /// In case of RGB data, if there is an embedded icc profile, build an `AEGP_ColorProfile` out of
    /// this icc profile using [`suites::ColorSettings::new_color_profile_from_icc_profile`](aegp::suites::ColorSettings::new_color_profile_from_icc_profile)
    /// and set the profile description set to NULL.
    ///
    /// In case of non-RGB data, if there is an embedded non-RGB icc profile or you know the color space the data is in,
    /// set the color profile set to NULL, and provide the description as a NULL-terminated unicode string.
    /// Doing this disables color management UI that allows user to affect profile choice in the application UI.
    /// If you are unpacking non-RGB data directly into working space (to get working space use [`suites::ColorSettings::new_working_space_color_profile`](aegp::suites::ColorSettings::new_working_space_color_profile), you are done.
    /// If you are unpacking non-RGB data into specific RGB color space, you must pass the profile describing this space to [`set_in_spec_assigned_color_profile()`](Self::set_in_spec_assigned_color_profile) below.
    /// Otherwise, your RGB data will be incorrectly interpreted as being in working space.
    /// Either color profile or profile description should be NULL in this function. You cannot use both.
    pub fn set_in_spec_embedded_color_profile(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, color_profile: Option<ae_sys::AEGP_ConstColorProfileP>, profile_desc: Option<&str>) -> Result<(), Error> {
        let profile_desc = if let Some(profile_desc) = profile_desc {
            Some(widestring::U16CString::from_str(profile_desc).map_err(|_| Error::InvalidParms)?)
        } else {
            None
        };

        call_suite_fn!(self, AEGP_SetInSpecEmbeddedColorProfile, in_spec_handle.as_ptr(), color_profile.unwrap_or(std::ptr::null()), profile_desc.as_ref().map_or(std::ptr::null(), |pd| pd.as_ptr()))
    }

    /// Assign a valid RGB color profile to the footage.
    pub fn set_in_spec_assigned_color_profile(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, color_profile: ae_sys::AEGP_ConstColorProfileP) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecAssignedColorProfile, in_spec_handle.as_ptr(), color_profile)
    }

    /// New in CC. Retrieves the native start time of the footage.
    pub fn in_spec_native_start_time(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecNativeStartTime -> ae_sys::A_Time, in_spec_handle.as_ptr())?.into())
    }

    /// New in CC. Assign a native start time to the footage.
    pub fn set_in_spec_native_start_time(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, start_time: Time) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecNativeStartTime, in_spec_handle.as_ptr(), &start_time.into() as *const _)
    }

    /// New in CC. Clear the native start time of the footage.
    /// Setting the native start time to 0 using [`set_in_spec_native_start_time()`](Self::set_in_spec_native_start_time) doesn't do this.
    /// It still means there is a special native start time provided.
    pub fn clear_in_spec_native_start_time(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ClearInSpecNativeStartTime, in_spec_handle.as_ptr())
    }

    /// New in CC. Retrieve the drop-frame setting of the footage.
    pub fn in_spec_native_display_drop_frame(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInSpecNativeDisplayDropFrame -> ae_sys::A_Boolean, in_spec_handle.as_ptr())? != 0)
    }

    /// New in CC. Assign the drop-frame setting of the footage.
    pub fn set_in_spec_native_display_drop_frame(&self, in_spec_handle: impl AsPtr<AEIO_InSpecH>, drop_frame: bool) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetInSpecNativeDisplayDropFrame, in_spec_handle.as_ptr(), drop_frame as _)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_suite_item_wrapper!(
    ae_sys::AEIO_InSpecH, aeio::InSpecHandle,
    suite: IOInSuite,
    /// This struct manages an input specification, After Effects' internal representation of data gathered from any source.
    ///
    /// Any image or audio data in After Effects (except solids) is obtained from an input specification handle, or [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
    InputSpecification {
        dispose: ;

        /// Retrieves the options data (created by your AEIO) for the given [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        options_handle() -> aeio::Handle => suite.in_spec_options_handle,

        /// Sets the options data for the given [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        ///
        /// Must be allocated using the [`suites::Memory`](aegp::suites::Memory).
        ///
        /// Returns the old options handle.
        set_options_handle(options: &aeio::Handle) -> aeio::Handle => suite.set_in_spec_options_handle,

        /// Retrieves the file path for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        file_path() -> String => suite.in_spec_file_path,

        /// Retrieves the frame rate of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        native_fps() -> i32 => suite.in_spec_native_fps,

        /// Sets the frame rate of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_native_fps(native_fps: i32) -> () => suite.set_in_spec_native_fps,

        /// Retrieves the bit depth of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        depth() -> i16 => suite.in_spec_depth,

        /// Indicates to After Effects the bit depth of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_depth(depth: i16) -> () => suite.set_in_spec_depth,

        /// Retrieves the size (in bytes) of the data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        size() -> u64 => suite.in_spec_size,

        /// Indicates to After Effects the size (in bytes) of the data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_size(size: u64) -> () => suite.set_in_spec_size,

        /// Retrieves field information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        interlace_label() -> ae_sys::FIEL_Label => suite.in_spec_interlace_label,

        /// Specifies field information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_interlace_label(interlace_label: &ae_sys::FIEL_Label) -> () => suite.set_in_spec_interlace_label,

        /// Retrieves alpha channel interpretation information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        alpha_label() -> ae_sys::AEIO_AlphaLabel => suite.in_spec_alpha_label,

        /// Sets alpha channel interpretation information for the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_alpha_label(alpha_label: &ae_sys::AEIO_AlphaLabel) -> () => suite.set_in_spec_alpha_label,

        /// Retrieves the duration of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        duration() -> Time => suite.in_spec_duration,

        /// Sets the duration of the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        ///
        /// NOTE: As of 5.5, this must be called, even for frame-based file formats.
        /// If you don't set the `A_Time.scale` to something other than zero, your file(s) will not import.
        ///
        /// This will be fixed in future versions.
        set_duration(duration: Time) -> () => suite.set_in_spec_duration,

        /// Retrieves the width and height of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        dimensions() -> (i32, i32) => suite.in_spec_dimensions,

        /// Indicates to After Effects the width and height of the image data in the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_dimensions(width: i32, height: i32) -> () => suite.set_in_spec_dimensions,

        /// Retrieves the width, height, bounding rect, and scaling factor applied to an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        get_rational_dimensions() -> (ae_sys::AEIO_RationalScale, i32, i32, Rect) => suite.in_spec_get_rational_dimensions,

        /// Retrieves the horizontal scaling factor applied to an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        hsf() -> Ratio => suite.in_spec_hsf,

        /// Sets the horizontal scaling factor of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_hsf(hsf: Ratio) -> () => suite.set_in_spec_hsf,

        /// Obtains the sampling rate (in samples per second) for the audio data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        sound_rate() -> f64 => suite.in_spec_sound_rate,

        /// Sets the sampling rate (in samples per second) for the audio data referenced by the [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_sound_rate(rate: f64) -> () => suite.set_in_spec_sound_rate,

        /// Obtains the encoding method (signed PCM, unsigned PCM, or floating point) from an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        sound_encoding() -> aeio::SoundEncoding => suite.in_spec_sound_encoding,

        /// Sets the encoding method of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_sound_encoding(encoding: aeio::SoundEncoding) -> () => suite.set_in_spec_sound_encoding,

        /// Retrieves the bytes-per-sample (1,2, or 4) from an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        sound_sample_size() -> aeio::SoundSampleSize => suite.in_spec_sound_sample_size,

        /// Set the bytes per sample of an [`aeio::InSpecHandle`](crate::aeio::InSpecHandle).
        set_sound_sample_size(bytes_per_sample: aeio::SoundSampleSize) -> () => suite.set_in_spec_sound_sample_size,

        /// Determines whether the audio in the [`aeio::SoundChannels`] is mono or stereo.
        sound_channels() -> aeio::SoundChannels => suite.in_spec_sound_channels,

        /// Sets the audio in an [`aeio::SoundChannels`] to mono or stereo.
        set_sound_channels(num_channels: aeio::SoundChannels) -> () => suite.set_in_spec_sound_channels,

        /// In case of RGB data, if there is an embedded icc profile, build an `AEGP_ColorProfile` out of
        /// this icc profile using [`suites::ColorSettings::new_color_profile_from_icc_profile`](aegp::suites::ColorSettings::new_color_profile_from_icc_profile)
        /// and set the profile description set to NULL.
        ///
        /// In case of non-RGB data, if there is an embedded non-RGB icc profile or you know the color space the data is in,
        /// set the color profile set to NULL, and provide the description as a NULL-terminated unicode string.
        /// Doing this disables color management UI that allows user to affect profile choice in the application UI.
        /// If you are unpacking non-RGB data directly into working space (to get working space use [`suites::ColorSettings::new_working_space_color_profile`](aegp::suites::ColorSettings::new_working_space_color_profile), you are done.
        /// If you are unpacking non-RGB data into specific RGB color space, you must pass the profile describing this space to [`set_assigned_color_profile()`](Self::set_assigned_color_profile) below.
        /// Otherwise, your RGB data will be incorrectly interpreted as being in working space.
        /// Either color profile or profile description should be NULL in this function. You cannot use both.
        set_embedded_color_profile(color_profile: Option<ae_sys::AEGP_ConstColorProfileP>, profile_desc: Option<&str>) -> () => suite.set_in_spec_embedded_color_profile,

        /// Assign a valid RGB color profile to the footage.
        set_assigned_color_profile(color_profile: ae_sys::AEGP_ConstColorProfileP) -> () => suite.set_in_spec_assigned_color_profile,

        /// New in CC. Retrieves the native start time of the footage.
        native_start_time() -> Time => suite.in_spec_native_start_time,

        /// New in CC. Assign a native start time to the footage.
        set_native_start_time(start_time: Time) -> () => suite.set_in_spec_native_start_time,

        /// New in CC. Clear the native start time of the footage.
        /// Setting the native start time to 0 using [`set_native_start_time()`](Self::set_native_start_time) doesn't do this.
        /// It still means there is a special native start time provided.
        clear_native_start_time() -> () => suite.clear_in_spec_native_start_time,

        /// New in CC. Retrieve the drop-frame setting of the footage.
        native_display_drop_frame() -> bool => suite.in_spec_native_display_drop_frame,

        /// New in CC. Assign the drop-frame setting of the footage.
        set_native_display_drop_frame(drop_frame: bool) -> () => suite.set_in_spec_native_display_drop_frame,
    }
);

impl InputSpecification {
    /// If your file format has auxiliary files which you want to prevent users from opening directly,
    /// pass it's extension, file type and creator to this function to keep it from appearing in input dialogs.
    pub fn add_aux_ext_map(extension: &str, file_type: aeio::FileType, creator: i32) -> Result<(), Error> {
        IOInSuite::new()?.add_aux_ext_map(extension, file_type, creator)
    }
}
