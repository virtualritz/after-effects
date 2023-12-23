// FIXME: make ALL the functions below return Result-wrapped values
#![allow(temporary_cstring_as_ptr)]
//! High(er) level bindings for the Adobe AfterEffects® (Ae) SDK.
//!
//! This wraps many of the API suites in the Ae SDK and exposes them in safe
//! Rust.
//!
//! This is WIP – only tested on `macOS`. Will likely require additional work to
//! build on `Windows`.
//!
//! ## Prequisites
//!
//! Download the [*Adobe After Effects SDK*](https://console.adobe.io/downloads/ae).
//! > ⚠️ The SDK published by Adobe is outdated if you are using the 3D
//! > Artisan API to write your own 3D renderer plug-in.
//! > Also see [Features](#features) below for more information.
//! >
//! > Ignore this if you just want to develop 2D plugins (which still have
//! > access to 3D data).
//!
//! Define the `AESDK_ROOT` environment variable that contains the path to your
//! Ae SDK. Typically the directory structure will look like this:
//!
//! ```text
//! AfterEffectsSDK
//! ├── After_Effects_SDK_Guide.pdf
//! ├── Examples
//!     ├── AEGP
//!     ├── Effect
//!     ├── ...
//! ```
//! ## Features
//!
//! * `artisan-2-api` – Use the 2nd generation Artisan 3D API. This is not
//!   included in the official Ae SDK. Specifically it requires:
//!   * `AE_Scene3D_Private.h`
//!   * `PR_Feature.h`
//!
//!   Contact the Adobe Ae SDK team and ask nicely and they may send you
//!   theses headers.
//!
//! ## Using
//!
//! Add `after-effects` to your dependencies.
//!
//! ```bash
//! cargo add after-effects
//! ```
//!
//! ## Getting Started
//!
//! There are currently no examples. Use the C/C++ examples in the Ae SDK as
//! guides for now. They translate more or less 1:1 to Rust when using this
//! crate.
//!
//! ## Help Wanted/To Do
//!
//! * Examples! I have a few plug-ins but they need polishing.
//!
//! * A build system extension that creates the bundle for Ae using
//!   `cargo-post`/`cargo-make`/`cargo-bundle`. I.e. enter one command to get a
//!   plug-in ready to load in Ae. Currently there are manual steps and they
//!   need documenting too.
//!
//! * Better error handling. Possibly using [`color`](https://crates.io/crates/color-eyre)`-`[`eyre`](https://crates.io/crates/eyre)?
use after_effects_sys as ae_sys;
use num_enum::UnsafeFromPrimitive;
use num_traits::identities::Zero;
use std::{
    cell::RefCell,
    cmp::{max, min, PartialEq, PartialOrd},
    convert::TryInto,
    error,
    fmt::Display,
    ops::{Add, RemAssign},
    ptr,
};
#[cfg(feature = "ultraviolet")]
use ultraviolet as uv;

#[cfg(target_os = "macos")]
pub type EnumIntType = u32;
#[cfg(target_os = "windows")]
pub type EnumIntType = i32;

#[macro_use]
mod macros;

pub mod aegp;
pub use aegp::*;
pub mod aeio;
pub use aeio::*;
pub mod drawbot;
pub use drawbot::*;
pub mod pf;
pub use pf::*;
pub mod pr;

thread_local!(
    pub(crate) static PICA_BASIC_SUITE: RefCell<*const ae_sys::SPBasicSuite> =
        RefCell::new(ptr::null_mut())
);

#[inline]
pub(crate) fn borrow_pica_basic_as_ptr() -> *const ae_sys::SPBasicSuite {
    let mut pica_basic_ptr: *const ae_sys::SPBasicSuite = ptr::null();

    PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
        pica_basic_ptr = *pica_basic_ptr_cell.borrow();
    });

    pica_basic_ptr
}

/// This lets us access a thread-local version of the `PicaBasic`
/// suite. Whenever we generate a new `SPBasic_Suite` from Ae somehow,
/// we create a PicaBasicSuite::new() from that and use that to initialize
/// access to any suites.
///
/// When we leave scope, `drop()` ic alled automatically and restores the
/// previous value to our thread-local storage so the caller
/// can continue using their pointer to the suite.
///
/// FIXME: Is this really neccessary? Check if the pointer is always the
///        same and if so, confirm with Adobe we can get rid of it.
pub struct PicaBasicSuite {
    previous_pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
}

