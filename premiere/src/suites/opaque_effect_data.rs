use crate::*;

define_suite!(OpaqueEffectDataSuite, PrSDKOpaqueEffectDataSuite, kPrSDKOpaqueEffectDataSuite, kPrSDKOpaqueEffectDataSuiteVersion);

/// This suite provides effects a means to share unflattened sequence data between its instances.
/// The data is opaque to the host and effects are responsible for maintaining thread safety of the shared data.
/// The host provides ref counting that the effect can use to manage the lifetime of the shared data.
impl OpaqueEffectDataSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Acquire pointer to opaque effect data. This is reference counted meaning that
    /// [`acquire_opaque_effect_data`] and [`release_opaque_effect_data`] should always be called in pairs.
    /// If no opaque effect was registered for the given effect_ref AcquireOpaqueEffectData
    /// will return 0 and the reference count remains 0.
    pub fn acquire_opaque_effect_data(&self, instance_id: i32) -> Result<*mut pr_sys::OpaqueEffectDataType, Error> {
        call_suite_fn_single!(self, AcquireOpaqueEffectData -> *mut pr_sys::OpaqueEffectDataType, instance_id)
    }

    /// Register opaque effect data. If multiple threads invoke [`register_opaque_effect_data`] only one will be successful.
    /// The `opaque_effect_data` of the successful thread will be returned to all callers.
    /// Calling threads are always responsible for managing the data they register.
    /// This is the case whether or not threads are successful registering their data.
    /// Similarly, [`register_opaque_effect_data`] always increments the internal reference count.
    ///
    /// # Examples
    ///
    /// ```
    /// // Try to acquire first, in case another thread registered the opaque effect data earlier
    /// let mut data = opaque_effect_data_suite.acquire_opaque_effect_data(instance_id)?;
    ///
    /// // If acquire did not return a valid pointer, create a new object and register it otherwise we are done
    /// if data.is_null() {
    ///     let mut new_data = Box::new(OpaqueEffectDataType::default());
    ///     let mut new_data = Box::into_raw(new_data) as *mut _;
    ///     data = new_data;
    ///     opaque_effect_data_suite.register_opaque_effect_data(instance_id, &mut data as *mut _)?;
    ///     // now we check if this thread actually succeeded registering
    ///     // if the returned data is unchanged it means that it was successful
    ///     if data != new_data {
    ///         let _ = Box::from_raw(new_data); // delete the new data
    ///     }
    /// }
    /// // data now points to the right OpaqueEffectDataType object and we can start using it
    /// ```
    pub fn register_opaque_effect_data(&self, instance_id: i32, opaque_effect_data: *mut *mut pr_sys::OpaqueEffectDataType) -> Result<(), Error> {
        call_suite_fn!(self, RegisterOpaqueEffectData, instance_id, opaque_effect_data)
    }

    /// Release opaque effect data. This decrements the internal reference count.
    ///
    /// If the internal reference count goes to 0 `out_dispose_opaque_effect_data` is set
    /// to the managed data that should be deleted, otherwise it is set to NULL.
    ///
    /// If the internal reference count goes to 0 any calls made to [`acquire_opaque_effect_data`]
    /// will return 0 until new opaque effect data is registered via [`register_opaque_effect_data`].
    pub fn release_opaque_effect_data(&self, instance_id: i32, out_dispose_opaque_effect_data: *mut *mut pr_sys::OpaqueEffectDataType) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseOpaqueEffectData, instance_id, out_dispose_opaque_effect_data)
    }
}
