use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    if env::var("PRSDK_ROOT").is_err() {
        println!("cargo:rustc-cfg=builtin_bindings");
        return;
    }

    let pr_sdk_path = &env::var("PRSDK_ROOT").expect(
        "PRSDK_ROOT environment variable not set – cannot find Adobe Premiere SDK.\n\
        Please set PRSDK_ROOT to the root folder of your Adobe Premiere SDK\n\
        installation (this folder contains the Examples folder & the SDK\n\
        Guide PDF).",
    );

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut pr_bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //.derive_debug(true)
        .allowlist_item("PF.*")
        .allowlist_var("PF.*")
        .allowlist_item("kPF.*")
        .allowlist_item("kPR.*")
        .allowlist_item("kPr.*")
        .allowlist_item("PR_.*")
        .allowlist_item("kPr.*")
        .allowlist_item("kVideo.*")
        .allowlist_item("Pr.*")
        .allowlist_item("cs.*")
        .allowlist_item("kSP.*")
        .allowlist_item("kMax.*")
        .allowlist_item("suiteError_.*")
        .layout_tests(false)
        //.clang_arg("-include stdint.h")
        .clang_arg(format!(
            "-I{}",
            Path::new(pr_sdk_path)
                .join("Examples")
                .join("Headers")
                .display()
        ));
    if cfg!(target_os = "windows") {
        pr_bindings = pr_bindings.clang_arg("-D_WINDOWS");
    }

    if let Ok(ae_sdk) = env::var("AESDK_ROOT") {
        pr_bindings = pr_bindings.clang_arg(format!(
            "-I{}",
            Path::new(&ae_sdk)
                .join("Examples")
                .join("Headers")
                .display()
        ))
        .clang_arg("--define-macro=HAS_AE_SDK");
    }

    if cfg!(target_os = "macos") {
        pr_bindings = pr_bindings
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

    pr_bindings
        .generate()
        .expect("Unable to generate Premiere bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write Premiere bindings!");
}
