// build.rs
extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    println!("cargo::rustc-check-cfg=cfg(builtin_bindings)");
    if !env::var("AESDK_ROOT").is_ok_and(|x| !x.is_empty()) {
        println!("cargo:rustc-cfg=builtin_bindings");
        return;
    }

    let ae_sdk_path = &env::var("AESDK_ROOT").ok().filter(|x| !x.is_empty()).expect(
        "AESDK_ROOT environment variable not set – cannot find AfterEffects SDK.\n\
        Please set AESDK_ROOT to the root folder of your AfterEffects SDK\n\
        installation (this folder contains the Examples folder & the SDK\n\
        Guide PDF).",
    );

    if !Path::new(ae_sdk_path).exists() {
        panic!("AESDK_ROOT environment variable points to non-existent path: {ae_sdk_path}");
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut ae_bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //.derive_debug(true)
        .allowlist_item("A_.*")
        .allowlist_var("A_.*")
        .allowlist_item("AEGP.*")
        .allowlist_var("AEGP.*")
        .allowlist_item("kAEGP.*")
        .allowlist_item("AEIO_.*")
        .allowlist_item("DRAWBOT_.*")
        .allowlist_item("kDRAWBOT_.*")
        .allowlist_var("kDRAWBOT_.*")
        .allowlist_item("FIEL_.*")
        .allowlist_item("PF.*")
        .allowlist_var("PF.*")
        .allowlist_item("kPF.*")
        .allowlist_item("kPR.*")
        .allowlist_item("kPr.*")
        .allowlist_item("PR_.*")
        .allowlist_item("Pr.*")
        .allowlist_item("kSP.*")
        .allowlist_var("suiteError.*")
        .layout_tests(false)
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
    if cfg!(target_os = "windows") {
        ae_bindings = ae_bindings.clang_arg("-D_WINDOWS");
    }

    if cfg!(feature = "artisan-2-api") {
        ae_bindings = ae_bindings.clang_arg("--define-macro=ARTISAN_2_API");
    }

    if cfg!(target_os = "macos") {
        ae_bindings = ae_bindings
            .clang_arg("-Wno-elaborated-enum-base")
            //.clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
            //.clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
            //.clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
            .clang_arg(
                // FIXME: This will bitrot when clang updates or on really old macos instances
                "-I/Library/Developer/CommandLineTools/usr/lib/clang/12.0.0/include/stdint.h",
            )
            .clang_arg(
                "-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/",
            );
    }

    ae_bindings
        .generate()
        .expect("Unable to generate AfterEffects bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write AfterEffects bindings!");

    // Newer SDK changed the definition of PF_Point to contain two unions
    // This adds a lot of complexity in handling two different versions in Rust safe APIs
    // Let's just rewrite the bindings here and pretend it never happened
    if let Ok(mut content) = std::fs::read_to_string(out_path.join("bindings.rs")) {
        if content.contains("pub __bindgen_anon_1: PF_Point__bindgen_ty_1,") {
            content = content.replace("pub __bindgen_anon_1: PF_Point__bindgen_ty_1,", "pub h: A_long,");
            content = content.replace("pub __bindgen_anon_2: PF_Point__bindgen_ty_2,", "pub v: A_long,");
            std::fs::write(out_path.join("bindings.rs"), content).expect("Couldn't rewrite AfterEffects bindings!");
        }
    }
}
