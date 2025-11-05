/// This macro defines the main entry point for an After Effects plugin.
///
/// You have to pass 3 arguments to this macro:
/// - The global type, which will be the same across all instances of the plugin
/// - The instance type, which will be created for each instance of the plugin
/// - The parameters type
///
/// # 1. The global type
/// This type will be the same across all instances of the plugin. It can be used to store global state, such as the plugin version, data cache, or any other global data.
/// It must implement the `Default` trait which will be called on [`Command::GlobalSetup`](crate::pf::Command::GlobalSetup). You can use the `Drop` trait to clean up any resources on [`Command::GlobalSetdown`](crate::pf::Command::GlobalSetdown).
///
/// Your global type must implement the `AdobePluginGlobal` trait, which defines the plugin's command selectors.
/// ```ignore
/// trait AdobePluginGlobal : Default {
///     fn params_setup(&self,
///         params: &mut Parameters<ParamsType>,
///         in_data: InData,
///         out_data: OutData
///     ) -> Result<(), Error>;
///
///     fn handle_command(&mut self,
///         command: Command,
///         in_data: InData,
///         out_data: OutData,
///         params: &mut Parameters<ParamsType>
///     ) -> Result<(), Error>;
/// }
/// ```
///
/// # 2. The instance type
/// This type will be created in [`Command::SequenceSetup`](crate::pf::Command::SequenceSetup) and destroyed in [`Command::SequenceSetdown`](crate::pf::Command::SequenceSetdown).
/// It can be used to store instance-specific state, such as some data calculated from the plugin's parameters.
/// It must implement the `Default` trait which will be called on [`Command::SequenceSetup`](crate::pf::Command::SequenceSetup).
/// You can use the `Drop` trait to clean up any resources on [`Command::SequenceSetdown`](crate::pf::Command::SequenceSetdown).
///
/// Your instance type must implement the `AdobePluginInstance` trait, which defines the instance's command selectors.
///
/// ```ignore
/// trait AdobePluginInstance : Default {
///     fn flatten(&self) -> Result<(u16, Vec<u8>), Error>;
///     fn unflatten(version: u16, serialized: &[u8]) -> Result<Self, Error>;
///
///     fn render(&self,
///         plugin: &mut PluginState,
///         in_layer: &Layer,
///         out_layer: &mut Layer
///     ) -> Result<(), ae::Error>;
///
///     #[cfg(does_dialog)]
///     fn do_dialog(&mut self,
///         plugin: &mut PluginState
///     ) -> Result<(), ae::Error>;
///
///     fn handle_command(&mut self,
///         plugin: &mut PluginState,
///         command: Command
///     ) -> Result<(), Error>;
/// }
/// ```
///
/// Use the `flatten()` method to serialize the contents of your struct to a `Vec<u8>` (you can use serde, bincode, or any other serialization library).
/// The serialized data will be stored in the project file and used to create copies of the instance for Multi-Frame Rendering. It will be passed to the `unflatten()` method to restore the instance.
///
/// The `unflatten()` method will be called to restore the instance from the serialized data. The `u16` parameter specifies the version of the serialized data,
/// so you can always restore the data correctly even if user updates your plugin and tries to load a project file with older data version in it.
///
/// The `PluginState` struct allows you to access the global struct, instance struct, parameters, and input/output data in your plugin's command selectors.
/// ```ignore
/// struct PluginState {
///    global: &mut GlobalType,
///    sequence: Option<&mut InstanceType>,
///    params: &mut Parameters<ParamsType>,
///    in_data: InData,
///    out_data: OutData
/// }
/// ```
///
/// ### ⚠️ Caution: The creation and destruction of the instance type is not intuitive when Multi-Frame Rendering is enabled.
/// After Effects will create many copies of this instance using [`Command::SequenceFlatten`](crate::pf::Command::SequenceFlatten) and [`Command::SequenceResetup`](crate::pf::Command::SequenceResetup).
/// These copies may be created and destroyed in a weird order and in different threads.
///
/// If you want to keep an consistent and shared state between all these copies, consider using the [`define_cross_thread_type!`](crate::define_cross_thread_type) macro.
///
/// If your state is entirely serializable to the `Vec<u8>` in `flatten()` and doesn't depend on any global pointers or shared resources, you don't have to worry about this.
///
/// # 3. The parameters type
/// This is an enum which covers all parameters in your plugin. It must implement the `Eq`, `PartialEq`, `Hash`, `Clone`, `Copy`, and `Debug` traits.
/// You will use this enum to define the parameters in `params_setup()` and to access the parameters from the [`Parameters`](crate::pf::Parameters) struct in the `handle_command()` method.
///
///
/// ## Refer to the [Adobe After Effects SDK](https://ae-plugins.docsforadobe.dev/effect-basics/command-selectors.html) to learn more about the plugin entry point and command selectors.
///
/// # Example usage:
///
/// ```ignore
/// #[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
/// enum Params { Opacity }
///
/// #[derive(Default)]
/// struct Plugin { }
///
/// #[derive(Default)]
/// struct Instance {
///     my_state: i32,
/// }
///
/// ae::define_effect!(Plugin, Instance, Params);
/// ```
///
/// If you don't need any specific state for each instance, you can pass the unit type (`()`) as the second argument.
///
/// ```ignore
/// #[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
/// enum Params { Opacity }
///
/// #[derive(Default)]
/// struct Plugin { }
///
/// ae::define_effect!(Plugin, (), Params);
/// ```
#[macro_export]
macro_rules! define_effect {
    ($global_type:ty, $sequence_type:tt, $params_type:ty) => {
        use $crate::*;
        use std::collections::HashMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        struct PluginState<'main, 'global, 'sequence, 'params> {
            global: &'global mut $global_type,
            sequence: Option<&'sequence mut $sequence_type>,
            params: &'params mut $crate::Parameters<'main, $params_type>,
            in_data: $crate::InData,
            out_data: $crate::OutData
        }

        // This struct **must** be thread safe
        struct GlobalData {
            params_map: std::sync::OnceLock<HashMap<$params_type, $crate::ParamMapInfo>>,
            params_num: usize,
            plugin_instance: $global_type
        }

        trait AdobePluginGlobal : Default {
            fn params_setup(&self, params: &mut Parameters<$params_type>, in_data: InData, out_data: OutData) -> Result<(), Error>;

            fn handle_command(&mut self, command: Command, in_data: InData, out_data: OutData, params: &mut Parameters<$params_type>) -> Result<(), Error>;
        }
        trait AdobePluginInstance : Default {
            fn flatten(&self) -> Result<(u16, Vec<u8>), Error>;
            fn unflatten(version: u16, serialized: &[u8]) -> Result<Self, Error>;

            fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error>;

            #[cfg(does_dialog)]
            fn do_dialog(&mut self, plugin: &mut PluginState) -> Result<(), ae::Error>;

            fn handle_command(&mut self, plugin: &mut PluginState, command: Command) -> Result<(), Error>;
        }
        impl AdobePluginInstance for () {
            fn flatten(&self) -> Result<(u16, Vec<u8>), Error> { Ok((0, Vec::new())) }
            fn unflatten(_: u16, _: &[u8]) -> Result<Self, Error> { Ok(Default::default()) }
            fn render(&self, _: &mut PluginState, _: &Layer, _: &mut Layer) -> Result<(), ae::Error> { Ok(()) }
            fn handle_command(&mut self, _: &mut PluginState, _: Command) -> Result<(), Error> { Ok(()) }

            #[cfg(does_dialog)]
            fn do_dialog(&mut self, _: &mut PluginState) -> Result<(), ae::Error> { Ok(()) }
        }

        fn get_sequence_handle<'a, S: AdobePluginInstance>(cmd: RawCommand, in_data: &InData) -> Result<Option<(pf::Handle::<'a, S>, bool)>, Error> {
            // Sequence data is not available during these commands:
            const EXCLUDES: &[RawCommand] = &[RawCommand::GlobalSetup, RawCommand::GlobalSetdown, RawCommand::GpuDeviceSetup, RawCommand::GpuDeviceSetdown, RawCommand::ArbitraryCallback];
            if EXCLUDES.contains(&cmd) {
                return Ok(None);
            }
            Ok(if std::any::type_name::<S>() == "()" {
                // Don't allocate sequence data
                None
            } else if cmd == RawCommand::SequenceSetup {
                // Allocate new sequence data
                Some((pf::Handle::new(S::default())?, true))
            } else if cmd == RawCommand::SequenceResetup {
                // Restore from flat handle
                // SAFETY: in_data.as_ptr() returns a valid non-null pointer to PF_InData that is
                // guaranteed to be valid for the lifetime of this command. The pointer is provided
                // by After Effects and we're only reading the sequence_data field, not mutating it.
                // This upholds the invariant that the InData pointer remains valid throughout the
                // command execution. Undefined behavior would occur if: (1) in_data.as_ptr() returned
                // null, (2) the pointer became invalid during execution, or (3) we accessed fields
                // beyond the struct bounds.
                if unsafe { (*in_data.as_ptr()).sequence_data.is_null() } {
                    Some((pf::Handle::new(S::default())?, true))
                } else {
                    // SAFETY: We just verified that sequence_data is not null in the previous check.
                    // The sequence_data pointer is owned by After Effects and is valid for the duration
                    // of the SequenceResetup command. We're converting it to a PF_Handle type for
                    // processing. This is safe because: (1) the pointer is non-null (checked above),
                    // (2) it points to valid flat handle data, and (3) we're only reading from it.
                    // Undefined behavior would occur if the pointer became invalid between the null
                    // check and the cast.
                    let instance = FlatHandle::from_raw(unsafe { (*in_data.as_ptr()).sequence_data as $crate::sys::PF_Handle })?;
                    let bytes = instance.as_slice().ok_or(Error::InvalidIndex)?;
                    if bytes.len() < 2 {
                        return Ok(None);
                    }
                    let version = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

                    let handle = pf::Handle::new(S::unflatten(version, &bytes[2..]).map_err(|_| Error::Struct)?)?;
                    Some((handle, true))
                }
            // SAFETY: in_data.as_ptr() returns a valid non-null pointer to PF_InData provided by
            // After Effects for this command's lifetime. We're only dereferencing to read the
            // sequence_data field to check if it's null. This is safe because: (1) the pointer
            // is guaranteed valid by the AE SDK contract, (2) we're only reading a pointer field,
            // not modifying memory. Undefined behavior would occur if in_data.as_ptr() returned
            // an invalid pointer or if PF_InData's memory layout changed unexpectedly.
            } else if unsafe { (*in_data.as_ptr()).sequence_data.is_null() } {
                // Read-only sequence data available through a suite only
                // SAFETY: This accesses sequence_data in the same manner as the if condition above.
                // The pointer is valid and we're performing a read-only cast from a mutable pointer
                // to a const pointer. This is safe because: (1) in_data.as_ptr() is valid (guaranteed
                // by AE SDK), (2) sequence_data points to valid memory or is null (handled by the
                // suite fallback), (3) we're only performing a type cast, not dereferencing yet.
                // Undefined behavior would occur if the pointer was dangling or unaligned.
                let seq_ptr = in_data.effect().const_sequence_data().unwrap_or(unsafe { (*in_data.as_ptr()).sequence_data as *const _ });
                if !seq_ptr.is_null() {
                    let instance_handle = pf::Handle::<S>::from_raw(seq_ptr as *mut _, false)?;
                    Some((instance_handle, false))
                } else {
                    $crate::log::error!("Sequence data pointer got through EffectSequenceDataSuite is null in cmd: {:?}!", cmd);
                    None
                }
            } else {
                let should_dispose_sequence = cmd == RawCommand::SequenceSetdown || cmd == RawCommand::SequenceFlatten;
                // SAFETY: We've reached this else branch which means: (1) sequence_data is not null
                // (checked in previous conditions), (2) in_data.as_ptr() is a valid pointer to
                // PF_InData guaranteed by AE SDK, (3) sequence_data points to valid allocated memory
                // owned by After Effects. We're reading this pointer to pass it to Handle::from_raw
                // which will properly manage its lifetime. Undefined behavior would occur if: (1)
                // sequence_data was deallocated before this access, (2) the pointer was invalid or
                // misaligned, or (3) the memory didn't actually contain a valid handle structure.
                let instance_handle = pf::Handle::<S>::from_raw(unsafe { (*in_data.as_ptr()).sequence_data }, should_dispose_sequence)?;
                Some((instance_handle, false))
            })
        }

        fn handle_effect_main<T: AdobePluginGlobal, S: AdobePluginInstance, P>(
            cmd: $crate::sys::PF_Cmd,
            in_data_ptr: *mut $crate::sys::PF_InData,
            out_data_ptr: *mut $crate::sys::PF_OutData,
            params: *mut *mut $crate::sys::PF_ParamDef,
            output: *mut $crate::sys::PF_LayerDef,
            extra: *mut std::ffi::c_void) -> Result<(), Error>
        {
            let _pica = $crate::PicaBasicSuite::from_pf_in_data_raw(in_data_ptr);

            let in_data = InData::from_raw(in_data_ptr);
            let out_data = OutData::from_raw(out_data_ptr);

            #[cfg(with_premiere)]
            let _pr_pica = if in_data.is_premiere() {
                Some(::premiere::PicaBasicSuite::from_sp_basic_suite_raw(in_data.pica_basic_suite_ptr() as _))
            }  else {
                None
            };

            let cmd = RawCommand::from(cmd);

            // Allocate or restore global data pointer
            let mut global_handle = if cmd == RawCommand::GlobalSetup {
                // Allocate global data
                pf::Handle::new(GlobalData {
                    params_map: std::sync::OnceLock::new(),
                    params_num: 1,
                    plugin_instance: <$global_type>::default()
                })?
            } else {
                // SAFETY: in_data_ptr is a valid non-null pointer to PF_InData provided by After
                // Effects as a parameter to this entry point. We're dereferencing it to read the
                // global_data field and check if it's null. This is safe because: (1) the pointer
                // is guaranteed valid by the AE SDK contract for the lifetime of this function,
                // (2) we're only reading a single pointer field without mutation, (3) the memory
                // layout of PF_InData is fixed by the AE SDK. Undefined behavior would occur if
                // in_data_ptr was null, dangling, or pointed to incorrectly aligned memory.
                if unsafe { (*in_data_ptr).global_data.is_null() } {
                    $crate::log::error!("Global data pointer is null in cmd: {:?}!", cmd);
                    return Err(Error::BadCallbackParameter);
                }
                // SAFETY: We just verified global_data is non-null in the check above. The
                // global_data pointer was set by us in a previous GlobalSetup call and has been
                // preserved by After Effects. We're reading this pointer to restore our global
                // state. This is safe because: (1) the pointer is non-null (checked above),
                // (2) it points to a valid GlobalData structure we allocated, (3) After Effects
                // guarantees this pointer remains valid until GlobalSetdown. Undefined behavior
                // would occur if the pointer was corrupted or deallocated by external code.
                pf::Handle::<GlobalData>::from_raw(unsafe { (*in_data_ptr).global_data }, cmd == RawCommand::GlobalSetdown)?
            };

            // Allocate or restore sequence data pointer
            let sequence_handle = get_sequence_handle::<$sequence_type>(cmd, &in_data).unwrap_or(None);

            let global_lock = global_handle.lock()?;
            let global_inst = global_lock.as_ref_mut()?;

            if cmd == RawCommand::ParamsSetup {
                let mut params = Parameters::<$params_type>::new();
                params.set_in_data(in_data_ptr);
                global_inst.plugin_instance.params_setup(&mut params, InData::from_raw(in_data_ptr), OutData::from_raw(out_data_ptr))?;
                global_inst.params_num = params.num_params();
                // SAFETY: out_data_ptr is a valid non-null pointer to PF_OutData provided by After
                // Effects as a parameter to this entry point. We're writing to the num_params field
                // to communicate the parameter count back to AE. This is safe because: (1) out_data_ptr
                // is guaranteed valid by the AE SDK for this function's lifetime, (2) we're writing
                // to a properly aligned i32 field that AE expects us to modify, (3) params.map is a
                // valid Rc that we're cloning (not moving). Undefined behavior would occur if
                // out_data_ptr was null/invalid, or if we wrote to the wrong field/offset.
                unsafe {
                    (*out_data_ptr).num_params = params.num_params() as i32;
                    global_inst.params_map.set((*params.map).clone()).unwrap();
                }
            }

            let params_slice = if params.is_null() || global_inst.params_num == 0 {
                &[]
            } else {
                // SAFETY: We've verified that params is not null and params_num > 0. The params
                // pointer is provided by After Effects and points to an array of PF_ParamDef pointers.
                // This upholds the invariants required by from_raw_parts: (1) params is non-null
                // (checked above), (2) it's properly aligned (guaranteed by AE SDK), (3) it points
                // to params_num valid, initialized elements, (4) the total size doesn't exceed isize::MAX,
                // (5) the memory remains valid for the lifetime of this function. Undefined behavior
                // would occur if: params was dangling, misaligned, or params_num exceeded the actual
                // array length.
                unsafe { std::slice::from_raw_parts(params, global_inst.params_num) }
            };

            let mut params_state = Parameters::<$params_type>::with_params(in_data_ptr, params_slice, global_inst.params_map.get(), global_inst.params_num);
            let mut plugin_state = PluginState {
                global: &mut global_inst.plugin_instance,
                sequence: sequence_handle.as_ref().map(|x| x.0.as_mut().unwrap()),
                params: &mut params_state,
                in_data,
                out_data
            };

            let command = Command::from_entry_point(cmd, in_data_ptr, params, output, extra);

            let global_err = plugin_state.global.handle_command(command, in_data, out_data, plugin_state.params);
            let mut sequence_err = None;

            if let Some((mut sequence_handle, needs_lock)) = sequence_handle {
                let (lock, inst) = if needs_lock {
                    let lock = sequence_handle.lock()?;
                    let inst = lock.as_ref_mut()?;
                    (Some(lock), inst)
                } else {
                    (None, sequence_handle.as_mut().unwrap())
                };
                let in_data = InData::from_raw(in_data_ptr);
                let out_data = OutData::from_raw(out_data_ptr);
                let command = Command::from_entry_point(cmd, in_data_ptr, params, output, extra);

                sequence_err = Some(inst.handle_command(&mut plugin_state, command));

                match cmd {
                    #[cfg(does_dialog)]
                    RawCommand::DoDialog => {
                        sequence_err = Some(inst.do_dialog(&mut plugin_state));
                    }
                    RawCommand::Render => {
                        // SAFETY: During a Render command, params points to a valid array of PF_ParamDef
                        // pointers (guaranteed by AE SDK). The first element (*params) is the input layer
                        // parameter, and we're accessing its union field 'u.ld' which contains the layer
                        // definition. This is safe because: (1) params is non-null during Render,
                        // (2) dereferencing *params gives us the first parameter which is always the input
                        // layer, (3) accessing u.ld is valid because the union is initialized as a layer
                        // def for the input parameter, (4) we create a mutable reference with a lifetime
                        // tied to this scope. Undefined behavior would occur if params was null or if the
                        // union field was accessed incorrectly.
                        let in_layer = $crate::Layer::from_raw(unsafe { &mut (*(*params)).u.ld }, in_data, None);
                        let mut out_layer = $crate::Layer::from_raw(output, in_data, None);
                        sequence_err = Some(inst.render(&mut plugin_state, &in_layer, &mut out_layer));
                    }
                    // RawCommand::UserChangedParam => {
                    //     let extra = extra as *mut $crate::sys::PF_UserChangedParamExtra;
                    //     let param = plugin_state.params.type_at((*extra).param_index as usize);
                    //     sequence_err = Some(inst.user_changed_param(&mut plugin_state, param));
                    // }
                    _ => { }
                }

                // SAFETY: This entire block manages the lifecycle of sequence data through the FFI
                // boundary with After Effects. The operations are safe because:
                // 1. out_data_ptr is guaranteed valid by AE SDK for this function's lifetime
                // 2. We properly manage ownership transfer through Handle::into_raw/from_raw
                // 3. Each command follows the AE SDK's contract for sequence data management:
                //    - Setup/Resetup: Transfer ownership of live handle to AE
                //    - Flatten/GetFlattenedSequenceData: Serialize and return flat data
                //    - Setdown: Signal deallocation completion by nulling the pointer
                //    - Other: Keep handle alive without transferring ownership
                // 4. Memory layout of PF_OutData.sequence_data matches void* expectations
                // Undefined behavior would occur if: (1) out_data_ptr was invalid, (2) we violated
                // the ownership contract (double-free or use-after-free), (3) sequence_data pointer
                // was not properly aligned, or (4) the serialized data format was incompatible.
                unsafe {
                    match cmd {
                        RawCommand::SequenceSetup | RawCommand::SequenceResetup => {
                            drop(lock);
                            (*out_data_ptr).sequence_data = pf::Handle::into_raw(sequence_handle);
                        }
                        RawCommand::SequenceFlatten | RawCommand::GetFlattenedSequenceData => {
                            let serialized = inst.flatten().map_err(|_| Error::InternalStructDamaged)?;
                            drop(lock);
                            drop(sequence_handle);
                            let mut final_bytes = serialized.0.to_le_bytes().to_vec(); // version
                            final_bytes.extend(&serialized.1);
                            (*out_data_ptr).sequence_data = pf::FlatHandle::into_raw(FlatHandle::new(final_bytes)?) as *mut _;
                        }
                        RawCommand::SequenceSetdown => {
                            (*out_data_ptr).sequence_data = std::ptr::null_mut();
                            // sequence will be dropped and deallocated here
                        }
                        _ => {
                            drop(lock);
                        }
                    }
                }
            } else if std::any::type_name::<S>() == "()" && cmd == RawCommand::GetFlattenedSequenceData {
                // Even if we don't need the sequence data, AE expects us to set this pointer explicitly
                // Otherwise clicking on "Options..." in the Effect Controls panel will crash AE
                // SAFETY: out_data_ptr is a valid pointer to PF_OutData guaranteed by AE SDK. We're
                // writing null to sequence_data to explicitly signal to After Effects that there is
                // no sequence data for this plugin (when using unit type). This is safe because:
                // (1) out_data_ptr is valid for this function's lifetime, (2) sequence_data is a
                // pointer field that expects null as a valid value, (3) this satisfies AE's contract
                // for GetFlattenedSequenceData when no data exists. Undefined behavior would occur
                // if out_data_ptr was invalid or if we wrote to the wrong offset.
                unsafe { (*out_data_ptr).sequence_data = std::ptr::null_mut(); }
            }
            drop(plugin_state);
            drop(params_state);

            // SAFETY: This block manages the lifecycle of global plugin data through the FFI boundary.
            // The operations are safe because:
            // 1. out_data_ptr is guaranteed valid by AE SDK for this function's lifetime
            // 2. We properly manage ownership transfer of global_handle through Handle::into_raw
            // 3. Each command follows the AE SDK's contract for global data management:
            //    - GlobalSetup: Transfer ownership of newly created global handle to AE
            //    - GlobalSetdown: Signal deallocation by nulling pointer; global_handle drops here
            //    - Other commands: Keep global handle alive without transferring ownership
            // 4. The global_data pointer is preserved by AE between calls and passed back to us
            // 5. Memory layout of PF_OutData.global_data matches void* expectations
            // Undefined behavior would occur if: (1) out_data_ptr was invalid, (2) we violated
            // ownership semantics (double-free or use-after-free), (3) the pointer was misaligned,
            // or (4) AE corrupted/deallocated the global_data pointer between calls.
            unsafe {
                match cmd {
                    RawCommand::GlobalSetup => {
                        drop(global_lock);
                        (*out_data_ptr).global_data = pf::Handle::into_raw(global_handle);
                    }
                    RawCommand::GlobalSetdown => {
                        (*out_data_ptr).global_data = std::ptr::null_mut();
                        // global will be dropped and de-allocated here
                    }
                    _ => {
                        drop(global_lock);
                    }
                }
            }

            if global_err.is_err() {
                return global_err;
            }
            if sequence_err.is_some() && sequence_err.unwrap().is_err() {
                return sequence_err.unwrap();
            }

            Ok(())
        }

        #[cfg(debug_assertions)]
        static BACKTRACE_STR: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn PluginDataEntryFunction2(
            in_ptr: $crate::sys::PF_PluginDataPtr,
            in_plugin_data_callback_ptr: $crate::sys::PF_PluginDataCB2,
            in_sp_basic_suite_ptr: *const $crate::sys::SPBasicSuite,
            in_host_name: *const std::ffi::c_char,
            in_host_version: *const std::ffi::c_char) -> $crate::sys::PF_Err
        {
            // let _pica = ae::PicaBasicSuite::from_sp_basic_suite_raw(in_sp_basic_suite_ptr);

            if in_host_name.is_null() || in_host_version.is_null() {
                return $crate::sys::PF_Err_INVALID_CALLBACK as $crate::sys::PF_Err;
            }

            if let Some(cb_ptr) = in_plugin_data_callback_ptr {
                use $crate::cstr_literal::cstr;
                // SAFETY: This function is marked unsafe and is an FFI boundary called by After Effects
                // during plugin registration. We're calling the callback function provided by AE to
                // register our plugin metadata. This is safe because:
                // 1. cb_ptr is a valid function pointer (checked by the Option unwrap above)
                // 2. in_ptr was provided by AE and is valid for this call's lifetime
                // 3. All string pointers come from cstr! macro which creates valid null-terminated
                //    C strings with static lifetime
                // 4. All numeric values are parsed from environment variables at compile time
                // 5. The callback signature matches PF_PluginDataCB2 exactly
                // Undefined behavior would occur if: (1) cb_ptr was invalid despite being Some,
                // (2) in_ptr was invalid, (3) any string pointer was not null-terminated, or
                // (4) the callback had incorrect signature/calling convention.
                unsafe {
                    cb_ptr(in_ptr,
                        cstr!(env!("PIPL_NAME"))       .as_ptr() as *const u8, // Name
                        cstr!(env!("PIPL_MATCH_NAME")) .as_ptr() as *const u8, // Match Name
                        cstr!(env!("PIPL_CATEGORY"))   .as_ptr() as *const u8, // Category
                        cstr!(env!("PIPL_ENTRYPOINT")) .as_ptr() as *const u8, // Entry point
                        env!("PIPL_KIND")              .parse().unwrap(),
                        env!("PIPL_AE_SPEC_VER_MAJOR") .parse().unwrap(),
                        env!("PIPL_AE_SPEC_VER_MINOR") .parse().unwrap(),
                        env!("PIPL_AE_RESERVED")       .parse().unwrap(),
                        cstr!(env!("PIPL_SUPPORT_URL")).as_ptr() as *const u8, // Support url
                    )
                }
            } else {
                $crate::sys::PF_Err_INVALID_CALLBACK as $crate::sys::PF_Err
            }
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn EffectMain(
            cmd: $crate::sys::PF_Cmd,
            in_data_ptr: *mut $crate::sys::PF_InData,
            out_data_ptr: *mut $crate::sys::PF_OutData,
            params: *mut *mut $crate::sys::PF_ParamDef,
            output: *mut $crate::sys::PF_LayerDef,
            extra: *mut std::ffi::c_void) -> $crate::sys::PF_Err
        {
            if cmd == $crate::sys::PF_Cmd_GLOBAL_SETUP as $crate::sys::PF_Cmd {
                // SAFETY: During GlobalSetup, we initialize the plugin's version and capability flags.
                // out_data_ptr is a valid non-null pointer to PF_OutData provided by After Effects.
                // This is safe because:
                // 1. out_data_ptr is guaranteed valid by the AE SDK for this call's lifetime
                // 2. We're writing to properly aligned i32/u32 fields that AE expects us to set
                // 3. All values come from compile-time environment variables, so they're always valid
                // 4. These fields define the plugin's capabilities and must be set during GlobalSetup
                // 5. The PF_OutData struct layout is fixed by the AE SDK
                // Undefined behavior would occur if: (1) out_data_ptr was null/invalid, (2) we wrote
                // to wrong offsets, (3) the memory was not properly aligned, or (4) PF_OutData layout
                // changed unexpectedly.
                unsafe {
                    (*out_data_ptr).my_version = env!("PIPL_VERSION")  .parse::<u32>().unwrap();
                    (*out_data_ptr).out_flags  = env!("PIPL_OUTFLAGS") .parse::<i32>().unwrap();
                    (*out_data_ptr).out_flags2 = env!("PIPL_OUTFLAGS2").parse::<i32>().unwrap();
                }

                #[cfg(debug_assertions)]
                {
                    #[cfg(target_os = "windows")]
                    {
                        let _ = $crate::log::set_logger(&$crate::win_dbg_logger::DEBUGGER_LOGGER);
                    }
                    #[cfg(target_os = "macos")]
                    {
                        let _ = $crate::oslog::OsLogger::new(env!("CARGO_PKG_NAME")).init();
                    }
                    $crate::log::set_max_level($crate::log::LevelFilter::Debug);

                    std::panic::set_hook(Box::new(|_| {
                        *BACKTRACE_STR.write().unwrap() = std::backtrace::Backtrace::force_capture().to_string();
                    }));
                }
            }

            #[cfg(threaded_rendering)]
            {
                fn assert_impl<T: Sync>() { }
                assert_impl::<$global_type>();
                assert_impl::<$sequence_type>();
            }

            define_effect!(check_size: $sequence_type);

            // log::info!("EffectMain start {:?} {:?}", RawCommand::from(cmd), std::thread::current().id());
            // struct X { cmd: i32 } impl Drop for X { fn drop(&mut self) { log::info!("EffectMain end {:?} {:?}", RawCommand::from(self.cmd), std::thread::current().id()); } }
            // let _x = X { cmd: cmd as i32 };

            #[cfg(any(debug_assertions, catch_panics))]
            {
                let result = std::panic::catch_unwind(|| {
                    handle_effect_main::<$global_type, $sequence_type, $params_type>(cmd, in_data_ptr, out_data_ptr, params, output, extra)
                });
                match result {
                    Ok(Ok(_)) => $crate::sys::PF_Err_NONE as $crate::sys::PF_Err,
                    Ok(Err(e)) => {
                        $crate::log::error!("EffectMain returned error: {e:?}");

                        if e != Error::InterruptCancel && !out_data_ptr.is_null() {
                            $crate::OutData::from_raw(out_data_ptr).set_error_msg(&format!("EffectMain returned error: {e:?}"));
                        }

                        e as $crate::sys::PF_Err
                    }
                    Err(e) => {
                        let s = if let Some(s) = e.downcast_ref::<&str>() { s.to_string() }
                           else if let Some(s) = e.downcast_ref::<String>() { s.clone() }
                           else { format!("{e:?}") };

                        let mut msg = format!("EffectMain panicked! {s}");

                        #[cfg(debug_assertions)]
                        {
                            $crate::log::error!("{msg}, backtrace: {}", BACKTRACE_STR.read().unwrap());
                        }

                        if msg.len() > 255 {
                            msg.truncate(255);
                        }
                        if !out_data_ptr.is_null() {
                            $crate::OutData::from_raw(out_data_ptr).set_error_msg(&msg);
                        }

                        $crate::sys::PF_Err_NONE as $crate::sys::PF_Err
                    }
                }
            }

            #[cfg(not(any(debug_assertions, catch_panics)))]
            match handle_effect_main::<$global_type, $sequence_type, $params_type>(cmd, in_data_ptr, out_data_ptr, params, output, extra) {
                Ok(_) => $crate::sys::PF_Err_NONE as $crate::sys::PF_Err,
                Err(e) => {
                    $crate::log::error!("EffectMain returned error: {e:?}");
                    e as $crate::sys::PF_Err
                }
            }
        }
    };
    (check_size: ()) => { };
    (check_size: $t:tt) => {
        const _: () = assert!(std::mem::size_of::<$t>() > 0, concat!("Type `", stringify!($t), "` cannot be zero-sized"));
    };
}

