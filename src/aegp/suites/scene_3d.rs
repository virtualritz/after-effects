use crate::*;
use crate::aegp::*;

define_suite!(
    Scene3DSuite,
    AEGP_Scene3DSuite2,
    kAEGPScene3DSuite,
    kAEGPScene3DSuiteVersion2
);

impl Scene3DSuite {
    pub fn alloc(&self) -> Result<*mut ae_sys::AEGP_Scene3D, Error> {
        call_suite_fn_single!(self, AEGP_Scene3DAlloc -> *mut ae_sys::AEGP_Scene3D)
    }

    pub fn dispose(&self, scene3d_ptr: *mut ae_sys::AEGP_Scene3D) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_Scene3DDispose, scene3d_ptr)
    }

    pub fn alloc_texture_context(&self, in_data_handle: pr::InDataHandle, render_context: pr::RenderContextHandle, global_texture_cache_handle: Scene3DTextureCacheHandle) -> Result<*mut ae_sys::AEGP_Scene3DTextureContext, Error> {
        call_suite_fn_single!(self,
            AEGP_Scene3DTextureContextAlloc -> *mut ae_sys::AEGP_Scene3DTextureContext,
            in_data_handle.as_ptr(),
            render_context.as_ptr(),
            global_texture_cache_handle.as_ptr(),
            false as u8 // unlock all)
        )
    }
    pub fn dispose_texture_context(&self, texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_Scene3DTextureContextDispose, texture_context_ptr)
    }

    pub fn alloc_texture_cache(&self) -> Result<Scene3DTextureCacheHandle, Error> {
        Ok(Scene3DTextureCacheHandle::from_raw(
            call_suite_fn_single!(self, AEGP_Scene3DTextureCacheAlloc -> *mut ae_sys::AEGP_Scene3DTextureCache)?
        ))
    }

    /*pub fn setup_motion_blur_samples(&self, scene3d: *mut ae_sys::AEGP_Scene3D, in_data_handle: pr::InDataHandle, render_context: pr::RenderContextHandle, motion_samples: usize, sample_method: ae_sys::Scene3DMotionSampleMethod,) -> Result<(), Error> {
        call_suite_fn!(self,
            AEGP_Scene3D_SetupMotionBlurSamples,
            self.in_data_ptr,
            self.render_context_ptr,
            // the empty scene, modified
            scene3d,
            // how many motion samples
            motion_samples as i32,
            sample_method
        )
    }

    pub fn build(&self, progress_abort_callback_ptr: *mut ae_sys::AEGP_Scene3DProgressAbort) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_Scene3D_Build,
            self.in_data_ptr,
            self.render_context_ptr,
            self.texture_context_ptr,
            progress_abort_callback_ptr,
            self.scene3d_ptr
        )
    }*/

    pub fn scene_num_lights(&self, scene3d: *mut ae_sys::AEGP_Scene3D) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_Scene3DSceneNumLights -> ae_sys::A_long, scene3d)? as usize)
    }

    // FIXME: make this neat, see
    // https://blog.seantheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
    /*#[inline]
    pub fn node_traverser(
        &self,
        node_visitor_func: ae_sys::Scene3DNodeVisitorFunc,
        reference_context: *mut std::os::raw::c_void, /* FIXME: can we use a Box
                                                       * here? Box<*
                                                       * mut
                                                       * ::std::os::raw::c_void> */
        flags: ae_sys::Scene3DTraverseFlags,
    ) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_Scene3DNodeTraverser,
            self.scene3d_ptr,
            node_visitor_func,
            reference_context,
            flags
        )
    }*/

    pub fn layer_num_post_xform(&self, scene3d_layer_handle: &Scene3DLayerHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_Scene3DLayerNumPostXform -> ae_sys::A_long, scene3d_layer_handle.as_ptr())? as usize)
    }

    pub fn num_sub_frame_times(&self, scene3d: *mut ae_sys::AEGP_Scene3D) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_Scene3DNumSubFrameTimes -> ae_sys::A_long, scene3d)? as usize)
    }

    pub fn layer_post_xform(&self, layer_handle: &Scene3DLayerHandle, index: usize) -> Result<Matrix4, Error> {
        let mut matrix_ptr = std::mem::MaybeUninit::<*const Matrix4>::uninit();

        call_suite_fn!(self, AEGP_Scene3DLayerGetPostXform, layer_handle.as_ptr(), index as i32, matrix_ptr.as_mut_ptr() as *mut *const _)?;

        Ok(unsafe {
            let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();
            std::ptr::copy(matrix_ptr.assume_init(), matrix.as_mut_ptr(), 1);
            matrix.assume_init()
        })
    }

    pub fn sub_frame_time(&self, scene3d: *mut ae_sys::AEGP_Scene3D, index: usize) -> Result<Time, Error> {
        Ok(call_suite_fn_single!(self, AEGP_Scene3DGetSubFrameTime -> ae_sys::A_Time, scene3d, index as i32)?.into())
    }
}

