pub use crate::*;
pub use ae_sys::*;
use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use std::{ffi::CString, mem::MaybeUninit};
use widestring::U16CString;

pub type PluginID = ae_sys::AEGP_PluginID;

pub type MaterialBasic = ae_sys::AEGP_MaterialBasic_v1;

pub type ItemID = i32;

pub type CompFlags = u32;

pub const COMP_FLAG_SHOW_ALL_SHY: u32 =
    ae_sys::AEGP_CompFlag_SHOW_ALL_SHY;
pub const COMP_FLAG_RESERVED_1: u32 = ae_sys::AEGP_CompFlag_RESERVED_1;
pub const COMP_FLAG_RESERVED_2: u32 = ae_sys::AEGP_CompFlag_RESERVED_2;
pub const COMP_FLAG_ENABLE_MOTION_BLUR: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_MOTION_BLUR;
pub const COMP_FLAG_ENABLE_TIME_FILTER: u32 =
    ae_sys::AEGP_CompFlag_ENABLE_TIME_FILTER;
pub const COMP_FLAG_GRID_TO_FRAMES: u32 =
    ae_sys::AEGP_CompFlag_GRID_TO_FRAMES;
pub const COMP_FLAG_GRID_TO_FIELDS: u32 =
    ae_sys::AEGP_CompFlag_GRID_TO_FIELDS;
pub const COMP_FLAG_USE_LOCAL_DSF: u32 =
    ae_sys::AEGP_CompFlag_USE_LOCAL_DSF;
pub const COMP_FLAG_DRAFT_3D: u32 = ae_sys::AEGP_CompFlag_DRAFT_3D;
pub const COMP_FLAG_SHOW_GRAPH: u32 = ae_sys::AEGP_CompFlag_SHOW_GRAPH;
pub const COMP_FLAG_RESERVED_3: u32 = ae_sys::AEGP_CompFlag_RESERVED_3;

pub type MemFlag = u32;

pub const MEM_FLAG_NONE: u32 = ae_sys::AEGP_MemFlag_NONE;
pub const MEM_FLAG_CLEAR: u32 = ae_sys::AEGP_MemFlag_CLEAR;
pub const MEM_FLAG_QUIET: u32 = ae_sys::AEGP_MemFlag_QUIET;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(isize)]
pub enum LayerStream {
    None = AEGP_LayerStream_NONE as isize,
    AnchorPoint = AEGP_LayerStream_ANCHORPOINT as isize,
    Position = AEGP_LayerStream_POSITION as isize,
    Scale = AEGP_LayerStream_SCALE as isize,
    // This is the layer's rotation for a 2D layer
    RotateZ = AEGP_LayerStream_ROTATION as isize,
    Opcaity = AEGP_LayerStream_OPACITY as isize,
    Audio = AEGP_LayerStream_AUDIO as isize,
    Marker = AEGP_LayerStream_MARKER as isize,
    TimeRemap = AEGP_LayerStream_TIME_REMAP as isize,
    RotateX = AEGP_LayerStream_ROTATE_X as isize,
    RotateY = AEGP_LayerStream_ROTATE_Y as isize,
    Orientation = AEGP_LayerStream_ORIENTATION as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_CAMERA
    Zoom = AEGP_LayerStream_ZOOM as isize,
    DepthOfField = AEGP_LayerStream_DEPTH_OF_FIELD as isize,
    FocusDistance = AEGP_LayerStream_FOCUS_DISTANCE as isize,
    Aperture = AEGP_LayerStream_APERTURE as isize,
    BlurLevel = AEGP_LayerStream_BLUR_LEVEL as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_LIGHT
    Intensity = AEGP_LayerStream_INTENSITY as isize,
    Color = AEGP_LayerStream_COLOR as isize,
    ConeAngle = AEGP_LayerStream_CONE_ANGLE as isize,
    ConeFeather = AEGP_LayerStream_CONE_FEATHER as isize,
    ShadowDarkness = AEGP_LayerStream_SHADOW_DARKNESS as isize,
    ShadowDiffusion = AEGP_LayerStream_SHADOW_DIFFUSION as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_AV
    AcceptsShadows = AEGP_LayerStream_ACCEPTS_SHADOWS as isize,
    AcceptsLights = AEGP_LayerStream_ACCEPTS_LIGHTS as isize,
    AmbientCoeff = AEGP_LayerStream_AMBIENT_COEFF as isize,
    DiffuseCoeff = AEGP_LayerStream_DIFFUSE_COEFF as isize,
    SpecularIntensity = AEGP_LayerStream_SPECULAR_INTENSITY as isize,
    SpecularShininess = AEGP_LayerStream_SPECULAR_SHININESS as isize,