impl PicaBasicSuite {
    fn set(pica_basic_suite_ptr: *const ae_sys::SPBasicSuite) -> *const ae_sys::SPBasicSuite {
        let mut previous_pica_basic_suite_ptr: *const ae_sys::SPBasicSuite = ptr::null();

        PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
            previous_pica_basic_suite_ptr = pica_basic_ptr_cell.replace(pica_basic_suite_ptr);
        });

        previous_pica_basic_suite_ptr
    }

    #[inline]
    pub fn from_pr_in_data_raw(in_data_ptr: *const ae_sys::PR_InData) -> Self {
        Self {
            previous_pica_basic_suite_ptr: PicaBasicSuite::set(unsafe { *in_data_ptr }.pica_basicP),
        }
    }

    #[inline]
    pub fn from_pr_in_data(in_data_handle: pr::InDataHandle) -> Self {
        Self {
            previous_pica_basic_suite_ptr: PicaBasicSuite::set(
                unsafe { *in_data_handle.as_ptr() }.pica_basicP,
            ),
        }
    }

    #[inline]
    pub fn from_pf_in_data_raw(in_data_ptr: *const ae_sys::PF_InData) -> Self {
        Self {
            previous_pica_basic_suite_ptr: PicaBasicSuite::set(unsafe { *in_data_ptr }.pica_basicP),
        }
    }

    #[inline]
    pub fn from_sp_basic_suite_raw(pica_basic_suite_ptr: *const ae_sys::SPBasicSuite) -> Self {
        Self {
            previous_pica_basic_suite_ptr: PicaBasicSuite::set(pica_basic_suite_ptr),
        }
    }
}

impl Drop for PicaBasicSuite {
    #[inline]
    fn drop(&mut self) {
        PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
            pica_basic_ptr_cell.replace(self.previous_pica_basic_suite_ptr);
        });
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, UnsafeFromPrimitive)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(target_os = "macos", repr(u32))]
pub enum Error {
    Generic = ae_sys::A_Err_GENERIC as EnumIntType,
    Struct = ae_sys::A_Err_STRUCT as EnumIntType,
    Parameter = ae_sys::A_Err_PARAMETER as EnumIntType,
    // Also called A_Err_ALLOC in Ae.
    OutOfMemory = ae_sys::A_Err_ALLOC as EnumIntType,
    // Some calls can only be used on UI (Main) or Render threads.
    // Also, calls back to Ae can only be made from the same thread Ae
    // called you on.
    WrongThread = ae_sys::A_Err_WRONG_THREAD as EnumIntType,
    // An attempt was made to write to a read only copy of an Ae
    // project. Project changes must originate in the UI/Main thread.
    ConstProjectModification = ae_sys::A_Err_CONST_PROJECT_MODIFICATION as EnumIntType,
    // Acquire suite failed on a required suite.
    MissingSuite = ae_sys::A_Err_MISSING_SUITE as EnumIntType,

    InternalStructDamaged = ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED as EnumIntType,
    // Out of range, or action not allowed on this index.
    InvalidIndex = ae_sys::PF_Err_INVALID_INDEX as EnumIntType,
    UnrecogizedParameterType = ae_sys::PF_Err_UNRECOGNIZED_PARAM_TYPE as EnumIntType,
    InvalidCallback = ae_sys::PF_Err_INVALID_CALLBACK as EnumIntType,
    BadCallbackParameter = ae_sys::PF_Err_BAD_CALLBACK_PARAM as EnumIntType,
    // Returned when user interrupts rendering.
    InterruptCancel = ae_sys::PF_Interrupt_CANCEL as EnumIntType,
    // Returned from PF_Arbitrary_SCAN_FUNC when effect cannot parse
    // arbitrary data from text
    CannotParseKeyframeText = ae_sys::PF_Err_CANNOT_PARSE_KEYFRAME_TEXT as EnumIntType,

    None = ae_sys::PF_Err_NONE as EnumIntType,
}
impl From<i32> for Error {
    fn from(value: i32) -> Self {
        unsafe { Self::unchecked_transmute_from(value as EnumIntType) }
    }
}
impl From<u32> for Error {
    fn from(value: u32) -> Self {
        unsafe { Self::unchecked_transmute_from(value as EnumIntType) }
    }
}

