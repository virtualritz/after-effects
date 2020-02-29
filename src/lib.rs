// FIXME: make ALL the functions below return Result-wrapped values

#![feature(new_uninit)]

//#[macro_use]
//extern crate casey;

//use std::mem; //::MaybeUninit;

use aftereffects_sys as ae_sys;
use std::{mem, ptr};

use num_enum::{IntoPrimitive, UnsafeFromPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive)]
#[repr(i32)]
pub enum Error {
    Generic = ae_sys::A_Err_GENERIC as i32,
    Struct = ae_sys::A_Err_STRUCT as i32,
    Parameter = ae_sys::A_Err_PARAMETER as i32,
    Alloc = ae_sys::A_Err_ALLOC as i32,
    // Some calls can only be used on UI (Main) or Render threads.
    // Also, calls back to Ae can only be made from the same thread Ae
    // called you on.
    WrongThread = ae_sys::A_Err_WRONG_THREAD as i32,
    // An attempt was made to write to a read only copy of an AE
    // project. Project changes must originate in the UI/Main thread.
    ConstProjectModification =
        ae_sys::A_Err_CONST_PROJECT_MODIFICATION as i32,
    // Acquire suite failed on a required suite.
    MissingSuite = ae_sys::A_Err_MISSING_SUITE as i32,
}

struct AeError(pub i32);

impl From<Result<(), Error>> for AeError {
    fn from(result: Result<(), Error>) -> Self {
        match result {
            Ok(()) => AeError(0),
            Err(e) => AeError(e.into()),
        }
    }
}

impl From<AeError> for i32 {
    fn from(ae_error: AeError) -> Self {
        ae_error.0
    }
}

#[macro_use]
pub mod macros;

#[repr(C)]
pub struct Time {
    value: ae_sys::A_long,
    scale: ae_sys::A_u_long,
}

// This is confusing: for some structs, Ae expects the caller to
// manage the memory and for others it doesn't (the caller only
// deals with a pointer that gets dereferenced for actually
// calling into the suite). In this case the struct ends
// with a 'H' (for handle).
// When the struct misses the trailing 'H', Ae does expect us to
// manage the memory. We then use a Box<T>.

pub struct PicaBasicSuiteHandle {
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
}

impl PicaBasicSuiteHandle {
    pub fn from_raw(
        pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    ) -> PicaBasicSuiteHandle {
        /*if pica_basic_suite_ptr == ptr::null() {
            panic!()
        }*/
        PicaBasicSuiteHandle {
            pica_basic_suite_ptr: pica_basic_suite_ptr,
        }
    }

    pub fn as_ptr(&self) -> *const ae_sys::SPBasicSuite {
        self.pica_basic_suite_ptr
    }
}

pub trait Suite: Drop {
    fn new(pica_basic_suite: &crate::PicaBasicSuiteHandle) -> Self;

    fn from_raw(
        pica_basic_suite_raw_ptr: *const crate::ae_sys::SPBasicSuite,
    ) -> Self;
}

pub mod pf {
    use aftereffects_sys as ae_sys;

    // FIXME: wrap this nicely
    pub struct EffectWorld {
        pub effect_world: Box<ae_sys::PF_EffectWorld>,
    }
}

pub mod pr {
    use aftereffects_sys as ae_sys;

    #[derive(Copy, Clone, Debug, Hash)]
    pub struct InDataHandle {
        in_data_ptr: *const ae_sys::PR_InData,
    }

    impl InDataHandle {
        pub fn from_raw(
            in_data_ptr: *const ae_sys::PR_InData,
        ) -> InDataHandle {
            InDataHandle { in_data_ptr }
        }

        pub fn as_ptr(&self) -> *const ae_sys::PR_InData {
            self.in_data_ptr
        }

        pub fn pica_basic_handle(&self) -> crate::PicaBasicSuiteHandle {
            crate::PicaBasicSuiteHandle::from_raw(unsafe {
                (*self.in_data_ptr).pica_basicP
            })
        }

        pub fn plugin_id(&self) -> i32 {
            unsafe { (*self.in_data_ptr).aegp_plug_id }
        }

        pub fn reference_context_ptr(
            &self,
        ) -> Box<std::os::raw::c_void> {
            unsafe {
                Box::<std::os::raw::c_void>::from_raw(
                    (*self.in_data_ptr).aegp_refconPV,
                )
            }
        }
    }

    define_handle_wrapper!(
        RenderContextHandle,
        PR_RenderContextH,
        render_context_ptr
    );

    /*
    // FIXME: wrap this nicely
    #[derive(Copy, Clone, Debug, Hash)]
    pub struct RenderContextHandle {
        pub render_context_ptr: ae_sys::PR_RenderContextH,
    }

    impl RenderContextHandle {
        fn as_ptr(&self): ae_sys::PR_RenderContextH {
            render_context_ptr
        }
    }*/
}

// FIXME: combine handles and suite traits for
// all below.

pub mod aegp {
    use crate::{pf::EffectWorld, Suite};
    use aftereffects_sys as ae_sys;

    // FIXME: wrap this nicely or combine WorldHandle & WorldSuite into
    // single World
    #[derive(Copy, Clone, Debug, Hash)]
    pub struct WorldHandle {
        world_ptr: ae_sys::AEGP_WorldH,
    }

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
            let mut effect_world =
                Box::<ae_sys::PF_EffectWorld>::new_uninit();

