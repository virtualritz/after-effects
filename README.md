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
| AEGP                                       | PF                                                   | DRAWBOT                     | Other                                 |
|--------------------------------------------|------------------------------------------------------|-----------------------------|---------------------------------------|
| :white_square_button: Artisan Util         | :white_check_mark:    AE Adv App                     | :white_check_mark: Draw     | :white_check_mark: AE Plugin Helper   |
| :white_check_mark:    Camera               | :white_square_button: AE Adv Item                    | :white_check_mark: Image    | :white_check_mark: AE Plugin Helper 2 |
| :white_check_mark:    Canvas               | :white_square_button: AE Adv Time                    | :white_check_mark: Path     |                                       |
| :white_square_button: Collection           | :white_check_mark:    AE App                         | :white_check_mark: Pen      |                                       |
| :white_square_button: Command              | :white_square_button: AngleParam                     | :white_check_mark: Supplier |                                       |
| :white_check_mark:    Comp                 | :white_square_button: ANSI                           | :white_check_mark: Surface  |                                       |
| :white_check_mark:    Composite            | :white_square_button: Background Frame               |                             |                                       |
| :white_square_button: Compute              | :white_square_button: Batch Sampling                 |                             |                                       |
| :white_check_mark:    Dynamic Stream       | :white_square_button: Cache On Load                  |                             |                                       |
| :white_check_mark:    Effect               | :white_check_mark:    Color Settings                 |                             |                                       |
| :white_square_button: File Import Manager  | :white_square_button: Color                          |                             |                                       |
| :white_check_mark:    Footage              | :white_square_button: Color16                        |                             |                                       |
| :white_square_button: Hash                 | :white_square_button: ColorFloat                     |                             |                                       |
| :white_check_mark:    IO In                | :white_square_button: ColorParam                     |                             |                                       |
| :white_square_button: IO Out               | :white_check_mark:    Effect Custom UI               |                             |                                       |
| :white_check_mark:    Item                 | :white_check_mark:    Effect Custom UI Overlay Theme |                             |                                       |
| :white_square_button: Item View            | :white_check_mark:    Effect Sequence Data           |                             |                                       |
| :white_square_button: Iterate              | :white_check_mark:    Effect UI                      |                             |                                       |
| :white_check_mark:    Keyframe             | :white_square_button: Fill Matte                     |                             |                                       |
| :white_square_button: Layer Mask           | :white_check_mark:    GPU Device                     |                             |                                       |
| :white_square_button: Layer Render Options | :white_square_button: Handle                         |                             |                                       |
| :white_check_mark:    Layer                | :white_check_mark:    Iterate8                       |                             |                                       |
| :white_check_mark:    Light                | :white_check_mark:    iterate16                      |                             |                                       |
| :white_square_button: Marker               | :white_check_mark:    iterateFloat                   |                             |                                       |
| :white_check_mark:    Mask Outline         | :white_check_mark:    Param Utils                    |                             |                                       |
| :white_check_mark:    Mask                 | :white_square_button: Path Data                      |                             |                                       |
| :white_square_button: Math                 | :white_square_button: Path Query                     |                             |                                       |
| :white_check_mark:    Memory               | :white_square_button: Pixel Data                     |                             |                                       |
| :white_square_button: Output Module        | :white_check_mark:    Pixel Format                   |                             |                                       |
| :white_square_button: Persistent Data      | :white_square_button: PointParam                     |                             |                                       |
| :white_check_mark:    PF Interface         | :white_square_button: Sampling8                      |                             |                                       |
| :white_square_button: Proj                 | :white_square_button: Sampling16                     |                             |                                       |
| :white_square_button: QueryXform           | :white_square_button: SamplingFloat                  |                             |                                       |
| :white_square_button: Register             | :white_square_button: Source Settings                |                             |                                       |
| :white_square_button: Render Asyc Manager  | :white_square_button: Transition                     |                             |                                       |
| :white_square_button: Render Options       | :white_check_mark:    Utility                        |                             |                                       |
| :white_square_button: Render Queue Item    | :white_check_mark:    World                          |                             |                                       |
| :white_square_button: Render Queue         | :white_check_mark:    World Transform                |                             |                                       |
| :white_square_button: Render               |                                                      |                             |                                       |
| :white_square_button: RenderQueue Monitor  |                                                      |                             |                                       |
| :white_square_button: Sound Data           |                                                      |                             |                                       |
| :white_check_mark:    Stream               |                                                      |                             |                                       |
| :white_square_button: Text Document        |                                                      |                             |                                       |
| :white_square_button: Text Layer           |                                                      |                             |                                       |
| :white_square_button: Tracker              |                                                      |                             |                                       |
| :white_square_button: Tracker Utility      |                                                      |                             |                                       |
| :white_check_mark:    Utility              |                                                      |                             |                                       |
| :white_square_button: Workspace Panel      |                                                      |                             |                                       |
| :white_check_mark:    World                |                                                      |                             |                                       |

