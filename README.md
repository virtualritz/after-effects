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
- [x] AE Plugin Helper
- [x] AE Plugin Helper 2
- [ ] AEGP Artisan Util
- [x] AEGP Camera
- [x] AEGP Canvas
- [ ] AEGP Collection
- [ ] AEGP Command
- [x] AEGP Comp
- [x] AEGP Composite
- [ ] AEGP Compute
- [x] AEGP Dynamic Stream
- [x] AEGP Effect
- [ ] AEGP File Import Manager
- [x] AEGP Footage
- [ ] AEGP Hash
- [x] AEGP IO In
- [ ] AEGP IO Out
- [x] AEGP Item
- [ ] AEGP Item View
- [ ] AEGP Iterate
- [x] AEGP Keyframe
- [ ] AEGP Layer Mask
- [ ] AEGP Layer Render Options
- [x] AEGP Layer
- [x] AEGP Light
- [ ] AEGP Marker
- [x] AEGP Mask Outline
- [x] AEGP Mask
- [ ] AEGP Math
- [x] AEGP Memory
- [ ] AEGP Output Module
- [ ] AEGP Persistent Data
- [x] AEGP PF Interface
- [ ] AEGP Proj
- [ ] AEGP QueryXform
- [ ] AEGP Register
- [ ] AEGP Render Asyc Manager
- [ ] AEGP Render Options
- [ ] AEGP Render Queue Item
- [ ] AEGP Render Queue
- [ ] AEGP Render
- [ ] AEGP RenderQueue Monitor
- [ ] AEGP Sound Data
- [x] AEGP Stream
- [ ] AEGP Text Document
- [ ] AEGP Text Layer
- [ ] AEGP Tracker
- [ ] AEGP Tracker Utility
- [x] AEGP Utility
- [ ] AEGP Workspace Panel
- [x] AEGP World
- [x] DRAWBOT Draw
- [x] DRAWBOT Image
- [x] DRAWBOT Path
- [x] DRAWBOT Pen
- [x] DRAWBOT Supplier
- [x] DRAWBOT Surface
- [x] PF AE Adv App
- [ ] PF AE Adv Item
- [ ] PF AE Adv Time
- [x] PF AE App
- [ ] PF AngleParam
- [ ] PF ANSI
- [ ] PF Background Frame
- [ ] PF Batch Sampling
- [ ] PF Cache On Load
- [x] PF Color Settings
- [ ] PF Color
- [ ] PF Color16
- [ ] PF ColorFloat
- [ ] PF ColorParam
- [x] PF Effect Custom UI
- [x] PF Effect Custom UI Overlay Theme
- [x] PF Effect Sequence Data
- [x] PF Effect UI
- [ ] PF Fill Matte
- [x] PF GPU Device
- [ ] PF Handle
- [x] PF Iterate8
- [x] PF iterate16
- [x] PF iterateFloat
- [x] PF Param Utils
- [ ] PF Path Data
- [ ] PF Path Query
- [ ] PF Pixel Data
- [x] PF Pixel Format
- [ ] PF PointParam
- [ ] PF Sampling8
- [ ] PF Sampling16
- [ ] PF SamplingFloat
- [ ] PF Source Settings
- [ ] PF Transition
- [x] PF Utility
- [x] PF World
- [ ] PF World Transform

## Adobe Premiere:
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
