use after_effects_sys as ae_sys;
use after_effects as ae;
use cstr::cstr;

#[repr(usize)]
enum PluginParams {
    InputLayer = 0,
    Slider = 1
}

#[derive(Default)]
struct PortableRenderInfo {
    slider_value: f64
}

fn detect_host(in_data: ae::InDataHandle, out_data: *mut ae_sys::PF_OutData) {
    use ae_sys::*;
    let v = (in_data.version().0 as u32, in_data.version().1 as u32);

    let app = match &in_data.application_id() {
        b"FXTC" => {
            if v.0 >= 12 {
                     if v.0 == PF_AE234_PLUG_IN_VERSION && v.1 >= PF_AE234_PLUG_IN_SUBVERS { "After Effects 2023 (23.4) or later." }
                else if v.0 == PF_AE220_PLUG_IN_VERSION && v.1 == PF_AE220_PLUG_IN_SUBVERS { "After Effects 2022 (22.0)." }
                else if v.0 == PF_AE184_PLUG_IN_VERSION && v.1 == PF_AE184_PLUG_IN_SUBVERS { "After Effects 2021 (18.4)." }
                else if v.0 == PF_AE182_PLUG_IN_VERSION && v.1 == PF_AE182_PLUG_IN_SUBVERS { "After Effects 2021 (18.2)." }
                else if v.0 == PF_AE180_PLUG_IN_VERSION && v.1 == PF_AE180_PLUG_IN_SUBVERS { "After Effects 2021 (18.0)." }
                else if v.0 == PF_AE177_PLUG_IN_VERSION && v.1 == PF_AE177_PLUG_IN_SUBVERS { "After Effects 2020 (17.7)." }
                else if v.0 == PF_AE176_PLUG_IN_VERSION && v.1 == PF_AE176_PLUG_IN_SUBVERS { "After Effects 2020 (17.6)." }
                else if v.0 == PF_AE175_PLUG_IN_VERSION && v.1 == PF_AE175_PLUG_IN_SUBVERS { "After Effects 2020 (17.5)." }
                else if v.0 == PF_AE171_PLUG_IN_VERSION && v.1 >= PF_AE171_PLUG_IN_SUBVERS { "After Effects 2020 (17.1)." }
                else if v.0 == PF_AE170_PLUG_IN_VERSION && v.1 == PF_AE170_PLUG_IN_SUBVERS { "After Effects 2020 (17.0)." }
                else if v.0 == PF_AE161_PLUG_IN_VERSION && v.1 == PF_AE161_PLUG_IN_SUBVERS { "After Effects 2019 (16.1)." }
                else if v.0 == PF_AE160_PLUG_IN_VERSION && v.1 >= PF_AE160_PLUG_IN_SUBVERS { "After Effects CC 2019 (16.0)." }
                else if v.0 == PF_AE151_PLUG_IN_VERSION && v.1 >= PF_AE151_PLUG_IN_SUBVERS { "After Effects CC 2018 (15.1)." }
                else if v.0 == PF_AE150_PLUG_IN_VERSION && v.1 >= PF_AE150_PLUG_IN_SUBVERS { "After Effects CC 2017 (15.0)." }
                else if v.0 == PF_AE140_PLUG_IN_VERSION && v.1 >= PF_AE140_PLUG_IN_SUBVERS { "After Effects CC 2017 (14.0)." }
                else if v.0 == PF_AE138_PLUG_IN_VERSION && v.1 == PF_AE138_PLUG_IN_SUBVERS { "After Effects CC 2015.3 (13.8)." }
                else if v.0 == PF_AE136_PLUG_IN_VERSION && v.1 == PF_AE136_PLUG_IN_SUBVERS { "After Effects CC 2015.1 or 2015.2 (13.6 or 13.7)."  }
                else if v.0 == PF_AE135_PLUG_IN_VERSION && v.1 == PF_AE135_PLUG_IN_SUBVERS { "After Effects CC 2015 (13.5)." }
                else if v.0 == PF_AE130_PLUG_IN_VERSION && v.1 == PF_AE130_PLUG_IN_SUBVERS { "After Effects CC 2014 (13.0 - 13.2)." }
                else if v.0 == PF_AE122_PLUG_IN_VERSION && v.1 == PF_AE122_PLUG_IN_SUBVERS { "After Effects CC (12.2)." }
                else if v.0 == PF_AE121_PLUG_IN_VERSION && v.1 == PF_AE121_PLUG_IN_SUBVERS { "After Effects CC (12.1)." }
                else if v.0 == PF_AE120_PLUG_IN_VERSION && v.1 == PF_AE120_PLUG_IN_SUBVERS { "After Effects CC (12.0)." }
                else if v.0 == PF_AE1101_PLUG_IN_VERSION && v.1 == PF_AE1101_PLUG_IN_SUBVERS { "After Effects CS6.0.1 or CS6.0.2." }
                else if v.0 == PF_AE110_PLUG_IN_VERSION && v.1 == PF_AE110_PLUG_IN_SUBVERS { "After Effects CS6.0." }
                else {
                    // Q. How can I tell the difference between versions where the API version is the same, such as AE 6.5 and 7.0?
                    // A. The effect API didn't change the only way to differentiate between them is to check for the presence of a version of a suite new in 7.0.
                    // Say, something 32bpc-ish. To avoid AEGP_SuiteHandler throwing if the suite isn't present, we'll acquire it the old-school way.
                    if let Ok(_) = ae::pf::IterateFloatSuite::new() {
                        "After Effects between 7.0 and CS4."
                    } else {
                        "After Effects 6.5 or earlier."
                    }
                }
            } else { // Wow, an antique!
                "some unknown version of After Effects!"
            }
        },
        b"PrMr" => {
            // let pixel_format = ae::pf::PixelFormatSuite::new().unwrap();
            // pixel_format.clear_supported_pixel_formats(in_data.effect_ref()).unwrap();
            // pixel_format.add_supported_pixel_format(in_data.effect_ref(), ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f).unwrap();

            // The major/minor versions provide basic differentiation.
            // If you need finer granularity, e.g. differentiating between
            // PPro CC 2015.3 and CC 2017, then use the App Info Suite from/ the Premiere Pro SDK
            if v.0 == 13 && v.1 >= 4 {
                "Premiere Pro CC, CC 2014, or later!"
            } else if v.0 == 13 && v.1 == 2 {
                "Premiere Pro CS6!"
            } else {
                "some unknown version of Premiere!"
            }
        },
        _ => {
            "some oddball host."
        }
    };

    log::info!("Running in {app}");

    unsafe { write_str(&mut (*out_data).return_msg, format!("Running in {app}")) };
}

