#![allow(clippy::all)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
//! Raw After Effects® (Ae) API bindings via [bindgen](https://github.com/rust-lang/rust-bindgen).
//!
//! This is WIP – only tested on `macOS`. Will likely require additional work to
//! build on `Windows`.
//!
//! # Prequisites
//!
//! Download the [*Adobe After Effects SDK*](https://console.adobe.io/downloads/ae).
//! > Note that the SDK published by Adobe is outdated if you are using the 3D
//! > Artisan API to write your own 3D renderer plug-in.
//! > Also see [Features](#features) below for more information.
//! >
//! > Ignore this if you just want to develop 2D plugins (which still have
//! > access to 3D data).
//!
//! Define the `AESDK_ROOT` environment variable that contains the path to your
//! Ae SDK. Typically the directory structure will look like this:
//!
//! ```text
//! AfterEffectsSDK
//! ├── After_Effects_SDK_Guide.pdf
//! ├── Examples
//!     ├── AEGP
//!     ├── Effect
//!     ├── ...
//! ```
//!
//! Crate `version 0.1.5` was tested with the *Ae SDK* from **October 2021**.
//!
//! # Configuration
//!
//! The `build.rs` specifically looks into some of the folders under
//! `$AESDK_ROOT/Examples`.
//!
//! The `wrapper.hpp` file contains the headers you need to build your Ae
//! plugin. Modify as needed. If the header is some (new) SDK folder that
//! `build.rs` does not yet list, add it.
//!
//! ## Features
//!
//! * `artisan-2-api` – Use the 2nd generation Artisan 3D API. This is not
//!   included in the official Ae SDK. Specifically it requires:
//!   * `AE_Scene3D_Private.h`
//!   * `PR_Feature.h`
//!
//!   Contact the Adobe Ae SDK team and ask nicely and they may send you
//!   theses headers.
//!
//! ## macOS
//!
//! The `build.rs` contains `Clang` arguments to find neccessary system headers.
//! This is configured for _macOS 10.15_ (Catalina).
//!
//! You may need to edit this if you are targeting older system headers.
//!
//! ## Windows
//!
//! TBD. Help wanted!
//!
//! # Usage
//!
//! Until this is published it is suggested to use a link to this repository:
//!
//! ````
//! cargo add after-effects-sys.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