/// This is a marker trait - it is meant to discourage users from
/// implementing AegpPlugin without the scaffolding in `define_general_plugin`.
/// You should implement this *once* and only once in any given plugin. It is used to
/// mark a singleton type for retrieval from a raw pointer in the [RegisterSuite] api's.
pub unsafe trait AegpSeal {}

/// Trait used to implement generic plugins such as menu commands and background tasks.
/// A struct which implements this will be passed to all register suite callbacks
/// Warning: Do not implement this without calling `define_general_plugin`
pub trait AegpPlugin: Sized + AegpSeal {
    fn entry_point(
        major_version: i32,
        minor_version: i32,
        aegp_plugin_id: crate::sys::AEGP_PluginID,
    ) -> Result<Self, crate::Error>;
}

/// This macro defines the main entry point for an After Effects general plugin.
///
/// The macro generates an `EntryPointFunc` function that must match the entry point name
/// specified in your PIPL configuration:
///
/// ```ignore
/// Property::CodeWin64X86("EntryPointFunc"),
/// Property::CodeMacIntel64("EntryPointFunc"),
/// // etc.
/// ```
#[macro_export]
macro_rules! define_general_plugin {
    ($main_type:ty) => {
        // Static Assertion
        const _: () = {
            fn assert_implements_aegp_plugin<T: AegpPlugin>() {}
            fn call_with_main_type() { assert_implements_aegp_plugin::<$main_type>(); }
        };

        unsafe impl $crate::AegpSeal for $main_type {}

        // SAFETY: This entire function is an FFI entry point called by After Effects for AEGP plugin
        // initialization. It's marked unsafe because it's an extern "C" function that interfaces with
        // C code. The function is safe to call from AE because:
        // 1. All parameters are provided by AE and follow the AEGP plugin contract
        // 2. pica_basic points to a valid SPBasicSuite structure with static lifetime
        // 3. global_refcon is a valid mutable pointer where we store our plugin instance
        // 4. We properly manage memory by boxing the plugin instance and transferring ownership
        //    through the raw pointer
        // 5. On error, we explicitly null the refcon to signal initialization failure
        // The #[unsafe(no_mangle)] attribute is required to prevent name mangling for C interop.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn EntryPointFunc(
            pica_basic: *const $crate::sys::SPBasicSuite,
            major_version: i32,
            minor_version: i32,
            aegp_plugin_id: $crate::sys::AEGP_PluginID,
            global_refcon: *mut $crate::sys::AEGP_GlobalRefcon,
        ) -> Error {
            #[cfg(target_os = "windows")]
            {
                let _ = $crate::log::set_logger(&$crate::win_dbg_logger::DEBUGGER_LOGGER);
            }
            #[cfg(target_os = "macos")]
            {
                let _ = $crate::oslog::OsLogger::new(env!("CARGO_PKG_NAME")).init();
            }
            $crate::log::set_max_level($crate::log::LevelFilter::Debug);
            $crate::log::debug!(
                "Logging initialized for {} - entry point found.",
                env!("PIPL_NAME")
            );

            let mut basic_suite = $crate::PicaBasicSuite::from_sp_basic_suite_raw(pica_basic);

            let result = <$main_type>::entry_point(major_version, minor_version, aegp_plugin_id);

            // When the basic suite `Drop` runs it removes the basic suite
            // pointer from memory and nulls it. For AEGP's it's standard to
            // store the pointer in the global ref con. Doing it statically by
            // leaking the basic suite here is a more general solution.
            std::mem::forget(basic_suite);

            match result {
                Ok(t) => {
                    let boxed_instance = Box::new(t);
                    // SAFETY: global_refcon is a valid mutable pointer provided by AE where we must
                    // store our plugin instance pointer. We're writing a pointer obtained from
                    // Box::into_raw, transferring ownership to AE. This is safe because: (1) global_refcon
                    // is guaranteed valid by the AEGP contract, (2) Box::into_raw produces a valid
                    // non-null pointer to heap memory, (3) the cast to *mut _ preserves pointer validity,
                    // (4) AE will preserve this pointer and pass it back to us in callbacks. Undefined
                    // behavior would occur if global_refcon was null or invalid.
                    *global_refcon = Box::into_raw(boxed_instance) as *mut _;
                    $crate::log::debug!("AEGP setup successful for {}.", env!("PIPL_NAME"));
                    Error::None
                }
                Err(e) => {
                    // SAFETY: global_refcon is a valid mutable pointer provided by AE. We're writing
                    // null to signal that initialization failed and no instance was created. This is
                    // safe because: (1) global_refcon is guaranteed valid by AEGP contract, (2) null
                    // is a valid value for this pointer field, (3) this signals to AE that the plugin
                    // failed to initialize. Undefined behavior would occur only if global_refcon itself
                    // was an invalid pointer.
                    *global_refcon = std::ptr::null_mut();
                    $crate::log::error!("Error while setting up {}.", env!("PIPL_NAME"));
                    $crate::log::error!("{:?}", e.clone());
                    e.into()
                }
            }
        }
    };
}
