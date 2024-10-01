#![allow(clippy::all)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#![doc = include_str!("../README.md")]

// Included bindings are generated from After Effects SDK dated May 2023

#[cfg(all(target_os = "windows", builtin_bindings))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings_win.rs"));

#[cfg(all(target_os = "macos", builtin_bindings))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings_macos.rs"));

#[cfg(not(builtin_bindings))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
