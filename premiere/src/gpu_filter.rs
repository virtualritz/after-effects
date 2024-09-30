use crate::*;

#[derive(Clone)]
/// Information about a frame render
pub struct RenderParams {
    ptr: *const crate::sys::PrGPUFilterRenderParams,
}
impl RenderParams {
    pub fn from_raw(ptr: *const crate::sys::PrGPUFilterRenderParams) -> Self {
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
    pub fn render_field_type(&self) -> crate::sys::prFieldType {
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
    pub instance_ptr: *mut crate::sys::PrGPUFilterInstance,
    pub gpu_device_suite: suites::GPUDevice,
    pub gpu_image_processing_suite: suites::GPUImageProcessing,
    pub memory_manager_suite: suites::MemoryManager,
    pub ppix_suite: suites::PPix,
    pub ppix2_suite: suites::PPix2,
    pub video_segment_suite: suites::VideoSegment,
    pub gpu_info: crate::sys::PrGPUDeviceInfo,
}
impl GpuFilterData {
    pub fn timeline_id(&self) -> crate::sys::PrTimelineID {
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

    /// Get a specific param value at a specific time
    /// * `index` - The index of the param
    /// * `time` - The time requested (in Media time)
    ///
    /// Returns the param
    pub fn param(&self, index: usize, time: i64) -> Result<crate::Param, Error> {
        let index = index as i32 - 1; // GPU filters don't include the input frame as first paramter

        self.video_segment_suite.param(self.node_id(), index, time)
    }

    /// Get the next keyframe time after the specified time.
    /// Example: Keyframes at 0 and 10
    /// - `time` = -1, keyframe_time = 0
    /// - `time` = 0, keyframe_time = 10
    /// - `time` = 9, keyframe_time = 10
    /// - `time` = 10, returns [`Error::NoKeyframeAfterInTime`]
    ///
    /// Parameters:
    /// * `index` - The index of the param
    /// * `time` - The lower bound time
    ///
    /// Returns a tuple containing:
    /// * `keyframe_time` - The time of the next keyframe > inTime
    /// * `keyframe_interpolation_mode` - The temporal interpolation mode of the keyframe
    pub fn next_keyframe_time(&self, index: usize, time: i64) -> Result<(i64, KeyframeInterpolationMode), Error> {
        let index = index as i32 - 1; // GPU filters don't include the input frame as first paramter

        self.video_segment_suite.next_keyframe_time(self.node_id(), index, time)
    }

    pub fn param_arbitrary_data<T: for<'a> serde::Deserialize<'a>>(&self, index: usize, time: i64) -> Result<T, Error> {
        let ptr = self.param(index, time)?;
        if let crate::Param::MemoryPtr(ptr) = ptr {
            if !ptr.is_null() {
                let serialized = unsafe { std::slice::from_raw_parts(ptr as *mut u8, self.memory_manager_suite.ptr_size(ptr) as _) };
                if let Ok(t) = bincode::deserialize::<T>(serialized) {
                    return Ok(t);
                }
            }
        }
        Err(Error::InvalidParms)
    }
    pub fn property(&self, property: Property) -> Result<PropertyData, Error> {
        self.video_segment_suite.node_property(self.node_id(), property)
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
    fn get_frame_dependencies(&self, filter: &GpuFilterData, render_params: RenderParams, query_index: &mut i32) -> Result<crate::sys::PrGPUFilterFrameDependency, Error>;

    /// Precompute a result into preallocated uninitialized host (pinned) memory.
    /// Will only be called if PrGPUDependency_Precompute was returned from GetFrameDependencies.
    /// Precomputation may be called ahead of render time. Results will be
    /// uploaded to the GPU by the host. If outPrecomputePixelFormat is not custom,
    /// frames will be converted to the GPU pixel format.
    fn precompute(&self, filter: &GpuFilterData, render_params: RenderParams, index: i32, frame: crate::sys::PPixHand) -> Result<(), Error>;

    /// Render into an allocated outFrame allocated with PrSDKGPUDeviceSuite or operate
    /// in place. Result must be in the same pixel format as the input. For effects, frame 0
    /// will always be the frame at the current time, other input frames will be in the same order as
    /// returned from GetFrameDependencies. For transitions frame 0 will be the incoming frame and
    /// frame 1 the outgoing frame. Transitions may not have other frame dependencies.
    fn render(&self, filter: &GpuFilterData, render_params: RenderParams, frames: *const crate::sys::PPixHand, frame_count: usize, out_frame: *mut crate::sys::PPixHand) -> Result<(), Error>;
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
/// To share data between AfterEffects plugin interface and GPU filter interface, see [`suites::OpaqueEffectData`]
#[macro_export]
macro_rules! define_gpu_filter {
    ($struct_name:ty) => {
        use $crate::GpuFilter;

        unsafe extern "C" fn gpu_filter_create_instance(instance_data: *mut $crate::sys::PrGPUFilterInstance) -> $crate::sys::prSuiteError {
            assert!(!instance_data.is_null());

            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let sp_basic_suite = (util_funcs.getSPBasicSuite.unwrap())();

            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw(sp_basic_suite);

            let result = (|| -> Result<Box<$crate::GpuFilterInstance<$struct_name>>, $crate::Error> {
                let gpu_suite = $crate::suites::GPUDevice::new()?;
                let gpu_info = gpu_suite.device_info((*instance_data).inDeviceIndex)?;
                Ok(Box::new($crate::GpuFilterInstance {
                    data: $crate::GpuFilterData {
                        instance_ptr: instance_data,
                        gpu_device_suite:           gpu_suite,
                        gpu_image_processing_suite: $crate::suites::GPUImageProcessing::new()?,
                        memory_manager_suite:       $crate::suites::MemoryManager::new()?,
                        ppix_suite:                 $crate::suites::PPix::new()?,
                        ppix2_suite:                $crate::suites::PPix2::new()?,
                        video_segment_suite:        $crate::suites::VideoSegment::new()?,
                        gpu_info
                    },
                    instance: <$struct_name>::default(),
                }))
            })();

            match result {
                Ok(instance) => {
                    (*instance_data).ioPrivatePluginData = Box::into_raw(instance) as *mut _;
                    $crate::sys::suiteError_NoError
                }
                Err(e) => {
                    e as $crate::sys::prSuiteError
                }
            }
        }

        unsafe extern "C" fn gpu_filter_dispose_instance(instance_data: *mut $crate::sys::PrGPUFilterInstance) -> $crate::sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let _ = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            (*instance_data).ioPrivatePluginData = std::ptr::null_mut();

            $crate::sys::suiteError_NoError
        }

        unsafe extern "C" fn gpu_filter_get_frame_dependencies(
            instance_data: *mut $crate::sys::PrGPUFilterInstance,
            render_params: *const $crate::sys::PrGPUFilterRenderParams,
            io_query_index: *mut $crate::sys::csSDK_int32,
            out_frame_dependencies: *mut $crate::sys::PrGPUFilterFrameDependency,
        ) -> $crate::sys::prSuiteError {
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
                    $crate::sys::suiteError_NoError
                },
                Err(e) => e as $crate::sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_precompute(
            instance_data: *mut $crate::sys::PrGPUFilterInstance,
            render_params: *const $crate::sys::PrGPUFilterRenderParams,
            index: $crate::sys::csSDK_int32,
            frame: $crate::sys::PPixHand,
        ) -> $crate::sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.precompute(&instance.data, render_params, index, frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => $crate::sys::suiteError_NoError,
                Err(e) => e as $crate::sys::prSuiteError,
            }
        }

        unsafe extern "C" fn gpu_filter_render(
            instance_data: *mut $crate::sys::PrGPUFilterInstance,
            render_params: *const $crate::sys::PrGPUFilterRenderParams,
            frames: *const $crate::sys::PPixHand,
            frame_count: $crate::sys::csSDK_size_t,
            out_frame: *mut $crate::sys::PPixHand,
        ) -> $crate::sys::prSuiteError {
            let util_funcs = (*(*(*instance_data).piSuites).utilFuncs);
            let _pica = $crate::PicaBasicSuite::from_sp_basic_suite_raw((util_funcs.getSPBasicSuite.unwrap())());

            let mut instance = Box::<$crate::GpuFilterInstance<$struct_name>>::from_raw((*instance_data).ioPrivatePluginData as *mut _);

            instance.data.instance_ptr = instance_data;

            let render_params = $crate::RenderParams::from_raw(render_params);
            let result = instance.instance.render(&instance.data, render_params, frames, frame_count as usize, out_frame);

            let _ = Box::into_raw(instance); // leak the box so it doesn't run the destructor

            match result {
                Ok(_) => $crate::sys::suiteError_NoError,
                Err(e) => e as $crate::sys::prSuiteError,
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn xGPUFilterEntry(
            host_interface_version: $crate::sys::csSDK_uint32,
            io_index: *mut $crate::sys::csSDK_int32,
            is_startup: $crate::sys::prBool,
            pi_suites: $crate::sys::piSuitesPtr,
            out_filter: *mut $crate::sys::PrGPUFilter,
            out_filter_info: *mut $crate::sys::PrGPUFilterInfo,
        ) -> $crate::sys::prSuiteError {

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
                    return $crate::sys::suiteError_InvalidParms;
                }
                if index + 1 < plugin_count {
                    *io_index += 1;
                }

                // let match_name = $crate::sys::PrSDKString::default();
                (*out_filter_info).outMatchName = unsafe { std::mem::zeroed() };
                (*out_filter_info).outInterfaceVersion = $crate::sys::PrSDKGPUFilterInterfaceVersion;

                <$struct_name>::global_init();
            } else {
                <$struct_name>::global_destroy();
            }

            $crate::sys::suiteError_NoError
        }
    };
}
