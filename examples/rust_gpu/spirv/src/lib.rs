#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]

use spirv_std::glam::{ UVec4, IVec2, UVec2, UVec3, Vec3Swizzles};
use spirv_std::{ spirv, Image, image::{ ImageWithMethods, sample_with } };

#[repr(C)]
pub struct Params {
    param_mirror: f32,
    param_r: f32,
    param_g: f32,
    param_b: f32,
}

#[spirv(compute(threads(16, 16, 1)))]
pub fn main(
    #[spirv(global_invocation_id)] global_id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(descriptor_set = 0, binding = 1)] input: &Image!(2D, type=u32, sampled=true),
    #[spirv(descriptor_set = 0, binding = 2)] output: &Image!(2D, format=rgba8ui, sampled=false)
) {
    let coord = global_id.xy();
    let pixel = input.fetch_with(IVec2::new(global_id.x as i32, global_id.y as i32), sample_with::lod(0));

    let dims: UVec2 = input.query_size_lod(0);
    let mirror_coord = if params.param_mirror == 1.0 {
        UVec2::new(dims.x - coord.x, coord.y)
    } else {
        coord
    };

    let add_pixel = UVec4::new(0, (params.param_r * 255.0) as u32, (params.param_g * 255.0) as u32, (params.param_b * 255.0) as u32);

    unsafe {
        output.write(mirror_coord, pixel + add_pixel);
    }
}


