# `after-effects`

Current version: 0.2.0

High level bindings for the Adobe AfterEffects® (Ae) SDK and Adobe Premiere Pro®.

This wraps many of the API suites in the Ae and Pr SDK and exposes them in safe
Rust.
It also defines a set of macros that implement all the plugin boilerplate for you,
so you can focus just on your actual plugin implementation.

Building the plugins is done entirely with Rust - there's no need to use any external
programs or dependencies.

Packaging of the final plugin is done using a `just` script. Install with `cargo install just` and
download [`AdobePlugin.just`](https://raw.githubusercontent.com/virtualritz/after-effects/master/AdobePlugin.just)
and put it next to your Cargo.toml.

Adobe plugins contain a special resource describing the plugin called `PiPL`. This repository
includes a `PiPL` tool written in Rust which generates the needed resource in `build.rs`.

Pre-generated SDK bindings are included, so you can compile the final plugin by just running
`just release`, and it works on both macOS and Windows.

You can also re-generate the bindings by downloading the SDK headers from Adobe and setting
`AESDK_ROOT` and/or `PRSDK_ROOT` environment variables.

### Features

* `artisan-2-api` – Use the 2nd generation Artisan 3D API. This is not
  included in the official Ae SDK. Specifically it requires:
  * `AE_Scene3D_Private.h`
  * `PR_Feature.h`

  Contact the Adobe Ae SDK team and ask nicely and they may send you
  theses headers.

### Using

Add `after-effects` or `premiere` to your dependencies and `pipl` to your dev-dependencies.

```rust
cargo add after-effects
```

### After Effects vs Premiere
Adobe plugins are shared between After Effects and Premiere.
The main engine is based on After Effects, but Premiere loads most of the Ae plugins.
While they have many common parts, there are some areas that are separated.
- Premiere is missing all `AEGP` suites
- Premiere uses only software rendering, even if the AE plugin supports GPU render and Smart Render
- Premiere has a separate entry point for GPU rendering, which can be defined using `premiere::define_gpu_filter!` macro.
- After Effects and Premiere also have some separate areas that are implemented independently
- You can't write a video filter plugin using only the Premiere SDK, the base engine is using Ae SDK

### Getting Started

### Examples

A few basic examples are included in the repository. For more advanced use cases,
refer to the C/C++ examples from the SDK.

For a more advanced sample with full GPU rendering you can check out the [Gyroflow plugin](https://github.com/gyroflow/gyroflow-plugins)

### Help Wanted/To Do

* If you need a suite that's not yet wrapped, feel free to create a PR wrapping that suite.

* Examples and documentation.

* Better error handling. Possibly using [`color`](https://crates.io/crates/color-eyre)`-`[`eyre`](https://crates.io/crates/eyre)?

### Using the Adobe SDK C++ headers

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


# Wrapped suites:
## After Effects:
| AEGP                                            | PF                                                      | DRAWBOT                                | Other                                    |
|-------------------------------------------------|---------------------------------------------------------|----------------------------------------|------------------------------------------|
| :white_square_button: AEGP Artisan Util         | :white_check_mark:    PF AE Adv App                     | :white_check_mark:    DRAWBOT Draw     | :white_check_mark:    AE Plugin Helper   |
| :white_check_mark:    AEGP Camera               | :white_square_button: PF AE Adv Item                    | :white_check_mark:    DRAWBOT Image    | :white_check_mark:    AE Plugin Helper 2 |
| :white_check_mark:    AEGP Canvas               | :white_square_button: PF AE Adv Time                    | :white_check_mark:    DRAWBOT Path     |                                          |
| :white_square_button: AEGP Collection           | :white_check_mark:    PF AE App                         | :white_check_mark:    DRAWBOT Pen      |                                          |
| :white_square_button: AEGP Command              | :white_square_button: PF AngleParam                     | :white_check_mark:    DRAWBOT Supplier |                                          |
| :white_check_mark:    AEGP Comp                 | :white_square_button: PF ANSI                           | :white_check_mark:    DRAWBOT Surface  |                                          |
| :white_check_mark:    AEGP Composite            | :white_square_button: PF Background Frame               |                                        |                                          |
| :white_square_button: AEGP Compute              | :white_square_button: PF Batch Sampling                 |                                        |                                          |
| :white_check_mark:    AEGP Dynamic Stream       | :white_square_button: PF Cache On Load                  |                                        |                                          |
| :white_check_mark:    AEGP Effect               | :white_check_mark:    PF Color Settings                 |                                        |                                          |
| :white_square_button: AEGP File Import Manager  | :white_square_button: PF Color                          |                                        |                                          |
| :white_check_mark:    AEGP Footage              | :white_square_button: PF Color16                        |                                        |                                          |
| :white_square_button: AEGP Hash                 | :white_square_button: PF ColorFloat                     |                                        |                                          |
| :white_check_mark:    AEGP IO In                | :white_square_button: PF ColorParam                     |                                        |                                          |
| :white_square_button: AEGP IO Out               | :white_check_mark:    PF Effect Custom UI               |                                        |                                          |
| :white_check_mark:    AEGP Item                 | :white_check_mark:    PF Effect Custom UI Overlay Theme |                                        |                                          |
| :white_square_button: AEGP Item View            | :white_check_mark:    PF Effect Sequence Data           |                                        |                                          |
| :white_square_button: AEGP Iterate              | :white_check_mark:    PF Effect UI                      |                                        |                                          |
| :white_check_mark:    AEGP Keyframe             | :white_square_button: PF Fill Matte                     |                                        |                                          |
| :white_square_button: AEGP Layer Mask           | :white_check_mark:    PF GPU Device                     |                                        |                                          |
| :white_square_button: AEGP Layer Render Options | :white_square_button: PF Handle                         |                                        |                                          |
| :white_check_mark:    AEGP Layer                | :white_check_mark:    PF Iterate8                       |                                        |                                          |
| :white_check_mark:    AEGP Light                | :white_check_mark:    PF iterate16                      |                                        |                                          |
| :white_square_button: AEGP Marker               | :white_check_mark:    PF iterateFloat                   |                                        |                                          |
| :white_check_mark:    AEGP Mask Outline         | :white_check_mark:    PF Param Utils                    |                                        |                                          |
| :white_check_mark:    AEGP Mask                 | :white_square_button: PF Path Data                      |                                        |                                          |
| :white_square_button: AEGP Math                 | :white_square_button: PF Path Query                     |                                        |                                          |
| :white_check_mark:    AEGP Memory               | :white_square_button: PF Pixel Data                     |                                        |                                          |
| :white_square_button: AEGP Output Module        | :white_check_mark:    PF Pixel Format                   |                                        |                                          |
| :white_square_button: AEGP Persistent Data      | :white_square_button: PF PointParam                     |                                        |                                          |
| :white_check_mark:    AEGP PF Interface         | :white_square_button: PF Sampling8                      |                                        |                                          |
| :white_square_button: AEGP Proj                 | :white_square_button: PF Sampling16                     |                                        |                                          |
| :white_square_button: AEGP QueryXform           | :white_square_button: PF SamplingFloat                  |                                        |                                          |
| :white_square_button: AEGP Register             | :white_square_button: PF Source Settings                |                                        |                                          |
| :white_square_button: AEGP Render Asyc Manager  | :white_square_button: PF Transition                     |                                        |                                          |
| :white_square_button: AEGP Render Options       | :white_check_mark:    PF Utility                        |                                        |                                          |
| :white_square_button: AEGP Render Queue Item    | :white_check_mark:    PF World                          |                                        |                                          |
| :white_square_button: AEGP Render Queue         | :white_check_mark:    PF World Transform                |                                        |                                          |
| :white_square_button: AEGP Render               |                                                         |                                        |                                          |
| :white_square_button: AEGP RenderQueue Monitor  |                                                         |                                        |                                          |
| :white_square_button: AEGP Sound Data           |                                                         |                                        |                                          |
| :white_check_mark:    AEGP Stream               |                                                         |                                        |                                          |
| :white_square_button: AEGP Text Document        |                                                         |                                        |                                          |
| :white_square_button: AEGP Text Layer           |                                                         |                                        |                                          |
| :white_square_button: AEGP Tracker              |                                                         |                                        |                                          |
| :white_square_button: AEGP Tracker Utility      |                                                         |                                        |                                          |
| :white_check_mark:    AEGP Utility              |                                                         |                                        |                                          |
| :white_square_button: AEGP Workspace Panel      |                                                         |                                        |                                          |
| :white_check_mark:    AEGP World                |                                                         |                                        |                                          |

## Premiere:
- [ ] ADOBESDK ControlSurface
- [ ] ADOBESDK ControlSurface Command
- [ ] ADOBESDK ControlSurface Lumetri
- [ ] ADOBESDK ControlSurface Marker
- [ ] ADOBESDK ControlSurface Mixer
- [ ] ADOBESDK ControlSurface Transport
- [ ] ADOBESDK ControlSurfaceHost
- [ ] ADOBESDK ControlSurfaceHost Command
- [ ] ADOBESDK ControlSurfaceHost Lumetri
- [ ] ADOBESDK ControlSurfaceHost Marker
- [ ] ADOBESDK ControlSurfaceHost Mixer
- [ ] ADOBESDK ControlSurfaceHost Transport
- [ ] ADOBESDK ControlSurfacePlugin
- [ ] ADOBESDK String
- [ ] Captioning
- [ ] Effect Stream Label
- [ ] FlashCueMarkerData
- [ ] Importer File Manager
- [ ] Marker
- [ ] MediaCore Accelerated Render Invocation
- [ ] MediaCore App Info
- [ ] MediaCore Application Settings
- [ ] MediaCore Async File Reader
- [ ] MediaCore Async Operation
- [ ] MediaCore Export File
- [ ] MediaCore Export Info
- [ ] MediaCore Export Param
- [ ] MediaCore Export Progress
- [ ] MediaCore Exporter Utility
- [x] MediaCore GPU Device
- [x] MediaCore GPU Image Processing
- [ ] MediaCore Playmod Immersive Video
- [ ] MediaCore Playmod Overlay
- [ ] MediaCore Sequence Audio
- [x] MediaCore Sequence Info
- [ ] MediaCore Sequence Render
- [ ] MediaCore Smart Rendering
- [ ] MediaCore String
- [ ] MediaCore Transmit Invocation
- [x] MediaCore Video Segment
- [ ] MediaCore Video Segment Render
- [x] Opaque Effect Data
- [ ] PF Background Frame
- [ ] PF Cache On Load
- [ ] PF Pixel Format
- [ ] PF Source Settings
- [ ] PF Transition
- [ ] PF Utility
- [ ] Premiere Audio
- [ ] Premiere Clip Render
- [ ] Premiere Deferred Processing
- [ ] Premiere Error
- [ ] Premiere Export Audio Param
- [ ] Premiere Export Standard Param
- [ ] Premiere ExportController
- [ ] Premiere File Registration
- [ ] Premiere Image Processing
- [ ] Premiere Legacy
- [ ] Premiere Media Accelerator
- [x] Premiere Memory Manager
- [ ] Premiere Palette
- [ ] Premiere Pixel Format
- [ ] Premiere Playmod Audio
- [ ] Premiere Playmod Device Control
- [x] Premiere PPix
- [x] Premiere PPix 2
- [ ] Premiere PPix Cache
- [ ] Premiere PPix Creator
- [ ] Premiere PPix Creator 2
- [ ] Premiere RollCrawl
- [ ] Premiere Threaded Work
- [x] Premiere Time
- [ ] Premiere Window

## License

Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib at your option.
