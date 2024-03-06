use crate::*;
use ae_sys::*;

define_suite!(
    /// [ `SoundDataSuite`] allows AEGPs to obtain and manipulate the audio associated with compositions and footage items.
    ///
    /// Audio-only items may be added to the render queue using [`aegp::suites::Render::render_new_item_sound_data()`].
    SoundDataSuite,
    AEGP_SoundDataSuite1,
    kAEGPSoundDataSuite,
    kAEGPSoundDataVersion1
);

impl SoundDataSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Creates a new [`SoundDataHandle`]. It's disposed on drop.
    pub fn new_sound_data(&self, sound_format: &AEGP_SoundDataFormat) -> Result<SoundDataHandle, Error> {
        Ok(SoundDataHandle::from_raw_owned(
            call_suite_fn_single!(self, AEGP_NewSoundData -> AEGP_SoundDataH, sound_format)?,
        ))
    }

    /// Frees an [`SoundDataHandle`].
    pub fn dispose_sound_data(&self, sound_data: impl AsPtr<AEGP_SoundDataH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeSoundData, sound_data.as_ptr())
    }

    /// Obtains information about the format of a given [`SoundDataHandle`].
    pub fn get_sound_data_format(&self, sound_data: impl AsPtr<AEGP_SoundDataH>) -> Result<AEGP_SoundDataFormat, Error> {
        call_suite_fn_single!(self, AEGP_GetSoundDataFormat -> AEGP_SoundDataFormat, sound_data.as_ptr())
    }

    /// Locks the [`SoundDataHandle`] in memory
    pub fn lock_sound_data_samples(&self, sound_data: impl AsPtr<AEGP_SoundDataH>) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, AEGP_LockSoundDataSamples -> *mut std::ffi::c_void, sound_data.as_ptr())
    }

    /// Unlocks an [`SoundDataHandle`].
    pub fn unlock_sound_data_samples(&self, sound_data: impl AsPtr<AEGP_SoundDataH>) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_UnlockSoundDataSamples, sound_data.as_ptr())
    }

    /// Obtains the number of samples in the given [`SoundDataHandle`].
    pub fn get_num_samples(&self, sound_data: impl AsPtr<AEGP_SoundDataH>) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumSamples -> A_long, sound_data.as_ptr())?)
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

register_handle!(AEGP_SoundDataH);
define_owned_handle_wrapper!(SoundDataHandle, AEGP_SoundDataH);
impl Drop for SoundDataHandle {
    fn drop(&mut self) {
        if let Ok(suite) = SoundDataSuite::new() {
            suite.dispose_sound_data(self.as_ptr()).unwrap();
        }
    }
}
