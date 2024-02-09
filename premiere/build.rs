use std::path::Path;

fn main() {
    if let Ok(sdk) = std::env::var("AESDK_ROOT") {
        if Path::new(&sdk).join("Examples/Headers/AE_Effect.h").exists() {
            println!("cargo:rustc-cfg=has_ae_sdk");
        }
    }
}