## Premiere:
| Premiere                                     | MediaCore                                           | PF                                     | ADOBESDK                                                    | Other                                       |
|----------------------------------------------|-----------------------------------------------------|----------------------------------------|-------------------------------------------------------------|---------------------------------------------|
| :white_square_button: Audio                  | :white_square_button: Accelerated Render Invocation | :white_square_button: Background Frame | :white_square_button: ADOBESDK ControlSurface               | :white_square_button: Captioning            |
| :white_square_button: Clip Render            | :white_square_button: App Info                      | :white_square_button: Cache On Load    | :white_square_button: ADOBESDK ControlSurface Command       | :white_square_button: Effect Stream Label   |
| :white_square_button: Deferred Processing    | :white_square_button: Application Settings          | :white_square_button: Pixel Format     | :white_square_button: ADOBESDK ControlSurface Lumetri       | :white_square_button: FlashCueMarkerData    |
| :white_square_button: Error                  | :white_square_button: Async File Reader             | :white_square_button: Source Settings  | :white_square_button: ADOBESDK ControlSurface Marker        | :white_square_button: Importer File Manager |
| :white_square_button: Export Audio Param     | :white_square_button: Async Operation               | :white_square_button: Transition       | :white_square_button: ADOBESDK ControlSurface Mixer         | :white_square_button: Marker                |
| :white_square_button: Export Standard Param  | :white_square_button: Export File                   | :white_square_button: Utility          | :white_square_button: ADOBESDK ControlSurface Transport     | :white_check_mark:    Opaque Effect Data    |
| :white_square_button: ExportController       | :white_square_button: Export Info                   |                                        | :white_square_button: ADOBESDK ControlSurfaceHost           |                                             |
| :white_square_button: File Registration      | :white_square_button: Export Param                  |                                        | :white_square_button: ADOBESDK ControlSurfaceHost Command   |                                             |
| :white_square_button: Image Processing       | :white_square_button: Export Progress               |                                        | :white_square_button: ADOBESDK ControlSurfaceHost Lumetri   |                                             |
| :white_square_button: Legacy                 | :white_square_button: Exporter Utility              |                                        | :white_square_button: ADOBESDK ControlSurfaceHost Marker    |                                             |
| :white_square_button: Media Accelerator      | :white_check_mark:    GPU Device                    |                                        | :white_square_button: ADOBESDK ControlSurfaceHost Mixer     |                                             |
| :white_check_mark:    Memory Manager         | :white_check_mark:    GPU Image Processing          |                                        | :white_square_button: ADOBESDK ControlSurfaceHost Transport |                                             |
| :white_square_button: Palette                | :white_square_button: Playmod Immersive Video       |                                        | :white_square_button: ADOBESDK ControlSurfacePlugin         |                                             |
| :white_square_button: Pixel Format           | :white_square_button: Playmod Overlay               |                                        | :white_square_button: ADOBESDK String                       |                                             |
| :white_square_button: Playmod Audio          | :white_square_button: Sequence Audio                |                                        |                                                             |                                             |
| :white_square_button: Playmod Device Control | :white_check_mark:    Sequence Info                 |                                        |                                                             |                                             |
| :white_check_mark:    PPix                   | :white_square_button: Sequence Render               |                                        |                                                             |                                             |
| :white_check_mark:    PPix 2                 | :white_square_button: Smart Rendering               |                                        |                                                             |                                             |
| :white_square_button: PPix Cache             | :white_square_button: String                        |                                        |                                                             |                                             |
| :white_square_button: PPix Creator           | :white_square_button: Transmit Invocation           |                                        |                                                             |                                             |
| :white_square_button: PPix Creator 2         | :white_check_mark:    Video Segment                 |                                        |                                                             |                                             |
| :white_square_button: RollCrawl              | :white_square_button: Video Segment Render          |                                        |                                                             |                                             |
| :white_square_button: Threaded Work          |                                                     |                                        |                                                             |                                             |
| :white_check_mark:    Time                   |                                                     |                                        |                                                             |                                             |
| :white_square_button: Window                 |                                                     |                                        |                                                             |                                             |

## License

Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib at your option.
