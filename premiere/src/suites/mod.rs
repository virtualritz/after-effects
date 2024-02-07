use crate::*;

mod gpu_device;           pub use gpu_device::*;
mod gpu_image_processing; pub use gpu_image_processing::*;
mod memory_manager;       pub use memory_manager::*;
mod ppix;                 pub use ppix::*;
mod video_segment;        pub use video_segment::*;

use std::ptr;
use std::cell::RefCell;

thread_local!(
    pub(crate) static PICA_BASIC_SUITE: RefCell<*const pr_sys::SPBasicSuite> =
        RefCell::new(ptr::null_mut())
);

#[inline]
pub(crate) fn borrow_pica_basic_as_ptr() -> *const pr_sys::SPBasicSuite {
    let mut pica_basic_ptr: *const pr_sys::SPBasicSuite = ptr::null();

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
    previous_pica_basic_suite_ptr: *const pr_sys::SPBasicSuite,
}

impl PicaBasicSuite {
    fn set(pica_basic_suite_ptr: *const pr_sys::SPBasicSuite) -> *const pr_sys::SPBasicSuite {
        let mut previous_pica_basic_suite_ptr: *const pr_sys::SPBasicSuite = ptr::null();

        PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
            previous_pica_basic_suite_ptr = pica_basic_ptr_cell.replace(pica_basic_suite_ptr);
        });

        previous_pica_basic_suite_ptr
    }

    #[inline]
    pub fn from_sp_basic_suite_raw(pica_basic_suite_ptr: *const pr_sys::SPBasicSuite) -> Self {
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

// This is confusing: for some structs Ae expects the caller to
// manage the memory and for others it doesn't (the caller only
// deals with a pointer that gets dereferenced for actually
// calling into the suite). In this case the struct ends
// with a `H` (for handle).
// When the struct misses the trailing `H`, Ae does expect us to
// manage the memory. We then use a Box<T>.
pub struct PicaBasicSuiteHandle {
    pica_basic_suite_ptr: *const pr_sys::SPBasicSuite,
}

impl PicaBasicSuiteHandle {
    #[inline]
    pub fn from_raw(pica_basic_suite_ptr: *const pr_sys::SPBasicSuite) -> PicaBasicSuiteHandle {
        /*if pica_basic_suite_ptr == ptr::null() {
            panic!()
        }*/
        PicaBasicSuiteHandle {
            pica_basic_suite_ptr,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const pr_sys::SPBasicSuite {
        self.pica_basic_suite_ptr
    }
}

pub(crate) trait Suite {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

    fn from_raw(
        pica_basic_suite_raw_ptr: *const crate::pr_sys::SPBasicSuite,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}
