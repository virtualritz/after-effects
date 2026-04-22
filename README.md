# Rust Bindings for Adobe After Effects® & Premiere Pro® SDKs

High-level, safe Rust bindings for the Adobe After Effects (Ae) and Adobe Premiere Pro (Pr) SDKs.

* Wraps many Ae & Pr API suites in safe Rust.
* Provides macros to generate all plugin boilerplate.
* Build and package entirely with **Rust** – no external build tools needed.

## Quick Start

1. Install [`just`](https://github.com/casey/just):

   ```bash
   cargo install just
   ```

2. Add dependencies in `Cargo.toml`:

   ```bash
   cargo add after-effects
   cargo add --dev pipl
   ```

3. Define your plugin description in `build.rs` and write your plugin code. Check out [examples/](https://github.com/virtualritz/after-effects/tree/master/examples).

4. Download [`AdobePlugin.just`](https://raw.githubusercontent.com/virtualritz/after-effects/master/AdobePlugin.just) into your project root (next to `Cargo.toml`).

5. Build your plugin:

   ```bash
   just build   # Debug build (logging and catching panics)
   just release # Optimized release build
   ```

> [!NOTE]
> If using Premiere SDK features, add to your `build.rs`:
>
> ```rust
> println!("cargo:rustc-cfg=with_premiere");
> ```

## Examples

Basic plugin examples: [examples/](https://github.com/virtualritz/after-effects/tree/master/examples).

Build an example:

```bash
CARGO_TARGET_DIR=$(pwd)/target
just build
```

#### Advanced plugins
* [Gyroflow plugin](https://github.com/gyroflow/gyroflow-plugins) is an example of pure Rust plugin with efficient zero-copy GPU rendering.
* [ntsc-rs](https://github.com/valadaptive/ntsc-rs) is a video effect which emulates NTSC and VHS video artifacts.
* For more advanced use cases, refer to the C/C++ examples from the SDK.

#### Debugging tips:

* If the plugin fails to load, check `Plugin Loading.log` or run AfterEffects from the CLI in your debugger of choice.
* On macOS, common pitfails are signing or an issue with `PkgInfo`/`Info.plist`.
* On Windows, logs appear in **DbgView**. On macOS, use **Console**.


## Development Notes

When developing your plugin it's best to use the debug build - it will catch and display panics
for you and it will log any messages and stack traces to `DbgView` (on Windows) or `Console` (on
macOS). This is done by running `just build`.

The release version can be built using `just release`

* To enable optimizations in the debug build:

  ```toml
  [profile.dev]
  opt-level = 3
  ```
* To add debug symbols in release builds:

  ```toml
  [profile.release]
  debug = true
  debug-assertions = true
  ```
* To enable panic catching in release builds, add this to `build.rs`:

  ```rust
  println!("cargo:rustc-cfg=catch_panics");
  ```

## After Effects vs Premiere

* Ae is the main plugin engine; Pr loads most Ae plugins.
* Pr lacks `AEGP` suites and always uses software rendering.
* Pr defines a separate GPU entry point via [`premiere::define_gpu_filter!`](https://docs.rs/premiere/latest/premiere/macro.define_gpu_filter.html).
* You cannot build a Pr-only video filter – Ae SDK is required.

## Regenerating Bindings

Pre-generated bindings are included, so building the plugin _just works_.

If you want to regenerate the SDK bindings:

1. Download the [Adobe After Effects SDK](https://console.adobe.io/downloads/ae).
2. Set environment variables:

   ```bash
   export AESDK_ROOT=/path/to/AfterEffectsSDK
   export PRSDK_ROOT=/path/to/PremiereSDK
   ```
3. Run `just build`.

⚠️ Note: Adobe’s public SDK may be outdated for 3D Artisan API. This mainly affects custom 3D renderer plugins. 2D plugins are unaffected.

## Wrapped Suites

Extensive portions of Ae and Pr SDKs are wrapped:
<details>
<summary>After Effects suites</summary>

| AEGP                    | PF                                | DRAWBOT     | Other                 |
| ----------------------- | --------------------------------- | ----------- | --------------------- |
| 🔳 Artisan Util         | ✅ AE Adv App                     | ✅ Draw     | ✅ AE Plugin Helper   |
| ✅ Camera               | ✅ AE Adv Item                    | ✅ Image    | ✅ AE Plugin Helper 2 |
| ✅ Canvas               | 🔳 AE Adv Time                    | ✅ Path     |                       |
| 🔳 Collection           | ✅ AE App                         | ✅ Pen      |                       |
| ✅ Command              | ✅ AngleParam                     | ✅ Supplier |                       |
| ✅ Comp                 | 🔳 ANSI                           | ✅ Surface  |                       |
| ✅ Composite            | ✅ Background Frame               |             |                       |
| 🔳 Compute              | 🔳 Batch Sampling                 |             |                       |
| ✅ Dynamic Stream       | ✅ Cache On Load                  |             |                       |
| ✅ Effect               | ✅ Channel                        |             |                       |
| 🔳 File Import Manager  | ✅ Color Settings                 |             |                       |
| ✅ Footage              | ✅ Color Callbacks                |             |                       |
| 🔳 Hash                 | ✅ Color Callbacks 16             |             |                       |
| ✅ IO In                | ✅ Color Callbacks Float          |             |                       |
| 🔳 IO Out               | ✅ ColorParam                     |             |                       |
| ✅ Item                 | ✅ Effect Custom UI               |             |                       |
| 🔳 Item View            | ✅ Effect Custom UI Overlay Theme |             |                       |
| 🔳 Iterate              | ✅ Effect Sequence Data           |             |                       |
| ✅ Keyframe             | ✅ Effect UI                      |             |                       |
| 🔳 Layer Mask           | ✅ Fill Matte                     |             |                       |
| ✅ Layer Render Options | ✅ GPU Device                     |             |                       |
| ✅ Layer                | ✅ Handle                         |             |                       |
| ✅ Light                | ✅ Iterate8                       |             |                       |
| 🔳 Marker               | ✅ iterate16                      |             |                       |
| ✅ Mask Outline         | ✅ iterateFloat                   |             |                       |
| ✅ Mask                 | ✅ Param Utils                    |             |                       |
| 🔳 Math                 | ✅ Path Data                      |             |                       |
| ✅ Memory               | ✅ Path Query                     |             |                       |
| ✅ Output Module        | ✅ Pixel Data                     |             |                       |
| ✅ Persistent Data      | ✅ Pixel Format                   |             |                       |
| ✅ PF Interface         | ✅ PointParam                     |             |                       |
| ✅ Proj                 | 🔳 Sampling8                      |             |                       |
| 🔳 QueryXform           | 🔳 Sampling16                     |             |                       |
| ✅ Register             | 🔳 SamplingFloat                  |             |                       |
| ✅ Render Asyc Manager  | ✅ Source Settings                |             |                       |
| ✅ Render Options       | ✅ Transition                     |             |                       |
| ✅ Render Queue Item    | ✅ Utility                        |             |                       |
| ✅ Render Queue         | ✅ World                          |             |                       |
| ✅ Render               | ✅ World Transform                |             |                       |
| 🔳 RenderQueue Monitor  |                                   |             |                       |
| ✅ Sound Data           |                                   |             |                       |
| ✅ Stream               |                                   |             |                       |
| 🔳 Text Document        |                                   |             |                       |
| 🔳 Text Layer           |                                   |             |                       |
| 🔳 Tracker              |                                   |             |                       |
| 🔳 Tracker Utility      |                                   |             |                       |
| ✅ Utility              |                                   |             |                       |
| 🔳 Workspace Panel      |                                   |             |                       |
| ✅ World                |                                   |             |                       |

*The register suite currently excludes the artisan and AEIO registration API
</details>

<details>
<summary>Premiere suites</summary>

| Premiere                  | MediaCore                        | Control Surface                 | Other                    |
| ------------------------- | -------------------------------- | ------------------------------- | ------------------------ |
| 🔳 Audio                  | 🔳 Accelerated Render Invocation | 🔳 ControlSurface               | ✅ PF Background Frame   |
| 🔳 Clip Render            | 🔳 App Info                      | 🔳 ControlSurface Command       | ✅ PF Cache On Load      |
| 🔳 Deferred Processing    | 🔳 Application Settings          | 🔳 ControlSurface Lumetri       | ✅ PF Pixel Format       |
| 🔳 Error                  | 🔳 Async File Reader             | 🔳 ControlSurface Marker        | ✅ PF Source Settings    |
| 🔳 Export Audio Param     | 🔳 Async Operation               | 🔳 ControlSurface Mixer         | ✅ PF Transition         |
| 🔳 Export Standard Param  | 🔳 Export File                   | 🔳 ControlSurface Transport     | ✅ PF Utility            |
| 🔳 ExportController       | 🔳 Export Info                   | 🔳 ControlSurfaceHost           | ✅ Opaque Effect Data    |
| 🔳 File Registration      | 🔳 Export Param                  | 🔳 ControlSurfaceHost Command   | 🔳 Captioning            |
| 🔳 Image Processing       | 🔳 Export Progress               | 🔳 ControlSurfaceHost Lumetri   | 🔳 Effect Stream Label   |
| 🔳 Legacy                 | 🔳 Exporter Utility              | 🔳 ControlSurfaceHost Marker    | 🔳 FlashCueMarkerData    |
| 🔳 Media Accelerator      | ✅ GPU Device                    | 🔳 ControlSurfaceHost Mixer     | 🔳 Importer File Manager |
| ✅ Memory Manager         | ✅ GPU Image Processing          | 🔳 ControlSurfaceHost Transport | 🔳 Marker                |
| 🔳 Palette                | 🔳 Playmod Immersive Video       | 🔳 ControlSurfacePlugin         |                          |
| 🔳 Pixel Format           | 🔳 Playmod Overlay               | 🔳 String                       |                          |
| 🔳 Playmod Audio          | 🔳 Sequence Audio                |                                 |                          |
| 🔳 Playmod Device Control | ✅ Sequence Info                 |                                 |                          |
| ✅ PPix                   | 🔳 Sequence Render               |                                 |                          |
| ✅ PPix 2                 | 🔳 Smart Rendering               |                                 |                          |
| 🔳 PPix Cache             | 🔳 String                        |                                 |                          |
| 🔳 PPix Creator           | 🔳 Transmit Invocation           |                                 |                          |
| 🔳 PPix Creator 2         | ✅ Video Segment                 |                                 |                          |
| 🔳 RollCrawl              | 🔳 Video Segment Render          |                                 |                          |
| 🔳 Threaded Work          |                                  |                                 |                          |
| ✅ Time                   |                                  |                                 |                          |
| ✅ Window                 |                                  |                                 |                          |
</details>

## Contributing / Help Wanted

* Wrap missing suites (see tables above).
* Add more examples & docs.
* Improve error handling (e.g. [`color-eyre`](https://crates.io/crates/color-eyre)).

## License

Licensed under **Apache-2.0** or **BSD-3-Clause** or **MIT** or **Zlib**  at your option.