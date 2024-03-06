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
| AEGP                                   | PF                                               | DRAWBOT                    | Other                                |
|----------------------------------------|--------------------------------------------------|----------------------------|--------------------------------------|
| <small>🔳 Artisan Util         </small> | <small>✅ AE Adv App                     </small> | <small>✅ Draw     </small> | <small>✅ AE Plugin Helper   </small> |
| <small>✅ Camera               </small> | <small>🔳 AE Adv Item                    </small> | <small>✅ Image    </small> | <small>✅ AE Plugin Helper 2 </small> |
| <small>✅ Canvas               </small> | <small>🔳 AE Adv Time                    </small> | <small>✅ Path     </small> |                                      |
| <small>🔳 Collection           </small> | <small>✅ AE App                         </small> | <small>✅ Pen      </small> |                                      |
| <small>🔳 Command              </small> | <small>🔳 AngleParam                     </small> | <small>✅ Supplier </small> |                                      |
| <small>✅ Comp                 </small> | <small>🔳 ANSI                           </small> | <small>✅ Surface  </small> |                                      |
| <small>✅ Composite            </small> | <small>🔳 Background Frame               </small> |                            |                                      |
| <small>🔳 Compute              </small> | <small>🔳 Batch Sampling                 </small> |                            |                                      |
| <small>✅ Dynamic Stream       </small> | <small>🔳 Cache On Load                  </small> |                            |                                      |
| <small>✅ Effect               </small> | <small>✅ Color Settings                 </small> |                            |                                      |
| <small>🔳 File Import Manager  </small> | <small>🔳 Color                          </small> |                            |                                      |
| <small>✅ Footage              </small> | <small>🔳 Color16                        </small> |                            |                                      |
| <small>🔳 Hash                 </small> | <small>🔳 ColorFloat                     </small> |                            |                                      |
| <small>✅ IO In                </small> | <small>🔳 ColorParam                     </small> |                            |                                      |
| <small>🔳 IO Out               </small> | <small>✅ Effect Custom UI               </small> |                            |                                      |
| <small>✅ Item                 </small> | <small>✅ Effect Custom UI Overlay Theme </small> |                            |                                      |
| <small>🔳 Item View            </small> | <small>✅ Effect Sequence Data           </small> |                            |                                      |
| <small>🔳 Iterate              </small> | <small>✅ Effect UI                      </small> |                            |                                      |
| <small>✅ Keyframe             </small> | <small>🔳 Fill Matte                     </small> |                            |                                      |
| <small>🔳 Layer Mask           </small> | <small>✅ GPU Device                     </small> |                            |                                      |
| <small>🔳 Layer Render Options </small> | <small>🔳 Handle                         </small> |                            |                                      |
| <small>✅ Layer                </small> | <small>✅ Iterate8                       </small> |                            |                                      |
| <small>✅ Light                </small> | <small>✅ iterate16                      </small> |                            |                                      |
| <small>🔳 Marker               </small> | <small>✅ iterateFloat                   </small> |                            |                                      |
| <small>✅ Mask Outline         </small> | <small>✅ Param Utils                    </small> |                            |                                      |
| <small>✅ Mask                 </small> | <small>🔳 Path Data                      </small> |                            |                                      |
| <small>🔳 Math                 </small> | <small>🔳 Path Query                     </small> |                            |                                      |
| <small>✅ Memory               </small> | <small>🔳 Pixel Data                     </small> |                            |                                      |
| <small>🔳 Output Module        </small> | <small>✅ Pixel Format                   </small> |                            |                                      |
| <small>🔳 Persistent Data      </small> | <small>🔳 PointParam                     </small> |                            |                                      |
| <small>✅ PF Interface         </small> | <small>🔳 Sampling8                      </small> |                            |                                      |
| <small>🔳 Proj                 </small> | <small>🔳 Sampling16                     </small> |                            |                                      |
| <small>🔳 QueryXform           </small> | <small>🔳 SamplingFloat                  </small> |                            |                                      |
| <small>🔳 Register             </small> | <small>🔳 Source Settings                </small> |                            |                                      |
| <small>🔳 Render Asyc Manager  </small> | <small>🔳 Transition                     </small> |                            |                                      |
| <small>🔳 Render Options       </small> | <small>✅ Utility                        </small> |                            |                                      |
| <small>🔳 Render Queue Item    </small> | <small>✅ World                          </small> |                            |                                      |
| <small>🔳 Render Queue         </small> | <small>✅ World Transform                </small> |                            |                                      |
| <small>🔳 Render               </small> |                                                  |                            |                                      |
| <small>🔳 RenderQueue Monitor  </small> |                                                  |                            |                                      |
| <small>🔳 Sound Data           </small> |                                                  |                            |                                      |
| <small>✅ Stream               </small> |                                                  |                            |                                      |
| <small>🔳 Text Document        </small> |                                                  |                            |                                      |
| <small>🔳 Text Layer           </small> |                                                  |                            |                                      |
| <small>🔳 Tracker              </small> |                                                  |                            |                                      |
| <small>🔳 Tracker Utility      </small> |                                                  |                            |                                      |
| <small>✅ Utility              </small> |                                                  |                            |                                      |
| <small>🔳 Workspace Panel      </small> |                                                  |                            |                                      |
| <small>✅ World                </small> |                                                  |                            |                                      |

