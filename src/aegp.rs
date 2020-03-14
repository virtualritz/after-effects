use crate::{pf::EffectWorld, Suite, *};
use aftereffects_sys as ae_sys;
use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use std::ffi::CString; //, mem::transmute};

pub type PluginID = aftereffects_sys::AEGP_PluginID;

pub type ItemID = i32;

pub type CompFlags = u32;

pub const COMP_FLAG_SHOW_ALL_SHY: u32 =
    ae_sys::AEGP_CompFlag_SHOW_ALL_SHY;
pub const COMP_FLAG_RESERVED_1: u32 = ae_sys::AEGP_CompFlag_RESERVED_1;
pub const COMP_FLAG_RESERVED_2: u32 = ae_sys::AEGP_CompFlag_RESERVED_2;
pub const COMP_FLAG_ENABLE_MOTION_BLUR: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_MOTION_BLUR;
pub const COMP_FLAG_ENABLE_TIME_FILTER: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_TIME_FILTER;
pub const COMP_FLAG_GRID_TO_FRAMES: u32 =
    ae_sys::AEGP_CompFlag_GRID_TO_FRAMES;
pub const COMP_FLAG_GRID_TO_FIELDS: u32 =
    ae_sys::AEGP_CompFlag_GRID_TO_FIELDS;
pub const COMP_FLAG_USE_LOCAL_DSF: u32 =
    ae_sys::AEGP_CompFlag_USE_LOCAL_DSF;
pub const COMP_FLAG_DRAFT_3D: u32 = ae_sys::AEGP_CompFlag_DRAFT_3D;
pub const COMP_FLAG_SHOW_GRAPH: u32 = ae_sys::AEGP_CompFlag_SHOW_GRAPH;
pub const COMP_FLAG_RESERVED_3: u32 = ae_sys::AEGP_CompFlag_RESERVED_3;

pub type MemFlag = u32;

pub const MEM_FLAG_NONE: u32 = ae_sys::AEGP_MemFlag_NONE;
pub const MEM_FLAG_CLEAR: u32 = ae_sys::AEGP_MemFlag_CLEAR;
pub const MEM_FLAG_QUIET: u32 = ae_sys::AEGP_MemFlag_QUIET;

#[allow(dead_code)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    IntoPrimitive,
    UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum FilmSizeUnits {
    None = ae_sys::AEGP_FilmSizeUnits_NONE as i32,
    Horizontal = ae_sys::AEGP_FilmSizeUnits_HORIZONTAL as i32,
    Vertical = ae_sys::AEGP_FilmSizeUnits_VERTICAL as i32,
    Diagonal = ae_sys::AEGP_FilmSizeUnits_DIAGONAL as i32,
}

#[allow(dead_code)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    IntoPrimitive,
    UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum CameraType {
    None = ae_sys::AEGP_CameraType_NONE as i32,
    Perspective = ae_sys::AEGP_CameraType_PERSPECTIVE as i32,
    Orthographic = ae_sys::AEGP_CameraType_ORTHOGRAPHIC as i32,
    NumTypes = ae_sys::AEGP_CameraType_NUM_TYPES as i32,
}

//define_handle_wrapper!(MemHandle, AEGP_MemHandle, mem_ptr);

define_suite!(
    MemorySuite,
    AEGP_MemorySuite1,
    kAEGPMemorySuite,
    kAEGPMemorySuiteVersion1
);

pub struct MemHandle<T> {
    ptr: *mut T,
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    mem_handle: ae_sys::AEGP_MemHandle,
}

