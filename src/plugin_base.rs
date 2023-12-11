#[macro_export]
macro_rules! register_plugin {
	($plugin_type:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn PluginDataEntryFunction2(
            in_ptr: ae_sys::PF_PluginDataPtr,
            in_plugin_data_callback_ptr: ae_sys::PF_PluginDataCB2,
            _in_sp_basic_suite_ptr: *const ae_sys::SPBasicSuite,
            _in_host_name: *const std::ffi::c_char,
            _in_host_version: *const std::ffi::c_char) -> ae_sys::PF_Err
        {
            if !$plugin_type::can_load(host_name: &str, host_version: &str) {
                // Plugin said we don't want to load in this host, so exit here
                return ae_sys::PF_Err_INVALID_CALLBACK as ae_sys::PF_Err;
            }
            if let Some(cb_ptr) = in_plugin_data_callback_ptr {
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
                ae_sys::PF_Err_INVALID_CALLBACK as ae_sys::PF_Err
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn EffectMain(
            cmd: ae_sys::PF_Cmd,
            in_data_ptr: *const ae_sys::PF_InData,
            out_data: *mut ae_sys::PF_OutData,
            params: *mut *mut ae_sys::PF_ParamDef,
            output: *mut ae_sys::PF_LayerDef,
            extra: *mut std::ffi::c_void) -> ae_sys::PF_Err
        {
            let _pica = ae::PicaBasicSuite::from_pf_in_data_raw(in_data_ptr);

            let err = Error::None;

            let in_data = ae::pf::InDataHandle::from_raw(in_data_ptr);

            let command = ae::pf::Command::try_from(cmd).unwrap();

            log::info!("cmd: {cmd}, in seq: {:?}, out seq: {:?}", (*in_data_ptr).sequence_data, (*out_data).sequence_data);

            match command {

            }


            err.into()
        }
	};
}
/*

pub trait AdobePlugin {
    fn can_load(host_name: &str, host_version: &str) -> bool;

    fn new() -> Self;
    fn handle_command(
        &mut self,
        command: Command,
        in_data: &InDataHandle,
        out_data: &mut OutDataHandle,
        params: &[ParamDef],
    );
}

struct MyPlugin {
    //
}

impl AdobePlugin for MyPlugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn new() -> Self {
        Self {}
    }

    fn handle_command(
        &mut self,
        command: Command,
        in_data: &InDataHandle,
        out_data: &mut OutDataHandle,
        params: &[ParamDef],
    ) {
        match command {
            Command::About => {
                out_data.set_return_msg("Hello");
            }
        }
    }
}

register_plugin!(MyPlugin);
*/