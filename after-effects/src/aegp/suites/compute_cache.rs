use std::{ffi::CString, marker::PhantomData, str::FromStr};

use after_effects_sys::{
    AEGP_CCCheckoutReceiptP, AEGP_CCComputeKeyP, AEGP_CCComputeOptionsRefconP,
    AEGP_CCComputeValueRefconP, AEGP_ComputeCacheCallbacks,
};

use crate::{aegp::Guid, *};

#[inline(always)]
fn conjure<F>() -> F {
    const { assert!(std::mem::size_of::<F>() == 0) }
    unsafe { std::mem::zeroed() }
}

define_suite!(
    ComputeCacheSuite,
    AEGP_ComputeCacheSuite1,
    kAEGPComputeCacheSuite,
    kAEGPComputeCacheSuiteVersion1
);

pub struct ComputeCacheReceipt<V> {
    receipt_ptr: AEGP_CCCheckoutReceiptP,
    _phantom_data_v: PhantomData<V>,
}

/// A type safe class ID - this is essentiall a &'str which in most cases will be static.
/// because the underlying API relies on void* we track the option and value types on the id
/// to avoid casting to the wrong type.
///
/// See the simulation example for an example of usage.
pub struct ComputeClassId<'a, O, V> {
    _phantom_data_options: PhantomData<O>,
    _phantom_data_value: PhantomData<V>,
    pub id: &'a str,
}

impl<'a, O, V> ComputeClassId<'a, O, V> {
    pub const fn new(id: &'a str) -> Self {
        Self {
            _phantom_data_options: PhantomData,
            _phantom_data_value: PhantomData,
            id,
        }
    }

    pub fn id(&self) -> &'a str { self.id }
}

impl<V> AsPtr<AEGP_CCCheckoutReceiptP> for ComputeCacheReceipt<V> {
    #[inline]
    fn as_ptr(&self) -> ae_sys::AEGP_CCCheckoutReceiptP { self.receipt_ptr }
}

