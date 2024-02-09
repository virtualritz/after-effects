use crate::*;

mod gpu_device;               pub use gpu_device::*;
mod gpu_image_processing;     pub use gpu_image_processing::*;
mod memory_manager;           pub use memory_manager::*;
mod ppix;                     pub use ppix::*;
mod time;                     pub use time::*;
mod sequence_info;            pub use sequence_info::*;
mod video_segment;            pub use video_segment::*;
mod video_segment_properties; pub use video_segment_properties::*;

#[cfg(has_ae_sdk)]
mod opaque_effect_data;
#[cfg(has_ae_sdk)]
pub use opaque_effect_data::*;

use std::ptr;
use std::cell::RefCell;

thread_local! {
    pub(crate) static PICA_BASIC_SUITE: RefCell<*const pr_sys::SPBasicSuite> = RefCell::new(ptr::null_mut());
}

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

pub(crate) trait Suite {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

}