    CastsShadows = AEGP_LayerStream_CASTS_SHADOWS as isize, /* LIGHT as isize, and AV only, no CAMERA */
    LightTransmission = AEGP_LayerStream_LIGHT_TRANSMISSION as isize, /* AV Layer only */
    Metal = AEGP_LayerStream_METAL as isize, // AV layer only

    SourceText = AEGP_LayerStream_SOURCE_TEXT as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_CAMERA
    IrisShape = AEGP_LayerStream_IRIS_SHAPE as isize,
    IrisRotation = AEGP_LayerStream_IRIS_ROTATION as isize,
    IrisRoundness = AEGP_LayerStream_IRIS_ROUNDNESS as isize,
    IrisAspectRatio = AEGP_LayerStream_IRIS_ASPECT_RATIO as isize,
    IrisDiffractionFringe =
        AEGP_LayerStream_IRIS_DIFFRACTION_FRINGE as isize,
    IrisHighlightGain = AEGP_LayerStream_IRIS_HIGHLIGHT_GAIN as isize,
    IrisHighlightThreshold =
        AEGP_LayerStream_IRIS_HIGHLIGHT_THRESHOLD as isize,
    IrisHighlightSaturation =
        AEGP_LayerStream_IRIS_HIGHLIGHT_SATURATION as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectTyp_LIGHT
    LightFalloffType = AEGP_LayerStream_LIGHT_FALLOFF_TYPE as isize,
    LightFalloffStart = AEGP_LayerStream_LIGHT_FALLOFF_START as isize,
    LightFalloffDistance =
        AEGP_LayerStream_LIGHT_FALLOFF_DISTANCE as isize,

    // only valid for AEGP_ObjectType == AEGP_ObjectType_AV
    ReflactionIntensity =
        AEGP_LayerStream_REFLECTION_INTENSITY as isize,
    ReflactionSharpness =
        AEGP_LayerStream_REFLECTION_SHARPNESS as isize,
    ReflactionRolloff = AEGP_LayerStream_REFLECTION_ROLLOFF as isize,
    TransparencyCoeff = AEGP_LayerStream_TRANSPARENCY_COEFF as isize,
    TransparencyRolloff =
        AEGP_LayerStream_TRANSPARENCY_ROLLOFF as isize,
    IndexOfRefraction = AEGP_LayerStream_INDEX_OF_REFRACTION as isize,