impl From<Error> for &'static str {
    fn from(error: Error) -> &'static str {
        match error {
            Error::Generic => "Generic error.",
            Error::Struct => "Wrong struct.",
            Error::Parameter => "Wrong parameter.",
            Error::OutOfMemory => "Out of memory.",
            Error::WrongThread => "Call made from wrong thread.",
            Error::ConstProjectModification => {
                " Project changes must originate in the UI/Main thread."
            }
            Error::MissingSuite => "Could no aquire suite.",
            Error::InternalStructDamaged => "Internal struct is damaged.",
            Error::InvalidIndex => "Out of range, or action not allowed on this index.",
            Error::UnrecogizedParameterType => "Unrecognized parameter type",
            Error::InvalidCallback => "Invalid callback.",
            Error::BadCallbackParameter => "Bad callback parameter.",
            Error::InterruptCancel => "Rendering interrupted.",
            Error::CannotParseKeyframeText => "Keyframe data damaged.",
            Error::None => "No error – wtf?",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err_str: &'static str = (*self).into();
        write!(f, "{}", err_str)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

//FIXME uncomment this once TryReserve() becomes stable in nightly
impl From<std::collections::TryReserveError> for Error {
    fn from(_: std::collections::TryReserveError) -> Self {
        Error::OutOfMemory
    }
}

/*
impl From<Error> for ae_sys::A_Err {
    fn from(err: Error) -> ae_sys::A_Err {
        err as ae_sys::A_Err
    }
}*/

/*
impl From<Result<(), Error>> for AeError {
    #[inline]
    fn from(result: Result<(), Error>) -> Self {
        match result {
            Ok(()) => AeError(0),
            Err(e) => AeError(e.into()),
        }
    }
}

impl From<AeError> for i32 {
    #[inline]
    fn from(ae_error: AeError) -> Self {
        ae_error.0
    }
}*/

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Matrix4([[f64; 4]; 4]);

impl Matrix4 {
    #[inline]
    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as _, 16) }
    }
}

impl From<Matrix4> for [f64; 16] {
    #[inline]
    fn from(m: Matrix4) -> Self {
        // This can not panic.
        m.as_slice().try_into().unwrap()
    }
}

#[cfg(feature = "ultraviolet")]
impl From<Matrix4> for uv::DMat4 {
    #[inline]
    fn from(m: Matrix4) -> Self {
        // Ae is row-based – transpose
        uv::DMat4::from(m.0).transposed()
    }
}

#[cfg(feature = "ultraviolet")]
impl From<uv::DMat4> for Matrix4 {
    #[inline]
    fn from(m: uv::DMat4) -> Self {
        // Ae is row-based – transpose
        Self(m.transposed().into())
    }
}

#[cfg(feature = "ultraviolet")]
#[test]
fn test_from() {
    let m = Matrix4 {
        0: [[0.; 4], [0.; 4], [0.; 4], [0.; 4]],
    };
    let _matrix = uv::mat4D::from(m);
}

#[cfg(feature = "nalgebra")]
impl From<Matrix4> for nalgebra::Matrix4<f64> {
    #[inline]
    fn from(m: Matrix4) -> Self {
        nalgebra::Matrix4::<f64>::from_row_slice(m.as_slice())
    }
}

#[cfg(feature = "nalgebra")]
#[test]
fn test_from() {
    let m = Matrix4 {
        0: [[0.; 4], [0.; 4], [0.; 4], [0.; 4]],
    };
    let _matrix = nalgebra::Matrix4::<f64>::from(m);
}

pub type Color = ae_sys::A_Color;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(C)]
pub struct Time {
    pub value: ae_sys::A_long,
    pub scale: ae_sys::A_u_long,
}

impl From<Time> for f64 {
    #[inline]
    fn from(time: Time) -> Self {
        debug_assert!(time.scale != 0);
        time.value as Self / time.scale as Self
    }
}

impl From<Time> for f32 {
    #[inline]
    fn from(time: Time) -> Self {
        debug_assert!(time.scale != 0);
        time.value as Self / time.scale as Self
    }
}

impl From<Time> for ae_sys::A_Time {
    #[inline]
    fn from(time: Time) -> Self {
        Self {
            value: time.value,
            scale: time.scale,
        }
    }
}