## Premiere:
| Premiere                                 | MediaCore                                      | Control Surface                                | Other                                   |
|------------------------------------------|------------------------------------------------|------------------------------------------------|-----------------------------------------|
| <small>🔳 Audio                  </small> | <small>🔳 Accelerated Render Invocation</small> | <small>🔳 ControlSurface               </small> | <small>🔳 PF Background Frame   </small> |
| <small>🔳 Clip Render            </small> | <small>🔳 App Info                     </small> | <small>🔳 ControlSurface Command       </small> | <small>🔳 PF Cache On Load      </small> |
| <small>🔳 Deferred Processing    </small> | <small>🔳 Application Settings         </small> | <small>🔳 ControlSurface Lumetri       </small> | <small>🔳 PF Pixel Format       </small> |
| <small>🔳 Error                  </small> | <small>🔳 Async File Reader            </small> | <small>🔳 ControlSurface Marker        </small> | <small>🔳 PF Source Settings    </small> |
| <small>🔳 Export Audio Param     </small> | <small>🔳 Async Operation              </small> | <small>🔳 ControlSurface Mixer         </small> | <small>🔳 PF Transition         </small> |
| <small>🔳 Export Standard Param  </small> | <small>🔳 Export File                  </small> | <small>🔳 ControlSurface Transport     </small> | <small>🔳 PF Utility            </small> |
| <small>🔳 ExportController       </small> | <small>🔳 Export Info                  </small> | <small>🔳 ControlSurfaceHost           </small> | <small>✅ Opaque Effect Data    </small> |
| <small>🔳 File Registration      </small> | <small>🔳 Export Param                 </small> | <small>🔳 ControlSurfaceHost Command   </small> | <small>🔳 Captioning            </small> |
| <small>🔳 Image Processing       </small> | <small>🔳 Export Progress              </small> | <small>🔳 ControlSurfaceHost Lumetri   </small> | <small>🔳 Effect Stream Label   </small> |
| <small>🔳 Legacy                 </small> | <small>🔳 Exporter Utility             </small> | <small>🔳 ControlSurfaceHost Marker    </small> | <small>🔳 FlashCueMarkerData    </small> |
| <small>🔳 Media Accelerator      </small> | <small>✅ GPU Device                   </small> | <small>🔳 ControlSurfaceHost Mixer     </small> | <small>🔳 Importer File Manager </small> |
| <small>✅ Memory Manager         </small> | <small>✅ GPU Image Processing         </small> | <small>🔳 ControlSurfaceHost Transport </small> | <small>🔳 Marker                </small> |
| <small>🔳 Palette                </small> | <small>🔳 Playmod Immersive Video      </small> | <small>🔳 ControlSurfacePlugin         </small> |                                         |
| <small>🔳 Pixel Format           </small> | <small>🔳 Playmod Overlay              </small> | <small>🔳 String                       </small> |                                         |
| <small>🔳 Playmod Audio          </small> | <small>🔳 Sequence Audio               </small> |                                                |                                         |
| <small>🔳 Playmod Device Control </small> | <small>✅ Sequence Info                </small> |                                                |                                         |
| <small>✅ PPix                   </small> | <small>🔳 Sequence Render              </small> |                                                |                                         |
| <small>✅ PPix 2                 </small> | <small>🔳 Smart Rendering              </small> |                                                |                                         |
| <small>🔳 PPix Cache             </small> | <small>🔳 String                       </small> |                                                |                                         |
| <small>🔳 PPix Creator           </small> | <small>🔳 Transmit Invocation          </small> |                                                |                                         |
| <small>🔳 PPix Creator 2         </small> | <small>✅ Video Segment                </small> |                                                |                                         |
| <small>🔳 RollCrawl              </small> | <small>🔳 Video Segment Render         </small> |                                                |                                         |
| <small>🔳 Threaded Work          </small> |                                                |                                                |                                         |
| <small>✅ Time                   </small> |                                                |                                                |                                         |
| <small>🔳 Window                 </small> |                                                |                                                |                                         |

## License

Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib at your option.