    BevelStyle = AEGP_LayerStream_EXTRUSION_BEVEL_STYLE as isize,
    BevelDirection =
        AEGP_LayerStream_EXTRUSION_BEVEL_DIRECTION as isize,
    BevelDepth = AEGP_LayerStream_EXTRUSION_BEVEL_DEPTH as isize,
    ExtrusionHoleBeveDepth =
        AEGP_LayerStream_EXTRUSION_HOLE_BEVEL_DEPTH as isize,
    ExtrusionDepth = AEGP_LayerStream_EXTRUSION_DEPTH as isize,
    PlaneCurvature = AEGP_LayerStream_PLANE_CURVATURE as isize,
    PlaneSubdivision = AEGP_LayerStream_PLANE_SUBDIVISION as isize,
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
#[repr(C)]
pub enum TimeMode {
    LayerTime = ae_sys::AEGP_LTimeMode_LayerTime as isize,
    CompTime = ae_sys::AEGP_LTimeMode_CompTime as isize,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub enum StreamType {
    NoData = AEGP_StreamType_NO_DATA as isize,
    ThreeDSpatial = AEGP_StreamType_ThreeD_SPATIAL as isize,
    ThreeD = AEGP_StreamType_ThreeD as isize,
    TwoDSpatial = AEGP_StreamType_TwoD_SPATIAL as isize,
    TwoD = AEGP_StreamType_TwoD as isize,
    OneD = AEGP_StreamType_OneD as isize,
    Color = AEGP_StreamType_COLOR as isize,
    Arb = AEGP_StreamType_ARB as isize,
    Marker = AEGP_StreamType_MARKER as isize,
    LayerID = AEGP_StreamType_LAYER_ID as isize,
    MaskID = AEGP_StreamType_MASK_ID as isize,
    Mask = AEGP_StreamType_MASK as isize,
    TextDocument = AEGP_StreamType_TEXT_DOCUMENT as isize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union StreamValue {
    pub four_d: AEGP_FourDVal,
    pub three_d: AEGP_ThreeDVal,
    pub two_d: AEGP_TwoDVal,
    pub one_d: AEGP_OneDVal,
    pub color: AEGP_ColorVal,
    pub arb_handle: AEGP_ArbBlockVal,
    pub marker_ptr: AEGP_MarkerValP,
    pub layer_id: AEGP_LayerIDVal,
    pub mask_id: AEGP_MaskIDVal,
    pub mask: AEGP_MaskOutlineValH,
    pub text_document_handle: AEGP_TextDocumentH,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub enum LightType {
    None = AEGP_LightType_NONE as isize,
    Parallel = AEGP_LightType_PARALLEL as isize,
    Spot = AEGP_LightType_SPOT as isize,
    Point = AEGP_LightType_POINT as isize,
    Ambient = AEGP_LightType_AMBIENT as isize,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    IntoPrimitive,
    UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum ObjectType {
    None = AEGP_ObjectType_NONE,
    /// Includes all pre-AE 5.0 layer types (audio or video source,
    /// including adjustment layers).
    Av = AEGP_ObjectType_AV,
    Light = AEGP_ObjectType_LIGHT,
    Camera = AEGP_ObjectType_CAMERA,
    Text = AEGP_ObjectType_TEXT,
    Vector = AEGP_ObjectType_VECTOR,
    NumTypes = AEGP_ObjectType_NUM_TYPES,
}

#[allow(dead_code)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    IntoPrimitive,
    UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum FilmSizeUnits {
    None = ae_sys::AEGP_FilmSizeUnits_NONE as i32,
    Horizontal = ae_sys::AEGP_FilmSizeUnits_HORIZONTAL as i32,
    Vertical = ae_sys::AEGP_FilmSizeUnits_VERTICAL as i32,
    Diagonal = ae_sys::AEGP_FilmSizeUnits_DIAGONAL as i32,
}

#[allow(dead_code)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    IntoPrimitive,
    UnsafeFromPrimitive,
)]
#[repr(i32)]
pub enum CameraType {
    None = ae_sys::AEGP_CameraType_NONE as i32,
    Perspective = ae_sys::AEGP_CameraType_PERSPECTIVE as i32,
    Orthographic = ae_sys::AEGP_CameraType_ORTHOGRAPHIC as i32,
    NumTypes = ae_sys::AEGP_CameraType_NUM_TYPES as i32,
}

//define_handle_wrapper!(MemHandle, AEGP_MemHandle, mem_ptr);

define_suite!(
    MemorySuite,
    AEGP_MemorySuite1,
    kAEGPMemorySuite,
    kAEGPMemorySuiteVersion1
);

pub struct MemHandle<T: Copy> {
    ptr: *mut T,
    pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
    mem_handle: ae_sys::AEGP_MemHandle,
}

impl<T: Copy> MemHandle<T> {
    pub fn new(
        plugin_id: PluginID,
        name: &str,
        flags: MemFlag,
    ) -> Result<Self, Error> {
        let mut mem_handle: ae_sys::AEGP_MemHandle =
            std::ptr::null_mut();
        let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

        // The CString we construct here will be copied by Ae.
        #[allow(clippy::temporary_cstring_as_ptr)]
        match ae_acquire_suite_and_call_suite_fn!(
            pica_basic_suite_ptr,
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_NewMemHandle,
            // Arguments ----------
            plugin_id,
            CString::new(name).unwrap().as_ptr(),
            std::mem::size_of::<T>() as u32,
            flags as i32,
            &mut mem_handle,
        ) {
            Ok(()) => Ok(Self {
                ptr: std::ptr::null_mut(),
                pica_basic_suite_ptr,
                mem_handle,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn from_raw(mem_handle: ae_sys::AEGP_MemHandle) -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            pica_basic_suite_ptr: borrow_pica_basic_as_ptr(),
            mem_handle,
        }
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        assert!(!self.ptr.is_null());
        self.ptr as *mut T
    }

    pub fn as_ptr(&self) -> *const T {
        assert!(!self.ptr.is_null());
        self.ptr as *const T
    }

    pub fn get(&self) -> T {
        assert!(!self.ptr.is_null());
        unsafe { *(self.ptr) }
    }

    pub fn lock(&mut self) -> Result<&Self, Error> {
        match ae_acquire_suite_and_call_suite_fn!(
            (self.pica_basic_suite_ptr),
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_LockMemHandle,
            // Arguments ----------
            self.mem_handle,
            &mut self.ptr as *mut *mut _ as *mut *mut std::ffi::c_void
        ) {
            Ok(()) => Ok(self),
            Err(e) => Err(e),
        }
    }

    pub fn unlock(&mut self) -> Result<(), Error> {
        ae_acquire_suite_and_call_suite_fn!(
            self.pica_basic_suite_ptr,
            AEGP_MemorySuite1,
            kAEGPMemorySuite,
            kAEGPMemorySuiteVersion1,
            // Function -----------
            AEGP_UnlockMemHandle,
            // Arguments ----------
            self.mem_handle
        )
    }
}

impl<T: Copy> Drop for MemHandle<T> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.unlock();
    }
}

// FIXME: wrap this nicely or combine WorldHandle & WorldSuite into
// single World
define_handle_wrapper!(WorldHandle, AEGP_WorldH, world_ptr);

define_suite!(
    WorldSuite,
    AEGP_WorldSuite3,
    kAEGPWorldSuite,
    kAEGPWorldSuiteVersion3
);

impl WorldSuite {
    pub fn fill_out_pf_effect_world(
        &self,
        world: WorldHandle,
    ) -> Result<EffectWorld, Error> {
        let mut effect_world_boxed =
            Box::<ae_sys::PF_EffectWorld>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FillOutPFEffectWorld,
            world.as_ptr(),
            effect_world_boxed.as_mut_ptr()
        ) {
            Ok(()) => Ok(EffectWorld {
                effect_world_boxed: unsafe {
                    effect_world_boxed.assume_init()
                },
            }),
            Err(e) => Err(e),
        }
    }
}

define_handle_wrapper!(CompHandle, AEGP_CompH, comp_ptr);

define_suite!(
    CompSuite,
    AEGP_CompSuite11,
    kAEGPCompSuite,
    kAEGPCompSuiteVersion11
);

impl CompSuite {
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
            angle.as_mut_ptr() as *mut ae_sys::A_Ratio,
            phase.as_mut_ptr() as *mut ae_sys::A_Ratio,
        ) {
            Ok(()) => Ok(unsafe {
                (angle.assume_init(), phase.assume_init())
            }),
            Err(e) => Err(e),
        }
    }

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

