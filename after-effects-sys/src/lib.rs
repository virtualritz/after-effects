#![allow(clippy::all)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#![doc = include_str!("../README.md")]

// Included bindings are generated from After Effects SDK 27.0 (26.5 API additions) dated Oct 2026

#[cfg(all(target_os = "windows", builtin_bindings))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings_win.rs"));

#[cfg(all(any(target_os = "macos", target_os = "linux"), builtin_bindings))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings_macos.rs"));

#[cfg(not(builtin_bindings))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
