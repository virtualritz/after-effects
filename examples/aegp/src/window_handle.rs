use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle, WindowHandle,
};

use after_effects::Error;

pub struct WindowAndDisplayHandle {
    raw_handle: raw_window_handle::Win32WindowHandle,
}

impl WindowAndDisplayHandle {
    /// Safety: The window handle must be valid for however long you intend to use it for
    pub fn try_get_main_handles() -> Result<Self, Error> {
        let suite = after_effects::aegp::suites::Utility::new()?;
        let win = suite.main_hwnd()?;
        let handle = std::num::NonZeroIsize::new(win as usize as isize).ok_or(Error::Generic)?;
        let raw_handle = raw_window_handle::Win32WindowHandle::new(handle);
        let handle = WindowAndDisplayHandle { raw_handle };
        Ok(handle)
    }
}
impl HasWindowHandle for WindowAndDisplayHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(self.raw_handle)) })
    }
}

impl HasDisplayHandle for WindowAndDisplayHandle {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::windows())
    }
}
