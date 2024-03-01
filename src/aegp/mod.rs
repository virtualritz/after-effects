use crate::ae_sys;

use std::{ convert::TryFrom, ffi::CString, marker::PhantomData };
use widestring::U16CString;

#[cfg(feature = "artisan-2-api")]
mod scene_3d;
#[cfg(feature = "artisan-2-api")]
pub use scene_3d::*;

pub mod suites {
    pub(crate) mod camera;         pub use camera        ::CameraSuite          as Camera;
    pub(crate) mod canvas;         pub use canvas        ::CanvasSuite          as Canvas;
    pub(crate) mod color_settings; pub use color_settings::ColorSettingsSuite   as ColorSettings;
    pub(crate) mod comp;           pub use comp          ::CompSuite            as Comp;
    pub(crate) mod composite;      pub use composite     ::CompositeSuite       as Composite;
    pub(crate) mod effect;         pub use effect        ::EffectSuite          as Effect;
    pub(crate) mod footage;        pub use footage       ::FootageSuite         as Footage;
    pub(crate) mod io_in;          pub use io_in         ::IOInSuite            as IOIn;
    pub(crate) mod item;           pub use item          ::ItemSuite            as Item;
    pub(crate) mod keyframe;       pub use keyframe      ::KeyframeSuite        as Keyframe;
    pub(crate) mod layer;          pub use layer         ::LayerSuite           as Layer;
    pub(crate) mod light;          pub use light         ::LightSuite           as Light;
    pub(crate) mod mask;           pub use mask          ::{ MaskSuite          as Mask,
                                                             MaskOutlineSuite   as MaskOutline };
    pub(crate) mod memory;         pub use memory        ::MemorySuite          as Memory;
    pub(crate) mod pf_interface;   pub use pf_interface  ::PFInterfaceSuite     as PFInterface;
    pub(crate) mod stream;         pub use stream        ::{ StreamSuite        as Stream,
                                                             DynamicStreamSuite as DynamicStream };
    pub(crate) mod utility;        pub use utility       ::UtilitySuite         as Utility;
    pub(crate) mod world;          pub use world         ::WorldSuite           as World;
}

pub type PluginId = ae_sys::AEGP_PluginID;
pub type ItemId = i32;
pub type LayerId = u32;

pub use suites::camera::{
    Camera,
    CameraType,
    FilmSizeUnits,
};
pub use suites::canvas::{
    Canvas,
    BinType,
    DisplayChannel,
    RenderHints,
    RenderLayerContextHandle,
    RenderNumEffects,
    RenderReceiptHandle,
    RenderReceiptStatus,
};
pub use suites::color_settings::{
    ColorProfileHandle,
    ConstColorProfileHandle,
    ItemViewHandle,
};
pub use suites::comp::{
    Composition,
    Collection2Handle,
    CompFlags,
    CompHandle,
};
pub use suites::effect::{
    EffectFlags,
    EffectRefHandle,
    InstalledEffectKey,
};
pub use suites::footage::{
    Footage,
    FootageHandle,
    FootageSignature,
    InterpretationStyle,
    Platform,
};
pub use suites::item::{
    Item,
    ItemFlags,
    ItemHandle,
    ItemType,
    LabelId,
    ProjectHandle,
};
pub use suites::keyframe::{
    AddKeyframesInfoHandle,
    KeyframeFlags,
    KeyframeInterpolation,
    KeyframeInterpolationMask,
};
pub use suites::layer::{
    Layer,
    LayerFlags,
    LayerHandle,
    LayerQuality,
    LayerSamplingQuality,
    ObjectType,
    TimeMode,
    TrackMatte,
};
pub use suites::light::LightType;
pub use suites::mask::{
    MaskFeatherFalloff,
    MaskFeatherInterp,
    MaskFeatherType,
    MaskMBlur,
    MaskMode,
    MaskOutlineHandle,
    MaskRefHandle,
};
pub use suites::memory::{
    MemHandle,
    MemHandleLock,
};
pub use suites::stream::{
    DynamicStreamFlags,
    LayerStream,
    MaskStream,
    StreamFlags,
    StreamGroupingType,
    StreamReferenceHandle,
    StreamType,
    StreamValue,
    TextDocumentHandle,
};
pub use suites::utility::GetPathTypes;
pub use suites::world::{
    PlatformWorldHandle,
    World,
    WorldHandle,
    WorldType,
};
