use crate::*;

define_suite!(
    /// Time overview:
    ///
    /// Premiere uses a tick-based time counter that is stored in a signed 64-bit integer.
    /// The current number of ticks per second must be retrieved using the callback in this suite,
    /// but is guaranteed to be constant for the duration of runtime.
    TimeSuite,
    PrSDKTimeSuite,
    kPrSDKTimeSuite,
    kPrSDKTimeSuiteVersion
);

impl TimeSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Get the current ticks per second. This is guaranteed to be constant for the duration of runtime.
    ///
    /// Returns the number of time ticks per second.
    pub fn get_ticks_per_second(&self) -> Result<pr_sys::PrTime, Error> {
        let mut val = 0;
        call_suite_fn!(self, GetTicksPerSecond, &mut val)?;
        Ok(val)
    }

    /// Get the number of ticks in a video frame rate.
    /// * `frame_rate` - an enum value for a video frame rate.
    ///
    /// Returns the number of time ticks per frame.
    pub fn get_ticks_per_video_frame(&self, frame_rate: crate::VideoFrameRates) -> Result<pr_sys::PrTime, Error> {
        call_suite_fn_single!(self, GetTicksPerVideoFrame -> pr_sys::PrTime, frame_rate.into())
    }

    /// Get the number of ticks in an audio sample rate.
    /// * `sample_rate` - the audio sample rate as a float.
    ///
    /// Returns the number of time ticks per sample.
    ///
    /// Returns `kPrTimeSuite_RoundedAudioRate` if the requested audio sample rate is not an
    /// even divisor of the base tick count and therefore times in this rate will not be exact.
    pub fn get_ticks_per_audio_sample(&self, sample_rate: f32) -> Result<pr_sys::PrTime, Error> {
        call_suite_fn_single!(self, GetTicksPerAudioSample -> pr_sys::PrTime, sample_rate)
    }
}
