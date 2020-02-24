#![feature(new_uninit)]
#[macro_use]
mod macros;

//use std::mem; //::MaybeUninit;

use aftereffects_sys as ae_sys;
use std::{mem, ptr};

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

pub struct PFEffectWorld {
    pf_effect_world: Box<ae_sys::PF_EffectWorld>,
}

pub trait Suite: Drop {
    fn new(pica_basic_suite: &crate::PicaBasicSuiteHandle) -> Self;

    fn from_raw(
        pica_basic_suite_raw_ptr: *const crate::ae_sys::SPBasicSuite,
    ) -> Self;
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

    #[derive(Copy, Clone, Debug, Hash)]
    pub struct RenderContextHandle {
        pub render_context_ptr: ae_sys::PR_RenderContextH,
    }
}

// FIXME: combine handles and suite traits for
// all below.

pub mod aegp {
    use crate::{PFEffectWorld, Suite};
    use aftereffects_sys as ae_sys;

    #[derive(Copy, Clone, Debug, Hash)]
    pub struct WorldHandle {
        world_ptr: ae_sys::AEGP_WorldH,
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
        ) -> Scene3D {
            let suite_ptr = ae_acquire_suite_ptr!(
                pica_basic_suite_handle.as_ptr(),
                AEGP_Scene3DSuite2,
                kAEGPScene3DSuite,
                kAEGPScene3DSuiteVersion2
            )
            .expect(concat!(
                "Could not aquire ",
                stringify!(AEGP_Scene3DSuite2),
                "."
            ));

            let mut scene3d_ptr: *mut ae_sys::AEGP_Scene3D =
                std::ptr::null_mut();

            ae_call_suite_fn!(
                suite_ptr,
                AEGP_Scene3DAlloc,
                &mut scene3d_ptr,
            );

            let mut texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext = std::ptr::null_mut();

            ae_call_suite_fn!(
                suite_ptr,
                AEGP_Scene3DTextureContextAlloc,
                in_data_handle.as_ptr(),
                render_context.render_context_ptr,
                global_texture_cache_handle.texture_cache_ptr,
                false as u8, // unlock all
                &mut texture_context_ptr
            );

            Scene3D {
                pica_basic_suite_ptr: pica_basic_suite_handle.as_ptr(),
                suite_ptr: suite_ptr,
                scene3d_ptr: scene3d_ptr,
                texture_context_ptr: texture_context_ptr,
                in_data_ptr: in_data_handle.as_ptr(),
                render_context_ptr: render_context.render_context_ptr,
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
        ) {
            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_Scene3D_SetupMotionBlurSamples,
                self.in_data_ptr,
                self.render_context_ptr,
                self.scene3d_ptr, /* the empty scene,
                                   * modified */
                motion_samples as i32, // how many motion samples
                sample_method
            );
        }