    pub fn get_comp_flags(
        &self,
        comp_handle: CompHandle,
    ) -> Result<CompFlags, Error> {
        let mut comp_flags =
            std::mem::MaybeUninit::<CompFlags>::uninit();

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
    suite_ptr: *const ae_sys::AEGP_CompSuite11,
    comp_ptr: *const ae_sys::AEGP_CompH,
}

impl Comp {
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
            suite_ptr,
            comp_ptr,
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

#[derive(Copy, Clone, Debug, Hash)]
pub struct StreamReferenceHandle {
    stream_reference_ptr: ae_sys::AEGP_StreamRefH,
}

define_handle_wrapper!(LayerHandle, AEGP_LayerH, layer_ptr);

define_suite!(
    LayerSuite,
    AEGP_LayerSuite8,
    kAEGPLayerSuite,
    kAEGPLayerSuiteVersion8
);

impl LayerSuite {
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

    pub fn get_comp_layer_by_index(
        &self,
        comp_handle: CompHandle,
        layer_index: usize,
    ) -> Result<LayerHandle, Error> {
        let mut num_layers =
            MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCompLayerByIndex,
            comp_handle.as_ptr(),
            layer_index as i32,
            num_layers.as_mut_ptr()
        ) {
            Ok(()) => Ok(LayerHandle::from_raw(unsafe {
                num_layers.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

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
                unsafe {
                    U16CString::from_ptr_str(
                        MemHandle::<u16>::from_raw(
                            layer_name_mem_handle.assume_init(),
                        )
                        .lock()?
                        .as_ptr(),
                    )
                    .to_string_lossy()
                },
                unsafe {
                    U16CString::from_ptr_str(
                        MemHandle::<u16>::from_raw(
                            source_name_mem_handle.assume_init(),
                        )
                        .lock()?
                        .as_ptr(),
                    )
                    .to_string_lossy()
                },
            )),
            Err(e) => Err(e),
        }
    }

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

