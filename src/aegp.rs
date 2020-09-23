pub use crate::*;
pub use ae_sys::*;
use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use std::{
    convert::TryFrom, ffi::CString, marker::PhantomData, mem::MaybeUninit,
};
use widestring::U16CString;

pub type PluginID = ae_sys::AEGP_PluginID;

pub type MaterialBasic = ae_sys::AEGP_MaterialBasic_v1;

pub type ItemID = i32;

// FIXME: make this an enum
pub type CompFlags = u32;

pub const COMP_FLAG_SHOW_ALL_SHY: u32 = ae_sys::AEGP_CompFlag_SHOW_ALL_SHY;
pub const COMP_FLAG_RESERVED_1: u32 = ae_sys::AEGP_CompFlag_RESERVED_1;
pub const COMP_FLAG_RESERVED_2: u32 = ae_sys::AEGP_CompFlag_RESERVED_2;
pub const COMP_FLAG_ENABLE_MOTION_BLUR: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_MOTION_BLUR;
pub const COMP_FLAG_ENABLE_TIME_FILTER: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_TIME_FILTER;
pub const COMP_FLAG_GRID_TO_FRAMES: u32 = ae_sys::AEGP_CompFlag_GRID_TO_FRAMES;
pub const COMP_FLAG_GRID_TO_FIELDS: u32 = ae_sys::AEGP_CompFlag_GRID_TO_FIELDS;
pub const COMP_FLAG_USE_LOCAL_DSF: u32 = ae_sys::AEGP_CompFlag_USE_LOCAL_DSF;
pub const COMP_FLAG_DRAFT_3D: u32 = ae_sys::AEGP_CompFlag_DRAFT_3D;
pub const COMP_FLAG_SHOW_GRAPH: u32 = ae_sys::AEGP_CompFlag_SHOW_GRAPH;
pub const COMP_FLAG_RESERVED_3: u32 = ae_sys::AEGP_CompFlag_RESERVED_3;