impl<T> MemHandle<T> {
    pub fn new(
        plugin_id: PluginID,
        name: &str,
        flags: MemFlag,
    ) -> Result<Self, crate::Error> {
        let mut mem_handle: ae_sys::AEGP_MemHandle =
            std::ptr::null_mut();
        let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

        match ae_acquire_suite_and_call_suite_fn!(
            pica_basic_suite_ptr,
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_NewMemHandle,
            // Arguments ----------
            plugin_id,
            CString::new(name).unwrap().as_ptr(),
            std::mem::size_of::<T>() as u32,
            flags as i32,
            &mut mem_handle,
        ) {
            Ok(()) => Ok(Self {
                ptr: std::ptr::null_mut(),
                pica_basic_suite_ptr,
                mem_handle,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        assert!(self.ptr != std::ptr::null_mut());
        self.ptr
    }

    pub fn lock(&mut self) -> Result<*mut T, crate::Error> {
        match ae_acquire_suite_and_call_suite_fn!(
            (self.pica_basic_suite_ptr),
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_LockMemHandle,
            // Arguments ----------
            self.mem_handle,
            self.ptr as *mut _
        ) {
            Ok(()) => Ok(self.ptr),
            Err(e) => Err(e),
        }
    }

    pub fn unlock(&mut self) -> Result<(), crate::Error> {
        ae_acquire_suite_and_call_suite_fn!(
            self.pica_basic_suite_ptr,
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_UnlockMemHandle,
            // Arguments ----------
            self.mem_handle
        )
    }
}

impl<T> Drop for MemHandle<T> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.unlock();
    }
}

// FIXME: wrap this nicely or combine WorldHandle & WorldSuite into
// single World
define_handle_wrapper!(WorldHandle, AEGP_WorldH, world_ptr);

define_suite!(
    WorldSuite,
    AEGP_WorldSuite3,
    kAEGPWorldSuite,
    kAEGPWorldSuiteVersion3
);

impl WorldSuite {
    pub fn fill_out_pf_effect_world(
        &self,
        world: &WorldHandle,
    ) -> Result<EffectWorld, crate::Error> {
        let mut effect_world_boxed =
            Box::<ae_sys::PF_EffectWorld>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FillOutPFEffectWorld,
            world.as_ptr(),
            effect_world_boxed.as_mut_ptr()
        ) {
            Ok(()) => Ok(EffectWorld {
                effect_world_boxed: unsafe {
                    effect_world_boxed.assume_init()
                },
            }),
            Err(e) => Err(e),
        }
    }
}

define_handle_wrapper!(CompHandle, AEGP_CompH, comp_ptr);

define_suite!(
    CompSuite,
    AEGP_CompSuite11,
    kAEGPCompSuite,
    kAEGPCompSuiteVersion11
);