    pub fn get_layer_to_world_xform(
        &self,
        layer_handle: LayerHandle,
        time: Time,
    ) -> Result<Matrix4, Error> {
        let mut matrix = Box::<Matrix4>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerToWorldXform,
            layer_handle.as_ptr(),
            &time as *const _ as *const ae_sys::A_Time,
            matrix.as_mut_ptr() as *mut ae_sys::A_Matrix4,
        ) {
            Ok(()) => Ok(unsafe { *matrix.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
struct StreamValue2 {
    stream_ref_handle: AEGP_StreamRefH,
    value: StreamValue,
}

define_suite!(
    StreamSuite,
    AEGP_StreamSuite5,
    kAEGPStreamSuite,
    kAEGPStreamSuiteVersion5
);

impl StreamSuite {
    pub fn get_new_layer_stream(
        &self,
        plugin_id: PluginID,
        layer_handle: LayerHandle,
        stream_name: LayerStream,
    ) -> Result<StreamReferenceHandle, Error> {
        let mut stream_reference_ptr: ae_sys::AEGP_StreamRefH =
            std::ptr::null_mut();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewLayerStream,
            plugin_id,
            layer_handle.layer_ptr,
            stream_name as i32,
            &mut stream_reference_ptr
        ) {
            Ok(()) => Ok(StreamReferenceHandle {
                stream_reference_ptr,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_new_stream_value(
        &self,
        plugin_id: PluginID,
        stream_reference_handle: StreamReferenceHandle,
        time_mode: TimeMode,
        time: Time,
        sample_stream_pre_expression: bool,
    ) -> Result<StreamValue, Error> {
        let mut stream_value =
            std::mem::MaybeUninit::<StreamValue2>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetNewStreamValue,
            plugin_id,
            stream_reference_handle.stream_reference_ptr,
            time_mode as ae_sys::AEGP_LTimeMode,
            &time as *const _ as *const ae_sys::A_Time,
            sample_stream_pre_expression as u8,
            stream_value.as_mut_ptr() as *mut _,
        ) {
            Ok(()) => Ok(unsafe { stream_value.assume_init().value }),
            Err(e) => Err(e),
        }
    }

    pub fn get_layer_stream_value(
        &self,
        layer_handle: LayerHandle,
        stream: LayerStream,
        time_mode: TimeMode,
        time: Time,
        pre_expression: bool,
    ) -> Result<(StreamValue, StreamType), Error> {
        let mut stream_value =
            std::mem::MaybeUninit::<StreamValue>::uninit();
        let mut stream_type =
            std::mem::MaybeUninit::<StreamType>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetLayerStreamValue,
            layer_handle.as_ptr(),
            stream as i32,
            time_mode as ae_sys::AEGP_LTimeMode,
            &time as *const _ as *const ae_sys::A_Time,
            pre_expression as u8,
            stream_value.as_mut_ptr() as *mut _, /* as *mut ae_sys::AEGP_StreamVal2, */
            stream_type.as_mut_ptr() as *mut i32,
        ) {
            Ok(()) => Ok(unsafe {
                (stream_value.assume_init(), stream_type.assume_init())
            }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    CanvasSuite,
    AEGP_CanvasSuite8,
    kAEGPCanvasSuite,
    kAEGPCanvasSuiteVersion8
);

impl CanvasSuite {
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
            Ok(()) => Ok(CompHandle::from_raw(unsafe {
                comp_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }

    pub fn get_comp_render_time(
        &self,
        render_context_handle: pr::RenderContextHandle,
    ) -> Result<(Time, Time), Error> {
        let mut shutter_frame_start =
            std::mem::MaybeUninit::<Time>::uninit();

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
            Ok(()) => Ok(WorldHandle::from_raw(unsafe {
                world_ptr.assume_init()
            })),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    LightSuite,
    AEGP_LightSuite2,
    kAEGPLightSuite,
    kAEGPLightSuiteVersion2
);

impl LightSuite {
    pub fn get_light_type(
        &self,
        layer_handle: LayerHandle,
    ) -> Result<LightType, Error> {
        let mut light_type =
            std::mem::MaybeUninit::<LightType>::uninit();

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

define_handle_wrapper!(ItemHandle, AEGP_ItemH, item_ptr);

define_suite!(
    ItemSuite,
    AEGP_ItemSuite9,
    kAEGPItemSuite,
    kAEGPItemSuiteVersion9
);

impl ItemSuite {
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
            width.as_mut_ptr() as *mut i32,
            height.as_mut_ptr() as *mut i32
        ) {
            Ok(()) => Ok(unsafe {
                (width.assume_init(), height.assume_init())
            }),
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
    pub fn get_camera(
        &self,
        render_context_handle: pr::RenderContextHandle,
        time: Time,
    ) -> Result<LayerHandle, Error> {
        let mut layer_ptr =
            std::mem::MaybeUninit::<ae_sys::AEGP_LayerH>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetCamera,
            render_context_handle.as_ptr(),
            &time as *const _ as *const ae_sys::A_Time,
            layer_ptr.as_mut_ptr(),
        ) {
            Ok(()) => {
                // If the comp has no camera Ae will return a NULL
                // ptr instead of an error! We need to handle this
                // ourselves.
                let layer_ptr = unsafe { layer_ptr.assume_init() };
                if layer_ptr.is_null() {
                    Err(Error::Generic)
                } else {
                    Ok(LayerHandle::from_raw(layer_ptr))
                }
            }
            Err(e) => Err(e),
        }
    }

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
    pub fn from_raw(
        scene3d_layer_ptr: *const ae_sys::AEGP_Scene3DLayer,
    ) -> Self {
        Self { scene3d_layer_ptr }
    }

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
    pub fn new(
        in_data_handle: pr::InDataHandle,
        render_context: pr::RenderContextHandle,
        global_texture_cache_handle: aegp::Scene3DTextureCacheHandle,
    ) -> Result<Scene3D, Error> {
        let pica_basic_suite_ptr =
            in_data_handle.pica_basic_handle().as_ptr();

        let suite_ptr = ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            AEGP_Scene3DSuite2,
            kAEGPScene3DSuite,
            kAEGPScene3DSuiteVersion2
        )?;

        let mut scene3d_ptr: *mut ae_sys::AEGP_Scene3D =
            std::ptr::null_mut();

        ae_call_suite_fn!(
            suite_ptr,
            AEGP_Scene3DAlloc,
            &mut scene3d_ptr,
        )?;

        let mut texture_context_ptr: *mut ae_sys::AEGP_Scene3DTextureContext = std::ptr::null_mut();

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

    pub fn get_scene3d_ptr(&self) -> *mut ae_sys::AEGP_Scene3D {
        self.scene3d_ptr
    }

    pub fn get_scene3d_suite_ptr(
        &self,
    ) -> *const ae_sys::AEGP_Scene3DSuite2 {
        self.suite_ptr
    }

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

    pub fn num_sub_frame_times(&self) -> Result<usize, Error> {
        let mut num_motion_samples =
            std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DNumSubFrameTimes,
            self.scene3d_ptr,
            num_motion_samples.as_mut_ptr(),
        ) {
            Ok(()) => {
                Ok(unsafe { num_motion_samples.assume_init() } as usize)
            }
            Err(e) => Err(e),
        }
    }

    pub fn layer_get_post_xform(
        &self,
        layer_handle: &Scene3DLayerHandle,
        index: usize,
    ) -> Result<Matrix4, Error> {
        let mut matrix_ptr =
            std::mem::MaybeUninit::<*const Matrix4>::uninit();
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_Scene3DLayerGetPostXform,
            layer_handle.as_ptr(),
            index as i32,
            matrix_ptr.as_mut_ptr() as *mut *const _
        ) {
            Ok(()) => Ok({
                let mut matrix =
                    std::mem::MaybeUninit::<Matrix4>::uninit();
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

    pub fn get_sub_frame_time(
        &self,
        index: usize,
    ) -> Result<Time, Error> {
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
    pub fn new(
        scene3d: Scene3D,
    ) -> Result<Scene3DTextureCacheHandle, Error> {
        let mut texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache = std::ptr::null_mut();

        match ae_call_suite_fn!(
            scene3d.suite_ptr,
            AEGP_Scene3DTextureCacheAlloc,
            &mut texture_cache_ptr,
        ) {
            Ok(()) => {
                Ok(Scene3DTextureCacheHandle { texture_cache_ptr })
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_raw(
        texture_cache_ptr: *mut ae_sys::AEGP_Scene3DTextureCache,
    ) -> Scene3DTextureCacheHandle {
        Scene3DTextureCacheHandle { texture_cache_ptr }
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DMaterialHandle {
    material_ptr: *mut ae_sys::AEGP_Scene3DMaterial,
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DNodeHandle {
    node_ptr: ae_sys::AEGP_Scene3DNodeP,
}

impl Scene3DNodeHandle {
    pub fn new(
        node_ptr: ae_sys::AEGP_Scene3DNodeP,
    ) -> Scene3DNodeHandle {
        Scene3DNodeHandle { node_ptr }
    }

    pub fn as_ptr(self) -> ae_sys::AEGP_Scene3DNodeP {
        self.node_ptr
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct Scene3DMeshHandle {
    mesh_ptr: *mut ae_sys::AEGP_Scene3DMesh,
}

define_suite!(
    Scene3DMaterialSuite,
    AEGP_Scene3DMaterialSuite1,
    kAEGPScene3DMaterialSuite,
    kAEGPScene3DMaterialSuiteVersion1
);

impl Scene3DMaterialSuite {
    pub fn has_uv_color_texture(
        &self,
        material_handle: Scene3DMaterialHandle,
    ) -> Result<bool, Error> {
        let mut has_uv_color_texture: u8 = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_HasUVColorTexture,
            material_handle.material_ptr,
            &mut has_uv_color_texture
        ) {
            Ok(()) => Ok(has_uv_color_texture != 0),
            Err(e) => Err(e),
        }
    }

    pub fn get_uv_color_texture(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<WorldHandle, Error> {
        let mut world_handle = WorldHandle {
            world_ptr: std::ptr::null_mut(),
        };
        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetUVColorTexture,
            material.material_ptr,
            &mut world_handle.world_ptr
        ) {
            Ok(()) => Ok(world_handle),
            Err(e) => Err(e),
        }
    }

    pub fn get_basic_coeffs(
        &self,
        material: Scene3DMaterialHandle,
    ) -> Result<Box<ae_sys::AEGP_MaterialBasic_v1>, Error> {
        let mut basic_material_coefficients =
            Box::<ae_sys::AEGP_MaterialBasic_v1>::new_uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetBasicCoeffs,
            material.material_ptr,
            basic_material_coefficients.as_mut_ptr()
        ) {
            Ok(()) => {
                Ok(unsafe { basic_material_coefficients.assume_init() })
            }
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
    pub fn get_material_for_side(
        &self,
        node_handle: Scene3DNodeHandle,
        side: ae_sys::AEGP_Scene3DMaterialSide,
    ) -> Result<Scene3DMaterialHandle, Error> {
        let mut material_handle = Scene3DMaterialHandle {
            material_ptr: std::ptr::null_mut(),
        };

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialForSide,
            node_handle.node_ptr,
            side,
            &mut material_handle.material_ptr
        ) {
            Ok(()) => Ok(material_handle),
            Err(e) => Err(e),
        }
    }

    pub fn node_mesh_get(
        &self,
        node_handle: Scene3DNodeHandle,
    ) -> Result<Scene3DMeshHandle, Error> {
        let mut mesh_handle = Scene3DMeshHandle {
            mesh_ptr: std::ptr::null_mut(),
        };

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_NodeMeshGet,
            node_handle.node_ptr,
            &mut mesh_handle.mesh_ptr
        ) {
            Ok(()) => Ok(mesh_handle),
            Err(e) => Err(e),
        }
    }

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
    pub fn face_group_buffer_count(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<usize, Error> {
        let mut face_groups: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferCount,
            mesh_handle.mesh_ptr,
            &mut face_groups
        ) {
            Ok(()) => Ok(face_groups as usize),
            Err(e) => Err(e),
        }
    }

    pub fn face_group_buffer_size(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<usize, Error> {
        let mut face_count: ae_sys::A_long = 0;

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_FaceGroupBufferSize,
            mesh_handle.mesh_ptr,
            group_index as i32,
            &mut face_count
        ) {
            Ok(()) => Ok(face_count as usize),
            Err(e) => Err(e),
        }
    }

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
            mesh_handle.mesh_ptr,
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

    pub fn get_material_side_for_face_group(
        &self,
        mesh_handle: Scene3DMeshHandle,
        group_index: usize,
    ) -> Result<ae_sys::AEGP_Scene3DMaterialSide, Error> {
        let mut material_side = std::mem::MaybeUninit::<
            ae_sys::AEGP_Scene3DMaterialSide,
        >::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_GetMaterialSideForFaceGroup,
            mesh_handle.mesh_ptr,
            group_index as i32,
            material_side.as_mut_ptr()
        ) {
            Ok(()) => Ok(unsafe { material_side.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn mesh_get_info(
        &self,
        mesh_handle: Scene3DMeshHandle,
    ) -> Result<(usize, usize), Error> {
        let mut num_vertex = std::mem::MaybeUninit::<i32>::uninit();
        let mut num_face = std::mem::MaybeUninit::<i32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshGetInfo,
            mesh_handle.mesh_ptr,
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
            Vec<ae_sys::A_FpLong>,
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
            Vec::<ae_sys::A_long>::with_capacity(
                face_index_buffer_size,
            );

        // 2 uvs per vertex per face
        let uv_per_face_buffer_size: usize = num_face * 4 * 2;
        let mut uv_per_face_buffer =
            Vec::<ae_sys::A_FpLong>::with_capacity(
                uv_per_face_buffer_size,
            );

        match ae_call_suite_fn!(
            self.suite_ptr,
            AEGP_MeshFillBuffers,
            mesh_handle.mesh_ptr,
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

                Ok((
                    vertex_buffer,
                    face_index_buffer,
                    uv_per_face_buffer,
                ))
            }
            Err(e) => Err(e),
        }
    }
}
