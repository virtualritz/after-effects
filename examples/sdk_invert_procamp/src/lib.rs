use after_effects_sys as ae_sys;
use after_effects as ae;
use cstr_literal::cstr;

mod filter_image_cpu;

#[repr(usize)]
enum PluginParams {
    InputLayer = 0,
    Brightness,
    Contrast,
    Hue,
    Saturation,
    NumParams
}

extern "C" {
    //fn ProcAmp_CUDA(src: *const f32, dst: *mut f32, srcPitch: u32, dstPitch: u32, is16f: i32, width: u32, height: u32, inBrightness: f32, inContrast: f32, inHueCosSaturation: f32, inHueSinSaturation: f32);
    //fn Invert_Color_CUDA(src: *const f32, dst: *mut f32, srcPitch: u32, dstPitch: u32, is16f: i32, width: u32, height: u32);
}

#[derive(Default)]
struct InvertProcAmpParams {
	brightness: f32,
	contrast: f32,
	hue_cos_saturation: f32,
	hue_sin_saturation: f32,
}

#[no_mangle]
pub unsafe extern "C" fn PluginDataEntryFunction2(
    in_ptr: ae_sys::PF_PluginDataPtr,
    in_plugin_data_callback_ptr: ae_sys::PF_PluginDataCB2,
    _in_sp_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    in_host_name: *const std::ffi::c_char,
    in_host_version: *const std::ffi::c_char) -> ae_sys::PF_Err
{
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("PluginDataEntryFunction2: {:?}, {:?}", std::ffi::CStr::from_ptr(in_host_name), std::ffi::CStr::from_ptr(in_host_version));

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

fn write_str(ae_buffer: &mut [ae_sys::A_char], s: String) {
    let buf = std::ffi::CString::new(s).unwrap().into_bytes_with_nul();
    ae_buffer[0..buf.len()].copy_from_slice(unsafe { std::mem::transmute(buf.as_slice()) });
}

fn union_rect(src: &ae_sys::PF_LRect, dst: &mut ae_sys::PF_LRect) {
    fn is_empty_rect(r: &ae_sys::PF_LRect) -> bool{
        (r.left >= r.right) || (r.top >= r.bottom)
    }
	if is_empty_rect(dst) {
		*dst = *src;
	} else if !is_empty_rect(src) {
		dst.left 	= dst.left.min(src.left);
		dst.top  	= dst.top.min(src.top);
		dst.right 	= dst.right.max(src.right);
		dst.bottom  = dst.bottom.max(src.bottom);
	}
}
unsafe extern "C" fn dispose_data(d: *mut std::ffi::c_void) { let _ = Box::<InvertProcAmpParams>::from_raw(d as _); }
unsafe fn pre_render(in_data: ae::pf::InDataHandle, extra: *mut ae_sys::PF_PreRenderExtra) -> ae_sys::PF_Err {
	let req = (*(*extra).input).output_request;

	(*(*extra).output).flags |= ae_sys::PF_RenderOutputFlag_GPU_RENDER_POSSIBLE as i16;

	let mut info: Box<InvertProcAmpParams> = Box::new(InvertProcAmpParams::default());

	// Querying parameters to demonstrate they are available at PreRender, and data can be passed from PreRender to Render with pre_render_data.
    if let ae::Param::FloatSlider(slider) = ae::pf::ParamDef::checkout(in_data, PluginParams::Brightness as i32, in_data.current_time(), in_data.time_step(), in_data.time_scale()).to_param() {
        info.brightness = slider.value() as f32 / 100.0;
    }
    if let ae::Param::FloatSlider(slider) = ae::pf::ParamDef::checkout(in_data, PluginParams::Contrast as i32, in_data.current_time(), in_data.time_step(), in_data.time_scale()).to_param() {
        info.contrast = slider.value() as f32 / 100.0;
    }
    if let ae::Param::Angle(slider) = ae::pf::ParamDef::checkout(in_data, PluginParams::Hue as i32, in_data.current_time(), in_data.time_step(), in_data.time_scale()).to_param() {
        let hue_degrees = slider.value() as f32;
        if let ae::Param::FloatSlider(slider) = ae::pf::ParamDef::checkout(in_data, PluginParams::Saturation as i32, in_data.current_time(), in_data.time_step(), in_data.time_scale()).to_param() {
            let saturation = slider.value() as f32 / 100.0;

            let hue_radians = (std::f32::consts::PI / 180.0) * hue_degrees;
            info.hue_cos_saturation = hue_radians.cos() * saturation;
            info.hue_sin_saturation = hue_radians.sin() * saturation;
        }
    }

	(*(*extra).output).pre_render_data = Box::into_raw(info) as *mut std::ffi::c_void;
	(*(*extra).output).delete_pre_render_data_func = Some(dispose_data);

    let pre_render_cb = ae::pf::PreRenderCallbacks::from_raw((*extra).cb);
    if let Ok(in_result) = pre_render_cb.checkout_layer(in_data.effect_ref(), PluginParams::InputLayer as i32, 0, &req, in_data.current_time(), in_data.time_step(), in_data.time_scale()) {
        union_rect(&in_result.result_rect,     &mut (*(*extra).output).result_rect);
        union_rect(&in_result.max_result_rect, &mut (*(*extra).output).max_result_rect);
    }

	ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

fn smart_render_cpu(in_data: ae::pf::InDataHandle, pixel_format: ae_sys::PF_PixelFormat, input_world: ae::EffectWorld, output_world: ae::EffectWorld, info: *const InvertProcAmpParams) -> ae_sys::PF_Err {
    match pixel_format {
        ae_sys::PF_PixelFormat_ARGB128 => {
            if let Ok(suite) = ae::pf::IterateFloatSuite::new() {
                suite.iterate(in_data, 0, output_world.height() as i32, input_world, None, info as *const _, Some(filter_image_cpu::filter_image_32), output_world).unwrap();
            }
        },
        ae_sys::PF_PixelFormat_ARGB64 => {
            if let Ok(suite) = ae::pf::Iterate16Suite::new() {
                suite.iterate(in_data, 0, output_world.height() as i32, input_world, None, info as *const _, Some(filter_image_cpu::filter_image_16), output_world).unwrap();
            }
        },
        ae_sys::PF_PixelFormat_ARGB32 => {
            if let Ok(suite) = ae::pf::Iterate8Suite::new() {
                suite.iterate(in_data, 0, output_world.height() as i32, input_world, None, info as *const _, Some(filter_image_cpu::filter_image_8), output_world).unwrap();
            }
        },
        _ => {
            log::info!("Unhandled pixel format: {pixel_format}");
        }
    }
    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

unsafe fn smart_render(in_data: ae::pf::InDataHandle, extra: *mut ae_sys::PF_SmartRenderExtra, is_gpu: bool) -> ae_sys::PF_Err {
	// Parameters can be queried during render. In this example, we pass them from PreRender as an example of using pre_render_data.
	let info = (*(*extra).input).pre_render_data as *const InvertProcAmpParams;

	if !info.is_null() {
        let mut err = ae_sys::PF_Err_NONE as ae_sys::PF_Err;

        let cb = ae::pf::SmartRenderCallbacks::from_raw((*extra).cb);
        let input_world = cb.checkout_layer_pixels(in_data.effect_ref(), PluginParams::InputLayer as u32).unwrap();
        let output_world = cb.checkout_output(in_data.effect_ref()).unwrap();
        if let Ok(world_suite) = ae::WorldSuite2::new() {
            let pixel_format = world_suite.get_pixel_format(input_world).unwrap();
            log::info!("pixel_format: {pixel_format:?}, is_gpu: {is_gpu}");
            if is_gpu {
                // err = smart_render_gpu(in_data, out_data, pixel_format, input_worldP, output_worldP, extraP, infoP);
            } else {
                err = smart_render_cpu(in_data, pixel_format, input_world, output_world, info);
            }
        }
        cb.checkin_layer_pixels(in_data.effect_ref(), PluginParams::InputLayer as u32).unwrap();

        err
	} else {
		ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED as ae_sys::PF_Err
	}
}

unsafe fn render(in_data: ae::pf::InDataHandle, params: *mut *mut ae_sys::PF_ParamDef, dest: *mut ae_sys::PF_LayerDef) -> ae_sys::PF_Err {
    if &in_data.application_id() == b"PrMr" {
        let src = unsafe { &mut (*(*params.add(PluginParams::InputLayer as usize))).u.ld };

        if let ae::Param::FloatSlider(brightness) = ae::pf::ParamDef::from_raw(in_data.as_ptr(), unsafe { *params.add(PluginParams::Brightness as usize) }).to_param() {
            if let ae::Param::FloatSlider(contrast) = ae::pf::ParamDef::from_raw(in_data.as_ptr(), unsafe { *params.add(PluginParams::Contrast as usize) }).to_param() {
                if let ae::Param::Angle(hue) = ae::pf::ParamDef::from_raw(in_data.as_ptr(), unsafe { *params.add(PluginParams::Hue as usize) }).to_param() {
                    if let ae::Param::FloatSlider(saturation) = ae::pf::ParamDef::from_raw(in_data.as_ptr(), unsafe { *params.add(PluginParams::Saturation as usize) }).to_param() {
                        let brightness = brightness.value() as f32 / 100.0;
                        let contrast = contrast.value() as f32 / 100.0;
                        let hue = hue.value() as f32;
                        let saturation = saturation.value() as f32 / 100.0;

                        let hue_radians = (std::f32::consts::PI / 180.0) * hue;
                        let hue_cos_saturation = hue_radians.cos() * saturation;
                        let hue_sin_saturation = hue_radians.sin() * saturation;

                        let mut src_data = (*src).data as *const u8;
                        let mut dest_data = (*dest).data as *mut u8;

                        log::info!("pr render {brightness}");

                        for _ in 0..(*dest).height as usize {
                            for x in 0..(*dest).width as usize  {
                                let float_src = src_data as *const f32;
                                let mut v = *float_src.add(x * 4 + 0);
                                let mut u = *float_src.add(x * 4 + 1);
                                let mut y = *float_src.add(x * 4 + 2);
                                let a = *float_src.add(x * 4 + 3);

                                // invert
                                y = 1.0 - y;
                                u *= -1.0;
                                v *= -1.0;

                                let float_dst = dest_data as *mut f32;

                                *float_dst.add(x * 4 + 0) = (v * hue_cos_saturation) + (u *  hue_sin_saturation);
                                *float_dst.add(x * 4 + 1) = (u * hue_cos_saturation) + (v * -hue_sin_saturation);
                                *float_dst.add(x * 4 + 2) = (contrast * y) + brightness;
                                *float_dst.add(x * 4 + 3) = a;
                            }
                            src_data = src_data.add((*src).rowbytes as usize);
                            dest_data = dest_data.add((*dest).rowbytes as usize)
                        }
                    }
                }
            }
        }
	}
    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

/////////////////////////////////////////////////// GPU ///////////////////////////////////////////////////
struct OpenCLGPUData {
	invert_kernel: *mut std::ffi::c_void, // cl_kernel
	procamp_kernel: *mut std::ffi::c_void, // cl_kernel
}
impl Default for OpenCLGPUData {
    fn default() -> Self { Self { invert_kernel: std::ptr::null_mut(), procamp_kernel: std::ptr::null_mut() } }
}
struct MetalGPUData {
    invert_pipeline: *mut std::ffi::c_void, // id<MTLComputePipelineState>
    procamp_pipeline: *mut std::ffi::c_void, // id<MTLComputePipelineState>
}
impl Default for MetalGPUData {
    fn default() -> Self { Self { invert_pipeline: std::ptr::null_mut(), procamp_pipeline: std::ptr::null_mut() } }
}

unsafe fn gpu_device_setup(in_data: ae::pf::InDataHandle, out_data: *mut ae_sys::PF_OutData, extra: *mut ae_sys::PF_GPUDeviceSetupExtra) -> ae_sys::PF_Err {
    let device_info = ae::pf::GPUDeviceSuite1::new().unwrap().get_device_info(in_data, (*(*extra).input).device_index).unwrap();

    let what_gpu = (*(*extra).input).what_gpu;

    log::info!("Device info: {device_info:?}. GPU: {what_gpu}");

    match what_gpu {
        ae_sys::PF_GPU_Framework_CUDA => {
            // Nothing to do here. CUDA Kernel statically linked
            (*out_data).out_flags2 |= ae_sys::PF_OutFlag2_SUPPORTS_GPU_RENDER_F32;
        },
        ae_sys::PF_GPU_Framework_OPENCL => {
            let mut gpu_data = OpenCLGPUData::default();

            let context = device_info.contextPV as cl_sys::cl_context;
            let device = device_info.devicePV as cl_sys::cl_device_id;

            let mut result = cl_sys::CL_SUCCESS;
            let kernel = include_bytes!("kernel_opencl.cl");
            let kernel_len = kernel.len();

            let program = cl_sys::clCreateProgramWithSource(context, 1, kernel.as_ptr() as _, &kernel_len, &mut result);
            if result != cl_sys::CL_SUCCESS { log::debug!("clCreateProgramWithSource failed: {result}"); }

            result = cl_sys::clBuildProgram(program, 1, &device, cstr!("-cl-single-precision-constant -cl-fast-relaxed-math").as_ptr(), None, std::ptr::null_mut());
            if result != cl_sys::CL_SUCCESS { log::debug!("clBuildProgram failed: {result}"); }
            gpu_data.invert_kernel = cl_sys::clCreateKernel(program, cstr!("InvertColorKernel").as_ptr(), &mut result);
            if result != cl_sys::CL_SUCCESS { log::debug!("clCreateKernel failed: {result}"); }
            gpu_data.procamp_kernel = cl_sys::clCreateKernel(program, cstr!("ProcAmp2Kernel").as_ptr(), &mut result);
            if result != cl_sys::CL_SUCCESS { log::debug!("clCreateKernel failed: {result}"); }

            (*(*extra).output).gpu_data = ae::pf::Handle::into_raw(ae::pf::Handle::new(gpu_data).unwrap()) as *mut _;

            (*out_data).out_flags2 |= ae_sys::PF_OutFlag2_SUPPORTS_GPU_RENDER_F32;
        },
        ae_sys::PF_GPU_Framework_METAL => {
            let mut metal_data = MetalGPUData::default();
            /*ScopedAutoreleasePool pool;

            //Create a library from source
            NSString *source = [NSString stringWithCString:kSDK_Invert_ProcAmp_Kernel_MetalString encoding:NSUTF8StringEncoding];
            id<MTLDevice> device = (id<MTLDevice>)device_info.devicePV;

            NSError *error = nil;
            id<MTLLibrary> library = [[device newLibraryWithSource:source options:nil error:&error] autorelease];

            // An error code is set for Metal compile warnings, so use nil library as the error signal
            if(!err && !library) {
                err = NSError2PFErr(error);
            }

            // For debugging only. This will contain Metal compile warnings and erorrs.
            NSString *getError = error.localizedDescription;

            PF_Handle metal_handle = handle_suite->host_new_handle(sizeof(MetalGPUData));
            MetalGPUData *metal_data = reinterpret_cast<MetalGPUData *>(*metal_handle);

            // Create pipeline state from function extracted from library
            if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
                id<MTLFunction> invert_function = nil;
                id<MTLFunction> procamp_function = nil;
                NSString *invert_name = [NSString stringWithCString:"InvertColorKernel" encoding:NSUTF8StringEncoding];
                NSString *procamp_name = [NSString stringWithCString:"ProcAmp2Kernel" encoding:NSUTF8StringEncoding];

                invert_function =  [ [library newFunctionWithName:invert_name] autorelease];
                procamp_function = [ [library newFunctionWithName:procamp_name] autorelease];

                if (!invert_function || !procamp_function) {
                    err = PF_Err_INTERNAL_STRUCT_DAMAGED;
                }

                if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
                    metal_data->invert_pipeline = [device newComputePipelineStateWithFunction:invert_function error:&error];
                    err = NSError2PFErr(error);
                }

                if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
                    metal_data->procamp_pipeline = [device newComputePipelineStateWithFunction:procamp_function error:&error];
                    err = NSError2PFErr(error);
                }

                if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
                    (*(*extra).output).gpu_data = ae::pf::Handle::new(metal_data).into_raw();
                    (*out_data).out_flags2 |= ae_sys::PF_OutFlag2_SUPPORTS_GPU_RENDER_F32;
                }
            }*/
        },
        _ => { }
    }

    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

unsafe fn gpu_device_setdown(in_data: ae::pf::InDataHandle, out_data: *mut ae_sys::PF_OutData, extra: *mut ae_sys::PF_GPUDeviceSetdownExtra) -> ae_sys::PF_Err {
    let what_gpu = (*(*extra).input).what_gpu;

    log::info!("gpu_device_setdown: {what_gpu}");

	if what_gpu == ae_sys::PF_GPU_Framework_OPENCL {
        let mut gpu_data_handle = ae::pf::Handle::<OpenCLGPUData>::from_raw((*(*extra).input).gpu_data as ae_sys::PF_Handle).unwrap();
        let gpu_data = gpu_data_handle.lock().unwrap();
        log::info!("gpu_data.invert_kernel: {:?}", gpu_data.as_ref().unwrap().invert_kernel);
		// clReleaseKernel (cl_gpu_dataP->invert_kernel);
		// clReleaseKernel (cl_gpu_dataP->procamp_kernel);
	}

    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

/////////////////////////////////////////////////// GPU ///////////////////////////////////////////////////



#[no_mangle]
pub unsafe extern "C" fn EffectMain(
    cmd: ae_sys::PF_Cmd,
    in_data: *const ae_sys::PF_InData,
    out_data: *mut ae_sys::PF_OutData,
    params: *mut *mut ae_sys::PF_ParamDef,
    output: *mut ae_sys::PF_LayerDef,
    extra: *mut std::ffi::c_void) -> ae_sys::PF_Err
{
    let _pica = ae::PicaBasicSuite::from_pf_in_data_raw(in_data);

    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);

    let mut err = ae_sys::PF_Err_NONE as ae_sys::PF_Err;

    let in_data = ae::pf::InDataHandle::from_raw(in_data);

    match cmd as ae::EnumIntType {
        ae_sys::PF_Cmd_ABOUT => {
            write_str(&mut (*out_data).return_msg,
                format!("SDK_Invert_ProcAmp, v1.1\nCopyright 2018-2023 Adobe Inc.\rSample Invert ProcAmp effect.")
            );
        },
        ae_sys::PF_Cmd_GLOBAL_SETUP => {
            (*out_data).my_version = env!("PIPL_VERSION").parse().unwrap();
            (*out_data).out_flags  = env!("PIPL_OUTFLAGS").parse().unwrap();
            (*out_data).out_flags2 = env!("PIPL_OUTFLAGS2").parse().unwrap();

            if &in_data.application_id() == b"PrMr" {
                let pixel_format = ae::pf::PixelFormatSuite::new().unwrap();
                pixel_format.clear_supported_pixel_formats(in_data.effect_ref()).unwrap();
                pixel_format.add_supported_pixel_format(in_data.effect_ref(), ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f).unwrap();
            } else {
                (*out_data).out_flags2 |= ae_sys::PF_OutFlag2_SUPPORTS_GPU_RENDER_F32;
            }
        },
        ae_sys::PF_Cmd_PARAMS_SETUP => {
            use ae::*;
	        // Brightness
            ParamDef::new(in_data).name("Brightness").param(Param::FloatSlider(*FloatSliderDef::new()
                .set_valid_min(-100.0)
                .set_slider_min(-100.0)
                .set_valid_max(100.0)
                .set_slider_max(100.0)
                .set_value(0.0)
                .set_default(0.0)
                .precision(1)
                .display_flags(ValueDisplayFlag::NONE)
            )).add(-1);

	        // Contrast
            ParamDef::new(in_data).name("Contrast").param(Param::FloatSlider(*FloatSliderDef::new()
                .set_valid_min(0.0)
                .set_slider_min(0.0)
                .set_valid_max(200.0)
                .set_slider_max(200.0)
                .set_value(100.0)
                .set_default(100.0)
                .precision(1)
                .display_flags(ValueDisplayFlag::NONE)
            )).add(-1);

	        // Hue
            ParamDef::new(in_data).name("Hue").param(Param::Angle(*AngleDef::new()
                .set_value(0)
            )).add(-1);

	        // Saturation
            ParamDef::new(in_data).name("Saturation").param(Param::FloatSlider(*FloatSliderDef::new()
                .set_valid_min(0.0)
                .set_slider_min(0.0)
                .set_valid_max(200.0)
                .set_slider_max(200.0)
                .set_value(100.0)
                .set_default(100.0)
                .precision(1)
                .display_flags(ValueDisplayFlag::NONE)
            )).add(-1);

            (*out_data).num_params = PluginParams::NumParams as i32;
        },
        ae_sys::PF_Cmd_GPU_DEVICE_SETUP => {
            err = gpu_device_setup(in_data, out_data, extra as *mut ae_sys::PF_GPUDeviceSetupExtra);
        },
        ae_sys::PF_Cmd_GPU_DEVICE_SETDOWN => {
            err = gpu_device_setdown(in_data, out_data, extra as *mut ae_sys::PF_GPUDeviceSetdownExtra);
        },
        ae_sys::PF_Cmd_RENDER => {
            err = render(in_data, params, output);
        },
        ae_sys::PF_Cmd_SMART_PRE_RENDER => {
            err = pre_render(in_data, extra as *mut ae_sys::PF_PreRenderExtra);
        },
        ae_sys::PF_Cmd_SMART_RENDER => {
            err = smart_render(in_data, extra as *mut ae_sys::PF_SmartRenderExtra, false);
        },
        ae_sys::PF_Cmd_SMART_RENDER_GPU => {
            err = smart_render(in_data, extra as *mut ae_sys::PF_SmartRenderExtra, true);
        },
        _ => {
            log::debug!("Unknown cmd: {cmd:?}");
        }
    }

    err
}