// Euclid's two-thousand-year-old algorithm for finding the
// greatest common divisor. Copied non-recursive version from
// Rust docs.
#[inline]
fn greatest_common_divisor<T>(mut n: T, mut m: T) -> T
where
    T: Copy + PartialEq + PartialOrd + RemAssign + Zero,
{
    debug_assert!(n != Zero::zero() && m != Zero::zero());
    while m != Zero::zero() {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

/// Calculates the wrapped sum of two [`Time`]s. If no common integer
/// demoninator can be found, `None` is returned.
#[inline]
fn add_time_lossless(time1: Time, time2: Time) -> Option<Time> {
    if (time1.scale == 0) || (time2.scale == 0) {
        return None;
    }

    let gcd = {
        if time1.scale == time2.scale {
            time1.scale
        } else {
            greatest_common_divisor(time1.scale, time2.scale)
        }
    };

    let value1 = time1
        .value
        .checked_mul(time2.scale.checked_div(gcd)? as i32)?;
    let value2 = time2
        .value
        .checked_mul(time1.scale.checked_div(gcd)? as i32)?;

    Some(Time {
        value: value1.checked_add(value2)?,
        scale: time2.scale.checked_mul(time1.scale.checked_div(gcd)?)?,
    })
}

/// Calculates the sum of two [`Time`]s using floating point math.
#[inline]
fn add_time_lossy(time1: Time, time2: Time) -> Time {
    let time =
        (time1.value as f64 / time1.scale as f64) + (time2.value as f64 / time2.scale as f64);

    let num_bits = time.log2() as usize;
    let scale: u32 = 1u32 << (30 - num_bits);

    Time {
        value: (time * scale as f64) as i32,
        scale,
    }
}

// Next bit (std::ops::Add) ported from aeutility.cpp
// Rust version is so much shorter & cleaner!
//
/// Calculates the sum of two [`Time`]s, lossless if possible.
// FIXME: is it worth going the lossless route at all???
impl Add<Time> for Time {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        match add_time_lossless(self, rhs) {
            Some(time) => time,
            None => add_time_lossy(self, rhs),
        }
    }
}

/// Note that this has a different ordering
/// of values than [`LegacyRect`]!!!
#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<ae_sys::PF_LRect> for Rect {
    fn from(rect: ae_sys::PF_LRect) -> Self {
        Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl From<Rect> for ae_sys::PF_LRect {
    fn from(rect: Rect) -> Self {
        ae_sys::PF_LRect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl Rect {
    pub fn is_empty(&self) -> bool {
        (self.left >= self.right) || (self.top >= self.bottom)
    }

    pub fn union<'a>(&'a mut self, other: &Rect) -> &'a mut Rect {
        if other.is_empty() {
            *self = *other;
        } else {
            // if !other.is_empty()
            self.left = min(self.left, other.left);
            self.top = min(self.top, other.top);
            self.right = max(self.right, other.right);
            self.bottom = max(self.bottom, other.bottom);
        }
        self
    }

    pub fn is_edge_pixel(&self, x: i32, y: i32) -> bool {
        let mut x_hit = (x == self.left) || (x == self.right);
        let mut y_hit = (y == self.top) || (y == self.bottom);

        if x_hit {
            y_hit = (y >= self.top) && (y <= self.bottom);
        } else if y_hit {
            x_hit = (x >= self.left) && (x <= self.right);
        }
        x_hit && y_hit
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        (self.left <= x) && (x <= self.right) && (self.top <= y) && (y <= self.bottom)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FloatRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl FloatRect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        (self.left <= x) && (x <= self.right) && (self.top <= y) && (y <= self.bottom)
    }
}

/// Note that this has a different ordering
/// of values than [`Rect`]!!!
#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct LegacyRect {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Ratio {
    pub num: ae_sys::A_long,
    pub den: ae_sys::A_u_long,
}

impl From<Ratio> for ae_sys::A_Ratio {
    #[inline]
    fn from(ratio: Ratio) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<Ratio> for f64 {
    #[inline]
    fn from(ratio: Ratio) -> Self {
        debug_assert!(ratio.den != 0);
        ratio.num as Self / ratio.den as Self
    }
}

impl From<Ratio> for f32 {
    #[inline]
    fn from(ratio: Ratio) -> Self {
        debug_assert!(ratio.den != 0);
        ratio.num as Self / ratio.den as Self
    }
}

// This is confusing: for some structs Ae expects the caller to
// manage the memory and for others it doesn't (the caller only
// deals with a pointer that gets dereferenced for actually
// calling into the suite). In this case the struct ends
// with a `H` (for handle).
// When the struct misses the trailing `H`, Ae does expect us to
// manage the memory. We then use a Box<T>.
pub struct PicaBasicSuiteHandle {
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
}

impl PicaBasicSuiteHandle {
    #[inline]
    pub fn from_raw(pica_basic_suite_ptr: *const ae_sys::SPBasicSuite) -> PicaBasicSuiteHandle {
        /*if pica_basic_suite_ptr == ptr::null() {
            panic!()
        }*/
        PicaBasicSuiteHandle {
            pica_basic_suite_ptr,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::SPBasicSuite {
        self.pica_basic_suite_ptr
    }
}

pub(crate) trait Suite {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

    fn from_raw(
        pica_basic_suite_raw_ptr: *const crate::ae_sys::SPBasicSuite,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}
