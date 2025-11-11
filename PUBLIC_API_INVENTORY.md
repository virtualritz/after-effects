# COMPREHENSIVE PUBLIC API INVENTORY - after-effects crate

## TOP-LEVEL EXPORTS (lib.rs)

### Public Modules
- `pub mod aegp` - AEGP (Application Entry Point) suite APIs
- `pub mod aeio` - Audio/IO related types
- `pub mod drawbot` - DrawBot graphics APIs  
- `pub mod pf` - Pixel Format effect plugin APIs (re-exported with pub use)
- `pub mod pr` - Premiere Pro APIs
- `pub mod pr_string` - Premiere Pro string utilities

### Public Trait Implementations
- `pub trait AsPtr<T>` - Convert types to raw pointers
- `pub trait AsMutPtr<T>` - Convert types to mutable raw pointers
- `pub trait Suite` (internal) - Suite acquisition trait

### Public Structures
- `pub struct PicaBasicSuite` - Thread-local suite management
  * Methods: from_pr_in_data_raw(), from_pr_in_data(), from_pf_in_data_raw(), from_sp_basic_suite_raw()
  
- `pub struct PicaBasicSuiteHandle` - Raw suite handle wrapper
  * Methods: from_raw(), as_ptr()
  
- `pub struct Matrix3([[f64; 3]; 3])` - 3x3 matrix type
  
- `pub struct Matrix4([[f64; 4]; 4])` - 4x4 matrix type
  * Methods: as_slice(), From<[f64; 16]>, ultraviolet/nalgebra conversions
  
- `pub struct Time` - Represents time with scale/value
  * Fields: value (i32), scale (u32)
  * Implements: From<f64>, From<f32>, Add trait
  
- `pub struct Rect` - Integer rectangle (left, top, right, bottom)
  * Methods: empty(), is_empty(), width(), height(), origin(), set_width(), 
            set_height(), set_origin(), union(), is_edge_pixel(), contains()
  
- `pub struct FloatPoint` - Floating point coordinates (x, y)
  
- `pub struct FloatRect` - Floating point rectangle
  * Methods: contains()
  
- `pub struct Ratio` - Rational number (num, den)
  * Implements: From<f64>, From<f32>

### Public Enums
- `pub enum Error` - Error types with conversions to &'static str and Display
  * Variants: Generic, Struct, Parameter, OutOfMemory, WrongThread, 
            ConstProjectModification, MissingSuite, InternalStructDamaged,
            InvalidIndex, UnrecogizedParameterType, InvalidCallback, 
            BadCallbackParameter, InterruptCancel, CannotParseKeyframeText,
            StringNotFound, StringBufferTooSmall, InvalidParms, Unknown10007, None
  
- `pub enum Ownership<'a, T: Clone>` - Memory ownership wrapper
  * Variants: AfterEffects(&'a T), AfterEffectsMut(&'a mut T), Rust(T)
  * Implements: Deref, DerefMut
  
