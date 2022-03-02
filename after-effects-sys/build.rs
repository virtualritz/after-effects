// build.rs
extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
};
//use std::process::Command;

fn main() {
    // TODO: make this generic & work on bot macOS & Windows

    println!("cargo:rerun-if-changed=wrapper.hpp");

    let ae_sdk_path = &env::var("AESDK_ROOT").expect(
        "AESDK_ROOT environment variable not set – cannot find AfterEffcts SDK.\n\
        Please set AESDK_ROOT to the root folder of your AfterEffects SDK\n\
        installation (this folder contains the Examples folder & the SDK\n\
        Guide PDF).",
    );

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let ae_bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //.derive_debug(true)
        .allowlist_function("A_.*")
        .allowlist_type("A_.*")
        .allowlist_var("A_.*")
        .allowlist_function("AEGP.*")
        .allowlist_type("AEGP.*")
        .allowlist_var("AEGP.*")
        .allowlist_var("kAEGP.*")
        .allowlist_var("AEIO_.*")
        .allowlist_function("DRAWBOT_.*")
        .allowlist_type("DRAWBOT_.*")
        .allowlist_var("DRAWBOT_.*")
        .allowlist_var("kDRAWBOT_.*")
        .allowlist_var("FIEL_.*")
        .allowlist_function("PF_.*")
        .allowlist_type("PF_.*")
        .allowlist_var("PF_.*")
        .allowlist_var("kPF.*")
        .allowlist_function("PR_.*")
        .allowlist_type("PR_.*")
        .allowlist_var("PR_.*")
        .allowlist_var("kSP.*")
        //.clang_arg("-include stdint.h")
        .clang_arg(format!(
            "-I{}",
            Path::new(ae_sdk_path)
                .join("Examples")
                .join("Headers")
                .display()
        ))
        .clang_arg(format!(
            "-I{}",
            Path::new(ae_sdk_path)
                .join("Examples")
                .join("Headers")
                .join("SP")
                .display()
        ))
        .clang_arg(format!(
            "-I{}",
            Path::new(ae_sdk_path)
                .join("Examples")
                .join("Util")
                .display()
        ));

    let ae_bindings = if cfg!(feature = "artisan-2-api") {
        ae_bindings.clang_arg("--define-macro=ARTISAN_2_API")
    } else {
        ae_bindings
    };

    let ae_bindings = if cfg!(target_os = "macos") {
        ae_bindings
            //.clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
            //.clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
            //.clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
            .clang_arg(
                "-I/Library/Developer/CommandLineTools/usr/lib/clang/12.0.0/include/stdint.h",
            )
            .clang_arg(
                "-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/",
            )
    } else {
        // TODO: Windows SDK paths
        ae_bindings
    };

    ae_bindings
        .generate()
        .expect("Unable to generate AfterEffects bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write AfterEffects bindings!");
}
