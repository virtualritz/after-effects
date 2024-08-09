use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_ae_sdk)");
    if let Ok(sdk) = std::env::var("AESDK_ROOT") {
        if Path::new(&sdk).join("Examples/Headers/AE_Effect.h").exists() {
            println!("cargo:rustc-cfg=has_ae_sdk");
        }
    } else if std::env::var("PRSDK_ROOT").is_err() {
        // Likely using the built-in bindings
        println!("cargo:rustc-cfg=has_ae_sdk");
    }
}
