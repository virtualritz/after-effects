use crate::*;

#[derive(Clone)]
/// Information about a frame render
pub struct RenderParams {
    ptr: *const pr_sys::PrGPUFilterRenderParams,
}
impl RenderParams {
    pub fn from_raw(ptr: *const pr_sys::PrGPUFilterRenderParams) -> Self {
        Self {
            ptr
        }
    }
    /// Clip time of the current render
    pub fn clip_time(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inClipTime }
    }
    /// Sequence time of the current render
    pub fn sequence_time(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inSequenceTime }
    }
    pub fn quality(&self) -> RenderQuality {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inQuality.into() }
    }
    pub fn downsample_factor(&self) -> (f32, f32) {
        assert!(!self.ptr.is_null());
        unsafe { ((*self.ptr).inDownsampleFactorX, (*self.ptr).inDownsampleFactorY) }
    }
    pub fn render_width(&self) -> u32 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderWidth }
    }
    pub fn render_height(&self) -> u32 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderHeight }
    }
    pub fn render_pixel_aspect_ratio(&self) -> (u32, u32)  {
        assert!(!self.ptr.is_null());
        unsafe { ((*self.ptr).inRenderPARNum, (*self.ptr).inRenderPARDen) }
    }
    pub fn render_field_type(&self) -> pr_sys::prFieldType {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderFieldType }
    }
    pub fn render_ticks_per_frame(&self) -> i64 {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderTicksPerFrame }
    }

    /// GPU rendering is always on full height progressive frames unless outNeedsFieldSeparation is false.
    /// `render_field()` indicates which field is being rendered
    pub fn render_field(&self) -> FieldDisplay {
        assert!(!self.ptr.is_null());
        unsafe { (*self.ptr).inRenderField.into() }
    }
}

pub struct GpuFilterData {
    pub instance_ptr: *mut pr_sys::PrGPUFilterInstance,
    pub gpu_device_suite: GPUDeviceSuite,
    pub gpu_image_processing_suite: GPUImageProcessingSuite,
    pub memory_manager_suite: MemoryManagerSuite,
    pub ppix_suite: PPixSuite,
    pub ppix2_suite: PPix2Suite,
    pub video_segment_suite: VideoSegmentSuite,
    pub gpu_info: pr_sys::PrGPUDeviceInfo,
}
impl GpuFilterData {
    pub fn timeline_id(&self) -> pr_sys::PrTimelineID {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inTimelineID }
    }
    pub fn node_id(&self) -> i32 {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inNodeID as i32 }
    }
    pub fn device_index(&self) -> u32 {
        assert!(!self.instance_ptr.is_null());
        unsafe { (*self.instance_ptr).inDeviceIndex as u32 }
    }
    pub fn get_param(&self, index: usize, time: i64) -> Result<crate::Param, Error> {
        let index = index as i32 - 1; // GPU filters don't include the input frame as first paramter

        self.video_segment_suite.get_param(self.node_id(), index, time)
    }
    pub fn get_property(&self, property: Property) -> Result<PropertyData, Error> {
        self.video_segment_suite.get_node_property(self.node_id(), property)
    }
}

pub trait GpuFilter : Default {
    /// Called once at startup to initialize any global state.
    /// * Note that the instances are created and destroyed many times during the same render,
    /// so don't rely on `Default` or `Drop` for any global state
    fn global_init();

    /// Called once at shutdown to clean up any global state.
    /// * Note that the instances are created and destroyed many times during the same render,
    /// so don't rely on `Default` or `Drop` for any global state
    fn global_destroy();

    /// Return dependency information about a render, or nothing if only the current frame is required.
    fn get_frame_dependencies(&self, filter: &GpuFilterData, render_params: RenderParams, query_index: &mut i32) -> Result<pr_sys::PrGPUFilterFrameDependency, Error>;

    /// Precompute a result into preallocated uninitialized host (pinned) memory.
    /// Will only be called if PrGPUDependency_Precompute was returned from GetFrameDependencies.
    /// Precomputation may be called ahead of render time. Results will be
    /// uploaded to the GPU by the host. If outPrecomputePixelFormat is not custom,
    /// frames will be converted to the GPU pixel format.
    fn precompute(&self, filter: &GpuFilterData, render_params: RenderParams, index: i32, frame: pr_sys::PPixHand) -> Result<(), Error>;

    /// Render into an allocated outFrame allocated with PrSDKGPUDeviceSuite or operate
    /// in place. Result must be in the same pixel format as the input. For effects, frame 0
    /// will always be the frame at the current time, other input frames will be in the same order as
    /// returned from GetFrameDependencies. For transitions frame 0 will be the incoming frame and
    /// frame 1 the outgoing frame. Transitions may not have other frame dependencies.
    fn render(&self, filter: &GpuFilterData, render_params: RenderParams, frames: *const pr_sys::PPixHand, frame_count: usize, out_frame: *mut pr_sys::PPixHand) -> Result<(), Error>;
}

pub struct GpuFilterInstance<T: GpuFilter> {
    pub data: GpuFilterData,
    pub instance: T,
}

