use spirv_builder::SpirvBuilder;
use spirv_builder::Capability;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = SpirvBuilder::new("../spirv", "spirv-unknown-vulkan1.2")
        //.print_metadata(spirv_builder::MetadataPrintout::Full).spirv_metadata(spirv_builder::SpirvMetadata::Full)
        .preserve_bindings(true)
        .capability(Capability::ImageQuery)
        .build()?.module.unwrap_single().display().to_string();
    println!("cargo:rustc-env=shader_path={}", path);
    Ok(())
}
