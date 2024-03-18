use crate::*;
use pr_sys::*;
use std::ffi::CString;

define_suite!(
    PrStringSuite,
    PrSDKStringSuite,
    kPrSDKStringSuite,
    kPrSDKStringSuiteVersion
);

impl PrStringSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// This will dispose of an SDKString. It is OK to pass in an empty string.
    /// * `sdk_string` - the string to dispose of
    ///
    /// # Errors
    ///
    /// * `Error::StringNotFound` - this string has not been allocated, or may have already been disposed
    /// * `Error::InvalidParms` - one of the params is invalid
    pub fn dispose_string(&self, sdk_string: *const PrSDKString) -> Result<(), Error> {
        call_suite_fn!(self, DisposeString, sdk_string)
    }

    /// This will allocate an SDKString from a passed in null terminated string.
    /// * `string` - UTF8 string to copy into the SDK string
    ///
    /// Returns the allocated `PrSDKString` which must be disposed using [`dispose_string()`](Self::dispose_string)
    ///
    /// # Errors
    ///
    /// * `Error::StringNotFound` - this string has not been allocated, or may have already been disposed
    /// * `Error::InvalidParms` - one of the params is invalid
    pub fn allocate_from_utf8(&self, string: &str) -> Result<PrSDKString, Error> {
        let mut out_sdk_string = unsafe { std::mem::zeroed() };
        let in_string = CString::new(string).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AllocateFromUTF8, in_string.as_bytes_with_nul().as_ptr(), &mut out_sdk_string)?;
        Ok(out_sdk_string)
    }

    /// This will copy an PrSDKString into a Rust's String
    ///
    /// # Errors
    ///
    /// * `Error::InvalidParms` - one of the params is invalid
    pub fn copy_to_utf8_string(&self, sdk_string: *const PrSDKString) -> Result<String, Error> {
        let mut buffer = vec![0u8; 128];
        let mut buffer_size = buffer.len() as u32;

        let mut result = call_suite_fn!(self, CopyToUTF8String, sdk_string, buffer.as_mut_ptr() as *mut _, &mut buffer_size);
        if result == Err(Error::StringBufferTooSmall) {
            buffer = vec![0u8; buffer_size as usize + 1];
            result = call_suite_fn!(self, CopyToUTF8String, sdk_string, buffer.as_mut_ptr() as *mut _, &mut buffer_size);
        }
        match result {
            Ok(()) => {
                buffer.resize(buffer_size as usize - 1, 0u8);
                String::from_utf8(buffer).map_err(|_| Error::InvalidParms)
            }
            Err(e) => Err(Error::from(e))
        }
    }
}

#[repr(transparent)]
pub struct PrString(pub PrSDKString);
impl From<&str> for PrString {
    fn from(s: &str) -> Self {
        Self(PrStringSuite::new().unwrap().allocate_from_utf8(s).unwrap())
    }
}
impl From<PrString> for String {
    fn from(s: PrString) -> Self {
        PrStringSuite::new().unwrap().copy_to_utf8_string(&s.0).unwrap()
    }
}
impl Drop for PrString {
    fn drop(&mut self) {
        PrStringSuite::new().unwrap().dispose_string(&self.0).unwrap();
    }
}
