use crate::*;

define_suite!(
    /// This suite provides effects a way to share unflattened sequence data between instances of the same effect on a track item.
    /// The data is opaque to the host and effects are responsible for maintaining thread safety of the shared data.
    /// The host provides reference-counting that the effect can use to manage the lifetime of the shared data. Here's an overview of how this suite should be used:
    ///
    /// When the effect is applied, in `Command::SequenceSetup`, the effect plugin allocates and initializes the sequence data in PF_OutData->out_data.
    ///
    /// Then it calls AcquireOpaqueEffectData(). The opaque effect data does not yet exist, so the plugin allocates it, and calls RegisterOpaqueEffectData, and then copies over the data from the sequence data. So both sequence data and opaque effect data are allocated.
    ///
    /// Then `Command::SequenceResetup` is called (multiple times) for clones of the effect used for rendering. The effect instance knows it's a clone because the PF_InData->sequence_data is NULL (there is a special case if the effect has Opaque Effect Data - in that case, its render clones will receive `Command::SequenceResetup` with a NULL sequence_data pointer). It then calls AcquireOpaqueEffectData(). As a render clone, it relies on this opaque effect data, rather than sequence data, and does not try to copy the sequence data to opaque effect data.
    ///
    /// When, on the other hand, `Command::SequenceResetup` is called with valid sequence_data in PF_InData, this is not a render clone. The plugin unflattens this sequence data. It then calls AcquireOpaqueEffectData(), and if the opaque effect data does not yet exist (i.e. when reopening a saved project), the plugin allocates it, and calls RegisterOpaqueEffectData. It then copies the sequence data to opaque effect data.
    ///
    /// On `Command::SequenceFlatten`, the plugin takes the unflattened data, flattens it, and disposes of the un-flat data.
    ///
    /// When `Command::SequenceSetdown` is called (it may be called multiple times to dispose of render clones), ReleaseOpaqueEffectData() is called.
    ///
    /// # instanceID
    /// The `Opaque Effect Data Suite` functions need the instanceID of the effect.
    /// For the software entry point, you can obtain this using GetFilterInstanceID() in PF_UtilitySuite, defined in PrSDKAESupport.h. For the GPU Render entry point, you can use the following code:
    ///
    /// ```ignore
    /// let instance_id = filter.get_property(pr::Property::Effect_RuntimeInstanceID);
    /// if let Ok(pr::PropertyData::UInt32(id)) = instance_id {
    ///     ...
    /// }
    /// ```
    OpaqueEffectDataSuite,
    PrSDKOpaqueEffectDataSuite,
    kPrSDKOpaqueEffectDataSuite,
    kPrSDKOpaqueEffectDataSuiteVersion
);

impl OpaqueEffectDataSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Acquire pointer to opaque effect data. This is reference counted meaning that
    /// [`acquire_opaque_effect_data()`](Self::acquire_opaque_effect_data) and [`release_opaque_effect_data()`](Self::release_opaque_effect_data) should always be called in pairs.
    /// If no opaque effect was registered for the given effect_ref [`acquire_opaque_effect_data()`](Self::acquire_opaque_effect_data)
    /// will return 0 and the reference count remains 0.
    pub fn acquire_opaque_effect_data(&self, instance_id: i32) -> Result<*mut pr_sys::OpaqueEffectDataType, Error> {
        call_suite_fn_single!(self, AcquireOpaqueEffectData -> *mut pr_sys::OpaqueEffectDataType, instance_id)
    }

    /// Register opaque effect data. If multiple threads invoke [`register_opaque_effect_data()`](Self::register_opaque_effect_data) only one will be successful.
    /// The `opaque_effect_data` of the successful thread will be returned to all callers.
    /// Calling threads are always responsible for managing the data they register.
    /// This is the case whether or not threads are successful registering their data.
    /// Similarly, [`register_opaque_effect_data()`](Self::register_opaque_effect_data) always increments the internal reference count.
    ///
    /// # Examples
    ///
    /// ```ignore
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
    /// If the internal reference count goes to 0 any calls made to [`acquire_opaque_effect_data()`](Self::acquire_opaque_effect_data)
    /// will return 0 until new opaque effect data is registered via [`register_opaque_effect_data()`](Self::register_opaque_effect_data).
    pub fn release_opaque_effect_data(&self, instance_id: i32, out_dispose_opaque_effect_data: *mut *mut pr_sys::OpaqueEffectDataType) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseOpaqueEffectData, instance_id, out_dispose_opaque_effect_data)
    }
}