#[repr(u32)]
pub enum MemFlag {
    None = ae_sys::AEGP_MemFlag_NONE,
    Clear = ae_sys::AEGP_MemFlag_CLEAR,
    Quiet = ae_sys::AEGP_MemFlag_QUIET,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum LayerStream {
    None = AEGP_LayerStream_NONE,
    AnchorPoint = AEGP_LayerStream_ANCHORPOINT,
    Position = AEGP_LayerStream_POSITION,
    Scale = AEGP_LayerStream_SCALE,
    // This is the layer's rotation for a 2D layer
    RotateZ = AEGP_LayerStream_ROTATION,
    Opcaity = AEGP_LayerStream_OPACITY,
    Audio = AEGP_LayerStream_AUDIO,
    Marker = AEGP_LayerStream_MARKER,
    TimeRemap = AEGP_LayerStream_TIME_REMAP,
    RotateX = AEGP_LayerStream_ROTATE_X,
    RotateY = AEGP_LayerStream_ROTATE_Y,
    Orientation = AEGP_LayerStream_ORIENTATION,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_CAMERA
    Zoom = AEGP_LayerStream_ZOOM,
    DepthOfField = AEGP_LayerStream_DEPTH_OF_FIELD,
    FocusDistance = AEGP_LayerStream_FOCUS_DISTANCE,
    Aperture = AEGP_LayerStream_APERTURE,
    BlurLevel = AEGP_LayerStream_BLUR_LEVEL,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_LIGHT
    Intensity = AEGP_LayerStream_INTENSITY,
    Color = AEGP_LayerStream_COLOR,
    ConeAngle = AEGP_LayerStream_CONE_ANGLE,
    ConeFeather = AEGP_LayerStream_CONE_FEATHER,
    ShadowDarkness = AEGP_LayerStream_SHADOW_DARKNESS,
    ShadowDiffusion = AEGP_LayerStream_SHADOW_DIFFUSION,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_AV
    AcceptsShadows = AEGP_LayerStream_ACCEPTS_SHADOWS,
    AcceptsLights = AEGP_LayerStream_ACCEPTS_LIGHTS,
    AmbientCoeff = AEGP_LayerStream_AMBIENT_COEFF,
    DiffuseCoeff = AEGP_LayerStream_DIFFUSE_COEFF,
    SpecularIntensity = AEGP_LayerStream_SPECULAR_INTENSITY,
    SpecularShininess = AEGP_LayerStream_SPECULAR_SHININESS,

    CastsShadows = AEGP_LayerStream_CASTS_SHADOWS, /* LIGHT and AV only, no CAMERA */
    LightTransmission = AEGP_LayerStream_LIGHT_TRANSMISSION, /* AV Layer only */
    Metal = AEGP_LayerStream_METAL,                // AV layer only

    SourceText = AEGP_LayerStream_SOURCE_TEXT,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_CAMERA
    IrisShape = AEGP_LayerStream_IRIS_SHAPE,
    IrisRotation = AEGP_LayerStream_IRIS_ROTATION,
    IrisRoundness = AEGP_LayerStream_IRIS_ROUNDNESS,
    IrisAspectRatio = AEGP_LayerStream_IRIS_ASPECT_RATIO,
    IrisDiffractionFringe = AEGP_LayerStream_IRIS_DIFFRACTION_FRINGE,
    IrisHighlightGain = AEGP_LayerStream_IRIS_HIGHLIGHT_GAIN,
    IrisHighlightThreshold = AEGP_LayerStream_IRIS_HIGHLIGHT_THRESHOLD,
    IrisHighlightSaturation = AEGP_LayerStream_IRIS_HIGHLIGHT_SATURATION,

    // only valid for AEGP_ObjectType == AEGP_ObjectTyp_LIGHT
    LightFalloffType = AEGP_LayerStream_LIGHT_FALLOFF_TYPE,
    LightFalloffStart = AEGP_LayerStream_LIGHT_FALLOFF_START,
    LightFalloffDistance = AEGP_LayerStream_LIGHT_FALLOFF_DISTANCE,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_AV
    ReflactionIntensity = AEGP_LayerStream_REFLECTION_INTENSITY,
    ReflactionSharpness = AEGP_LayerStream_REFLECTION_SHARPNESS,
    ReflactionRolloff = AEGP_LayerStream_REFLECTION_ROLLOFF,
    TransparencyCoeff = AEGP_LayerStream_TRANSPARENCY_COEFF,
    TransparencyRolloff = AEGP_LayerStream_TRANSPARENCY_ROLLOFF,
    IndexOfRefraction = AEGP_LayerStream_INDEX_OF_REFRACTION,

    BevelStyle = AEGP_LayerStream_EXTRUSION_BEVEL_STYLE,
    BevelDirection = AEGP_LayerStream_EXTRUSION_BEVEL_DIRECTION,
    BevelDepth = AEGP_LayerStream_EXTRUSION_BEVEL_DEPTH,
    ExtrusionHoleBeveDepth = AEGP_LayerStream_EXTRUSION_HOLE_BEVEL_DEPTH,
    ExtrusionDepth = AEGP_LayerStream_EXTRUSION_DEPTH,
    PlaneCurvature = AEGP_LayerStream_PLANE_CURVATURE,
    PlaneSubdivision = AEGP_LayerStream_PLANE_SUBDIVISION,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum LayerFlag {
    None = 0x0000_0000,
    VideoActive = 0x0000_0001,
    AudioActive = 0x0000_0002,
    EffectsActive = 0x0000_0004,
    MotionBlur = 0x0000_0008,
    FrameBlending = 0x0000_0010,
    Locked = 0x0000_0020,
    Shy = 0x0000_0040,
    Collapse = 0x0000_0080,
    AutoOrientRotation = 0x0000_0100,
    AdjustmentLayer = 0x0000_0200,
    TimeRemapping = 0x0000_0400,
    LayerIs3D = 0x0000_0800,
    LookAtCamera = 0x0000_1000,
    LookAtPoi = 0x0000_2000,
    Solo = 0x0000_4000,
    MarkersLocked = 0x0000_8000,
    NullLayer = 0x0001_0000,
    HideLockedMasks = 0x0002_0000,
    GuideLayer = 0x0004_0000,
    AdvancedFrameBlending = 0x0008_0000,
    SublayersRenderSeparately = 0x0010_0000,
    EnvironmentLayer = 0x0020_0000,
}

pub type LayerFlags = u32;

pub type LayerID = u32;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum WorldType {
    None = ae_sys::AEGP_WorldType_NONE,
    Byte = ae_sys::AEGP_WorldType_8,
    Integer = ae_sys::AEGP_WorldType_16,
    Float = ae_sys::AEGP_WorldType_32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct DownsampleFactor {
    pub xs: ae_sys::A_short,
    pub ys: ae_sys::A_short,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum TimeMode {
    LayerTime = ae_sys::AEGP_LTimeMode_LayerTime,
    CompTime = ae_sys::AEGP_LTimeMode_CompTime,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum StreamType {
    NoData = AEGP_StreamType_NO_DATA,
    ThreeDSpatial = AEGP_StreamType_ThreeD_SPATIAL,
    ThreeD = AEGP_StreamType_ThreeD,
    TwoDSpatial = AEGP_StreamType_TwoD_SPATIAL,
    TwoD = AEGP_StreamType_TwoD,
    OneD = AEGP_StreamType_OneD,
    Color = AEGP_StreamType_COLOR,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum StreamValue {
    None,
    FourD(A_FpLong, A_FpLong, A_FpLong, A_FpLong),
    ThreeD {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
        z: ae_sys::A_FpLong,
    },
    ThreeDSpatial {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
        z: ae_sys::A_FpLong,
    },
    TwoD {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
    },
    TwoDSpatial {
        x: ae_sys::A_FpLong,
        y: ae_sys::A_FpLong,
    },
    OneD(ae_sys::A_FpLong),
    Color {
        alpha: ae_sys::A_FpLong,
        red: ae_sys::A_FpLong,
        green: ae_sys::A_FpLong,
        blue: ae_sys::A_FpLong,
    },
    Arb,          // FIXME
    Marker,       // FIXME
    LayerID,      // FIXME
    MaskID,       // FIXME
    Mask,         // FIXME
    TextDocument, // FIXME
}

impl TryFrom<StreamValue> for f32 {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as f32),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for f64 {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for usize {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as usize),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for u32 {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v as u32),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for bool {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::OneD(v) => Ok(v != 0.0f64),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 2] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::TwoD { x, y } | StreamValue::TwoDSpatial { x, y } => {
                Ok([x as f32, y as f32])
            }
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 2] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::TwoD { x, y } | StreamValue::TwoDSpatial { x, y } => {
                Ok([x, y])
            }
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 3] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::ThreeD { x, y, z }
            | StreamValue::ThreeDSpatial { x, y, z } => {
                Ok([x as f32, y as f32, z as f32])
            }
            StreamValue::Color {
                alpha: _,
                red,
                green,
                blue,
            } => Ok([red as f32, green as f32, blue as f32]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 3] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::ThreeD { x, y, z }
            | StreamValue::ThreeDSpatial { x, y, z } => Ok([x, y, z]),
            StreamValue::Color {
                alpha: _,
                red,
                green,
                blue,
            } => Ok([red, green, blue]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f32; 4] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::FourD(a, b, c, d) => {
                Ok([a as f32, b as f32, c as f32, d as f32])
            }
            StreamValue::Color {
                alpha,
                red,
                green,
                blue,
            } => Ok([alpha as f32, red as f32, green as f32, blue as f32]),
            _ => Err(Error::Parameter),
        }
    }
}

impl TryFrom<StreamValue> for [f64; 4] {
    type Error = Error;
    fn try_from(value: StreamValue) -> Result<Self, Error> {
        match value {
            StreamValue::FourD(a, b, c, d) => Ok([a, b, c, d]),
            StreamValue::Color {
                alpha,
                red,
                green,
                blue,
            } => Ok([alpha, red, green, blue]),
            _ => Err(Error::Parameter),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum LightType {
    None = AEGP_LightType_NONE,
    Parallel = AEGP_LightType_PARALLEL,
    Spot = AEGP_LightType_SPOT,
    Point = AEGP_LightType_POINT,
    Ambient = AEGP_LightType_AMBIENT,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum ObjectType {
    None = AEGP_ObjectType_NONE,
    /// Includes all pre-AE 5.0 layer types (audio or video source,
    /// including adjustment layers).
    AudioVideo = AEGP_ObjectType_AV,
    Light = AEGP_ObjectType_LIGHT,
    Camera = AEGP_ObjectType_CAMERA,
    Text = AEGP_ObjectType_TEXT,
    Vector = AEGP_ObjectType_VECTOR,
    NumTypes = AEGP_ObjectType_NUM_TYPES,
}

#[allow(dead_code)]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive,
)]
#[repr(u32)]
pub enum FilmSizeUnits {
    None = ae_sys::AEGP_FilmSizeUnits_NONE,
    Horizontal = ae_sys::AEGP_FilmSizeUnits_HORIZONTAL,
    Vertical = ae_sys::AEGP_FilmSizeUnits_VERTICAL,
    Diagonal = ae_sys::AEGP_FilmSizeUnits_DIAGONAL,
}

#[allow(dead_code)]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum CameraType {
    None = ae_sys::AEGP_CameraType_NONE,
    Perspective = ae_sys::AEGP_CameraType_PERSPECTIVE,
    Orthographic = ae_sys::AEGP_CameraType_ORTHOGRAPHIC,
    NumTypes = ae_sys::AEGP_CameraType_NUM_TYPES,
}

define_suite!(
    MemorySuite,
    AEGP_MemorySuite1,
    kAEGPMemorySuite,
    kAEGPMemorySuiteVersion1
);

#[derive(Debug)]
pub struct MemHandle<'a, T: 'a> {
    suite_ptr: *const ae_sys::AEGP_MemorySuite1,
    handle: ae_sys::AEGP_MemHandle,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'a> MemHandle<'a, T> {
    pub fn new(
        plugin_id: PluginID,
        name: &str,
        value: T,
    ) -> Result<MemHandle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let mut handle: ae_sys::AEGP_MemHandle = std::ptr::null_mut();

                match ae_call_suite_fn!(
                    suite_ptr,
                    AEGP_NewMemHandle,
                    plugin_id,
                    CString::new(name).unwrap().as_ptr(),
                    std::mem::size_of::<T>() as u32,
                    0,
                    &mut handle,
                ) {
                    Ok(()) => {
                        let handle = Self {
                            suite_ptr,
                            handle,
                            _marker: PhantomData,
                        };

                        *handle.lock()?.get_mut()? = value;

                        Ok(handle)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn lock(&self) -> Result<MemHandleLock<T>, Error> {
        let mut ptr = std::mem::MaybeUninit::<*mut T>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_LockMemHandle,
            self.handle,
            ptr.as_mut_ptr() as *mut *mut _ as _
        ) {
            Ok(()) => Ok(MemHandleLock {
                parent_handle: self,
                ptr: unsafe { ptr.assume_init() },
            }),
            Err(e) => Err(e),
        }
    }

    /// Only call this if you know what you're doing.
    #[inline]
    pub(crate) fn unlock(&self) -> Result<(), Error> {
        ae_call_suite_fn!(self.suite_ptr, AEGP_UnlockMemHandle, self.handle)
    }

    pub fn from_raw(
        handle: ae_sys::AEGP_MemHandle,
    ) -> Result<MemHandle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1
        ) {
            Ok(suite_ptr) => Ok(Self {
                suite_ptr,
                handle,
                _marker: PhantomData,
            }),
            Err(e) => Err(e),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: Self) -> ae_sys::AEGP_MemHandle {
        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here.
        std::mem::forget(handle);
        // Make sure drop(Handle) does *not*
        // actually drop anything since we're
        // passing ownership.
        return_handle
    }

    /// Returns the raw handle.
    pub fn as_raw(&self) -> ae_sys::AEGP_MemHandle {
        self.handle
    }
}

impl<'a, T: 'a> Drop for MemHandle<'a, T> {
    fn drop(&mut self) {
        if let Ok(lock) = self.lock() {
            // Call destructors for data
            // owned by MemHandle
            unsafe { lock.ptr.read() };
        }

        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_FreeMemHandle,
            self.handle
        );
    }
}

pub struct MemHandleLock<'a, T> {
    parent_handle: &'a MemHandle<'a, T>,
    ptr: *mut T,
}

impl<'a, T> MemHandleLock<'a, T> {
    pub fn get(&self) -> Result<&'a T, Error> {
        if self.ptr.is_null() {
            Err(Error::Generic)
        } else {
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn get_mut(&self) -> Result<&'a mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::Generic)
        } else {
            Ok(unsafe { &mut *self.ptr })
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<'a, T> Drop for MemHandleLock<'a, T> {
    fn drop(&mut self) {
        self.parent_handle.unlock().unwrap();
    }
}

define_suite!(
    IOInSuite,
    AEGP_IOInSuite4,
    kAEGPIOInSuite,
    kAEGPIOInSuiteVersion4
);

impl IOInSuite {
    pub fn get_in_spec_options_handle(
        &self,
        in_spec_handle: aeio::InSpecHandle,
    ) -> Result<aeio::Handle, Error> {
        let mut in_spec_options_handle =
            std::mem::MaybeUninit::<ae_sys::AEIO_Handle>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetInSpecOptionsHandle,
            in_spec_handle.as_ptr(),
            in_spec_options_handle.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(aeio::Handle::from_raw(unsafe {
                in_spec_options_handle.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    /*
    pub fn set_in_spec_options_handle(&self, in_spec_handle: aeio::InSpecHandle) -> Result<aeio::Handle, Error> {
        let mut in_spec_options_handle = std::mem::MaybeUninit::<ae_sys::AEIO_Handle>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_SetInSpecOptionsHandle,
            in_spec_handle.as_ptr(),
            in_spec_options_handle.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(aeio::Handle {
                handle_ptr: unsafe { in_spec_options_handle.assume_init() },
            }),
            Err(e) => Err(e),
        }
    }*/
}

define_handle_wrapper!(EffectRefHandle, AEGP_EffectRefH);

define_suite!(
    EffectSuite,
    AEGP_EffectSuite4,
    kAEGPEffectSuite,
    kAEGPEffectSuiteVersion4
);

impl EffectSuite {
    pub fn effect_call_generic<T: Sized>(
        &self,
        plugin_id: PluginID,
        effect_ref: EffectRefHandle,
        time: Time,
        command: pf::Command,
        extra_payload: Option<&T>,
    ) -> Result<(), Error> {
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_EffectCallGeneric,
            plugin_id,
            effect_ref.as_ptr(),
            &time as *const _ as *const ae_sys::A_Time,
            command as ae_sys::PF_Cmd,
            // T is Sized so it can never be a fat pointer
            // which means we are safe to transmute here.
            // Alternatively we could write
            // extra_payload.map(|p| p as *const _).unwrap_or(core::ptr::null())
            std::mem::transmute(extra_payload)
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    PFInterfaceSuite,
    AEGP_PFInterfaceSuite1,
    kAEGPPFInterfaceSuite,
    kAEGPPFInterfaceSuiteVersion1
);

impl PFInterfaceSuite {
    pub fn get_effect_layer(
        &self,
        effect_ref: pf::ProgPtr,
    ) -> Result<LayerHandle, Error> {
        let mut layer_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetEffectLayer,
            effect_ref,
            layer_handle.as_mut_ptr()
        ) {
            Ok(()) => Ok(LayerHandle(unsafe { layer_handle.assume_init() })),
            Err(e) => Err(e),
        }
    }

    pub fn get_effect_camera(
        &self,
        effect_ref: pf::ProgPtr,
        time: Time,
    ) -> Result<LayerHandle, Error> {
        let mut camera_layer_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetEffectCamera,
            effect_ref,
            &time as *const _ as _, // as *const ae_sys::A_Time,
            camera_layer_handle.as_mut_ptr()
        ) {
            Ok(()) => {
                let camera_layer_handle =
                    unsafe { camera_layer_handle.assume_init() };
                if camera_layer_handle.is_null() {
                    Err(Error::Generic)
                } else {
                    Ok(LayerHandle(camera_layer_handle))
                }
            }
            Err(e) => Err(e),
        }
    }
}

// FIXME: wrap this nicely or combine WorldHandle & WorldSuite into
// single World
define_handle_wrapper!(WorldHandle, AEGP_WorldH);

define_suite!(
    WorldSuite,
    AEGP_WorldSuite3,
    kAEGPWorldSuite,
    kAEGPWorldSuiteVersion3
);

impl WorldSuite {
    #[inline]
    pub fn fill_out_pf_effect_world(
        &self,
        world: WorldHandle,
    ) -> Result<EffectWorld, Error> {
        let mut effect_world =
            std::mem::MaybeUninit::<ae_sys::PF_EffectWorld>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FillOutPFEffectWorld,
            world.as_ptr(),
            effect_world.as_mut_ptr()
        ) {
            Ok(()) => Ok(EffectWorld {
                effect_world: unsafe { effect_world.assume_init() },
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_base_addr8(
        &self,
        world_handle: WorldHandle,
    ) -> Result<*mut pf::Pixel8, Error> {
        let mut base_addr = std::mem::MaybeUninit::<*mut pf::Pixel8>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBaseAddr8,
            world_handle.as_ptr(),
            base_addr.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(unsafe { base_addr.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_base_addr16(
        &self,
        world_handle: WorldHandle,
    ) -> Result<*mut pf::Pixel16, Error> {
        let mut base_addr = std::mem::MaybeUninit::<*mut pf::Pixel16>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBaseAddr16,
            world_handle.as_ptr(),
            base_addr.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(unsafe { base_addr.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_base_addr32(
        &self,
        world_handle: WorldHandle,
    ) -> Result<*mut pf::Pixel32, Error> {
        let mut base_addr = std::mem::MaybeUninit::<*mut pf::Pixel32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBaseAddr32,
            world_handle.as_ptr(),
            base_addr.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(unsafe { base_addr.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_type(&self, world: WorldHandle) -> Result<WorldType, Error> {
        let mut world_type = std::mem::MaybeUninit::<WorldType>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetType,
            world.as_ptr(),
            world_type.as_mut_ptr() as _
        ) {
            Ok(()) => Ok(unsafe { world_type.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_size(&self, world: WorldHandle) -> Result<(u32, u32), Error> {
        let mut width = std::mem::MaybeUninit::<u32>::uninit();
        let mut height = std::mem::MaybeUninit::<u32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetSize,
            world.as_ptr(),
            width.as_mut_ptr() as _,
            height.as_mut_ptr() as _,
        ) {
            Ok(()) => {
                Ok(unsafe { (width.assume_init(), height.assume_init()) })
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_row_bytes(&self, world: WorldHandle) -> Result<usize, Error> {
        let mut row_bytes = std::mem::MaybeUninit::<usize>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetRowBytes,
            world.as_ptr(),
            row_bytes.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { row_bytes.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

pub struct World {
    world_handle: AEGP_WorldH,
    is_owned: bool,
}

impl World {
    pub fn from_raw(world_handle: ae_sys::AEGP_WorldH) -> Self {
        Self {
            world_handle,
            is_owned: false,
        }
    }

    pub fn into_raw(world: World) -> ae_sys::AEGP_WorldH {
        world.world_handle
    }

    pub fn as_ptr(&self) -> ae_sys::AEGP_WorldH {
        self.world_handle
    }

    #[inline]
    pub fn new(
        plugin_id: PluginID,
        world_type: WorldType,
        width: u32,
        height: u32,
    ) -> Result<World, Error> {
        let mut world_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_WorldH>::uninit();
        let world_suite = WorldSuite::new()?;

        match ae_call_suite_fn!(
            world_suite.suite_ptr,
            AEGP_New,
            plugin_id,
            world_type as ae_sys::AEGP_WorldType,
            width as i32,
            height as i32,
            world_handle.as_mut_ptr()
        ) {
            Ok(()) => Ok(World {
                world_handle: unsafe { world_handle.assume_init() },
                is_owned: true,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn handle(&self) -> WorldHandle {
        WorldHandle::from_raw(self.world_handle)
    }
}

impl Drop for World {
    #[inline]
    fn drop(&mut self) {
        if self.is_owned {
            let world_suite = WorldSuite::new().unwrap();
            // Dispose memory we allocated
            ae_call_suite_fn!(
                world_suite.suite_ptr,
                AEGP_Dispose,
                self.world_handle
            )
            .expect("Failed to dispose world handle.");
        }
    }
}

define_handle_wrapper!(CompHandle, AEGP_CompH);

define_suite!(
    CompSuite,
    AEGP_CompSuite11,
    kAEGPCompSuite,
    kAEGPCompSuiteVersion11
);

impl CompSuite {
    #[inline]
    pub fn get_comp_shutter_angle_phase(
        &self,
        comp_handle: CompHandle,
    ) -> Result<(Ratio, Ratio), Error> {
        let mut angle = std::mem::MaybeUninit::<Ratio>::uninit();
        let mut phase = std::mem::MaybeUninit::<Ratio>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompShutterAnglePhase,
            comp_handle.as_ptr(),
            angle.as_mut_ptr() as _,
            phase.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { (angle.assume_init(), phase.assume_init()) }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_suggested_motion_blur_samples(
        &self,
        comp_handle: CompHandle,
    ) -> Result<u32, Error> {
        let mut samples = std::mem::MaybeUninit::<A_long>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompSuggestedMotionBlurSamples,
            comp_handle.as_ptr(),
            samples.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { samples.assume_init() as u32 }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_item_from_comp(
        &self,
        comp_handle: CompHandle,
    ) -> Result<ItemHandle, Error> {
        let mut item_handle_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_ItemH>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemFromComp,
            comp_handle.as_ptr(),
            item_handle_ptr.as_mut_ptr()
        ) {
            Ok(()) => Ok(ItemHandle::from_raw(unsafe {
                item_handle_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_flags(
        &self,
        comp_handle: CompHandle,
    ) -> Result<CompFlags, Error> {
        let mut comp_flags = std::mem::MaybeUninit::<CompFlags>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompFlags,
            comp_handle.as_ptr(),
            comp_flags.as_mut_ptr() as *mut ae_sys::A_long
        ) {
            Ok(()) => Ok(unsafe { comp_flags.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_framerate(
        &self,
        comp_handle: CompHandle,
    ) -> Result<f64, Error> {
        let mut framerate = std::mem::MaybeUninit::<f64>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompFramerate,
            comp_handle.as_ptr(),
            framerate.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { framerate.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

pub struct Comp {
    // We need to store this pointer to be able to
    // drop resources at the end of our lifetime
    // using release_suite()
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    /*suite_ptr: *const ae_sys::AEGP_CompSuite11,
     *comp_ptr: *const ae_sys::AEGP_CompH, */
}

impl Comp {
    #[inline]
    pub fn from_item(
        pica_basic_suite_handle: &PicaBasicSuiteHandle,
        item_handle: ItemHandle,
    ) -> Result<Self, Error> {
        let pica_basic_suite_ptr = pica_basic_suite_handle.as_ptr();
        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_CompSuite11,
            kAEGPCompSuite,
            kAEGPCompSuiteVersion11
        )?;

        let comp_ptr: *mut ae_sys::AEGP_CompH = std::ptr::null_mut();
        ae_call_suite_fn!(
            suite_ptr,
            AEGP_GetCompFromItem,
            item_handle.as_ptr(),
            comp_ptr,
        )?;

        Ok(Self {
            pica_basic_suite_ptr,
            /*suite_ptr,
             *comp_ptr, */
        })
    }
}

impl Drop for Comp {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        // release suite
        ae_release_suite_ptr!(
            self.pica_basic_suite_ptr,
            kAEGPCompSuite,
            kAEGPCompSuiteVersion11
        );
    }
}

define_handle_wrapper!(LayerHandle, AEGP_LayerH);

define_suite!(
    LayerSuite,
    AEGP_LayerSuite8,
    kAEGPLayerSuite,
    kAEGPLayerSuiteVersion8
);

impl LayerSuite {
    #[inline]
    pub fn get_layer_parent_comp(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<CompHandle, Error> {
        let mut parent_comp_handle =
            MaybeUninit::<ae_sys::AEGP_CompH>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerParentComp,
            layer_handle.as_ptr(),
            parent_comp_handle.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe {
                CompHandle::from_raw(parent_comp_handle.assume_init())
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_num_layers(
        &self,
        comp_handle: CompHandle,
    ) -> Result<usize, Error> {
        let mut num_layers = MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompNumLayers,
            comp_handle.as_ptr(),
            num_layers.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_layers.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_layer_by_index(
        &self,
        comp_handle: CompHandle,
        layer_index: usize,
    ) -> Result<LayerHandle, Error> {
        let mut num_layers = MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompLayerByIndex,
            comp_handle.as_ptr(),
            layer_index as i32,
            num_layers.as_mut_ptr()
        ) {
            Ok(()) => {
                Ok(LayerHandle::from_raw(unsafe { num_layers.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_name(
        &self,
        plugin_id: PluginID,
        layer_handle: LayerHandle,
    ) -> Result<(String, String), Error> {
        let mut layer_name_mem_handle =
            MaybeUninit::<ae_sys::AEGP_MemHandle>::uninit();
        let mut source_name_mem_handle =
            MaybeUninit::<ae_sys::AEGP_MemHandle>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerName,
            plugin_id,
            layer_handle.as_ptr(),
            layer_name_mem_handle.as_mut_ptr(),
            source_name_mem_handle.as_mut_ptr(),
        ) {
            Ok(()) => Ok((
                // Create a mem handle each and lock it.
                // When the lock goes out of scope itr
                // uinlocks and when the handle goes out
                // of scope it gives the memory back to Ae.
                unsafe {
                    U16CString::from_ptr_str(
                        MemHandle::<u16>::from_raw(
                            layer_name_mem_handle.assume_init(),
                        )?
                        .lock()?
                        .as_ptr(),
                    )
                    .to_string_lossy()
                },
                unsafe {
                    U16CString::from_ptr_str(
                        MemHandle::<u16>::from_raw(
                            source_name_mem_handle.assume_init(),
                        )?
                        .lock()?
                        .as_ptr(),
                    )
                    .to_string_lossy()
                },
            )),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_id(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<LayerID, Error> {
        let mut id = MaybeUninit::<LayerID>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerID,
            layer_handle.as_ptr(),
            id.as_mut_ptr() as *mut i32
        ) {
            Ok(()) => Ok(unsafe { id.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_flags(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<LayerFlags, Error> {
        let mut flags = MaybeUninit::<LayerFlags>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerFlags,
            layer_handle.as_ptr(),
            flags.as_mut_ptr() as *mut i32
        ) {
            Ok(()) => Ok(unsafe { flags.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_object_type(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<ObjectType, Error> {
        let mut object_type = MaybeUninit::<ObjectType>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerObjectType,
            layer_handle.as_ptr(),
            object_type.as_mut_ptr() as *mut i32
        ) {
            Ok(()) => Ok(unsafe { object_type.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_to_world_xform(
        &self,
        layer_handle: LayerHandle,
        time: Time,
    ) -> Result<Matrix4, Error> {
        let mut matrix = MaybeUninit::<Matrix4>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerToWorldXform,
            layer_handle.as_ptr(),
            &time as *const _ as _,
            matrix.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(unsafe { matrix.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_masked_bounds(
        &self,
        layer_handle: LayerHandle,
        time_mode: TimeMode,
        time: Time,
    ) -> Result<FloatRect, Error> {
        let mut rect = MaybeUninit::<FloatRect>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerMaskedBounds,
            layer_handle.as_ptr(),
            time_mode as ae_sys::AEGP_LTimeMode,
            &time as *const _ as _,
            rect.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(unsafe { rect.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct StreamValue2 {
    stream_reference_handle: StreamReferenceHandle,
    pub value: StreamValue,
}

//impl Drop for StreamValue2 {}

define_suite!(
    StreamSuite,
    AEGP_StreamSuite5,
    kAEGPStreamSuite,
    kAEGPStreamSuiteVersion5
);

define_owned_handle_wrapper!(StreamReferenceHandle, AEGP_StreamRefH);

impl Drop for StreamReferenceHandle {
    fn drop(&mut self) {
        if self.is_owned() {
            StreamSuite::new().unwrap().dispose_stream(self).unwrap();
        }
    }
}

impl StreamSuite {
    #[inline]
    pub fn get_new_layer_stream(
        &self,
        plugin_id: PluginID,
        layer_handle: LayerHandle,
        stream_name: LayerStream,
    ) -> Result<StreamReferenceHandle, Error> {
        let mut stream_reference_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_StreamRefH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewLayerStream,
            plugin_id,
            layer_handle.as_ptr(),
            stream_name as i32,
            stream_reference_ptr.as_mut_ptr()
        ) {
            Ok(()) => Ok(StreamReferenceHandle(
                unsafe { stream_reference_ptr.assume_init() },
                true, // is_owned
            )),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn dispose_stream(
        &self,
        stream_reference_handle: &mut StreamReferenceHandle,
    ) -> Result<(), Error> {
        let result = ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_DisposeStream,
            stream_reference_handle.as_ptr(),
        );
        stream_reference_handle.0 = std::ptr::null_mut();
        result
    }

    // FIXME: should this handle memory owned by Ae properly?
    // Currently we just copy and dispose immedately. Should be fine
    // for what we're doing atm but for stream data like image buffers this
    // is wasteful and potentially slow.
    #[inline]
    pub fn get_new_stream_value(
        &self,
        plugin_id: PluginID,
        stream_reference_handle: StreamReferenceHandle,
        time_mode: TimeMode,
        time: Time,
        sample_stream_pre_expression: bool,
    ) -> Result<StreamValue2, Error> {
        //let mut stream_value = std::mem::MaybeUninit::<StreamValue2>::uninit();
        let stream_value_ptr: *const StreamValue = std::ptr::null();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewStreamValue,
            plugin_id,
            stream_reference_handle.as_ptr(),
            time_mode as ae_sys::AEGP_LTimeMode,
            &time as *const _ as *const ae_sys::A_Time,
            sample_stream_pre_expression as u8,
            stream_value_ptr as *mut ae_sys::AEGP_StreamValue2,
        ) {
            Ok(()) => {
                let value = unsafe { *stream_value_ptr };
                ae_call_suite_fn!(
                    self.suite_ptr,
                    AEGP_DisposeStreamValue,
                    stream_value_ptr
                        as *mut ae_sys::AEGP_StreamValue2,
                )
                .unwrap();
                Ok(StreamValue2 {
                    stream_reference_handle,
                    value,
                })
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn _dispose_stream_value(
        &self,
        mut stream_value: StreamValue2,
    ) -> Result<(), Error> {
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_DisposeStreamValue,
            &mut stream_value as *mut _ as _
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_layer_stream_value(
        &self,
        layer_handle: LayerHandle,
        stream: LayerStream,
        time_mode: TimeMode,
        time: Time,
        pre_expression: bool,
    ) -> Result<StreamValue, Error> {
        let mut stream_value =
            std::mem::MaybeUninit::<ae_sys::AEGP_StreamVal2>::uninit();
        let mut stream_type =
            std::mem::MaybeUninit::<ae_sys::AEGP_StreamType>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerStreamValue,
            layer_handle.as_ptr(),
            stream as i32,
            time_mode as ae_sys::AEGP_LTimeMode,
            &time as *const _ as *const ae_sys::A_Time,
            pre_expression as u8,
            stream_value.as_mut_ptr() as *mut _,
            stream_type.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(match unsafe { stream_type.assume_init() } as u32 {
                ae_sys::AEGP_StreamType_NO_DATA => StreamValue::None,
                ae_sys::AEGP_StreamType_ThreeD_SPATIAL => unsafe {
                    let value = stream_value.assume_init().three_d;
                    StreamValue::ThreeDSpatial {
                        x: value.x,
                        y: value.y,
                        z: value.z,
                    }
                },
                ae_sys::AEGP_StreamType_ThreeD => unsafe {
                    let value = stream_value.assume_init().three_d;
                    StreamValue::ThreeD {
                        x: value.x,
                        y: value.y,
                        z: value.z,
                    }
                },
                ae_sys::AEGP_StreamType_TwoD_SPATIAL => unsafe {
                    let value = stream_value.assume_init().two_d;
                    StreamValue::TwoDSpatial {
                        x: value.x,
                        y: value.y,
                    }
                },
                ae_sys::AEGP_StreamType_TwoD => unsafe {
                    let value = stream_value.assume_init().two_d;
                    StreamValue::TwoD {
                        x: value.x,
                        y: value.y,
                    }
                },
                ae_sys::AEGP_StreamType_OneD => unsafe {
                    StreamValue::OneD(stream_value.assume_init().one_d)
                },
                ae_sys::AEGP_StreamType_COLOR => unsafe {
                    let value = stream_value.assume_init().color;
                    StreamValue::Color {
                        alpha: value.alphaF,
                        red: value.redF,
                        green: value.greenF,
                        blue: value.blueF,
                    }
                },
                /*
                Arb = AEGP_StreamType_ARB,
                Marker = AEGP_StreamType_MARKER,
                LayerID = AEGP_StreamType_LAYER_ID,
                MaskID = AEGP_StreamType_MASK_ID,
                Mask = AEGP_StreamType_MASK,
                TextDocument = AEGP_StreamType_TEXT_DOCUMENT,*/
                _ => StreamValue::None,
            }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    DynamicStreamSuite,
    AEGP_DynamicStreamSuite4,
    kAEGPDynamicStreamSuite,
    kAEGPDynamicStreamSuiteVersion4
);

impl DynamicStreamSuite {
    #[inline]
    pub fn get_new_stream_ref_for_layer(
        &self,
        plugin_id: PluginID,
        layer_handle: LayerHandle,
    ) -> Result<StreamReferenceHandle, Error> {
        let mut stream_reference_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_StreamRefH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewStreamRefForLayer,
            plugin_id,
            layer_handle.as_ptr(),
            stream_reference_ptr.as_mut_ptr(),
        ) {
            Ok(()) => Ok(StreamReferenceHandle(
                unsafe { stream_reference_ptr.assume_init() },
                true,
            )),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_num_streams_in_group(
        &self,
        stream_reference_handle: StreamReferenceHandle,
    ) -> Result<usize, Error> {
        let mut num_streams = std::mem::MaybeUninit::<ae_sys::A_long>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNumStreamsInGroup,
            stream_reference_handle.as_ptr(),
            num_streams.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_streams.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    pub fn get_match_name(
        &self,
        stream_reference_handle: StreamReferenceHandle,
    ) -> Result<String, Error> {
        let mut stream_value = std::mem::MaybeUninit::<
            [i8; ae_sys::AEGP_MAX_STREAM_MATCH_NAME_SIZE as usize],
        >::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMatchName,
            stream_reference_handle.as_ptr(),
            stream_value.as_mut_ptr() as *mut _
        ) {
            Ok(()) => Ok(unsafe {
                CString::from_raw(stream_value.as_mut_ptr() as *mut _)
            }
            .into_string()
            .unwrap()),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    UtilitySuite,
    AEGP_UtilitySuite6,
    kAEGPUtilitySuite,
    kAEGPUtilitySuiteVersion6
);

impl UtilitySuite {
    #[inline]
    pub fn register_with_aegp(
        &self,
        //global_refcon:,
        plug_in_name: impl Into<Vec<u8>>,
    ) -> Result<PluginID, Error> {
        let mut plugin_id =
            std::mem::MaybeUninit::<ae_sys::AEGP_PluginID>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_RegisterWithAEGP,
            std::ptr::null_mut() as _,
            CString::new(plug_in_name).unwrap().as_ptr(),
            plugin_id.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { plugin_id.assume_init() }),
            Err(e) => Err(e),
        }
    }

    /*
    #[inline]
    pub fn get_plugin_paths(
        &self,
    ) -> Result<Path, Error>
    {
        let mut path = std::mem::MaybeUninit::<ae_sys::AEGP_PluginID>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetPluginPaths
        ) {
            Ok(()) => Ok(unsafe { plugin_id.assume_init() }),
            Err(e) => Err(e),
        }
    }*/
    #[inline]
    pub fn write_to_os_console(
        &self,
        //global_refcon:,
        message: impl Into<Vec<u8>>,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_WriteToOSConsole,
            CString::new(message).unwrap().as_ptr(),
        )
    }
}

define_suite!(
    CompositeSuite,
    AEGP_CompositeSuite2,
    kAEGPCompositeSuite,
    kAEGPCompositeSuiteVersion2
);

impl CompositeSuite {
    #[inline]
    pub fn transfer_rect(
        &self,
        quality: pf::Quality,
        alpha: pf::ModeFlags,
        field: pf::Field,
        src_rect: &crate::Rect,
        src_world: &EffectWorld,
        comp_mode: &pf::CompositeMode,
        blending_tables: Option<&EffectBlendingTables>,
        mask_world: Option<pf::MaskWorld>,
        dst_x: u32,
        dst_y: u32,
        dst_world: &mut EffectWorld,
    ) -> Result<(), Error> {
        let mask_world = match mask_world {
            None => None,
            Some(m) => Some(ae_sys::PF_MaskWorld {
                mask: m.mask.effect_world.clone(),
                offset: ae_sys::PF_Point {
                    v: m.offset.v,
                    h: m.offset.h,
                },
                what_is_mask: m.what_is_mask as i32,
            }),
        };
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_TransferRect,
            quality as i32,
            alpha as i32,
            field as i32,
            src_rect as *const _ as _,
            src_world.as_ptr(),
            comp_mode as *const _ as _,
            blending_tables.map_or(std::ptr::null(), |b| b.as_ptr()) as _,
            mask_world.map_or(std::ptr::null(), |m| &m) as _,
            dst_x as i32,
            dst_y as i32,
            dst_world.as_mut_ptr()
        )
    }
}

define_suite!(
    CanvasSuite,
    AEGP_CanvasSuite8,
    kAEGPCanvasSuite,
    kAEGPCanvasSuiteVersion8
);

impl CanvasSuite {
    #[inline]
    pub fn get_comp_to_render(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<CompHandle, Error> {
        let mut comp_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_CompH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompToRender,
            render_context_handle.as_ptr(),
            comp_ptr.as_mut_ptr()
        ) {
            Ok(()) => {
                Ok(CompHandle::from_raw(unsafe { comp_ptr.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_render_time(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<(Time, Time), Error> {
        let mut shutter_frame_start = std::mem::MaybeUninit::<Time>::uninit();

        let mut shutter_frame_duration =
            std::mem::MaybeUninit::<Time>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompRenderTime,
            render_context_handle.as_ptr(),
            shutter_frame_start.as_mut_ptr() as *mut ae_sys::A_Time,
            shutter_frame_duration.as_mut_ptr() as *mut ae_sys::A_Time
        ) {
            Ok(()) => Ok(unsafe {
                (
                    shutter_frame_start.assume_init(),
                    shutter_frame_duration.assume_init(),
                )
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_comp_destination_buffer(
        &self,
        render_context_handle: pr::RenderContextHandle,
        comp_handle: CompHandle,
    ) -> Result<WorldHandle, Error> {
        let mut world_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_WorldH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompDestinationBuffer,
            render_context_handle.as_ptr(),
            comp_handle.as_ptr(),
            world_ptr.as_mut_ptr(),
        ) {
            Ok(()) => {
                Ok(WorldHandle::from_raw(unsafe { world_ptr.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn report_artisan_progress(
        &self,
        render_context_handle: pr::RenderContextHandle,
        count: u16,
        total: u16,
    ) -> Option<Error> {
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_ReportArtisanProgress,
            render_context_handle.as_ptr(),
            count as i32,
            total as i32,
        ) {
            Ok(()) => None,
            Err(e) => Some(e),
        }
    }

    #[inline]
    pub fn get_roi(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<Rect, Error> {
        let mut roi = std::mem::MaybeUninit::<ae_sys::A_LegacyRect>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetROI,
            render_context_handle.as_ptr(),
            roi.as_mut_ptr() as _,
        ) {
            Ok(()) => {
                let rect = unsafe { roi.assume_init() };
                Ok(Rect {
                    left: rect.left as i32,
                    top: rect.top as i32,
                    right: rect.right as i32,
                    bottom: rect.bottom as i32,
                })
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_render_downsample_factor(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<DownsampleFactor, Error> {
        let mut dsf = std::mem::MaybeUninit::<DownsampleFactor>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetRenderDownsampleFactor,
            render_context_handle.as_ptr(),
            dsf.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { dsf.assume_init() }),
            Err(e) => Err(e),
        }
    }

    /*
    pub fn get_render_layer_bounds(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<FloatRect, Error> {

        let mut roi = std::mem::MaybeUninit::<FloatRect>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            render_context_handle.as_ptr(),
                                                                  layer.handle(),
                                                                  &compTime,
                                                                  &layerBounds ) );
    return layerBounds;*/
}

define_suite!(
    LightSuite,
    AEGP_LightSuite2,
    kAEGPLightSuite,
    kAEGPLightSuiteVersion2
);

impl LightSuite {
    #[inline]
    pub fn get_light_type(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<LightType, Error> {
        let mut light_type = std::mem::MaybeUninit::<LightType>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLightType,
            layer_handle.as_ptr(),
            light_type.as_mut_ptr() as *mut u32,
        ) {
            Ok(()) => Ok(unsafe { light_type.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

define_handle_wrapper!(ItemHandle, AEGP_ItemH);

define_suite!(
    ItemSuite,
    AEGP_ItemSuite9,
    kAEGPItemSuite,
    kAEGPItemSuiteVersion9
);

impl ItemSuite {
    #[inline]
    pub fn get_item_id(
        &self,
        item_handle: ItemHandle,
    ) -> Result<ItemID, Error> {
        let mut item_id = std::mem::MaybeUninit::<ItemID>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemID,
            item_handle.as_ptr(),
            item_id.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { item_id.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_item_dimensions(
        &self,
        item_handle: ItemHandle,
    ) -> Result<(u32, u32), Error> {
        let mut width = std::mem::MaybeUninit::<u32>::uninit();
        let mut height = std::mem::MaybeUninit::<u32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemDimensions,
            item_handle.as_ptr(),
            width.as_mut_ptr() as _,
            height.as_mut_ptr() as _
        ) {
            Ok(()) => {
                Ok(unsafe { (width.assume_init(), height.assume_init()) })
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_item_pixel_aspect_ratio(
        &self,
        item_handle: ItemHandle,
    ) -> Result<Ratio, Error> {
        let mut ratio = std::mem::MaybeUninit::<Ratio>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetItemPixelAspectRatio,
            item_handle.as_ptr(),
            ratio.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { ratio.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    CameraSuite,
    AEGP_CameraSuite2,
    kAEGPCameraSuite,
    kAEGPCameraSuiteVersion2
);

impl CameraSuite {
    #[inline]
    pub fn get_camera(
        &self,
        render_context_handle: pr::RenderContextHandle,
        time: Time,
    ) -> Result<LayerHandle, Error> {
        let mut camera_layer_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCamera,
            render_context_handle.as_ptr(),
            &time as *const _ as *const ae_sys::A_Time,
            camera_layer_handle.as_mut_ptr(),
        ) {
            Ok(()) => {
                // If the comp has no camera Ae will return a NULL
                // ptr instead of an error! We need to handle this
                // ourselves.
                let camera_layer_handle =
                    unsafe { camera_layer_handle.assume_init() };
                if camera_layer_handle.is_null() {
                    Err(Error::Generic)
                } else {
                    Ok(LayerHandle::from_raw(camera_layer_handle))
                }
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_camera_film_size(
        &self,
        camera_layer_handle: LayerHandle,
    ) -> Result<(FilmSizeUnits, f64), Error> {
        let mut film_size_units: FilmSizeUnits = FilmSizeUnits::None;
        let mut film_size: ae_sys::A_FpLong = 0.0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCameraFilmSize,
            camera_layer_handle.as_ptr(),
            &mut film_size_units as *mut _ as *mut i32,
            &mut film_size,
        ) {
            Ok(()) => Ok((film_size_units, film_size)),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_default_camera_distance_to_image_plane(
        &self,
        comp_handle: CompHandle,
    ) -> Result<f64, Error> {
        let mut distance: f64 = 0.0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetDefaultCameraDistanceToImagePlane,
            comp_handle.as_ptr(),
            &mut distance
        ) {
            Ok(()) => Ok(distance),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_camera_type(
        &self,
        camera_layer_handle: LayerHandle,
    ) -> Result<CameraType, Error> {
        let mut camera_type: CameraType = CameraType::None;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCameraType,
            camera_layer_handle.as_ptr(),
            &mut camera_type as *mut _ as *mut u32,
        ) {
            Ok(()) => Ok(camera_type),
            Err(e) => Err(e),
        }
    }
}

pub struct Scene3DLayerHandle {
    scene3d_layer_ptr: *const ae_sys::AEGP_Scene3DLayer,
}

impl Scene3DLayerHandle {
    #[inline]
    pub fn from_raw(
        scene3d_layer_ptr: *const ae_sys::AEGP_Scene3DLayer,
    ) -> Self {
        Self { scene3d_layer_ptr }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::AEGP_Scene3DLayer {
        self.scene3d_layer_ptr
    }
}

pub struct Scene3D {
    // We need to store this pointer to be able to
    // drop resources at the end of our lifetime
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,

    suite_ptr: *const ae_sys::AEGP_Scene3DSuite2,

    scene3d_ptr: *mut ae_sys::AEGP_Scene3D,
    texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext,

    in_data_ptr: *const ae_sys::PR_InData,
    render_context_ptr: ae_sys::PR_RenderContextH,
}

impl Scene3D {
    #[inline]
    pub fn new(
        in_data_handle: pr::InDataHandle,
        render_context: pr::RenderContextHandle,
        global_texture_cache_handle: aegp::Scene3DTextureCacheHandle,
    ) -> Result<Scene3D, Error> {
        let pica_basic_suite_ptr = in_data_handle.pica_basic_handle().as_ptr();

        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_Scene3DSuite2,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        )?;

        let mut scene3d_ptr: *mut ae_sys::AEGP_Scene3D = std::ptr::null_mut();

        ae_call_suite_fn!(suite_ptr, AEGP_Scene3DAlloc, &mut scene3d_ptr,)?;

        let mut texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext =
            std::ptr::null_mut();

        match ae_call_suite_fn!(
            suite_ptr,
            AEGP_Scene3DTextureContextAlloc,
            in_data_handle.as_ptr(),
            render_context.as_ptr(),
            global_texture_cache_handle.texture_cache_ptr,
            false as u8, // unlock all
            &mut texture_context_ptr
        ) {
            Ok(()) => Ok(Scene3D {
                pica_basic_suite_ptr,
                suite_ptr,
                scene3d_ptr,
                texture_context_ptr,
                in_data_ptr: in_data_handle.as_ptr(),
                render_context_ptr: render_context.as_ptr(),
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_scene3d_ptr(&self) -> *mut ae_sys::AEGP_Scene3D {
        self.scene3d_ptr
    }

    #[inline]
    pub fn get_scene3d_suite_ptr(&self) -> *const ae_sys::AEGP_Scene3DSuite2 {
        self.suite_ptr
    }

    #[inline]
    pub fn setup_motion_blur_samples(
        &self,
        motion_samples: usize,
        sample_method: ae_sys::Scene3DMotionSampleMethod,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3D_SetupMotionBlurSamples,
            self.in_data_ptr,
            self.render_context_ptr,
            // the empty scene, modified
            self.scene3d_ptr,
            // how many motion samples
            motion_samples as i32,
            sample_method
        )
    }

    #[inline]
    pub fn build(
        &self,
        progress_abort_callback_ptr: *mut ae_sys::AEGP_Scene3DProgressAbort,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3D_Build,
            self.in_data_ptr,
            self.render_context_ptr,
            self.texture_context_ptr,
            progress_abort_callback_ptr,
            self.scene3d_ptr
        )
    }

    #[inline]
    pub fn scene_num_lights(&self) -> Result<usize, Error> {
        let mut num_lights: i32 = 0;
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DSceneNumLights,
            self.scene3d_ptr,
            &mut num_lights
        ) {
            Ok(()) => Ok(num_lights as usize),
            Err(e) => Err(e),
        }
    }

    // FIXME: make this neat, see
    // https://blog.seantheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
    #[inline]
    pub fn node_traverser(
        &self,
        node_visitor_func: ae_sys::Scene3DNodeVisitorFunc,
        reference_context: *mut std::os::raw::c_void, /* FIXME: can we use a Box
                                                       * here? Box<*
                                                       * mut
                                                       * ::std::os::raw::c_void> */
        flags: ae_sys::Scene3DTraverseFlags,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNodeTraverser,
            self.scene3d_ptr,
            node_visitor_func,
            reference_context,
            flags
        )
    }

    #[inline]
    pub fn layer_num_post_xform(
        &self,
        scene3d_layer_handle: &Scene3DLayerHandle,
    ) -> Result<usize, Error> {
        let mut num_xform = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DLayerNumPostXform,
            scene3d_layer_handle.as_ptr(),
            num_xform.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_xform.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn num_sub_frame_times(&self) -> Result<usize, Error> {
        let mut num_motion_samples = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNumSubFrameTimes,
            self.scene3d_ptr,
            num_motion_samples.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { num_motion_samples.assume_init() } as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn layer_get_post_xform(
        &self,
        layer_handle: &Scene3DLayerHandle,
        index: usize,
    ) -> Result<Matrix4, Error> {
        let mut matrix_ptr = std::mem::MaybeUninit::<*const Matrix4>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DLayerGetPostXform,
            layer_handle.as_ptr(),
            index as i32,
            matrix_ptr.as_mut_ptr() as *mut *const _
        ) {
            Ok(()) => Ok({
                let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();
                unsafe {
                    std::ptr::copy(
                        matrix_ptr.assume_init(),
                        matrix.as_mut_ptr(),
                        1,
                    );
                    matrix.assume_init()
                }
            }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_sub_frame_time(&self, index: usize) -> Result<Time, Error> {
        let mut time = std::mem::MaybeUninit::<Time>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DGetSubFrameTime,
            self.scene3d_ptr,
            index as i32,
            time.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(unsafe { time.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

impl Drop for Scene3D {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        // dispose texture contex
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DTextureContextDispose,
            self.texture_context_ptr
        );

        // dispose scene
        ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DDispose,
            self.scene3d_ptr
        );

        // release suite
        ae_release_suite_ptr!(
            self.pica_basic_suite_ptr,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        );
    }
}

pub struct Scene3DTextureCacheHandle {
    texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
}

impl Scene3DTextureCacheHandle {
    #[inline]
    pub fn new(scene3d: Scene3D) -> Result<Scene3DTextureCacheHandle, Error> {
        let mut texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache =
            std::ptr::null_mut();

        match ae_call_suite_fn!(
            scene3d.suite_ptr,
            AEGP_Scene3DTextureCacheAlloc,
            &mut texture_cache_ptr,
        ) {
            Ok(()) => Ok(Scene3DTextureCacheHandle { texture_cache_ptr }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn from_raw(
        texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
    ) -> Scene3DTextureCacheHandle {
        Scene3DTextureCacheHandle { texture_cache_ptr }
    }
}

define_ptr_wrapper!(Scene3DMaterialHandle, AEGP_Scene3DMaterial);
define_handle_wrapper!(Scene3DNodeHandle, AEGP_Scene3DNodeP);
define_ptr_wrapper!(Scene3DMeshHandle, AEGP_Scene3DMesh);

define_suite!(
    Scene3DMaterialSuite,
    AEGP_Scene3DMaterialSuite1,
    kAEGPScene3DMaterialSuite,
    kAEGPScene3DMaterialSuiteVersion1
);

impl Scene3DMaterialSuite {
    #[inline]
    pub fn has_uv_color_texture(
        &self,
        material_handle: Scene3DMaterialHandle,
    ) -> Result<bool, Error> {
        let mut has_uv_color_texture: u8 = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_HasUVColorTexture,
            material_handle.as_ptr(),
            &mut has_uv_color_texture
        ) {
            Ok(()) => Ok(has_uv_color_texture != 0),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_uv_color_texture(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<WorldHandle, Error> {
        let mut world_handle =
            std::mem::MaybeUninit::<ae_sys::AEGP_WorldH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetUVColorTexture,
            material.as_ptr(),
            world_handle.as_mut_ptr()
        ) {
            Ok(()) => Ok(WorldHandle(unsafe { world_handle.assume_init() })),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_basic_coeffs(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, Error> {
        let mut basic_material_coefficients =
            Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBasicCoeffs,
            material.as_ptr(),
            basic_material_coefficients.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { basic_material_coefficients.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    Scene3DNodeSuite,
    AEGP_Scene3DNodeSuite1,
    kAEGPScene3DNodeSuite,
    kAEGPScene3DNodeSuiteVersion1
);

impl Scene3DNodeSuite {
    #[inline]
    pub fn get_material_for_side(
        &self,
        node_handle: Scene3DNodeHandle,
        side: ae_sys::AEGP_Scene3DMaterialSide,
    ) -> Result<Scene3DMaterialHandle, Error> {
        let mut material_handle = std::mem::MaybeUninit::<
            *const ae_sys::AEGP_Scene3DMaterial,
        >::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialForSide,
            node_handle.as_ptr(),
            side,
            material_handle.as_mut_ptr() as *mut *mut _
        ) {
            Ok(()) => Ok(Scene3DMaterialHandle(unsafe {
                material_handle.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn node_mesh_get(
        &self,
        node_handle: Scene3DNodeHandle,
    ) -> Result<Scene3DMeshHandle, Error> {
        let mut mesh_handle =
            std::mem::MaybeUninit::<*const ae_sys::AEGP_Scene3DMesh>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodeMeshGet,
            node_handle.as_ptr(),
            mesh_handle.as_mut_ptr() as *mut *mut _
        ) {
            Ok(()) => {
                Ok(Scene3DMeshHandle(unsafe { mesh_handle.assume_init() }))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn node_post_xform_get(
        &self,
        scene3d_node_handle: Scene3DNodeHandle,
        index: usize,
    ) -> Result<Matrix4, Error> {
        let mut matrix = std::mem::MaybeUninit::<Matrix4>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodePostXformGet,
            scene3d_node_handle.as_ptr(),
            index as i32,
            matrix.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => {
                Ok(unsafe { matrix.assume_init() })
                //Ok((num_vertex, num_face))
            }
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    Scene3DMeshSuite,
    AEGP_Scene3DMeshSuite1,
    kAEGPScene3DMeshSuite,
    kAEGPScene3DMeshSuiteVersion1
);

impl Scene3DMeshSuite {
    #[inline]
    pub fn face_group_buffer_count(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<usize, Error> {
        let mut face_groups: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferCount,
            mesh_handle.as_ptr() as *mut _,
            &mut face_groups
        ) {
            Ok(()) => Ok(face_groups as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn face_group_buffer_size(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<usize, Error> {
        let mut face_count: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferSize,
            mesh_handle.as_ptr(),
            group_index as i32,
            &mut face_count
        ) {
            Ok(()) => Ok(face_count as usize),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn face_group_buffer_fill(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<Vec<ae_sys::A_long>, Error> {
        let face_count =
            self.face_group_buffer_size(mesh_handle, group_index)?;

        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(face_count as usize);

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferFill,
            mesh_handle.as_ptr(),
            group_index as i32,
            face_count as i32,
            face_index_buffer.as_mut_ptr()
        ) {
            Ok(()) => {
                // If the previous called didn't bitch we are safe
                // to set the vector's length.
                unsafe {
                    face_index_buffer.set_len(face_count as usize);
                }

                Ok(face_index_buffer)
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn get_material_side_for_face_group(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<ae_sys::AEGP_Scene3DMaterialSide, Error> {
        let mut material_side =
            std::mem::MaybeUninit::<ae_sys::AEGP_Scene3DMaterialSide>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialSideForFaceGroup,
            mesh_handle.as_ptr(),
            group_index as i32,
            material_side.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { material_side.assume_init() }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn mesh_get_info(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<(usize, usize), Error> {
        let mut num_vertex = std::mem::MaybeUninit::<i32>::uninit();
        let mut num_face = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshGetInfo,
            mesh_handle.as_ptr(),
            //&mut num_vertex as *mut _ as *mut i32,
            //&mut num_face as *mut _ as *mut i32,
            num_vertex.as_mut_ptr() as *mut i32,
            num_face.as_mut_ptr() as *mut i32,
        ) {
            Ok(()) => {
                Ok(unsafe {
                    (
                        num_vertex.assume_init() as usize,
                        num_face.assume_init() as usize,
                    )
                })
                //Ok((num_vertex, num_face))
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn vertex_buffer_element_size(
        &self,
        vertex_type: ae_sys::Scene3DVertexBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_VertexBufferElementSize,
            vertex_type
        ) as usize
    }

    #[inline]
    pub fn face_index_element_size(
        &self,
        face_type: ae_sys::Scene3DFaceBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_FaceBufferElementSize,
            face_type
        ) as usize
    }

    #[inline]
    pub fn uv_buffer_element_size(
        &self,
        uv_type: ae_sys::Scene3DUVBufferType,
    ) -> usize {
        ae_call_suite_fn_no_err!(
            self.suite_ptr,
            AEGP_UVBufferElementSize,
            uv_type
        ) as usize
    }

    #[inline]
    pub fn mesh_fill_buffers(
        &self,
        mesh_handle: Scene3DMeshHandle,
        vertex_type: ae_sys::Scene3DVertexBufferType,
        face_type: ae_sys::Scene3DFaceBufferType,
        uv_type: ae_sys::Scene3DUVBufferType,
    ) -> Result<
        (
            Vec<ae_sys::A_FpLong>,
            Vec<ae_sys::A_long>,
            Vec<ae_sys::A_FpShort>,
        ),
        Error,
    > {
        let (num_vertex, num_face) = self.mesh_get_info(mesh_handle)?;

        // Points (3-tuples) of f64
        let vertex_buffer_size: usize = num_vertex * 3;
        let mut vertex_buffer =
            Vec::<ae_sys::A_FpLong>::with_capacity(vertex_buffer_size);

        // quad meshes
        let face_index_buffer_size: usize = num_face * 4;
        let mut face_index_buffer =
            Vec::<ae_sys::A_long>::with_capacity(face_index_buffer_size);

        // 2 uvs per vertex per face
        let uv_per_face_buffer_size: usize = match uv_type {
            ae_sys::Scene3DUVBufferType_SCENE3D_UVPERFACEBUFFER_QUAD_FpShort2 => num_face * 4 * 2,
            _ => 0,
        };
        let mut uv_per_face_buffer =
            Vec::<ae_sys::A_FpShort>::with_capacity(uv_per_face_buffer_size);

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshFillBuffers,
            mesh_handle.as_ptr() as *mut _,
            vertex_type,
            vertex_buffer.as_mut_ptr() as *mut _,
            face_type,
            face_index_buffer.as_mut_ptr() as *mut _,
            uv_type,
            uv_per_face_buffer.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => {
                unsafe {
                    vertex_buffer.set_len(vertex_buffer_size);
                    face_index_buffer.set_len(face_index_buffer_size);
                    uv_per_face_buffer.set_len(uv_per_face_buffer_size);
                }

                Ok((vertex_buffer, face_index_buffer, uv_per_face_buffer))
            }
            Err(e) => Err(e),
        }
    }
}