define_suite!(
    Scene3DMaterialSuite,
    AEGP_Scene3DMaterialSuite1,
    kAEGPScene3DMaterialSuite,
    kAEGPScene3DMaterialSuiteVersion1
);

impl Scene3DMaterialSuite {
    pub fn has_uv_color_texture(&self, material_handle: Scene3DMaterialHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_HasUVColorTexture -> ae_sys::A_Boolean,  material_handle.as_ptr())? != 0)
    }

    pub fn uv_color_texture(&self, material: Scene3DMaterialHandle) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetUVColorTexture -> ae_sys::AEGP_WorldH, material.as_ptr())?
        ))
    }

    pub fn basic_coeffs(&self, material: Scene3DMaterialHandle) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, Error> {
        let mut basic_material_coefficients = Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

        call_suite_fn!(self, AEGP_GetBasicCoeffs, material.as_ptr(), basic_material_coefficients.as_mut_ptr())?;

        Ok(unsafe { basic_material_coefficients.assume_init() })
    }
}

define_suite!(
    Scene3DNodeSuite,
    AEGP_Scene3DNodeSuite1,
    kAEGPScene3DNodeSuite,
    kAEGPScene3DNodeSuiteVersion1
);

impl Scene3DNodeSuite {
    pub fn material_for_side(&self, node_handle: Scene3DNodeHandle, side: ae_sys::AEGP_Scene3DMaterialSide) -> Result<Scene3DMaterialHandle, Error> {
        Ok(Scene3DMaterialHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetMaterialForSide -> *const ae_sys::AEGP_Scene3DMaterial, node_handle.as_ptr(), side)?
        ))
    }

    pub fn node_mesh(&self, node_handle: Scene3DNodeHandle) -> Result<Scene3DMeshHandle, Error> {
        Ok(Scene3DMeshHandle::from_raw(
            call_suite_fn_single!(self, AEGP_NodeMeshGet -> *const ae_sys::AEGP_Scene3DMesh, node_handle.as_ptr())?
        ))
    }

    pub fn node_post_xform(&self, scene3d_node_handle: Scene3DNodeHandle, index: usize) -> Result<Matrix4, Error> {
        let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();

        call_suite_fn!(self, AEGP_NodePostXformGet, scene3d_node_handle.as_ptr(), index as i32, matrix.as_mut_ptr() as *mut _)?;

        Ok(unsafe { matrix.assume_init() })
    }
}

define_suite!(
    Scene3DMeshSuite,
    AEGP_Scene3DMeshSuite1,
    kAEGPScene3DMeshSuite,
    kAEGPScene3DMeshSuiteVersion1
);

