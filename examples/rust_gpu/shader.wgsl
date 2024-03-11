
struct Params {
    param_a: f32,
    param_r: f32,
    param_g: f32,
    param_b: f32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var input: texture_2d<u32>;
@group(0) @binding(2) var output: texture_storage_2d<rgba8uint, read_write>;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coord = vec2<i32>(global_id.xy);
    let pixel = textureLoad(input, coord, 0);

    let dims = textureDimensions(input);
    let mirror_coord = vec2<i32>(i32(dims.x) - coord.x, coord.y);

    let add_pixel = vec4<u32>(u32(params.param_a * 255.0), u32(params.param_r * 255.0), u32(params.param_g * 255.0), u32(params.param_b * 255.0));

    textureStore(output, mirror_coord, pixel + add_pixel);
}