- `pub enum ReadOnlyOwnership<'a, T: Clone>` - Read-only wrapper
  * Variants: AfterEffects(&'a T), Rust(T)
  * Implements: Deref
  
- `pub enum PointerOwnership<T>` - Pointer-based ownership
  * Variants: AfterEffects(*mut T), Rust(T)
  * Implements: Deref, DerefMut

### Public Type Aliases
- `pub type Color = ae_sys::A_Color` - Color type
- `pub use after_effects_sys as sys` - Raw FFI bindings
- Various re-exports: log, cstr_literal, fastrand, parking_lot, paste, serde

### Public Constants (Thread-Local)
- `pub(crate) static PICA_BASIC_SUITE` - Global suite reference

---

## PF MODULE - Pixel Format Effects Plugin API

### Core Structures
- `pub struct InData` - Input data for PF effects
  * Methods: from_raw(), as_ptr(), application_id(), is_premiere(), is_after_effects(),
            quality(), field(), extent_hint(), effect(), effect_ref(), pica_basic_suite_ptr(),
            width(), height(), current_frame(), current_frame_local(), current_timestamp(),
            current_time(), time_step(), local_time_step(), time_scale(),
            pre_effect_source_origin(), output_origin(), pixel_aspect_ratio(),
            downsample_x(), downsample_y(), version(), 
            frame_data_mut<T>(), frame_data<T>(), destroy_frame_data<T>(),
            interact(), utils()
  
- `pub struct OutData` - Output data for effects
  
- `pub struct Layer` - Layer representation
  * Methods: from_aegp_world(), from_owned(), from_raw(), width(), height(),
            buffer_stride(), row_bytes(), extent_hint(), pix_aspect_ratio(),
            origin(), buffer(), buffer_mut(), copy_from(), utils(),
            fill(), fill16(), iterate_with(), iterate(),
            row_padding_bytes(), as_pixel8_mut(), as_pixel8(),
            as_pixel16_mut(), as_pixel16(), as_pixel32_mut(), as_pixel32(),
            world_type(), bit_depth(), pixel_format(), pr_pixel_format()
  
- `pub struct Handle<'a, T>` - Safe memory handle wrapper
  * Methods: as_ref(), as_ref_mut(), from_raw(), new(), set(), lock()
  
- `pub struct HandleLock<'a, T>` - Locked handle access
  * Methods: as_ref(), as_ref_mut()
  
- `pub struct BorrowedHandleLock<T>` - Borrowed locked handle
  * Methods: from_raw()
  * Implements: Deref, DerefMut, Drop
  
- `pub struct FlatHandle<'a>` - Flat byte buffer handle
  * Methods: new(), resize(), lock(), as_slice(), as_slice_mut(), 
            as_ptr(), as_ptr_mut(), to_vec(), size(),
            from_raw(), from_raw_owned(), into_owned(), into_raw(), as_raw()
  
- `pub struct FlatHandleLock<'a, 'b>` - Locked flat handle

- `pub struct Parameters<'p, P: Eq + PartialEq + Hash + Copy + Debug>` - Parameter collection
  * Methods: add(), checkout<T>()
  
- `pub struct ParamDef<'p>` - Parameter definition
  
- `pub struct ParamMapInfo` - Parameter mapping info

- `pub struct EventCallbacks<'a>` - Event handling callbacks

- `pub struct UtilCallbacks` - Utility callbacks
  * Methods: (many color/pixel conversion and utility functions)
  
- `pub struct HLSPixel` - HLS color space pixel
  
- `pub struct YIQPixel` - YIQ color space pixel
  
- `pub struct ColorCallbacks` - Color manipulation callbacks
  
- `pub struct RawHandle` - Opaque handle wrapper
  
- `pub struct RawHandleLock<'a>` - Locked raw handle
  
- `pub struct Sampling` - Sampling configuration
  
- `pub struct Fixed(ae_sys::PF_Fixed)` - Fixed-point number
  
- `pub struct CompositeMode` - Composite blending mode
  
- `pub struct MaskWorld` - Mask world representation
  
- `pub struct GpuDeviceSetupExtra` - GPU device setup
  
- `pub struct GpuDeviceSetdownExtra` - GPU device teardown
  
- `pub struct PF_RenderRequest` - Render request
  
- `pub struct PF_PreRenderInput` - Pre-render input
  
- `pub struct PF_PreRenderOutput` - Pre-render output
  
- `pub struct PreRenderExtra` - Pre-render extra data
  
- `pub struct SmartRenderExtra` - Smart render extra data
  
- `pub struct PreRenderCallbacks` - Pre-render callbacks
  
- `pub struct SmartRenderCallbacks` - Smart render callbacks
  
- `pub struct InteractCallbacks(InData)` - Interaction callbacks
  
- `pub struct Effect` - Effect reference wrapper

### Enums & Variants
- `pub enum Command` - Plugin command selectors
  * Variants: About, GlobalSetup, GlobalSetdown, ParamsSetup, SequenceSetup,
             SequenceResetup, SequenceFlatten, SequenceSetdown, DoDialog,
             FrameSetup, Render, FrameSetdown, UserChangedParam,
             UpdateParamsUi, Event, ...
  
- `pub enum Param<'p>` - Parameter value variants
  
- `pub enum Event` - UI event types
  * Variants: None, NewContext, Activate, Click, Drag, Draw, Deactivate,
             CloseContext, Idle, AdjustCursor, Keydown, MouseExited
  
- `pub enum GenericPixel<'a>` - Generic pixel format
  
- `pub enum GenericPixelMut<'a>` - Mutable generic pixel

- `pub enum TransferMode` - Pixel transfer/blend modes
  * Many variants (Copy, Behind, In, Out, etc.)

### Type Aliases
- `pub type Pixel8 = ae_sys::PF_Pixel`
- `pub type Pixel16 = ae_sys::PF_Pixel16`
- `pub type PixelF32 = ae_sys::PF_Pixel32`
- `pub type PixelF64 = ae_sys::AEGP_ColorVal`
- `pub type ProgPtr = ae_sys::PF_ProgPtr`
- `pub type XferMode = TransferMode`

### Constants
- `pub const ONCE_PER_PROCESSOR: i32`
- `pub const PARAM_INDEX_NONE: i32`
- `pub const PARAM_INDEX_CHECK_ALL: i32`
- `pub const PARAM_INDEX_CHECK_ALL_EXCEPT_LAYER_PARAMS: i32`
- `pub const PARAM_INDEX_CHECK_ALL_HONOR_EXCLUDE: i32`
- `pub const MAX_CHANNEL8`, `HALF_CHANNEL8`, `MAX_CHANNEL16`, `HALF_CHANNEL16`

### Public Functions
- `pub fn pixel8_to_16(p: Pixel8) -> Pixel16`
- `pub fn pixel16_to_8(p: Pixel16) -> Pixel8`

### PF Suites (Available via pub use)
- `AdvItem` - Advanced item suite
- `BackgroundFrame` - Background frame suite
- `CacheOnLoad` - Cache on load suite
- `Channel` - Channel suite
- `ColorCallbacks`, `ColorCallbacks16`, `ColorCallbacksFloat` - Color callback suites
- `EffectSequenceData` - Effect sequence data suite
- `EffectUI` - Effect UI suite
- `App`, `AdvApp` - Application suites
- `EffectCustomUI`, `EffectCustomUIOverlayTheme` - Custom UI suites
- `Iterate8`, `Iterate16`, `IterateFloat` - Iteration suites
- `PixelData` - Pixel data suite
- `PixelFormat` - Pixel format suite
- `SourceSettings` - Source settings suite
- `Transition` - Transition suite
- `Utility` - Utility suite
- `World` - World suite
- `WorldTransform` - World transform suite
- `Handle` - Handle suite
- `Helper`, `Helper2` - Helper suites
- `ParamUtils`, `AngleParam`, `ColorParam`, `PointParam` - Parameter utilities
- `GPUDevice` - GPU device suite
- `FillMatte` - Fill matte suite
- `PathQuery`, `PathData` - Path data suites

---

## AEGP MODULE - Application Entry Point API

### Core Type Aliases
- `pub type PluginId = ae_sys::AEGP_PluginID`
- `pub type ItemId = i32`
- `pub type LayerId = u32`

### Re-exported Types from Suites
- `MenuId`, `MenuOrder` - Menu identifiers
- `HookPriority`, `CommandHookStatus` - Hook registration
- `ProjectHandle`, `ProjectBitDepth` - Project management
- `Camera`, `CameraType`, `FilmSizeUnits` - Camera properties
- `Canvas`, `BinType`, `DisplayChannel`, `RenderHints`, `RenderLayerContextHandle`,
  `RenderNumEffects`, `RenderReceiptHandle`, `RenderReceiptStatus` - Canvas/render
- `ColorProfileHandle`, `ConstColorProfileHandle`, `ItemViewHandle` - Color management
- `Composition`, `Collection2Handle`, `CompFlags`, `CompHandle` - Composition
- `Effect`, `EffectFlags`, `EffectRefHandle`, `InstalledEffectKey` - Effects
- `Footage`, `FootageHandle`, `FootageSignature`, `InterpretationStyle`, `Platform` - Footage
- `InputSpecification` - Input specification
- `Item`, `ItemFlags`, `ItemHandle`, `ItemType`, `LabelId` - Items
- `Keyframe` types
- `Layer` and layer-related types
- `Light` types
- `Mask` types
- `MemHandle<T>`, `MemHandleLock<T>` - Memory management
- `PersistentData` types
- `TimeDisplayConfig` - Time display
- `StreamValue` - Stream values
- `AsyncManager` - Async rendering
- `Various render option types`
- `SoundDataHandle` - Sound data
- `GetPathTypes` - Path utilities

### Structures
- `pub struct Scene3D` - 3D scene (artisan-2-api feature)
- `pub struct Scene3DLayerHandle` - 3D layer handle
- `pub struct Scene3DTextureCacheHandle` - 3D texture cache

### AEGP Suites
- `Camera` - Camera suite
- `Canvas` - Canvas/rendering suite
- `ColorSettings` - Color settings suite
- `Command` - Command suite
- `Comp` - Composition suite
- `Composite` - Compositing suite
- `Effect` - Effect suite
- `Footage` - Footage suite
- `IOIn` - Input suite
- `Item` - Item suite
- `Keyframe` - Keyframe suite
- `Layer` - Layer suite
- `LayerRenderOptions` - Layer render options
- `Light` - Light suite
- `Mask`, `MaskOutline` - Mask suites
- `Memory` - Memory suite
- `PersistentData` - Persistent data suite
- `PFInterface` - PF interface suite
- `Project` - Project suite
- `Register`, `RegisterNonAegp` - Registration suites
- `Render` - Render suite
- `RenderAsyncManager` - Async render manager
- `RenderOptions` - Render options
- `Scene3D` - 3D scene suite (artisan-2-api)
- `SoundData` - Sound data suite
- `Stream`, `DynamicStream` - Stream suites
- `Utility` - Utility suite
- `World` - World suite

---

## DRAWBOT MODULE - Graphics Drawing API

### Core Structures
- `pub struct Drawbot` - Main drawing context
  * Methods: supplier(), surface(), fill_theme_path(), fill_theme_vertex()
  
- `pub struct Pen` - Drawing pen
  * Methods: set_dash_pattern()
  
- `pub struct Brush` - Drawing brush
  
- `pub struct Font` - Text font
  
- `pub struct Image` - Image drawing
  * Methods: set_scale_factor()
  
- `pub struct Path` - Vector path
  * Methods: move_to(), line_to(), bezier_to(), add_rect(), add_arc(),
            add_rounded_rect(), close()

### Re-exported Types from Suites
- `PixelLayout` - Pixel layout enum
- `Supplier` - Supplier interface
- `FillType` - Fill type enum
- `TextAlignment` - Text alignment enum
- `TextTruncation` - Text truncation enum
- `InterpolationPolicy` - Interpolation policy
- `AntiAliasPolicy` - Anti-alias policy
- `Surface` - Drawing surface

### Type Aliases
- `pub type PointF32 = ae_sys::DRAWBOT_PointF32`
- `pub type ColorRgba = ae_sys::DRAWBOT_ColorRGBA`
- `pub type RectF32 = ae_sys::DRAWBOT_RectF32`
- `pub type MatrixF32 = ae_sys::DRAWBOT_MatrixF32`
- `pub type Rect32 = ae_sys::DRAWBOT_Rect32`

### Drawbot Suites
- `Supplier` - Resource supplier
- `Surface` - Drawing surface

---

## PR MODULE - Premiere Pro API

### Core Structures
- `pub struct InDataHandle` - Premiere input data handle
  * Methods: from_raw(), as_ptr(), pica_basic_handle(), plugin_id(),
            reference_context_ptr()
  
- `pub struct RenderContextHandle` - Render context handle

### Other Handle Types (via define_handle_wrapper!)
- `InstanceDataHandle`
- `InstanceContextHandle`
- `GlobalContextHandle`
- `GlobalDataHandle`
- `RenderDataHandle`

### Enums
- `pub enum PixelFormat` - Premiere pixel formats (extensive list of variants)
  * Variants: Bgra4444_8u, Vuya4444_8u, Argb4444_8u, ... (50+ pixel format variants)

---

## PR_STRING MODULE - Premiere Pro String Utilities

### Structures
- `pub struct PrString` - Premiere string wrapper
  * Implements: From<&str>, Into<String>, Drop

### Suites
- `PrStringSuite` - String handling suite
  * Methods: new(), dispose_string(), allocate_from_utf8(), copy_to_utf8_string()

---

## AEIO MODULE - Audio/IO Types

### Enums
- `pub enum FileType` - File type indicators
  * Variants: Dir, None, Any, Generic
  
- `pub enum SoundEncoding` - Audio encoding
  * Variants: UnsignedPcm, SignedPcm, SignedFloat
  
- `pub enum SoundSampleSize` - Audio sample size
  * Variants: Size1, Size2, Size4
  
- `pub enum SoundChannels` - Audio channels
  * Variants: Mono, Stereo

### Handle Types
- `InSpecHandle` - Input specification handle
- `Handle` - Generic AEIO handle

---

## MACROS

### Main Macros (exported)
- `#[macro_export] macro_rules! define_effect` - Main plugin entry point definition
- Various internal macros for suite management and FFI calls:
  * `ae_acquire_suite_ptr!` - Acquire suite from After Effects
  * `ae_release_suite_ptr!` - Release acquired suite
  * `ae_get_suite_fn!` - Get function from suite
  * `call_suite_fn!` - Call suite function with error handling
  * `call_suite_fn_single!` - Call function returning single value
  * `call_suite_fn_double!` - Call function returning two values
  * `call_suite_fn_no_err!` - Call function returning value directly
  * And several others for handle creation, enum definition, etc.

---

## PLUGIN BASE TRAITS

### Traits (from plugin_base.rs)
- `pub trait AegpPlugin: Sized + AegpSeal` - AEGP plugin trait
  * Associated function: new() -> Result<Self, Error>
  
- `AegpSeal` - Sealing trait for AEGP plugins

---

## BITFLAGS

### EventInFlags
- NONE, DONT_DRAW

### EventOutFlags  
- NONE, HANDLED_EVENT, ALWAYS_UPDATE, NEVER_UPDATE, UPDATE_NOW

### ParamUIFlags
- NONE, TOPIC, CONTROL, CONTROL_ONLY, NO_ECW_UI, ECW_SEPARATOR, DISABLED,
  DO_NOT_ERASE_TOPIC, DO_NOT_ERASE_CONTROL, RADIO_BUTTON, INVISIBLE

### ParamFlag
- CANNOT_TIME_VARY, CANNOT_INTERP, TWIRLY, SUPERVISE, START_COLLAPSED,
  USE_VALUE_FOR_OLD_PROJECTS, LAYER_PARAM_IS_TRACKMATTE,
  EXCLUDE_FROM_HAVE_INPUTS_CHANGED, SKIP_REVEAL_WHEN_UNHIDDEN

### ChangeFlag
- NONE, CHANGED_VALUE, SET_TO_VARY, SET_TO_CONSTANT

### ValueDisplayFlag
- NONE, PERCENT, PIXEL, REVERSE

### FSliderFlag
- NONE, WANT_PHASE

---

## NAMING CONVENTION ANALYSIS

The crate generally follows Rust naming conventions:
- Structs: PascalCase ✓
- Enums: PascalCase ✓
- Functions: snake_case ✓
- Constants: SCREAMING_SNAKE_CASE ✓
- Type parameters: Single uppercase letters or PascalCase ✓
- Traits: PascalCase ✓
- Modules: snake_case ✓

Some notable patterns:
- Type aliases for Adobe types (pub type Color, pub type Matrix4, etc.)
- Wrapper types for C structures with safe APIs
- Comprehensive use of bitflags for options
- Extensive use of generics and lifetimes for memory safety