            match ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_FillOutPFEffectWorld,
                world.world_ptr,
                effect_world.as_mut_ptr()
            ) {
                Ok(()) => Ok(EffectWorld {
                    effect_world: unsafe { effect_world.assume_init() },
                }),
                Err(e) => Err(e),
            }
        }
    }

    #[derive(Copy, Clone, Debug, Hash)]
    pub struct StreamReferenceHandle {
        stream_reference_ptr: ae_sys::AEGP_StreamRefH,
    }

    /*
    #[derive(Copy, Clone, Debug, Hash)]
    pub struct LayerHandle {
        layer_ptr: ae_sys::AEGP_LayerH,
    }

    impl LayerHandle {
        fn from_raw( layer_ptr: ae_sys::AEGP_LayerH ) -> LayerHandle {
            LayerHandle {
                layer_ptr
            }
        }
        fn as_ptr(&self) -> ae_sys::AEGP_LayerH {
            layer_ptr
        }
    }*/

    define_handle_wrapper!(LayerHandle, AEGP_LayerH, layer_ptr);

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
            plugin_id: u32,
            layer_handle: &LayerHandle,
            stream_name: ae_sys::AEGP_LayerStream, // FIXME
        ) -> Result<StreamReferenceHandle, crate::Error> {
            let mut stream_reference_ptr: ae_sys::AEGP_StreamRefH =
                std::ptr::null_mut();

            match ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetNewLayerStream,
                plugin_id as i32,
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
            plugin_id: u32,
            stream_reference_handle: &StreamReferenceHandle,
            time_mode: ae_sys::AEGP_LTimeMode, // FIXME
            time: &ae_sys::A_Time,             // FIXME
            sample_stream_pre_expression: bool,
        ) -> StreamValue {
            let mut stream_value =
                Box::<ae_sys::AEGP_StreamValue2>::new_uninit();

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetNewStreamValue,
                plugin_id as i32,
                stream_reference_handle.stream_reference_ptr,
                time_mode,
                &(*time),
                sample_stream_pre_expression as u8,
                stream_value.as_mut_ptr(),
            );

            StreamValue {
                stream_value: unsafe { stream_value.assume_init() },
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
        fn get_camera(
            &self,
            render_context_handle: &crate::pr::RenderContextHandle,
            time: &crate::Time,
        ) -> LayerHandle {
            let mut layer_ptr: ae_sys::AEGP_LayerH =
                std::ptr::null_mut();

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetCamera,
                render_context_handle.as_ptr(),
                &(*time) as *const _ as *const ae_sys::A_Time,
                &mut layer_ptr,
            );

            LayerHandle::from_raw(layer_ptr)
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
            pica_basic_suite_handle: &crate::PicaBasicSuiteHandle,

            in_data_handle: crate::pr::InDataHandle,
            render_context: crate::pr::RenderContextHandle,
            global_texture_cache_handle: crate::aegp::Scene3DTextureCacheHandle,
        ) -> Result<Scene3D, crate::Error> {
            let suite_ptr = ae_acquire_suite_ptr!(
                pica_basic_suite_handle.as_ptr(),
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
                    pica_basic_suite_ptr: pica_basic_suite_handle
                        .as_ptr(),
                    suite_ptr: suite_ptr,
                    scene3d_ptr: scene3d_ptr,
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
                self.scene3d_ptr, /* the empty scene,
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
        ) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, crate::Error>
        {
            let mut basic_material_coefficients =
                Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

            match ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetBasicCoeffs,
                material.material_ptr,
                basic_material_coefficients.as_mut_ptr()
            ) {
                Ok(()) => Ok(unsafe {
                    basic_material_coefficients.assume_init()
                }),
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
                Vec::<ae_sys::A_long>::with_capacity(
                    face_count as usize,
                );

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
        ) -> Result<ae_sys::AEGP_Scene3DMaterialSide, crate::Error>
        {
            let mut material_side: ae_sys::AEGP_Scene3DMaterialSide =
                ae_sys::AEGP_Scene3DMaterialSide_SCENE3D_MATERIAL_FRONT;

            match ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetMaterialSideForFaceGroup,
                mesh_handle.mesh_ptr,
                group_index as i32,
                &mut material_side
            ) {
                Ok(()) => Ok(material_side),
                Err(e) => Err(e),
            }
        }

        pub fn mesh_get_info(
            &self,
            mesh_handle: &Scene3DMeshHandle,
        ) -> Result<(usize, usize), crate::Error> {
            let mut num_vertex = 0;
            //std::mem::MaybeUninit::<&usize>::uninit();
            let mut num_face = 0;
            //std::mem::MaybeUninit::<&usize>::uninit();

            match ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_MeshGetInfo,
                mesh_handle.mesh_ptr,
                &mut num_vertex as *mut _ as *mut i32,
                &mut num_face as *mut _ as *mut i32,
                /* num_vertex.as_mut_ptr() as *mut i32,
                 * num_face.as_mut_ptr() as *mut i32, */
            ) {
                Ok(()) => {
                    /*unsafe {
                        (*num_vertex.assume_init(), *num_face.assume_init())
                    }*/
                    Ok((num_vertex, num_face))
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
            let (num_vertex, num_face) =
                self.mesh_get_info(mesh_handle)?;

            let vertex_buffer_size: usize = num_vertex * 3;
            let mut vertex_buffer =
                Vec::<ae_sys::A_FpLong>::with_capacity(
                    vertex_buffer_size,
                );

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
                        face_index_buffer
                            .set_len(face_index_buffer_size);
                        uv_per_face_buffer
                            .set_len(uv_per_face_buffer_size);
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
}
