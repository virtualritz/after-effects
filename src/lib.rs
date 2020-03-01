// FIXME: make ALL the functions below return Result-wrapped values
#![feature(proc_macro_hygiene)]
#![feature(new_uninit)]

//#[macro_use]
//extern crate casey;

//use std::mem; //::MaybeUninit;

use aftereffects_sys as ae_sys;

use num_enum::{IntoPrimitive, UnsafeFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive)]
#[repr(i32)]
pub enum Error {
    Generic = ae_sys::A_Err_GENERIC as i32,
    Struct = ae_sys::A_Err_STRUCT as i32,
    Parameter = ae_sys::A_Err_PARAMETER as i32,
    Alloc = ae_sys::A_Err_ALLOC as i32,
    // Some calls can only be used on UI (Main) or Render threads.
    // Also, calls back to Ae can only be made from the same thread Ae
    // called you on.
    WrongThread = ae_sys::A_Err_WRONG_THREAD as i32,
    // An attempt was made to write to a read only copy of an AE
    // project. Project changes must originate in the UI/Main thread.
    ConstProjectModification =
        ae_sys::A_Err_CONST_PROJECT_MODIFICATION as i32,
    // Acquire suite failed on a required suite.
    MissingSuite = ae_sys::A_Err_MISSING_SUITE as i32,
}

struct AeError(pub i32);

impl From<Result<(), Error>> for AeError {
    fn from(result: Result<(), Error>) -> Self {
        match result {
            Ok(()) => AeError(0),
            Err(e) => AeError(e.into()),
        }
    }
}

impl From<AeError> for i32 {
    fn from(ae_error: AeError) -> Self {
        ae_error.0
    }
}

#[macro_use]
pub mod macros;

pub mod aegp;
pub mod pf;
pub mod pr;

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Time {
    value: ae_sys::A_long,
    scale: ae_sys::A_u_long,
}

impl From<Time> for ae_sys::A_Time {
    fn from(time: Time) -> Self {
        Self {
            value: time.value,
            scale: time.scale,
        }
    }
}

// This is confusing: for some structs, Ae expects the caller to
// manage the memory and for others it doesn't (the caller only
// deals with a pointer that gets dereferenced for actually
// calling into the suite). In this case the struct ends
// with a 'H' (for handle).
// When the struct misses the trailing 'H', Ae does expect us to
// manage the memory. We then use a Box<T>.

pub struct PicaBasicSuiteHandle {
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
}

impl PicaBasicSuiteHandle {
    pub fn from_raw(
        pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    ) -> PicaBasicSuiteHandle {
        /*if pica_basic_suite_ptr == ptr::null() {
            panic!()
        }*/
        PicaBasicSuiteHandle {
            pica_basic_suite_ptr: pica_basic_suite_ptr,
        }
    }

    pub fn as_ptr(&self) -> *const ae_sys::SPBasicSuite {
        self.pica_basic_suite_ptr
    }
}

pub trait Suite: Drop {
    fn new(pica_basic_suite: &crate::PicaBasicSuiteHandle) -> Self;

    fn from_raw(
        pica_basic_suite_raw_ptr: *const crate::ae_sys::SPBasicSuite,
    ) -> Self;
}
