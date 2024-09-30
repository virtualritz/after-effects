
use crate::*;
use pr_sys::*;

define_suite!(
    /// Premiere Pro loads all the plug-ins, reads the PiPL, and sends `Command::GlobalSetup` to determine the plug-ins' capabilities.
    /// To save time on future application launches, it saves some of these capabilities in what we call the plug-in cache (the registry on Windows, a Property List file on macOS).
    /// The next time the application is launched, the cached information is used wherever possible, rather than loading the plug-ins.
    ///
    /// If your effect needs to be reloaded each time, there is a way to disable this caching.
    ///
    /// The plug-in can use the PF Cache On Load Suite to call [`set_no_cache_on_load()`](Self::set_no_cache_on_load) during `Command::GlobalSetup`.
    CacheOnLoadSuite,
    PF_CacheOnLoadSuite1,
    kPFCacheOnLoadSuite,
    kPFCacheOnLoadSuiteVersion1
);

impl CacheOnLoadSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Pass a non-zero value if you want your effect to show up in the UI.
    /// Pass zero if loading failed, but you still want Premiere Pro to attempt to load it again on the next relaunch.
    pub fn set_no_cache_on_load(&self, effect_ref: impl AsPtr<PF_ProgPtr>, effect_available: bool) -> Result<(), Error> {
        call_suite_fn!(self, PF_SetNoCacheOnLoad, effect_ref.as_ptr(), effect_available as _)
    }
}