unsafe extern "C" fn portable_func(refcon: *mut std::ffi::c_void, _x: i32, _y: i32, in_p: *mut ae_sys::PF_Pixel, out_p: *mut ae_sys::PF_Pixel) -> ae_sys::PF_Err {
    if refcon.is_null() {
        return ae_sys::PF_Err_BAD_CALLBACK_PARAM as ae_sys::PF_Err;
    }
    let render_info = refcon as *const PortableRenderInfo;
    let slider_value = (*render_info).slider_value;

    // Mix the values. The higher the slider, the more we blend the channel with the average of all channels
    let average = ((*in_p).red as f64 + (*in_p).green as f64 + (*in_p).blue as f64) / 3.0;
    // let midway_calc = (slider_value * average) + (200.0 - slider_value) * (*in_p).red as f64;

    (*out_p).alpha = (*in_p).alpha;
    (*out_p).red   = (((slider_value * average) + (100.0 - slider_value) * (*in_p).red as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8;
    (*out_p).green = (((slider_value * average) + (100.0 - slider_value) * (*in_p).green as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8;
    (*out_p).blue  = (((slider_value * average) + (100.0 - slider_value) * (*in_p).blue as f64) / 100.0).min(ae_sys::PF_MAX_CHAN8 as f64) as u8;

    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

fn render(in_data: ae::pf::InDataHandle, params: *mut *mut ae_sys::PF_ParamDef, output: *mut ae_sys::PF_LayerDef) -> ae_sys::PF_Err {
    let mut err = ae_sys::PF_Err_NONE as ae_sys::PF_Err;
    let mut render_info = PortableRenderInfo::default();

    if let ae::Param::FloatSlider(slider) = ae::pf::ParamDef::from_raw(in_data.as_ptr(), unsafe { *params.add(PluginParams::Slider as usize) }).to_param() {
        render_info.slider_value = slider.value();
    }

    let in_layer = unsafe { &mut (*(*params.add(PluginParams::InputLayer as usize))).u.ld };

    // If the slider is 0 just make a direct copy.
    if render_info.slider_value < 0.001 {
        unsafe {
            if let Some(copy_fn) = (*(*in_data.as_ptr()).utils).copy {
                err = copy_fn((*in_data.as_ptr()).effect_ref, in_layer, output, std::ptr::null_mut(), std::ptr::null_mut());
            }
        }
    } else {
        let extent_hint = in_data.extent_hint();
        // clear all pixels outside extent_hint.
        if extent_hint.left   != extent_hint.left  ||
           extent_hint.top    != extent_hint.top   ||
           extent_hint.right  != extent_hint.right ||
           extent_hint.bottom != extent_hint.bottom {
            unsafe {
                if let Some(fill_fn) = (*(*in_data.as_ptr()).utils).fill {
                    err = fill_fn((*in_data.as_ptr()).effect_ref, std::ptr::null_mut(), &mut (*output).extent_hint, output);
                }
            }
        }

        if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
            // iterate over image data.
            let progress_height = extent_hint.top - extent_hint.bottom;
            unsafe {
                if let Some(iterate_fn) = (*(*in_data.as_ptr()).utils).iterate {
                    err = iterate_fn(in_data.as_ptr() as *mut _, 0, progress_height, in_layer, &ae_sys::PF_LRect::from(extent_hint), (&mut render_info) as *mut _ as *mut _, Some(portable_func), output);
                }
            }
        }
    }

    err
}

const AE_RESERVED_INFO: ae_sys::A_long = 8;

use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn PluginDataEntryFunction2(
    in_ptr: ae_sys::PF_PluginDataPtr,
    in_plugin_data_callback_ptr: ae_sys::PF_PluginDataCB2,
    in_sp_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    in_host_name: *const std::ffi::c_char,
    in_host_version: *const std::ffi::c_char) -> ae_sys::PF_Err
{
    log::set_max_level(log::LevelFilter::Debug);

    let _pica = ae::PicaBasicSuite::from_sp_basic_suite_raw(in_sp_basic_suite_ptr);

    log::info!("PluginDataEntryFunction2: {:?}, {:?}", CStr::from_ptr(in_host_name), CStr::from_ptr(in_host_version));

    if let Some(cb_ptr) = in_plugin_data_callback_ptr {
        cb_ptr(in_ptr,
            cstr!("Portable")       .as_ptr() as *const u8, // Name
            cstr!("ADBE Portable")  .as_ptr() as *const u8, // Match Name
            cstr!("Sample Plug-ins").as_ptr() as *const u8, // Category
            cstr!("EffectMain")     .as_ptr() as *const u8, // Entry point
            i32::from_be_bytes(*b"eFKT"),
            ae_sys::PF_AE_PLUG_IN_VERSION as i32,
            ae_sys::PF_AE_PLUG_IN_SUBVERS as i32,
            AE_RESERVED_INFO,
            cstr!("https://www.adobe.com").as_ptr() as *const u8,
        )
    } else {
        ae_sys::PF_Err_INVALID_CALLBACK as ae_sys::PF_Err
    }
}

fn write_str(ae_buffer: &mut [ae_sys::A_char], s: String) {
    let a = std::ffi::CString::new(s).unwrap();
    let b = a.to_bytes_with_nul();
    assert!(b.len() < ae_buffer.len());

    ae_buffer[0..b.len()].copy_from_slice(unsafe { std::mem::transmute(b) });
}

#[no_mangle]
pub unsafe extern "C" fn EffectMain(
    cmd: ae_sys::PF_Cmd,
    in_data: *const ae_sys::PF_InData,
    out_data: *mut ae_sys::PF_OutData,
    params: *mut *mut ae_sys::PF_ParamDef,
    output: *mut ae_sys::PF_LayerDef,
    _extra: *mut std::ffi::c_void) -> ae_sys::PF_Err
{
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);

    let mut err = ae_sys::PF_Err_NONE as ae_sys::PF_Err;

    let in_data = ae::pf::InDataHandle::from_raw(in_data);

    match cmd as ae::EnumIntType {
        ae_sys::PF_Cmd_ABOUT => {
            write_str(&mut (*out_data).return_msg,
                format!("Portable, v3.3\rThis example shows how to detect and \t respond to different hosts.\rCopyright 2007-2023 Adobe Inc.")
            );
        },
        ae_sys::PF_Cmd_GLOBAL_SETUP => {
            (*out_data).my_version = env!("PIPL_VERSION").parse::<u32>().unwrap();
            (*out_data).out_flags  = env!("PIPL_OUTFLAGS").parse::<i32>().unwrap();
            (*out_data).out_flags2 = env!("PIPL_OUTFLAGS2").parse::<i32>().unwrap();
        },
        ae_sys::PF_Cmd_PARAMS_SETUP => {
            let mut float_param = ae::FloatSliderDef::new();
            float_param.set_valid_min(0.0);
            float_param.set_slider_min(0.0);
            float_param.set_valid_max(200.0);
            float_param.set_slider_max(200.0);
            float_param.set_value(10.0);
            float_param.set_default(10.0);
            float_param.precision(1);
            float_param.display_flags(ae::ValueDisplayFlag::PERCENT);

            let mut param = ae::ParamDef::new(in_data);
            param.name("Mix channels");
            param.param(ae::Param::FloatSlider(float_param));
            param.add(-1);

            (*out_data).num_params = 2;
        },
        ae_sys::PF_Cmd_SEQUENCE_SETUP => {
            detect_host(in_data, out_data);
        },
        ae_sys::PF_Cmd_RENDER => {
            err = render(in_data, params, output);
        },
        _ => {
            log::debug!("Unknown cmd: {cmd:?}");
        }
    }

    err
}
