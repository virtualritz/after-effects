use crate::{
    ae_sys, borrow_pica_basic_as_ptr, pr, Error, Matrix4, Suite, Time,
    WorldHandle,
};

pub type MaterialBasic = ae_sys::AEGP_MaterialBasic_v1;

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
    #[inline]
    pub fn new(
        in_data_handle: pr::InDataHandle,
        render_context: pr::RenderContextHandle,
        global_texture_cache_handle: Scene3DTextureCacheHandle,
    ) -> Result<Scene3D, Error> {
        let pica_basic_suite_ptr = in_data_handle.pica_basic_handle().as_ptr();

        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_Scene3DSuite2,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        )?;

        let mut scene3d_ptr: *mut ae_sys::AEGP_Scene3D = std::ptr::null_mut();

        ae_call_suite_fn!(suite_ptr, AEGP_Scene3DAlloc, &mut scene3d_ptr,)?;

        let mut texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext =
            std::ptr::null_mut();

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
                texture_context_ptr,
                in_data_ptr: in_data_handle.as_ptr(),
                render_context_ptr: render_context.as_ptr(),
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_scene3d_ptr(&self) -> *mut ae_sys::AEGP_Scene3D {
        self.scene3d_ptr
    }

    #[inline]
    pub fn get_scene3d_suite_ptr(&self) -> *const ae_sys::AEGP_Scene3DSuite2 {
        self.suite_ptr
    }

    #[inline]
    pub fn setup_motion_blur_samples(
        &self,
        motion_samples: usize,
        sample_method: ae_sys::Scene3DMotionSampleMethod,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3D_SetupMotionBlurSamples,
            self.in_data_ptr,
            self.render_context_ptr,
            // the empty scene, modified
            self.scene3d_ptr,
            // how many motion samples
            motion_samples as i32,
            sample_method
        )
    }

    #[inline]
    pub fn build(
        &self,
        progress_abort_callback_ptr: *mut ae_sys::AEGP_Scene3DProgressAbort,
    ) -> Result<(), Error> {
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

    #[inline]
    pub fn scene_num_lights(&self) -> Result<usize, Error> {
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
    #[inline]
    pub fn node_traverser(
        &self,
        node_visitor_func: ae_sys::Scene3DNodeVisitorFunc,
        reference_context: *mut std::os::raw::c_void, /* FIXME: can we use a Box
                                                       * here? Box<*
                                                       * mut
                                                       * ::std::os::raw::c_void> */
        flags: ae_sys::Scene3DTraverseFlags,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNodeTraverser,
            self.scene3d_ptr,
            node_visitor_func,
            reference_context,
            flags
        )
    }

    #[inline]
    pub fn layer_num_post_xform(
        &self,
        scene3d_layer_handle: &Scene3DLayerHandle,
    ) -> Result<usize, Error> {
        let mut num_xform = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DLayerNumPostXform,
            scene3d_layer_handle.as_ptr(),
            num_xform.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_xform.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn num_sub_frame_times(&self) -> Result<usize, Error> {
        let mut num_motion_samples = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNumSubFrameTimes,
            self.scene3d_ptr,
            num_motion_samples.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_motion_samples.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn layer_get_post_xform(
        &self,
        layer_handle: &Scene3DLayerHandle,
        index: usize,
    ) -> Result<Matrix4, Error> {
        let mut matrix_ptr = std::mem::MaybeUninit::<*const Matrix4>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DLayerGetPostXform,
            layer_handle.as_ptr(),
            index as i32,
            matrix_ptr.as_mut_ptr() as *mut *const _
        ) {
            Ok(()) => Ok({
                let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();
                unsafe {
                    std::ptr::copy(
                        matrix_ptr.assume_init(),
                        matrix.as_mut_ptr(),
                        1,
                    );
                    matrix.assume_init()
                }
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_sub_frame_time(&self, index: usize) -> Result<Time, Error> {
        let mut time = std::mem::MaybeUninit::<Time>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DGetSubFrameTime,
            self.scene3d_ptr,
            index as i32,
            time.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(unsafe { time.assume_init() }),
            Err(e) => Err(e),
        }
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

pub struct Scene3DLayerHandle {
    scene3d_layer_ptr: *const ae_sys::AEGP_Scene3DLayer,
}

impl Scene3DLayerHandle {
    #[inline]
    pub fn from_raw(
        scene3d_layer_ptr: *const ae_sys::AEGP_Scene3DLayer,
    ) -> Self {
        Self { scene3d_layer_ptr }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::AEGP_Scene3DLayer {
        self.scene3d_layer_ptr
    }
}

pub struct Scene3DTextureCacheHandle {
    texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
}

impl Scene3DTextureCacheHandle {
    #[inline]
    pub fn new(scene3d: Scene3D) -> Result<Scene3DTextureCacheHandle, Error> {
        let mut texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache =
            std::ptr::null_mut();

        match ae_call_suite_fn!(
            scene3d.suite_ptr,
            AEGP_Scene3DTextureCacheAlloc,
            &mut texture_cache_ptr,
        ) {
            Ok(()) => Ok(Scene3DTextureCacheHandle { texture_cache_ptr }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn from_raw(
        texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
    ) -> Scene3DTextureCacheHandle {
        Scene3DTextureCacheHandle { texture_cache_ptr }
    }
}

define_ptr_wrapper!(Scene3DMaterialHandle, AEGP_Scene3DMaterial);
define_handle_wrapper!(Scene3DNodeHandle, AEGP_Scene3DNodeP);
define_ptr_wrapper!(Scene3DMeshHandle, AEGP_Scene3DMesh);

define_suite!(
    Scene3DMaterialSuite,
    AEGP_Scene3DMaterialSuite1,
    kAEGPScene3DMaterialSuite,
    kAEGPScene3DMaterialSuiteVersion1
);

impl Scene3DMaterialSuite {
    #[inline]
    pub fn has_uv_color_texture(
        &self,
        material_handle: Scene3DMaterialHandle,
    ) -> Result<bool, Error> {
        let mut has_uv_color_texture: u8 = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_HasUVColorTexture,
            material_handle.as_ptr(),
            &mut has_uv_color_texture
        ) {
            Ok(()) => Ok(has_uv_color_texture != 0),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_uv_color_texture(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<WorldHandle, Error> {
        let mut world_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_WorldH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetUVColorTexture,
            material.as_ptr(),
            world_handle.as_mut_ptr()
        ) {
            Ok(()) => Ok(WorldHandle(unsafe { world_handle.assume_init() })),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_basic_coeffs(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, Error> {
        let mut basic_material_coefficients =
            Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBasicCoeffs,
            material.as_ptr(),
            basic_material_coefficients.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { basic_material_coefficients.assume_init() }),
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
    #[inline]
    pub fn get_material_for_side(
        &self,
        node_handle: Scene3DNodeHandle,
        side: ae_sys::AEGP_Scene3DMaterialSide,
    ) -> Result<Scene3DMaterialHandle, Error> {
        let mut material_handle = std::mem::MaybeUninit::<
            *const ae_sys::AEGP_Scene3DMaterial,
        >::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialForSide,
            node_handle.as_ptr(),
            side,
            material_handle.as_mut_ptr() as *mut *mut _
        ) {
            Ok(()) => Ok(Scene3DMaterialHandle(unsafe {
                material_handle.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn node_mesh_get(
        &self,
        node_handle: Scene3DNodeHandle,
    ) -> Result<Scene3DMeshHandle, Error> {
        let mut mesh_handle =
            std::mem::MaybeUninit::<*const ae_sys::AEGP_Scene3DMesh>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodeMeshGet,
            node_handle.as_ptr(),
            mesh_handle.as_mut_ptr() as *mut *mut _
        ) {
            Ok(()) => {
                Ok(Scene3DMeshHandle(unsafe { mesh_handle.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn node_post_xform_get(
        &self,
        scene3d_node_handle: Scene3DNodeHandle,
        index: usize,
    ) -> Result<Matrix4, Error> {
        let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodePostXformGet,
            scene3d_node_handle.as_ptr(),
            index as i32,
            matrix.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => {
                Ok(unsafe { matrix.assume_init() })
                //Ok((num_vertex, num_face))
            }
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
    #[inline]
    pub fn face_group_buffer_count(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<usize, Error> {
        let mut face_groups: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferCount,
            mesh_handle.as_ptr() as *mut _,
            &mut face_groups
        ) {
            Ok(()) => Ok(face_groups as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn face_group_buffer_size(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<usize, Error> {
        let mut face_count: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferSize,
            mesh_handle.as_ptr(),
            group_index as i32,
            &mut face_count
        ) {
            Ok(()) => Ok(face_count as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn face_group_buffer_fill(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<Vec<ae_sys::A_long>, Error> {
        let face_count =
            self.face_group_buffer_size(mesh_handle, group_index)?;

        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(face_count as usize);

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferFill,
            mesh_handle.as_ptr(),
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

    #[inline]
    pub fn get_material_side_for_face_group(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<ae_sys::AEGP_Scene3DMaterialSide, Error> {
        let mut material_side =
            std::mem::MaybeUninit::<ae_sys::AEGP_Scene3DMaterialSide>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialSideForFaceGroup,
            mesh_handle.as_ptr(),
            group_index as i32,
            material_side.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { material_side.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn mesh_get_info(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<(usize, usize), Error> {
        let mut num_vertex = std::mem::MaybeUninit::<i32>::uninit();
        let mut num_face = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshGetInfo,
            mesh_handle.as_ptr(),
            //&mut num_vertex as *mut _ as *mut i32,
            //&mut num_face as *mut _ as *mut i32,
            num_vertex.as_mut_ptr() as *mut i32,
            num_face.as_mut_ptr() as *mut i32,
        ) {
            Ok(()) => {
                Ok(unsafe {
                    (
                        num_vertex.assume_init() as usize,
                        num_face.assume_init() as usize,
                    )
                })
                //Ok((num_vertex, num_face))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn mesh_fill_buffers(
        &self,
        mesh_handle: Scene3DMeshHandle,
        vertex_type: ae_sys::Scene3DVertexBufferType,
        face_type: ae_sys::Scene3DFaceBufferType,
        uv_type: ae_sys::Scene3DUVBufferType,
    ) -> Result<
        (
            Vec<ae_sys::A_FpLong>,
            Vec<ae_sys::A_long>,
            Vec<ae_sys::A_FpShort>,
        ),
        Error,
    > {
        let (num_vertex, num_face) = self.mesh_get_info(mesh_handle)?;

        // Points (3-tuples) of f64
        let vertex_buffer_size: usize = num_vertex * 3;
        let mut vertex_buffer =
            Vec::<ae_sys::A_FpLong>::with_capacity(vertex_buffer_size);

        // quad meshes
        let face_index_buffer_size: usize = num_face * 4;
        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(face_index_buffer_size);

        // 2 uvs per vertex per face
        let uv_per_face_buffer_size: usize = match uv_type {
            ae_sys::Scene3DUVBufferType_SCENE3D_UVPERFACEBUFFER_QUAD_FpShort2 => num_face * 4 * 2,
            _ => 0,
        };
        let mut uv_per_face_buffer =
            Vec::<ae_sys::A_FpShort>::with_capacity(uv_per_face_buffer_size);

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshFillBuffers,
            mesh_handle.as_ptr() as *mut _,
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

                Ok((vertex_buffer, face_index_buffer, uv_per_face_buffer))
            }
            Err(e) => Err(e),
        }
    }
}
