use crate::*;
use pr_sys::*;

define_suite!(
    WindowSuite,
    PrSDKWindowSuite,
    kPrSDKWindowSuite,
    kPrSDKWindowSuiteVersion
);

impl WindowSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns a handle to the main application window-- a `HWND` on Windows and a `*mut NSView` on macOS.
    /// These correspond to the `Win32WindowHandle` and `AppKitWindowHandle` types in the `raw-window-handle` crate.
    pub fn main_window(&self) -> prWnd {
        call_suite_fn_no_err!(self, GetMainWindow, )
    }

    /// Returns a handle to the main application window-- a `HWND` on Windows and a `*mut NSView` on macOS.
    /// These correspond to the `Win32WindowHandle` and `AppKitWindowHandle` types in the `raw-window-handle` crate.
    #[deprecated(since = "0.5.0", note = "renamed to `main_window`")]
    pub fn get_main_window(&self) -> prWnd {
        self.main_window()
    }

    /// Updates all windows. Windows only, doesn’t work on Mac OS.
    pub fn update_all_windows(&self) {
        call_suite_fn_no_err!(self, UpdateAllWindows, )
    }
}
