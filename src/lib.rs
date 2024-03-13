// FIXME: make ALL the functions below return Result-wrapped values
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
use num_traits::identities::Zero;
use std::{
    cell::RefCell,
    cmp::{max, min, PartialEq, PartialOrd},
    error,
    fmt::Display,
    ops::{Add, RemAssign},
    ptr,
};
#[cfg(feature = "ultraviolet")]
use ultraviolet as uv;

#[macro_use]
mod macros;

#[macro_use]
mod plugin_base;

#[macro_use]
mod cross_thread_type;

pub mod aegp;
pub mod aeio;
pub mod drawbot;
pub mod pf;
pub use pf::*;
pub mod pr;
pub mod pr_string;
use pr_string::*;

// re-exports
pub use after_effects_sys as sys;
pub use log;
pub use cstr_literal;
pub use fastrand;
pub use parking_lot;
pub use paste;
pub use serde;
#[cfg(windows)]
pub use win_dbg_logger;
#[cfg(macos)]
pub use oslog;

thread_local!(
    pub(crate) static PICA_BASIC_SUITE: RefCell<*const ae_sys::SPBasicSuite> = RefCell::new(ptr::null_mut())
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

define_enum! {
    ae_sys::PF_Err,
    Error {
        Generic                  = ae_sys::A_Err_GENERIC,
        Struct                   = ae_sys::A_Err_STRUCT,
        Parameter                = ae_sys::A_Err_PARAMETER,
        // Also called A_Err_ALLOC in Ae.
        OutOfMemory              = ae_sys::A_Err_ALLOC,
        // Some calls can only be used on UI (Main) or Render threads.
        // Also, calls back to Ae can only be made from the same thread Ae called you on.
        WrongThread              = ae_sys::A_Err_WRONG_THREAD,
        // An attempt was made to write to a read only copy of an Ae project.
        // Project changes must originate in the UI/Main thread.
        ConstProjectModification = ae_sys::A_Err_CONST_PROJECT_MODIFICATION,
        // Acquire suite failed on a required suite.
        MissingSuite             = ae_sys::A_Err_MISSING_SUITE,
        InternalStructDamaged    = ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED,
        // Out of range, or action not allowed on this index.
        InvalidIndex             = ae_sys::PF_Err_INVALID_INDEX,
        UnrecogizedParameterType = ae_sys::PF_Err_UNRECOGNIZED_PARAM_TYPE,
        InvalidCallback          = ae_sys::PF_Err_INVALID_CALLBACK,
        BadCallbackParameter     = ae_sys::PF_Err_BAD_CALLBACK_PARAM,
        // Returned when user interrupts rendering.
        InterruptCancel          = ae_sys::PF_Interrupt_CANCEL,
        // Returned from PF_Arbitrary_SCAN_FUNC when effect cannot parse arbitrary data from text
        CannotParseKeyframeText  = ae_sys::PF_Err_CANNOT_PARSE_KEYFRAME_TEXT,

        Reserved11               = ae_sys::A_Err_RESERVED_11,

        StringNotFound           = ae_sys::suiteError_StringNotFound,
        StringBufferTooSmall     = ae_sys::suiteError_StringBufferTooSmall,
        InvalidParms             = ae_sys::suiteError_InvalidParms,

        None = ae_sys::PF_Err_NONE,
    }
}

impl From<Error> for &'static str {
    fn from(error: Error) -> &'static str {
        match error {
            Error::Generic                  => "Generic error.",
            Error::Struct                   => "Wrong struct.",
            Error::Parameter                => "Wrong parameter.",
            Error::OutOfMemory              => "Out of memory.",
            Error::WrongThread              => "Call made from wrong thread.",
            Error::ConstProjectModification => "Project changes must originate in the UI/Main thread.",
            Error::MissingSuite             => "Could no aquire suite.",
            Error::InternalStructDamaged    => "Internal struct is damaged.",
            Error::InvalidIndex             => "Out of range, or action not allowed on this index.",
            Error::UnrecogizedParameterType => "Unrecognized parameter type",
            Error::InvalidCallback          => "Invalid callback.",
            Error::BadCallbackParameter     => "Bad callback parameter.",
            Error::InterruptCancel          => "Rendering interrupted.",
            Error::CannotParseKeyframeText  => "Keyframe data damaged.",
            Error::None                     => "No error",
            Error::StringNotFound           => "StringNotFound",
            Error::StringBufferTooSmall     => "StringBufferTooSmall",
            Error::InvalidParms             => "InvalidParms",
            Error::Reserved11               => "Reserved11",
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Matrix3([[f64; 3]; 3]);
impl From<Matrix3> for ae_sys::A_Matrix3 {
    #[inline]
    fn from(val: Matrix3) -> Self {
        ae_sys::A_Matrix3 {
            mat: val.0
        }
    }
}
impl From<ae_sys::A_Matrix3> for Matrix3 {
    #[inline]
    fn from(item: ae_sys::A_Matrix3) -> Self  {
        Self(item.mat)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Matrix4([[f64; 4]; 4]);

impl From<Matrix4> for ae_sys::A_Matrix4 {
    #[inline]
    fn from(val: Matrix4) -> Self {
        ae_sys::A_Matrix4 {
            mat: val.0
        }
    }
}
impl From<ae_sys::A_Matrix4> for Matrix4 {
    #[inline]
    fn from(item: ae_sys::A_Matrix4) -> Self  {
        Self(item.mat)
    }
}

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

define_struct! {
    ae_sys::A_Time,
    #[derive(Eq)]
    Time {
        value: i32,
        scale: u32,
    }
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

define_struct! {
    ae_sys::A_LRect,
    #[derive(Eq)]
    /// Note that this has a different ordering of values than LegacyRect!
    Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }
}
define_struct_conv!(ae_sys::A_LegacyRect, Rect { left, top, right, bottom });
define_struct_conv!(ae_sys::PF_LRect,     Rect { left, top, right, bottom });

impl Rect {
    pub fn empty() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.left >= self.right) || (self.top >= self.bottom)
    }
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
    pub fn origin(&self) -> Point {
        Point {
            h: self.left,
            v: self.top,
        }
    }

    pub fn union<'a>(&'a mut self, other: &Rect) -> &'a mut Rect {
        if self.is_empty() {
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

define_struct! {
    ae_sys::A_FloatPoint,
    FloatPoint {
        x: f64,
        y: f64,
    }
}

define_struct! {
    ae_sys::A_FloatRect,
    FloatRect {
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    }
}

impl FloatRect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        (self.left <= x) && (x <= self.right) && (self.top <= y) && (y <= self.bottom)
    }
}

define_struct! {
    ae_sys::A_Ratio,
    Ratio {
        num: i32,
        den: u32,
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

pub enum Ownership<'a, T: Clone> {
    AfterEffects(&'a T),
    AfterEffectsMut(&'a mut T),
    Rust(T),
}
impl<'a, T: Clone> Clone for Ownership<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::AfterEffects(ptr)    => Self::Rust((*ptr).clone()),
            Self::AfterEffectsMut(ptr) => Self::Rust((*ptr).clone()),
            Self::Rust(ptr)            => Self::Rust(ptr.clone()),
        }
    }
}
impl<'a, T: Clone> std::ops::Deref for Ownership<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::AfterEffects(ptr) => ptr,
            Self::AfterEffectsMut(ptr) => ptr,
            Self::Rust(ptr) => ptr,
        }
    }
}
impl<'a, T: Clone> std::ops::DerefMut for Ownership<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::AfterEffects(_) => panic!("Tried to mutably borrow immutable data"),
            Self::AfterEffectsMut(ptr) => ptr,
            Self::Rust(ptr) => ptr,
        }
    }
}

pub enum PointerOwnership<T> {
    AfterEffects(*mut T),
    Rust(T),
}
impl<T> std::ops::Deref for PointerOwnership<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::AfterEffects(ptr) => unsafe { &**ptr },
            Self::Rust(ptr) => ptr,
        }
    }
}
impl<T> std::ops::DerefMut for PointerOwnership<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::AfterEffects(ptr) => unsafe { &mut **ptr },
            Self::Rust(ptr) => ptr,
        }
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
}

pub trait AsPtr<T> {
    fn as_ptr(&self) -> T
    where
        T: Sized;
}

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> T
    where T: Sized;
}
