#[macro_export]
macro_rules! define_plugin {
	($global_type:ty, $sequence_type:ty, $params_type:ty) => {
        use $crate::*;
        use std::collections::HashMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        struct PluginState<'global, 'sequence, 'params> {
            global: &'global mut $global_type,
            sequence: Option<&'sequence mut $sequence_type>,
            params: &'params mut $crate::Parameters<$params_type>,
            in_data: $crate::InData,
            out_data: $crate::OutData
        }

        struct GlobalData {
            params_map: Rc<RefCell<HashMap<$params_type, usize>>>,
            params_num: usize,
            plugin_instance: $global_type
        }

        trait AdobePluginGlobal : Default {
            fn can_load(host_name: &str, host_version: &str) -> bool;

            fn params_setup(&self, params: &mut Parameters<$params_type>) -> Result<(), Error>;

            fn handle_command(&mut self, command: Command, in_data: InData, out_data: OutData, ) -> Result<(), Error>;
        }
        trait AdobePluginInstance : Default {
            fn flatten(&self) -> Result<(u16, Vec<u8>), Error>;
            fn unflatten(version: u16, serialized: &[u8]) -> Result<Self, Error>;

            fn render(&self, plugin: &mut PluginState, in_layer: &Layer, out_layer: &mut Layer) -> Result<(), ae::Error>;

            #[cfg(does_dialog)]
            fn do_dialog(&mut self, plugin: &mut PluginState) -> Result<(), ae::Error>;

            fn user_changed_param(&mut self, plugin: &mut PluginState, param: $params_type) -> Result<(), ae::Error>;

            fn handle_command(&mut self, plugin: &mut PluginState, command: Command) -> Result<(), Error>;
        }
        impl AdobePluginInstance for () {
            fn flatten(&self) -> Result<(u16, Vec<u8>), Error> { Ok((0, Vec::new())) }
            fn unflatten(_: u16, _: &[u8]) -> Result<Self, Error> { Ok(Default::default()) }
            fn render(&self, _: &mut PluginState, _: &Layer, _: &mut Layer) -> Result<(), ae::Error> { Ok(()) }
            fn user_changed_param(&mut self, _: &mut PluginState, _: $params_type) -> Result<(), ae::Error> { Ok(()) }
            fn handle_command(&mut self, _: &mut PluginState, _: Command) -> Result<(), Error> { Ok(()) }

            #[cfg(does_dialog)]
            fn do_dialog(&mut self, _: &mut PluginState) -> Result<(), ae::Error> { Ok(()) }
        }

        unsafe fn get_sequence_handle<'a, S: AdobePluginInstance>(cmd: RawCommand, in_data: &InData) -> Result<Option<(pf::Handle::<'a, S>, bool)>, Error> {
            Ok(if std::any::type_name::<S>() == "()" {
                // Don't allocate sequence data
                None
            } else if cmd == RawCommand::SequenceSetup {
                // Allocate new sequence data
                Some((pf::Handle::new(S::default())?, true))
            } else if cmd == RawCommand::SequenceResetup {
                // Restore from flat handle
                if (*in_data.as_ptr()).sequence_data.is_null() {
                    Some((pf::Handle::new(S::default())?, true))
                } else {
                    let instance = FlatHandle::from_raw((*in_data.as_ptr()).sequence_data as after_effects_sys::PF_Handle)?;
                    let bytes = instance.as_slice().ok_or(Error::InvalidIndex)?;
                    if bytes.len() < 2 {
                        return Ok(None);
                    }
                    let version = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

                    let handle = pf::Handle::new(S::unflatten(version, &bytes[2..]).map_err(|_| Error::Struct)?)?;
                    Some((handle, true))
                }
            } else if cmd == RawCommand::Render
                   || cmd == RawCommand::SmartRender
                   || cmd == RawCommand::SmartRenderGpu
                   || cmd == RawCommand::SmartPreRender
                   || cmd == RawCommand::FrameSetup
                   || cmd == RawCommand::ArbitraryCallback
                   || cmd == RawCommand::FrameSetdown {
                // Read-only sequence data available through a suite only
                let seq_ptr = pf::EffectSequenceDataSuite::new()
                    .and_then(|x| x.const_sequence_data(&in_data))
                    .unwrap_or((*in_data.as_ptr()).sequence_data as *const _);
                if !seq_ptr.is_null() {
                    let instance_handle = pf::Handle::<S>::from_raw(seq_ptr as *mut _)?;
                    Some((instance_handle, false))
                } else {
                    log::error!("Sequence data pointer got through EffectSequenceDataSuite is null in cmd: {:?}!", cmd);
                    None
                }
            } else {
                if (*in_data.as_ptr()).sequence_data.is_null() {
                    log::error!("Sequence data pointer is null in cmd: {:?}!", cmd);
                    None
                } else {
                    let instance_handle = pf::Handle::<S>::from_raw((*in_data.as_ptr()).sequence_data)?;
                    Some((instance_handle, false))
                }
            })
        }

        unsafe fn handle_effect_main<T: AdobePluginGlobal, S: AdobePluginInstance, P>(
            cmd: after_effects_sys::PF_Cmd,
            in_data_ptr: *mut after_effects_sys::PF_InData,
            out_data_ptr: *mut after_effects_sys::PF_OutData,
            params: *mut *mut after_effects_sys::PF_ParamDef,
            output: *mut after_effects_sys::PF_LayerDef,
            extra: *mut std::ffi::c_void) -> Result<(), Error>
        {
            let _pica = crate::PicaBasicSuite::from_pf_in_data_raw(in_data_ptr);

            let in_data = InData::from_raw(in_data_ptr);
            let out_data = OutData::from_raw(out_data_ptr);

            let cmd = RawCommand::from(cmd);

            // Allocate or restore global data pointer
            let mut global_handle = if cmd == RawCommand::GlobalSetup {
                // Allocate global data
                pf::Handle::new(GlobalData {
                    params_map: Rc::new(RefCell::new(HashMap::new())),
                    params_num: 1,
                    plugin_instance: <$global_type>::default()
                })?
            } else {
                if (*in_data_ptr).global_data.is_null() {
                    log::error!("Global data pointer is null in cmd: {:?}!", cmd);
                    return Err(Error::BadCallbackParameter);
                }
                pf::Handle::<GlobalData>::from_raw((*in_data_ptr).global_data)?
            };

            // Allocate or restore sequence data pointer
            let sequence_handle = get_sequence_handle::<$sequence_type>(cmd, &in_data).unwrap_or(None);

            let global_lock = global_handle.lock()?;
            let global_inst = global_lock.as_ref_mut()?;

            if cmd == RawCommand::ParamsSetup {
                let mut params = Parameters::<$params_type>::new(global_inst.params_map.clone());
                params.set_in_data(in_data_ptr);
                global_inst.plugin_instance.params_setup(&mut params)?;
                global_inst.params_num = params.num_params();
                (*out_data_ptr).num_params = params.num_params() as i32;
            }

            let mut plugin_state = PluginState {
                global: &mut global_inst.plugin_instance,
                sequence: sequence_handle.as_ref().map(|x| x.0.as_mut().unwrap()),
                params: &mut Parameters::<$params_type>::with_params(in_data_ptr, params, global_inst.params_map.clone(), global_inst.params_num),
                in_data,
                out_data
            };

            let command = Command::from_entry_point(cmd, in_data_ptr, params, output, extra);

            let global_err = plugin_state.global.handle_command(command, in_data, out_data);
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
                        let in_layer = $crate::Layer::from_raw(&mut (*(*params)).u.ld, &in_data);
                        let mut out_layer = $crate::Layer::from_raw(output, &in_data);
                        sequence_err = Some(inst.render(&mut plugin_state, &in_layer, &mut out_layer));
                    }
                    RawCommand::UserChangedParam => {
                        let extra = extra as *mut after_effects_sys::PF_UserChangedParamExtra;
                        let param = plugin_state.params.type_for_index((*extra).param_index as usize);
                        sequence_err = Some(inst.user_changed_param(&mut plugin_state, param));
                    }
                    _ => { }
                }

                match cmd {
                    RawCommand::SequenceSetup | RawCommand::SequenceResetup => {
                        drop(lock);
                        (*out_data_ptr).sequence_data = pf::Handle::into_raw(sequence_handle);
                    }
                    RawCommand::SequenceFlatten | RawCommand::GetFlattenedSequenceData => {
                        let serialized = inst.flatten().map_err(|_| Error::InternalStructDamaged)?;
                        drop(lock);
                        if cmd == RawCommand::GetFlattenedSequenceData {
                            let _ = pf::Handle::into_raw(sequence_handle); // don't deallocate
                        } else {
                            drop(sequence_handle);
                        }
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
                        let _ = pf::Handle::into_raw(sequence_handle); // don't deallocate
                    }
                }
            }

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
                    let _ = pf::Handle::into_raw(global_handle); // don't deallocate
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

        #[no_mangle]
        pub unsafe extern "C" fn PluginDataEntryFunction2(
            in_ptr: after_effects_sys::PF_PluginDataPtr,
            in_plugin_data_callback_ptr: after_effects_sys::PF_PluginDataCB2,
            in_sp_basic_suite_ptr: *const after_effects_sys::SPBasicSuite,
            in_host_name: *const std::ffi::c_char,
            in_host_version: *const std::ffi::c_char) -> after_effects_sys::PF_Err
        {
            // let _pica = ae::PicaBasicSuite::from_sp_basic_suite_raw(in_sp_basic_suite_ptr);

            if in_host_name.is_null() || in_host_version.is_null() {
                return after_effects_sys::PF_Err_INVALID_CALLBACK as after_effects_sys::PF_Err;
            }

            let in_host_name = std::ffi::CStr::from_ptr(in_host_name);
            let in_host_version = std::ffi::CStr::from_ptr(in_host_version);

            if !<$global_type>::can_load(in_host_name.to_str().unwrap(), in_host_version.to_str().unwrap()) {
                // Plugin said we don't want to load in this host, so exit here
                return after_effects_sys::PF_Err_INVALID_CALLBACK as after_effects_sys::PF_Err;
            }
            if let Some(cb_ptr) = in_plugin_data_callback_ptr {
                use $crate::cstr_literal::cstr;
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
            } else {
                after_effects_sys::PF_Err_INVALID_CALLBACK as after_effects_sys::PF_Err
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn EffectMain(
            cmd: after_effects_sys::PF_Cmd,
            in_data_ptr: *mut after_effects_sys::PF_InData,
            out_data_ptr: *mut after_effects_sys::PF_OutData,
            params: *mut *mut after_effects_sys::PF_ParamDef,
            output: *mut after_effects_sys::PF_LayerDef,
            extra: *mut std::ffi::c_void) -> after_effects_sys::PF_Err
        {
            if cmd == after_effects_sys::PF_Cmd_GLOBAL_SETUP as after_effects_sys::PF_Cmd {
                (*out_data_ptr).my_version = env!("PIPL_VERSION")  .parse::<u32>().unwrap();
                (*out_data_ptr).out_flags  = env!("PIPL_OUTFLAGS") .parse::<i32>().unwrap();
                (*out_data_ptr).out_flags2 = env!("PIPL_OUTFLAGS2").parse::<i32>().unwrap();
            }

            #[cfg(threaded_rendering)]
            {
                fn assert_impl<T: Sync>() { }
                assert_impl::<$global_type>();
                assert_impl::<$sequence_type>();
            }

            const _: () = assert!(std::mem::size_of::<$sequence_type>() > 0, concat!("Instance type `", stringify!($sequence_type), "` cannot be zero-sized"));

            // log::info!("EffectMain start {:?} {:?}", RawCommand::from(cmd), std::thread::current().id());
            // struct X { cmd: i32 } impl Drop for X { fn drop(&mut self) { log::info!("EffectMain end {:?} {:?}", RawCommand::from(self.cmd), std::thread::current().id()); } }
            // let _x = X { cmd: cmd as i32 };

            match handle_effect_main::<$global_type, $sequence_type, $params_type>(cmd, in_data_ptr, out_data_ptr, params, output, extra) {
                Ok(_) => after_effects_sys::PF_Err_NONE as after_effects_sys::PF_Err,
                Err(e) => {
                    log::error!("EffectMain returned error: {e:?}");
                    e as after_effects_sys::PF_Err
                }
            }
        }
	};
}
