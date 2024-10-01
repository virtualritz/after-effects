fn main() {
    std::fs::copy(env!("shader_path"), "../shader.spv").unwrap();
}