impl CompSuite {
    pub fn get_comp_shutter_angle_phase(
        &self,
        comp_handle: &CompHandle,
    ) -> Result<(Ratio, Ratio), Error> {
        let mut angle = std::mem::MaybeUninit::<Ratio>::uninit();
        let mut phase = std::mem::MaybeUninit::<Ratio>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompShutterAnglePhase,
            comp_handle.as_ptr(),
            angle.as_mut_ptr() as *mut ae_sys::A_Ratio,
            phase.as_mut_ptr() as *mut ae_sys::A_Ratio,
        ) {
            Ok(()) => Ok(unsafe {
                (angle.assume_init(), phase.assume_init())
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_item_from_comp(
        &self,
        comp_handle: &CompHandle,
    ) -> Result<ItemHandle, Error> {
        let mut item_handle_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_ItemH>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemFromComp,
            comp_handle.as_ptr(),
            item_handle_ptr.as_mut_ptr()
        ) {
            Ok(()) => Ok(ItemHandle::from_raw(unsafe {
                item_handle_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    pub fn get_comp_flags(
        &self,
        comp_handle: &CompHandle,
    ) -> Result<CompFlags, Error> {
        let mut comp_flags =
            std::mem::MaybeUninit::<CompFlags>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompFlags,
            comp_handle.as_ptr(),
            comp_flags.as_mut_ptr() as *mut ae_sys::A_long
        ) {
            Ok(()) => Ok(unsafe { comp_flags.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn get_comp_framerate(
        &self,
        comp_handle: &CompHandle,
    ) -> Result<f64, Error> {
        let mut framerate = std::mem::MaybeUninit::<f64>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompFramerate,
            comp_handle.as_ptr(),
            framerate.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { framerate.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

pub struct Comp {
    // We need to store this pointer to be able to
    // drop resources at the end of our lifetime
    // using release_suite()
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    suite_ptr: *const ae_sys::AEGP_CompSuite11,
    comp_ptr: *const ae_sys::AEGP_CompH,
}

impl Comp {
    pub fn from_item(
        pica_basic_suite_handle: &crate::PicaBasicSuiteHandle,
        item_handle: &ItemHandle,
    ) -> Result<Self, crate::Error> {
        let pica_basic_suite_ptr = pica_basic_suite_handle.as_ptr();
        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_CompSuite11,
            kAEGPCompSuite,
            kAEGPCompSuiteVersion11
        )?;

        let comp_ptr: *mut ae_sys::AEGP_CompH = std::ptr::null_mut();
        ae_call_suite_fn!(
            suite_ptr,
            AEGP_GetCompFromItem,
            item_handle.as_ptr(),
            comp_ptr,
        )?;

        Ok(Self {
            pica_basic_suite_ptr,
            suite_ptr,
            comp_ptr,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct StreamReferenceHandle {
    stream_reference_ptr: ae_sys::AEGP_StreamRefH,
}

define_handle_wrapper!(LayerHandle, AEGP_LayerH, layer_ptr);

define_suite!(
    LayerSuite,
    AEGP_LayerSuite5,
    kAEGPLayerSuite,
    kAEGPLayerSuiteVersion5
);

impl LayerSuite {
    pub fn get_layer_to_world_xform(
        &self,
        layer_handle: &LayerHandle,
        time: &crate::Time,
    ) -> Result<Matrix4, crate::Error> {
        let mut matrix = Box::<Matrix4>::new_uninit();

        //let mut matrix = nalgebra::Matrix4::<f64>::zeros();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerToWorldXform,
            layer_handle.as_ptr(),
            &(*time) as *const _ as *const ae_sys::A_Time,
            matrix.as_mut_ptr() as *mut ae_sys::A_Matrix4,
            /*transmute::<
                *mut nalgebra::Matrix4<f64>,
                *mut ae_sys::A_Matrix4,
            >(&mut matrix)*/
        ) {
            Ok(()) => Ok(unsafe { *matrix.assume_init() } ),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
pub struct StreamValue {
    pub stream_value: Box<ae_sys::AEGP_StreamValue2>,
}

define_suite!(
    StreamSuite,
    AEGP_StreamSuite4,
    kAEGPStreamSuite,
    kAEGPStreamSuiteVersion4
);

impl StreamSuite {
    pub fn get_new_layer_stream(
        &self,
        plugin_id: PluginID,
        layer_handle: &LayerHandle,
        stream_name: ae_sys::AEGP_LayerStream, // FIXME
    ) -> Result<StreamReferenceHandle, crate::Error> {
        let mut stream_reference_ptr: ae_sys::AEGP_StreamRefH =
            std::ptr::null_mut();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewLayerStream,
            plugin_id,
            layer_handle.layer_ptr,
            stream_name,
            &mut stream_reference_ptr
        ) {
            Ok(()) => Ok(StreamReferenceHandle {
                stream_reference_ptr,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_new_stream_value(
        &self,
        plugin_id: PluginID,
        stream_reference_handle: &StreamReferenceHandle,
        time_mode: ae_sys::AEGP_LTimeMode, // FIXME
        time: &crate::Time,                // FIXME
        sample_stream_pre_expression: bool,
    ) -> Result<StreamValue, crate::Error> {
        let mut stream_value =
            Box::<ae_sys::AEGP_StreamValue2>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewStreamValue,
            plugin_id,
            stream_reference_handle.stream_reference_ptr,
            time_mode,
            &(*time) as *const _ as *const ae_sys::A_Time,
            sample_stream_pre_expression as u8,
            stream_value.as_mut_ptr(),
        ) {
            Ok(()) => Ok(StreamValue {
                stream_value: unsafe { stream_value.assume_init() },
            }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    CanvasSuite,
    AEGP_CanvasSuite8,
    kAEGPCanvasSuite,
    kAEGPCanvasSuiteVersion8
);

impl CanvasSuite {
    pub fn get_comp_to_render(
        &self,
        render_context_handle: &crate::pr::RenderContextHandle,
    ) -> Result<CompHandle, Error> {
        let mut comp_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_CompH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompToRender,
            render_context_handle.as_ptr(),
            comp_ptr.as_mut_ptr()
        ) {
            Ok(()) => Ok(CompHandle::from_raw(unsafe {
                comp_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    pub fn get_comp_render_time(
        &self,
        render_context_handle: &crate::pr::RenderContextHandle,
    ) -> Result<(Time, Time), Error> {
        let mut shutter_frame_start =
            std::mem::MaybeUninit::<Time>::uninit();

        let mut shutter_frame_duration =
            std::mem::MaybeUninit::<Time>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompRenderTime,
            render_context_handle.as_ptr(),
            shutter_frame_start.as_mut_ptr() as *mut ae_sys::A_Time,
            shutter_frame_duration.as_mut_ptr() as *mut ae_sys::A_Time
        ) {
            Ok(()) => Ok(unsafe {
                (
                    shutter_frame_start.assume_init(),
                    shutter_frame_duration.assume_init(),
                )
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_comp_destination_buffer(
        &self,
        render_context_handle: &crate::pr::RenderContextHandle,
        comp_handle: &CompHandle,
    ) -> Result<WorldHandle, Error> {
        let mut world_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_WorldH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompDestinationBuffer,
            render_context_handle.as_ptr(),
            comp_handle.as_ptr(),
            world_ptr.as_mut_ptr(),
        ) {
            Ok(()) => Ok(WorldHandle::from_raw(unsafe {
                world_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }
}

define_handle_wrapper!(ItemHandle, AEGP_ItemH, item_ptr);

define_suite!(
    ItemSuite,
    AEGP_ItemSuite9,
    kAEGPItemSuite,
    kAEGPItemSuiteVersion9
);

impl ItemSuite {
    pub fn get_item_id(
        &self,
        item_handle: &ItemHandle,
    ) -> Result<ItemID, Error> {
        let mut item_id = std::mem::MaybeUninit::<ItemID>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemID,
            item_handle.as_ptr(),
            item_id.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { item_id.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn get_item_dimensions(
        &self,
        item_handle: &ItemHandle,
    ) -> Result<(u32, u32), Error> {
        let mut width = std::mem::MaybeUninit::<u32>::uninit();
        let mut height = std::mem::MaybeUninit::<u32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemDimensions,
            item_handle.as_ptr(),
            width.as_mut_ptr() as *mut i32,
            height.as_mut_ptr() as *mut i32
        ) {
            Ok(()) => Ok(unsafe {
                (width.assume_init(), height.assume_init())
            }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    CameraSuite,
    AEGP_CameraSuite2,
    kAEGPCameraSuite,
    kAEGPCameraSuiteVersion2
);

impl CameraSuite {
    pub fn get_camera(
        &self,
        render_context_handle: &crate::pr::RenderContextHandle,
        time: &crate::Time,
    ) -> Result<LayerHandle, crate::Error> {
        let mut layer_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCamera,
            render_context_handle.as_ptr(),
            &(*time) as *const _ as *const ae_sys::A_Time,
            layer_ptr.as_mut_ptr(),
        ) {
            Ok(()) => Ok(LayerHandle::from_raw(unsafe {
                layer_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    pub fn get_camera_film_size(
        &self,
        camera_layer_handle: &LayerHandle,
    ) -> Result<(FilmSizeUnits, f64), crate::Error> {
        let mut film_size_units: FilmSizeUnits = FilmSizeUnits::None;
        let mut film_size: ae_sys::A_FpLong = 0.0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCameraFilmSize,
            camera_layer_handle.as_ptr(),
            &mut film_size_units as *mut _ as *mut i32,
            &mut film_size,
        ) {
            Ok(()) => Ok((film_size_units, film_size)),
            Err(e) => Err(e),
        }
    }

    pub fn get_default_camera_distance_to_image_plane(
        &self,
        comp_handle: &CompHandle,
    ) -> Result<f64, crate::Error> {
        let mut distance: f64 = 0.0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetDefaultCameraDistanceToImagePlane,
            comp_handle.as_ptr(),
            &mut distance
        ) {
            Ok(()) => Ok(distance),
            Err(e) => Err(e),
        }
    }

    pub fn get_camera_type(
        &self,
        camera_layer_handle: &LayerHandle,
    ) -> Result<CameraType, crate::Error> {
        let mut camera_type: CameraType = CameraType::None;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCameraType,
            camera_layer_handle.as_ptr(),
            &mut camera_type as *mut _ as *mut u32,
        ) {
            Ok(()) => Ok(camera_type),
            Err(e) => Err(e),
        }
    }
}

pub struct Scene3D {
    // We need to store this pointer to be able to
    // drop resources at the end of our lifetime
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,

    suite_ptr: *const ae_sys::AEGP_Scene3DSuite2,

    scene3d_ptr: *mut ae_sys::AEGP_Scene3D,
    texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext,

    in_data_ptr: *const ae_sys::PR_InData,
    render_context_ptr: ae_sys::PR_RenderContextH,
}

impl Scene3D {
    pub fn new(
        in_data_handle: crate::pr::InDataHandle,
        render_context: crate::pr::RenderContextHandle,
        global_texture_cache_handle: crate::aegp::Scene3DTextureCacheHandle,
    ) -> Result<Scene3D, crate::Error> {
        let pica_basic_suite_ptr =
            in_data_handle.pica_basic_handle().as_ptr();

        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_Scene3DSuite2,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        )?;

        let mut scene3d_ptr: *mut ae_sys::AEGP_Scene3D =
            std::ptr::null_mut();

        ae_call_suite_fn!(
            suite_ptr,
            AEGP_Scene3DAlloc,
            &mut scene3d_ptr,
        )?;

        let mut texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext = std::ptr::null_mut();

        match ae_call_suite_fn!(
            suite_ptr,
            AEGP_Scene3DTextureContextAlloc,
            in_data_handle.as_ptr(),
            render_context.as_ptr(),
            global_texture_cache_handle.texture_cache_ptr,
            false as u8, // unlock all
            &mut texture_context_ptr
        ) {
            Ok(()) => Ok(Scene3D {
                pica_basic_suite_ptr,
                suite_ptr,
                scene3d_ptr,
                texture_context_ptr: texture_context_ptr,
                in_data_ptr: in_data_handle.as_ptr(),
                render_context_ptr: render_context.as_ptr(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_scene3d_ptr(&self) -> *mut ae_sys::AEGP_Scene3D {
        self.scene3d_ptr
    }

    pub fn get_scene3d_suite_ptr(
        &self,
    ) -> *const ae_sys::AEGP_Scene3DSuite2 {
        self.suite_ptr
    }

    pub fn setup_motion_blur_samples(
        &self,
        motion_samples: usize,
        sample_method: ae_sys::Scene3DMotionSampleMethod,
    ) -> Result<(), crate::Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3D_SetupMotionBlurSamples,
            self.in_data_ptr,
            self.render_context_ptr,
            self.scene3d_ptr,      /* the empty scene,
                                    * modified */
            motion_samples as i32, // how many motion samples
            sample_method
        )
    }

    pub fn build(
        &self,
        progress_abort_callback_ptr: *mut ae_sys::AEGP_Scene3DProgressAbort,
    ) -> Result<(), crate::Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3D_Build,
            self.in_data_ptr,
            self.render_context_ptr,
            self.texture_context_ptr,
            progress_abort_callback_ptr,
            self.scene3d_ptr
        )
    }

    pub fn scene_num_lights(&self) -> Result<usize, crate::Error> {
        let mut num_lights: i32 = 0;
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DSceneNumLights,
            self.scene3d_ptr,
            &mut num_lights
        ) {
            Ok(()) => Ok(num_lights as usize),
            Err(e) => Err(e),
        }
    }

    // FIXME: make this neat, see
    // https://blog.seantheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
    pub fn node_traverser(
        &self,
        node_visitor_func: ae_sys::Scene3DNodeVisitorFunc,
        reference_context: *mut std::os::raw::c_void, /* FIXME: can we use a Box
                                                       * here? Box<*
                                                       * mut
                                                       * ::std::os::raw::c_void> */
        flags: ae_sys::Scene3DTraverseFlags,
    ) -> Result<(), crate::Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNodeTraverser,
            self.scene3d_ptr,
            node_visitor_func,
            reference_context,
            flags
        )
        //.expect( "3Delight/Ae – ae_scene_to_final_frame(): Could
        //.expect( not traverse the scene." );
    }
}

impl Drop for Scene3D {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        // dispose texture contex
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DTextureContextDispose,
            self.texture_context_ptr
        );

        // dispose scene
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DDispose,
            self.scene3d_ptr
        );

        // release suite
        ae_release_suite_ptr!(
            self.pica_basic_suite_ptr,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        );
    }
}

pub struct Scene3DTextureCacheHandle {
    texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
}

impl Scene3DTextureCacheHandle {
    pub fn new(
        scene3d: Scene3D,
    ) -> Result<Scene3DTextureCacheHandle, crate::Error> {
        let mut texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache = std::ptr::null_mut();

        match ae_call_suite_fn!(
            scene3d.suite_ptr,
            AEGP_Scene3DTextureCacheAlloc,
            &mut texture_cache_ptr,
        ) {
            Ok(()) => {
                Ok(Scene3DTextureCacheHandle { texture_cache_ptr })
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_raw(
        texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
    ) -> Scene3DTextureCacheHandle {
        Scene3DTextureCacheHandle { texture_cache_ptr }
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DMaterialHandle {
    material_ptr: *mut ae_sys::AEGP_Scene3DMaterial,
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DNodeHandle {
    node_ptr: ae_sys::AEGP_Scene3DNodeP,
}

impl Scene3DNodeHandle {
    pub fn new(
        node_ptr: ae_sys::AEGP_Scene3DNodeP,
    ) -> Scene3DNodeHandle {
        Scene3DNodeHandle { node_ptr: node_ptr }
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DMeshHandle {
    mesh_ptr: *mut ae_sys::AEGP_Scene3DMesh,
}

define_suite!(
    Scene3DMaterialSuite,
    AEGP_Scene3DMaterialSuite1,
    kAEGPScene3DMaterialSuite,
    kAEGPScene3DMaterialSuiteVersion1
);

impl Scene3DMaterialSuite {
    pub fn has_uv_color_texture(
        &self,
        material_handle: &Scene3DMaterialHandle,
    ) -> Result<bool, crate::Error> {
        let mut has_uv_color_texture: u8 = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_HasUVColorTexture,
            material_handle.material_ptr,
            &mut has_uv_color_texture
        ) {
            Ok(()) => Ok(has_uv_color_texture != 0),
            Err(e) => Err(e),
        }
    }

    pub fn get_uv_color_texture(
        &self,
        material: &Scene3DMaterialHandle,
    ) -> Result<WorldHandle, crate::Error> {
        let mut world_handle = WorldHandle {
            world_ptr: std::ptr::null_mut(),
        };
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetUVColorTexture,
            material.material_ptr,
            &mut world_handle.world_ptr
        ) {
            Ok(()) => Ok(world_handle),
            Err(e) => Err(e),
        }
    }

    pub fn get_basic_coeffs(
        &self,
        material: &Scene3DMaterialHandle,
    ) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, crate::Error> {
        let mut basic_material_coefficients =
            Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBasicCoeffs,
            material.material_ptr,
            basic_material_coefficients.as_mut_ptr()
        ) {
            Ok(()) => {
                Ok(unsafe { basic_material_coefficients.assume_init() })
            }
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    Scene3DNodeSuite,
    AEGP_Scene3DNodeSuite1,
    kAEGPScene3DNodeSuite,
    kAEGPScene3DNodeSuiteVersion1
);

impl Scene3DNodeSuite {
    pub fn get_material_for_side(
        &self,
        node_handle: &Scene3DNodeHandle,
        side: ae_sys::AEGP_Scene3DMaterialSide,
    ) -> Result<Scene3DMaterialHandle, crate::Error> {
        let mut material_handle = Scene3DMaterialHandle {
            material_ptr: std::ptr::null_mut(),
        };

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialForSide,
            node_handle.node_ptr,
            side,
            &mut material_handle.material_ptr
        ) {
            Ok(()) => Ok(material_handle),
            Err(e) => Err(e),
        }
    }

    pub fn node_mesh_get(
        &self,
        node_handle: &Scene3DNodeHandle,
    ) -> Result<Scene3DMeshHandle, crate::Error> {
        let mut mesh_handle = Scene3DMeshHandle {
            mesh_ptr: std::ptr::null_mut(),
        };

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodeMeshGet,
            node_handle.node_ptr,
            &mut mesh_handle.mesh_ptr
        ) {
            Ok(()) => Ok(mesh_handle),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    Scene3DMeshSuite,
    AEGP_Scene3DMeshSuite1,
    kAEGPScene3DMeshSuite,
    kAEGPScene3DMeshSuiteVersion1
);

impl Scene3DMeshSuite {
    pub fn face_group_buffer_count(
        &self,
        mesh_handle: &Scene3DMeshHandle,
    ) -> Result<usize, crate::Error> {
        let mut face_groups: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferCount,
            mesh_handle.mesh_ptr,
            &mut face_groups
        ) {
            Ok(()) => Ok(face_groups as usize),
            Err(e) => Err(e),
        }
    }

    pub fn face_group_buffer_size(
        &self,
        mesh_handle: &Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<usize, crate::Error> {
        let mut face_count: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferSize,
            mesh_handle.mesh_ptr,
            group_index as i32,
            &mut face_count
        ) {
            Ok(()) => Ok(face_count as usize),
            Err(e) => Err(e),
        }
    }

    pub fn face_group_buffer_fill(
        &self,
        mesh_handle: &Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<Vec<ae_sys::A_long>, crate::Error> {
        let face_count =
            self.face_group_buffer_size(mesh_handle, group_index)?;

        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(face_count as usize);

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferFill,
            mesh_handle.mesh_ptr,
            group_index as i32,
            face_count as i32,
            face_index_buffer.as_mut_ptr()
        ) {
            Ok(()) => {
                // If the previous called didn't bitch we are safe
                // to set the vector's length.
                unsafe {
                    face_index_buffer.set_len(face_count as usize);
                }

                Ok(face_index_buffer)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_material_side_for_face_group(
        &self,
        mesh_handle: &Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<ae_sys::AEGP_Scene3DMaterialSide, crate::Error> {
        let mut material_side = std::mem::MaybeUninit::<
            ae_sys::AEGP_Scene3DMaterialSide,
        >::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialSideForFaceGroup,
            mesh_handle.mesh_ptr,
            group_index as i32,
            material_side.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { material_side.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn mesh_get_info(
        &self,
        mesh_handle: &Scene3DMeshHandle,
    ) -> Result<(usize, usize), crate::Error> {
        let mut num_vertex = std::mem::MaybeUninit::<usize>::uninit();
        let mut num_face = std::mem::MaybeUninit::<usize>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshGetInfo,
            mesh_handle.mesh_ptr,
            //&mut num_vertex as *mut _ as *mut i32,
            //&mut num_face as *mut _ as *mut i32,
            num_vertex.as_mut_ptr() as *mut i32,
            num_face.as_mut_ptr() as *mut i32,
        ) {
            Ok(()) => {
                Ok(unsafe {
                    (num_vertex.assume_init(), num_face.assume_init())
                })
                //Ok((num_vertex, num_face))
            }
            Err(e) => Err(e),
        }
    }

    pub fn vertex_buffer_element_size(
        &self,
        vertex_type: ae_sys::Scene3DVertexBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_VertexBufferElementSize,
            vertex_type
        ) as usize
    }

    pub fn face_index_element_size(
        &self,
        face_type: ae_sys::Scene3DFaceBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_FaceBufferElementSize,
            face_type
        ) as usize
    }

    pub fn uv_buffer_element_size(
        &self,
        uv_type: ae_sys::Scene3DUVBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_UVBufferElementSize,
            uv_type
        ) as usize
    }

    pub fn mesh_fill_buffers(
        &self,
        mesh_handle: &Scene3DMeshHandle,
        vertex_type: ae_sys::Scene3DVertexBufferType,
        face_type: ae_sys::Scene3DFaceBufferType,
        uv_type: ae_sys::Scene3DUVBufferType,
    ) -> Result<
        (
            Vec<ae_sys::A_FpLong>,
            Vec<ae_sys::A_long>,
            Vec<ae_sys::A_FpLong>,
        ),
        crate::Error,
    > {
        let (num_vertex, num_face) = self.mesh_get_info(mesh_handle)?;

        let vertex_buffer_size: usize = num_vertex * 3;
        let mut vertex_buffer =
            Vec::<ae_sys::A_FpLong>::with_capacity(vertex_buffer_size);

        let face_index_buffer_size: usize = num_face * 4;
        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(
                face_index_buffer_size,
            );

        let uv_per_face_buffer_size: usize = num_face * 4 * 2;
        let mut uv_per_face_buffer =
            Vec::<ae_sys::A_FpLong>::with_capacity(
                uv_per_face_buffer_size,
            );

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshFillBuffers,
            mesh_handle.mesh_ptr,
            vertex_type,
            vertex_buffer.as_mut_ptr() as *mut _,
            face_type,
            face_index_buffer.as_mut_ptr() as *mut _,
            uv_type,
            uv_per_face_buffer.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => {
                unsafe {
                    vertex_buffer.set_len(vertex_buffer_size);
                    face_index_buffer.set_len(face_index_buffer_size);
                    uv_per_face_buffer.set_len(uv_per_face_buffer_size);
                }

                Ok((
                    vertex_buffer,
                    face_index_buffer,
                    uv_per_face_buffer,
                ))
            }
            Err(e) => Err(e),
        }
    }
}