/// Define a GPU filter entry point and register the `struct_name` as the filter handler.
///
/// `struct_name` must implement the [`GpuFilter`] trait.
///
/// GPU filter instances are created and destroyed on demand. They work together with the AfterEffects main entry point, where you define
/// all the parameters and handle other properties. Premiere's GPU filter is an additional layer to just handle the rendering on the GPU.
///
/// To share data between AfterEffects plugin interface and GPU filter interface, see [`OpaqueEffectDataSuite`]
#[macro_export]
macro_rules! define_gpu_filter {
    ($struct_name:ty) => {
        use $crate::GpuFilter;

        unsafe extern "C" fn gpu_filter_create_instance(instance_data: *mut pr_sys::PrGPUFilterInstance) -> pr_sys::prSuiteError {
            assert!(!instance_data.is_null());

            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let sp_basic_suite = (util_funcs.getSPBasicSuite.unwrap())();

            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw(sp_basic_suite);

            let result = (|| -> Result<Box<$crate::GpuFilterInstance<$struct_name>>, $crate::Error> {
                let gpu_suite = $crate::GPUDeviceSuite::new()?;
                let gpu_info = gpu_suite.get_device_info((*instance_data).inDeviceIndex)?;
                Ok(Box::new($crate::GpuFilterInstance {
                    data: $crate::GpuFilterData {
                        instance_ptr: instance_data,
                        gpu_device_suite:           gpu_suite,
                        gpu_image_processing_suite: $crate::GPUImageProcessingSuite::new()?,
                        memory_manager_suite:       $crate::MemoryManagerSuite::new()?,
                        ppix_suite:                 $crate::PPixSuite::new()?,
                        ppix2_suite:                $crate::PPix2Suite::new()?,
                        video_segment_suite:        $crate::VideoSegmentSuite::new()?,
                        gpu_info
                    },
                    instance: <$struct_name>::default(),
                }))
            })();

            match result {
                Ok(instance) => {
                    (*instance_data).ioPrivatePluginData = Box::into_raw(instance) as *mut _;
                    pr_sys::suiteError_NoError
                }
                Err(e) => {
                    e as pr_sys::prSuiteError
                }
            }
        }

        unsafe extern "C" fn gpu_filter_dispose_instance(instance_data: *mut pr_sys::PrGPUFilterInstance) -> pr_sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let _ = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            (*instance_data).ioPrivatePluginData = std::ptr::null_mut();

            pr_sys::suiteError_NoError
        }

        unsafe extern "C" fn gpu_filter_get_frame_dependencies(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            io_query_index: *mut pr_sys::csSDK_int32,
            out_frame_dependencies: *mut pr_sys::PrGPUFilterFrameDependency,
        ) -> pr_sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.get_frame_dependencies(&instance.data, render_params, &mut *io_query_index);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(dep) => {
                    *out_frame_dependencies = dep;
                    pr_sys::suiteError_NoError
                },
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_precompute(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            index: pr_sys::csSDK_int32,
            frame: pr_sys::PPixHand,
        ) -> pr_sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.precompute(&instance.data, render_params, index, frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => pr_sys::suiteError_NoError,
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_render(
            instance_data: *mut pr_sys::PrGPUFilterInstance,
            render_params: *const pr_sys::PrGPUFilterRenderParams,
            frames: *const pr_sys::PPixHand,
            frame_count: pr_sys::csSDK_size_t,
            out_frame: *mut pr_sys::PPixHand,
        ) -> pr_sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.render(&instance.data, render_params, frames, frame_count as usize, out_frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => pr_sys::suiteError_NoError,
                Err(e) => e as pr_sys::prSuiteError,
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn xGPUFilterEntry(
            host_interface_version: pr_sys::csSDK_uint32,
            io_index: *mut pr_sys::csSDK_int32,
            is_startup: pr_sys::prBool,
            pi_suites: pr_sys::piSuitesPtr,
            out_filter: *mut pr_sys::PrGPUFilter,
            out_filter_info: *mut pr_sys::PrGPUFilterInfo,
        ) -> pr_sys::prSuiteError {

            let util_funcs = (*(*pi_suites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            if is_startup == 1 {
                (*out_filter).CreateInstance       = Some(gpu_filter_create_instance);
                (*out_filter).DisposeInstance      = Some(gpu_filter_dispose_instance);
                (*out_filter).GetFrameDependencies = Some(gpu_filter_get_frame_dependencies);
                (*out_filter).Precompute           = Some(gpu_filter_precompute);
                (*out_filter).Render               = Some(gpu_filter_render);

                let plugin_count = 1;

                let index = *io_index;
                if index + 1 > plugin_count {
                    return pr_sys::suiteError_InvalidParms;
                }
                if index + 1 < plugin_count {
                    *io_index += 1;
                }

                // let match_name = pr_sys::PrSDKString::default();
                (*out_filter_info).outMatchName = unsafe { std::mem::zeroed() };
                (*out_filter_info).outInterfaceVersion = pr_sys::PrSDKGPUFilterInterfaceVersion;

                <$struct_name>::global_init();
            } else {
                <$struct_name>::global_destroy();
            }

            pr_sys::suiteError_NoError
        }
    };
}