impl Scene3DMeshSuite {
    pub fn face_group_buffer_count(&self, mesh_handle: Scene3DMeshHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_FaceGroupBufferCount -> ae_sys::A_long, mesh_handle.as_ptr())? as _)
    }

    pub fn face_group_buffer_size(&self, mesh_handle: Scene3DMeshHandle, group_index: usize) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_FaceGroupBufferSize -> ae_sys::A_long, mesh_handle.as_ptr(), group_index as i32)? as _)
    }

    pub fn fill_face_group_buffer(&self, mesh_handle: Scene3DMeshHandle, group_index: usize) -> Result<Vec<ae_sys::A_long>, Error> {
        let face_count = self.face_group_buffer_size(mesh_handle, group_index)?;

        let mut face_index_buffer = Vec::<ae_sys::A_long>::with_capacity(face_count as usize);

        call_suite_fn!(
            self,
            AEGP_FaceGroupBufferFill,
            mesh_handle.as_ptr(),
            group_index as i32,
            face_count as i32,
            face_index_buffer.as_mut_ptr()
        )?;

        // If the previous called didn't bitch we are safe
        // to set the vector's length.
        unsafe {
            face_index_buffer.set_len(face_count as usize);
        }

        Ok(face_index_buffer)
    }

    pub fn material_side_for_face_group(&self, mesh_handle: Scene3DMeshHandle, group_index: usize) -> Result<ae_sys::AEGP_Scene3DMaterialSide, Error> {
        call_suite_fn_single!(self, AEGP_GetMaterialSideForFaceGroup -> ae_sys::AEGP_Scene3DMaterialSide, mesh_handle.as_ptr(), group_index as i32)
    }

    pub fn mesh_info(&self, mesh_handle: Scene3DMeshHandle) -> Result<(usize, usize), Error> {
        let (num_vertex, num_face) = call_suite_fn_double!(self, AEGP_MeshGetInfo -> i32, i32, mesh_handle.as_ptr())?;
        Ok((
            num_vertex as usize,
            num_face as usize,
        ))
    }

    pub fn vertex_buffer_element_size(&self, vertex_type: ae_sys::Scene3DVertexBufferType) -> usize {
        call_suite_fn_no_err!(self, AEGP_VertexBufferElementSize, vertex_type) as _
    }

    pub fn face_index_element_size(&self, face_type: ae_sys::Scene3DFaceBufferType) -> usize {
        call_suite_fn_no_err!(self, AEGP_FaceBufferElementSize, face_type) as _
    }

    pub fn uv_buffer_element_size(&self, uv_type: ae_sys::Scene3DUVBufferType) -> usize {
        call_suite_fn_no_err!(self, AEGP_UVBufferElementSize, uv_type) as _
    }

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
        let (num_vertex, num_face) = self.mesh_info(mesh_handle)?;

        // Points (3-tuples) of f64
        let vertex_buffer_size: usize = num_vertex * 3;
        let mut vertex_buffer = Vec::<ae_sys::A_FpLong>::with_capacity(vertex_buffer_size);

        // quad meshes
        let face_index_buffer_size: usize = num_face * 4;
        let mut face_index_buffer = Vec::<ae_sys::A_long>::with_capacity(face_index_buffer_size);

        // 2 uvs per vertex per face
        let uv_per_face_buffer_size: usize = match uv_type {
            ae_sys::Scene3DUVBufferType_SCENE3D_UVPERFACEBUFFER_QUAD_FpShort2 => num_face * 4 * 2,
            _ => 0,
        };
        let mut uv_per_face_buffer = Vec::<ae_sys::A_FpShort>::with_capacity(uv_per_face_buffer_size);

        call_suite_fn!(
            self,
            AEGP_MeshFillBuffers,
            mesh_handle.as_ptr() as *mut _,
            vertex_type,
            vertex_buffer.as_mut_ptr() as *mut _,
            face_type,
            face_index_buffer.as_mut_ptr() as *mut _,
            uv_type,
            uv_per_face_buffer.as_mut_ptr() as *mut _,
        )?;

        unsafe {
            vertex_buffer.set_len(vertex_buffer_size);
            face_index_buffer.set_len(face_index_buffer_size);
            uv_per_face_buffer.set_len(uv_per_face_buffer_size);
        }

        Ok((vertex_buffer, face_index_buffer, uv_per_face_buffer))
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

pub type MaterialBasic = ae_sys::AEGP_MaterialBasic_v1;

define_handle_wrapper!(Scene3DLayerHandle, AEGP_Scene3DLayer);
define_handle_wrapper!(Scene3DTextureCacheHandle, AEGP_Scene3DTextureCache);

define_ptr_wrapper!(Scene3DMaterialHandle, AEGP_Scene3DMaterial);
define_handle_wrapper!(Scene3DNodeHandle, AEGP_Scene3DNodeP);
define_ptr_wrapper!(Scene3DMeshHandle, AEGP_Scene3DMesh);