        pub fn build(
            &self,
            progress_abort_callback_ptr: *mut ae_sys::AEGP_Scene3DProgressAbort,
        ) {
            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_Scene3D_Build,
                self.in_data_ptr,
                self.render_context_ptr,
                self.texture_context_ptr,
                progress_abort_callback_ptr,
                self.scene3d_ptr
            );
        }

        pub fn scene_num_lights(&self) -> usize {
            let mut num_lights: i32 = 0;
            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_Scene3DSceneNumLights,
                self.scene3d_ptr,
                &mut num_lights
            );

            num_lights as usize
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
        ) {
            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_Scene3DNodeTraverser,
                self.scene3d_ptr,
                node_visitor_func,
                reference_context,
                flags
            );
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
        pub fn new(scene3d: Scene3D) -> Scene3DTextureCacheHandle {
            let mut texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache = std::ptr::null_mut();

            ae_call_suite_fn!(
                scene3d.suite_ptr,
                AEGP_Scene3DTextureCacheAlloc,
                &mut texture_cache_ptr,
            );

            Scene3DTextureCacheHandle { texture_cache_ptr }
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
        WorldSuite,
        AEGP_WorldSuite3,
        kAEGPWorldSuite,
        kAEGPWorldSuiteVersion3
    );

    impl WorldSuite {
        pub fn fill_out_pf_effect_world(
            &self,
            world: &WorldHandle,
        ) -> PFEffectWorld {
            let mut pf_effect_world =
                Box::<ae_sys::PF_EffectWorld>::new_uninit();

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_FillOutPFEffectWorld,
                world.world_ptr,
                pf_effect_world.as_mut_ptr()
            );
            unsafe {
                PFEffectWorld {
                    pf_effect_world: pf_effect_world.assume_init(),
                }
            }
        }
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
        ) -> bool {
            let mut has_uv_color_texture: u8 = 0;

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_HasUVColorTexture,
                material_handle.material_ptr,
                &mut has_uv_color_texture
            );

            has_uv_color_texture != 0
        }

        pub fn get_uv_color_texture(
            &self,
            material: &Scene3DMaterialHandle,
        ) -> WorldHandle {
            let mut world_handle = WorldHandle {
                world_ptr: std::ptr::null_mut(),
            };
            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetUVColorTexture,
                material.material_ptr,
                &mut world_handle.world_ptr
            );
            world_handle
        }

        pub fn get_basic_coeffs(
            &self,
            material: &Scene3DMaterialHandle,
        ) -> Box<ae_sys::AEGP_MaterialBasic_v1> {
            let mut basic_material_coefficients =
                Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetBasicCoeffs,
                material.material_ptr,
                basic_material_coefficients.as_mut_ptr()
            );
            unsafe { basic_material_coefficients.assume_init() }
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
        ) -> Scene3DMaterialHandle {
            let mut material_handle = Scene3DMaterialHandle {
                material_ptr: std::ptr::null_mut(),
            };

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetMaterialForSide,
                node_handle.node_ptr,
                side,
                &mut material_handle.material_ptr
            );

            material_handle
        }

        pub fn node_mesh_get(
            &self,
            node_handle: &Scene3DNodeHandle,
        ) -> Scene3DMeshHandle {
            let mut mesh_handle = Scene3DMeshHandle {
                mesh_ptr: std::ptr::null_mut(),
            };

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_NodeMeshGet,
                node_handle.node_ptr,
                &mut mesh_handle.mesh_ptr
            );

            mesh_handle
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
        ) -> usize {
            let mut face_groups: ae_sys::A_long = 0;

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_FaceGroupBufferCount,
                mesh_handle.mesh_ptr,
                &mut face_groups
            );

            face_groups as usize
        }

        pub fn face_group_buffer_size(
            &self,
            mesh_handle: &Scene3DMeshHandle,
            group_index: usize,
        ) -> usize {
            let mut face_count: ae_sys::A_long = 0;

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_FaceGroupBufferSize,
                mesh_handle.mesh_ptr,
                group_index as i32,
                &mut face_count
            );

            face_count as usize
        }

        pub fn face_group_buffer_fill(
            &self,
            mesh_handle: &Scene3DMeshHandle,
            group_index: usize,
        ) -> Vec<ae_sys::A_long> {
            let face_count =
                self.face_group_buffer_size(mesh_handle, group_index);

            let mut face_index_buffer =
                Vec::<ae_sys::A_long>::with_capacity(
                    face_count as usize,
                );

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_FaceGroupBufferFill,
                mesh_handle.mesh_ptr,
                group_index as i32,
                face_count as i32,
                face_index_buffer.as_mut_ptr()
            );

            // If the previous called didn't bitch we are safe to
            // set the vector's length.
            unsafe {
                face_index_buffer.set_len(face_count as usize);
            }

            face_index_buffer
        }

        pub fn get_material_side_for_face_group(
            &self,
            mesh_handle: &Scene3DMeshHandle,
            group_index: usize,
        ) -> ae_sys::AEGP_Scene3DMaterialSide {
            let mut material_side: ae_sys::AEGP_Scene3DMaterialSide =
                ae_sys::AEGP_Scene3DMaterialSide_SCENE3D_MATERIAL_FRONT;

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_GetMaterialSideForFaceGroup,
                mesh_handle.mesh_ptr,
                group_index as i32,
                &mut material_side
            );

            material_side
        }

        pub fn mesh_get_info(
            &self,
            mesh_handle: &Scene3DMeshHandle,
        ) -> (usize, usize) {
            let mut num_vertex = 0;
            //std::mem::MaybeUninit::<&usize>::uninit();
            let mut num_face = 0;
            //std::mem::MaybeUninit::<&usize>::uninit();

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_MeshGetInfo,
                mesh_handle.mesh_ptr,
                &mut num_vertex as *mut _ as *mut i32,
                &mut num_face as *mut _ as *mut i32,
                /* num_vertex.as_mut_ptr() as *mut i32,
                 * num_face.as_mut_ptr() as *mut i32, */
            );

            /*unsafe {
                (*num_vertex.assume_init(), *num_face.assume_init())
            }*/
            (num_vertex, num_face)
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
        ) -> (
            Vec<ae_sys::A_FpLong>,
            Vec<ae_sys::A_long>,
            Vec<ae_sys::A_FpLong>,
        ) {
            let (num_vertex, num_face) =
                self.mesh_get_info(mesh_handle);

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

            ae_call_suite_fn!(
                self.suite_ptr,
                AEGP_MeshFillBuffers,
                mesh_handle.mesh_ptr,
                vertex_type,
                vertex_buffer.as_mut_ptr() as *mut _,
                face_type,
                face_index_buffer.as_mut_ptr() as *mut _,
                uv_type,
                uv_per_face_buffer.as_mut_ptr() as *mut _,
            );

            unsafe {
                vertex_buffer.set_len(vertex_buffer_size);
                face_index_buffer.set_len(face_index_buffer_size);
                uv_per_face_buffer.set_len(uv_per_face_buffer_size);
            }

            (vertex_buffer, face_index_buffer, uv_per_face_buffer)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