impl ComputeCacheSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// The callback functions must be statically known function items (not closures
    /// with captured state). This is enforced at compile time.
    pub fn register_class<O, V, GenKey, Compute, ApproxSize, Delete>(
        &self,
        compute_class_id: &ComputeClassId<O, V>,
        _generate_key: GenKey,
        _compute: Compute,
        _approx_size: ApproxSize,
        _delete: Delete,
    ) -> Result<(), Error>
    where
        GenKey: Fn(&O) -> Result<Guid, Error>,
        Compute: Fn(&O) -> Result<V, Error>,
        ApproxSize: Fn(&V) -> usize,
        Delete: Fn(V),
    {
        const { assert!(std::mem::size_of::<GenKey>() == 0) }
        const { assert!(std::mem::size_of::<Compute>() == 0) }
        const { assert!(std::mem::size_of::<ApproxSize>() == 0) }
        const { assert!(std::mem::size_of::<Delete>() == 0) }

        unsafe extern "C" fn generate_key_trampoline<O, GenKey>(
            options_p: AEGP_CCComputeOptionsRefconP,
            out_key_p: AEGP_CCComputeKeyP,
        ) -> ae_sys::A_Err
        where
            GenKey: Fn(&O) -> Result<Guid, Error>,
        {
            let opts = unsafe { &*(options_p as *const O) };
            match conjure::<GenKey>()(opts) {
                Ok(guid) => {
                    unsafe { *out_key_p = guid.0 };
                    ae_sys::A_Err_NONE as _
                }
                Err(e) => e.into(),
            }
        }

        unsafe extern "C" fn compute_trampoline<O, V, Compute>(
            options_p: AEGP_CCComputeOptionsRefconP,
            out_value_pp: *mut AEGP_CCComputeValueRefconP,
        ) -> ae_sys::A_Err
        where
            Compute: Fn(&O) -> Result<V, Error>,
        {
            let opts = unsafe { &*(options_p as *const O) };
            match conjure::<Compute>()(opts) {
                Ok(value) => {
                    unsafe { *out_value_pp = Box::into_raw(Box::new(value)) as _ };
                    ae_sys::A_Err_NONE as _
                }
                Err(e) => e.into(),
            }
        }

        unsafe extern "C" fn approx_size_trampoline<V, ApproxSize>(
            value_p: AEGP_CCComputeValueRefconP,
        ) -> usize
        where
            ApproxSize: Fn(&V) -> usize,
        {
            conjure::<ApproxSize>()(unsafe { &*(value_p as *const V) })
        }

        unsafe extern "C" fn delete_trampoline<V, Delete>(value_p: AEGP_CCComputeValueRefconP)
        where
            Delete: Fn(V),
        {
            conjure::<Delete>()(unsafe { *Box::from_raw(value_p as *mut V) })
        }

        let c_str = CString::from_str(compute_class_id.id()).map_err(|_| Error::InvalidParms)?;

        let callbacks = AEGP_ComputeCacheCallbacks {
            generate_key: Some(generate_key_trampoline::<O, GenKey>),
            compute: Some(compute_trampoline::<O, V, Compute>),
            approx_size_value: Some(approx_size_trampoline::<V, ApproxSize>),
            delete_compute_value: Some(delete_trampoline::<V, Delete>),
        };

        call_suite_fn!(
            self,
            AEGP_ClassRegister,
            c_str.as_ptr(),
            &callbacks as *const _
        )
    }

    /// Checks if a cache value has already been computed without triggering computation.
    /// Returns the receipt if available, otherwise returns `None`.
    ///
    /// Useful for polling patterns where another thread handles computation.
    pub fn checkout_cached<O, V>(
        &self,
        compute_class_id: &ComputeClassId<O, V>,
        options: &mut O,
    ) -> Result<Option<ComputeCacheReceipt<V>>, Error> {
        let c_str = CString::from_str(compute_class_id.id()).map_err(|_| Error::InvalidParms)?;
        let result = call_suite_fn_single!(self, AEGP_CheckoutCached -> AEGP_CCCheckoutReceiptP, c_str.as_ptr(), options as *mut O as *mut _);

        match result {
            Ok(ptr) => Ok(Some(ComputeCacheReceipt {
                receipt_ptr: ptr as *mut _,
                _phantom_data_v: PhantomData,
            })),
            Err(Error::NotInComputeCache) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Unregisters a previously registered cache type using its globally unique identifier.
    /// All cached values will be purged at this time through calls to `delete_compute_value`.
    ///
    /// Typically invoked during `PF_Cmd_GLOBAL_SETDOWN`.
    pub fn unregister_class<O, V>(
        &self,
        compute_class_id: &ComputeClassId<O, V>,
    ) -> Result<(), Error> {
        let c_str = CString::from_str(compute_class_id.id()).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_ClassUnregister, c_str.as_ptr())
    }

    /// The primary checkout function that computes or retrieves a receipt.
    ///
    /// The `wait_for_other_thread` parameter determines behavior: when `true`,
    /// it always computes or waits for completion; when `false`, it returns
    /// [`Error::NotInComputeCache`] if another thread is already computing.
    pub fn compute_if_needed_and_checkout<O, V>(
        &self,
        compute_class_id: ComputeClassId<O, V>,
        options: &mut O,
        wait_for_other_thread: bool,
    ) -> Result<ComputeCacheReceipt<V>, Error> {
        let c_str = CString::from_str(compute_class_id.id()).map_err(|_| Error::InvalidParms)?;
        let receipt_ptr = call_suite_fn_single!(
            self,
            AEGP_ComputeIfNeededAndCheckout -> AEGP_CCCheckoutReceiptP,
            c_str.as_ptr(),
            options as *mut O as *mut _,
            wait_for_other_thread
        )?;
        Ok(ComputeCacheReceipt {
            receipt_ptr,
            _phantom_data_v: PhantomData,
        })
    }

    /// Signals completion of cache value usage before returning to the host.
    pub fn check_in_compute_receipt(
        &self,
        receipt: impl AsPtr<AEGP_CCCheckoutReceiptP>,
    ) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_CheckinComputeReceipt, receipt.as_ptr())
    }

    /// Retrieves the computed cache value using a receipt from either
    /// [`compute_if_needed_and_checkout`](Self::compute_if_needed_and_checkout) or
    /// [`checkout_cached`](Self::checkout_cached).
    ///
    /// The returned reference is valid until the receipt is checked in.
    ///
    /// Warn: This will transmute the underlying pointer to type V, if these do not
    /// line up it will induce UB.
    pub fn receipt_compute_value<'a, V>(
        &self,
        receipt: &ComputeCacheReceipt<V>,
    ) -> Result<&'a V, Error> {
        let value_ptr = call_suite_fn_single!(
            self,
            AEGP_GetReceiptComputeValue -> ae_sys::AEGP_CCComputeValueRefconP,
            receipt.as_ptr()
        )?;
        unsafe { (value_ptr as *const V).as_ref() }.ok_or(Error::Generic)
    }
}
