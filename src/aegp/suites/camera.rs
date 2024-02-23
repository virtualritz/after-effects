use crate::*;
use crate::aegp::*;

define_suite!(
    /// Obtains the camera geometry, including camera properties (type, lens, depth of field, focal distance, aperture, et cetera).
    ///
    /// # Notes Regarding Camera Behavior
    /// Camera orientation is in composition coordinates, and the rotations are in layer (the camera's layer) coordinates.
    ///
    /// If the camera layer has a parent, the position is in a coordinate space relative to the parent.
    ///
    /// # Orthographic Camera Matrix
    /// Internally, we use composition width and height to set the matrix described by the OpenGL specification as
    /// ```
    ///     glOrtho(-width/2, width/2, -height/2, height/2, -1, 100);
    /// ```
    /// The orthographic matrix describes the projection. The position of the camera is described by another, scaled matrix. The inverse of the camera position matrix provides the "eye" coordinates.
    ///
    /// # Focus On Focal
    /// Remember, focal length affects field of view; focal distance only affects depth of field.
    ///
    /// # Film Size
    /// In the real world, film size is measured in millimeters. In After Effects, it's measured in pixels. Multiply by 72 and divide by 25.4 to move from millimeters to pixels.
    ///
    /// Field of view is more complex;
    ///
    /// * ϴ = 1/2 field of view
    /// * tan(ϴ) = 1/2 composition height / focal length
    /// * focal length = 2 tan(ϴ) / composition height
    CameraSuite,
    AEGP_CameraSuite2,
    kAEGPCameraSuite,
    kAEGPCameraSuiteVersion2
);

impl CameraSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Given a layer handle and time, returns the current camera layer handle.
    pub fn camera(&self, render_context_handle: pr::RenderContextHandle, time: Time) -> Result<LayerHandle, Error> {
        let camera_layer_handle = call_suite_fn_single!(self, AEGP_GetCamera -> ae_sys::AEGP_LayerH, render_context_handle.as_ptr(), &time as *const _ as *const ae_sys::A_Time)?;
        if camera_layer_handle.is_null() {
            Err(Error::Generic)
        } else {
            Ok(LayerHandle::from_raw(camera_layer_handle))
        }
    }

    /// Given a layer, returns the camera type of the layer.
    pub fn camera_type(&self, camera_layer_handle: LayerHandle) -> Result<CameraType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetCameraType -> ae_sys::AEGP_CameraType, camera_layer_handle.as_ptr())?.into())
    }

    /// Retrieves the size (and units used to measure that size) of the film used by the designated camera.
    pub fn camera_film_size(&self, camera_layer_handle: LayerHandle) -> Result<(FilmSizeUnits, f64), Error> {
        let mut film_size_units: ae_sys::AEGP_FilmSizeUnits = 0;
        let mut film_size: ae_sys::A_FpLong = 0.0;

        call_suite_fn!(self, AEGP_GetCameraFilmSize, camera_layer_handle.as_ptr(), &mut film_size_units, &mut film_size)?;

        Ok((film_size_units.into(), film_size))
    }

    /// Sets the size (and unites used to measure that size) of the film used by the designated camera.
    pub fn set_camera_film_size(&self, camera_layer_handle: LayerHandle, film_size_units: FilmSizeUnits, mut film_size: f64) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetCameraFilmSize, camera_layer_handle.as_ptr(), film_size_units.into(), &mut film_size)
    }

    /// Given a composition handle, returns the camera distance to the image plane.
    pub fn default_camera_distance_to_image_plane(&self, comp_handle: CompHandle) -> Result<f64, Error> {
        call_suite_fn_single!(self, AEGP_GetDefaultCameraDistanceToImagePlane -> f64, comp_handle.as_ptr())
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_FilmSizeUnits,
    FilmSizeUnits {
        None       = ae_sys::AEGP_FilmSizeUnits_NONE,
        Horizontal = ae_sys::AEGP_FilmSizeUnits_HORIZONTAL,
        Vertical   = ae_sys::AEGP_FilmSizeUnits_VERTICAL,
        Diagonal   = ae_sys::AEGP_FilmSizeUnits_DIAGONAL,
    }
}

define_enum! {
    ae_sys::AEGP_CameraType,
    CameraType {
        None         = ae_sys::AEGP_CameraType_NONE,
        Perspective  = ae_sys::AEGP_CameraType_PERSPECTIVE,
        Orthographic = ae_sys::AEGP_CameraType_ORTHOGRAPHIC,
        NumTypes     = ae_sys::AEGP_CameraType_NUM_TYPES,
    }
}

define_suite_item_wrapper!(
    ae_sys::AEGP_LayerH, LayerHandle,
    suite: CameraSuite,
    /// Obtains the camera geometry, including camera properties (type, lens, depth of field, focal distance, aperture, et cetera).
    ///
    /// # Notes Regarding Camera Behavior
    /// Camera orientation is in composition coordinates, and the rotations are in layer (the camera's layer) coordinates.
    ///
    /// If the camera layer has a parent, the position is in a coordinate space relative to the parent.
    ///
    /// # Orthographic Camera Matrix
    /// Internally, we use composition width and height to set the matrix described by the OpenGL specification as
    /// ```
    ///     glOrtho(-width/2, width/2, -height/2, height/2, -1, 100);
    /// ```
    /// The orthographic matrix describes the projection. The position of the camera is described by another, scaled matrix. The inverse of the camera position matrix provides the "eye" coordinates.
    ///
    /// # Focus On Focal
    /// Remember, focal length affects field of view; focal distance only affects depth of field.
    ///
    /// # Film Size
    /// In the real world, film size is measured in millimeters. In After Effects, it's measured in pixels. Multiply by 72 and divide by 25.4 to move from millimeters to pixels.
    ///
    /// Field of view is more complex;
    ///
    /// * ϴ = 1/2 field of view
    /// * tan(ϴ) = 1/2 composition height / focal length
    /// * focal length = 2 tan(ϴ) / composition height
    Camera {
        dispose: ;

        /// Returns the camera type
        r#type() -> CameraType => suite.camera_type,

        /// Retrieves the size (and units used to measure that size) of the film used by the camera.
        film_size() -> (FilmSizeUnits, f64) => suite.camera_film_size,

        /// Sets the size (and unites used to measure that size) of the film used by the designated camera.
        set_film_size(film_size_units: FilmSizeUnits, film_size: f64) -> () => suite.set_camera_film_size,
    }
);

impl Camera {
    pub fn from_render_context(render_context_handle: pr::RenderContextHandle, time: Time) -> Result<Self, Error> {
        let suite = CameraSuite::new()?;
        let handle = suite.camera(render_context_handle, time)?;
        Ok(Self {
            suite,
            handle,
            is_owned: false
        })
    }
}
