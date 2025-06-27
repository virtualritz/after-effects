use crate::*;

define_suite!(
    HelperSuite,
    PF_HelperSuite1,
    kPFHelperSuite,
    kPFHelperSuiteVersion
);

impl HelperSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn current_tool(&self) -> Result<SuiteTool, Error> {
        Ok(call_suite_fn_single!(self, PF_GetCurrentTool -> ae_sys::PF_SuiteTool)?.into())
    }
}

define_suite!(
    HelperSuite2,
    PF_HelperSuite2,
    kPFHelperSuite2,
    kPFHelperSuite2Version2
);

impl HelperSuite2 {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Causes After Effects to parse the clipboard immediately
    pub fn parse_clipboard(&self) -> Result<(), Error> {
        call_suite_fn!(self, PF_ParseClipboard,)
    }

    /// Sets the current [`ExtendedSuiteTool`].
    pub fn set_current_extended_tool(&self, tool: ExtendedSuiteTool) -> Result<(), Error> {
        call_suite_fn!(self, PF_SetCurrentExtendedTool, tool.into())
    }

    /// Returns the current [`ExtendedSuiteTool`].
    pub fn current_extended_tool(&self) -> Result<ExtendedSuiteTool, Error> {
        Ok(call_suite_fn_single!(self, PF_GetCurrentExtendedTool -> ae_sys::PF_ExtendedSuiteTool)?.into())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::PF_SuiteTool,
    SuiteTool {
        None          = ae_sys::PF_SuiteTool_NONE,
        Arrow         = ae_sys::PF_SuiteTool_ARROW,
        Rotate        = ae_sys::PF_SuiteTool_ROTATE,
        Shape         = ae_sys::PF_SuiteTool_SHAPE,
        Obsolete      = ae_sys::PF_SuiteTool_OBSOLETE,
        Pen           = ae_sys::PF_SuiteTool_PEN,
        Pan           = ae_sys::PF_SuiteTool_PAN,
        Hand          = ae_sys::PF_SuiteTool_HAND,
        Magnify       = ae_sys::PF_SuiteTool_MAGNIFY,
        RoundedRect   = ae_sys::PF_SuiteTool_ROUNDED_RECT,
        Polygon       = ae_sys::PF_SuiteTool_POLYGON,
        Star          = ae_sys::PF_SuiteTool_STAR,
        Pin           = ae_sys::PF_SuiteTool_PIN,
        PinStarch     = ae_sys::PF_SuiteTool_PIN_STARCH,
        PinDepth      = ae_sys::PF_SuiteTool_PIN_DEPTH,
    }
}
define_enum! {
    ae_sys::PF_ExtendedSuiteTool,
    ExtendedSuiteTool {
        None              = ae_sys::PF_ExtendedSuiteTool_NONE,
        Arrow             = ae_sys::PF_ExtendedSuiteTool_ARROW,
        Rotate            = ae_sys::PF_ExtendedSuiteTool_ROTATE,
        PenNormal         = ae_sys::PF_ExtendedSuiteTool_PEN_NORMAL,
        PenAddPoint       = ae_sys::PF_ExtendedSuiteTool_PEN_ADD_POINT,
        PenDeletePoint    = ae_sys::PF_ExtendedSuiteTool_PEN_DELETE_POINT,
        PenConvertPoint   = ae_sys::PF_ExtendedSuiteTool_PEN_CONVERT_POINT,
        Rect              = ae_sys::PF_ExtendedSuiteTool_RECT,
        Oval              = ae_sys::PF_ExtendedSuiteTool_OVAL,
        CameraOrbitCamera = ae_sys::PF_ExtendedSuiteTool_CAMERA_ORBIT_CAMERA,
        CameraPanCamera   = ae_sys::PF_ExtendedSuiteTool_CAMERA_PAN_CAMERA,
        CameraDollyCamera = ae_sys::PF_ExtendedSuiteTool_CAMERA_DOLLY_CAMERA,
        PanBehind         = ae_sys::PF_ExtendedSuiteTool_PAN_BEHIND,
        Hand              = ae_sys::PF_ExtendedSuiteTool_HAND,
        Magnify           = ae_sys::PF_ExtendedSuiteTool_MAGNIFY,
        Paintbrush        = ae_sys::PF_ExtendedSuiteTool_PAINTBRUSH,
        Pencil            = ae_sys::PF_ExtendedSuiteTool_PENCIL,
        CloneStamp        = ae_sys::PF_ExtendedSuiteTool_CLONE_STAMP,
        Eraser            = ae_sys::PF_ExtendedSuiteTool_ERASER,
        Text              = ae_sys::PF_ExtendedSuiteTool_TEXT,
        TextVertical      = ae_sys::PF_ExtendedSuiteTool_TEXT_VERTICAL,
        Pin               = ae_sys::PF_ExtendedSuiteTool_PIN,
        PinStarch         = ae_sys::PF_ExtendedSuiteTool_PIN_STARCH,
        PinDepth          = ae_sys::PF_ExtendedSuiteTool_PIN_DEPTH,
        RoundedRect       = ae_sys::PF_ExtendedSuiteTool_ROUNDED_RECT,
        Polygon           = ae_sys::PF_ExtendedSuiteTool_POLYGON,
        Star              = ae_sys::PF_ExtendedSuiteTool_STAR,
        QuickSelect       = ae_sys::PF_ExtendedSuiteTool_QUICKSELECT,
        CameraMaya        = ae_sys::PF_ExtendedSuiteTool_CAMERA_MAYA,
        Hairbrush         = ae_sys::PF_ExtendedSuiteTool_HAIRBRUSH,
        Feather           = ae_sys::PF_ExtendedSuiteTool_FEATHER,
        PinBend           = ae_sys::PF_ExtendedSuiteTool_PIN_BEND,
        PinAdvanced       = ae_sys::PF_ExtendedSuiteTool_PIN_ADVANCED,
        CameraOrbitCursor = ae_sys::PF_ExtendedSuiteTool_CAMERA_ORBIT_CURSOR,
        CameraOrbitScene  = ae_sys::PF_ExtendedSuiteTool_CAMERA_ORBIT_SCENE,
        CameraPanCursor   = ae_sys::PF_ExtendedSuiteTool_CAMERA_PAN_CURSOR,
        CameraDollyTowardsCursor = ae_sys::PF_ExtendedSuiteTool_CAMERA_DOLLY_TOWARDS_CURSOR,
        CameraDollyToCursor = ae_sys::PF_ExtendedSuiteTool_CAMERA_DOLLY_TO_CURSOR,
        ObjectSelect      = ae_sys::PF_ExtendedSuiteTool_OBJECTSELECT,
        Cube              = ae_sys::PF_ExtendedSuiteTool_CUBE,
        Sphere            = ae_sys::PF_ExtendedSuiteTool_SPHERE,
        Plane             = ae_sys::PF_ExtendedSuiteTool_PLANE,
        Torus             = ae_sys::PF_ExtendedSuiteTool_TORUS,
        Cone              = ae_sys::PF_ExtendedSuiteTool_CONE,
        Cylinder          = ae_sys::PF_ExtendedSuiteTool_CYLINDER,
    }
}
