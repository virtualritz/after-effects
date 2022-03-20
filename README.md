# `after-effects`

Current version: 0.1.6

High(er) level bindings for the Adobe AfterEffects® (Ae) SDK.

This wraps many of the API suites in the Ae SDK and exposes them in safe
Rust.

This is WIP – only tested on `macOS`. Will likely require additional work to
build on `Windows`.

### Prequisites

Download the [*Adobe After Effects SDK*](https://console.adobe.io/downloads/ae).
> ⚠️ The SDK published by Adobe is outdated if you are using the 3D
> Artisan API to write your own 3D renderer plug-in.
> Also see [Features](#features) below for more information.
>
> Ignore this if you just want to develop 2D plugins (which still have
> access to 3D data).

Define the `AESDK_ROOT` environment variable that contains the path to your
Ae SDK. Typically the directory structure will look like this:

```
AfterEffectsSDK
├── After_Effects_SDK_Guide.pdf
├── Examples
    ├── AEGP
    ├── Effect
    ├── ...
```
### Features

* `artisan-2-api` – Use the 2nd generation Artisan 3D API. This is not
  included in the official Ae SDK. Specifically it requires:
  * `AE_Scene3D_Private.h`
  * `PR_Feature.h`

  Contact the Adobe Ae SDK team and ask nicely and they may send you
  theses headers.

### Using

Add `after-effects` to your dependencies.

```rust
cargo add after-effects
```

### Getting Started

There are currently no examples. Use the C/C++ examples in the Ae SDK as
guides for now. They translate more or less 1:1 to Rust when using this
crate.

### Help Wanted/To Do

* Examples! I have a few plug-ins but they need polishing.

* A build system extension that creates the bundle for Ae using
  `cargo-post`/`cargo-make`/`cargo-bundle`. I.e. enter one command to get a
  plug-in ready to load in Ae. Currently there are manual steps and they
  need documenting too.

* Better error handling. Possibly using [`color`](https://crates.io/crates/color-eyre)`-`[`eyre`](https://crates.io/crates/eyre)?

## License

Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib at your option.
