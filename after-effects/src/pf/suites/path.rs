use crate::*;
use ae_sys::*;

define_suite!(
    /// This suite is used to identify and access the paths associated with the effect’s source layer.
    PathQuerySuite,
    PF_PathQuerySuite1,
    kPFPathQuerySuite,
    kPFPathQuerySuiteVersion1
);

impl PathQuerySuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the number of paths associated with the effect’s source layer.
    pub fn num_paths(&self, effect_ref: impl AsPtr<PF_ProgPtr>) -> Result<i32, Error> {
        call_suite_fn_single!(self, PF_NumPaths -> A_long, effect_ref.as_ptr())
    }

    /// Retrieves the `PF_PathID` for the specified path.
    pub fn path_info(&self, effect_ref: impl AsPtr<PF_ProgPtr>, index: i32) -> Result<PF_PathID, Error> {
        call_suite_fn_single!(self, PF_PathInfo -> PF_PathID, effect_ref.as_ptr(), index)
    }

    /// Acquires the [`PathOutline`] for the path at the specified time.
    /// `PathOutline` automatically calls `checkin_path` on drop.
    /// Note the result may be `None` even if `unique_id != PF_PathID_NONE` (the path may have been deleted).
    pub fn checkout_path(&self, effect_ref: impl AsPtr<PF_ProgPtr>, unique_id: PF_PathID, what_time: i32, time_step: i32, time_scale: u32) -> Result<Option<PathOutline>, Error> {
        let effect_ref = effect_ref.as_ptr();
        let path = call_suite_fn_single!(self, PF_CheckoutPath -> PF_PathOutlinePtr, effect_ref, unique_id, what_time, time_step, time_scale)?;
        Ok(PathOutline::from_raw(effect_ref, unique_id, path))
    }

    /// Releases the path back to After Effects. Always do this, regardless of any error conditions
    /// encountered. Every checkout must be balanced by a checkin, or pain will ensue. This function
    /// is automatically called in `PathOutline`'s `Drop` implementation.
    pub fn checkin_path(&self, effect_ref: impl AsPtr<PF_ProgPtr>, unique_id: PF_PathID, changed: bool, path: PF_PathOutlinePtr) -> Result<(), Error> {
        call_suite_fn!(self, PF_CheckinPath, effect_ref.as_ptr(), unique_id, changed as _, path)
    }
}

define_suite!(
    /// This suite provides information about paths (sequences of vertices).
    PathDataSuite,
    PF_PathDataSuite1,
    kPFPathDataSuite,
    kPFPathDataSuiteVersion1
);

impl PathDataSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns `true` if the path is not closed (if the beginning and end vertex are not identical).
    pub fn path_is_open(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_PathIsOpen -> PF_Boolean, effect_ref.as_ptr(), path)? != 0)
    }

    /// Retrieves the number of segments in the path. N segments means there are segments `[0.N-1]`;
    /// segment J is defined by vertex `J` and `J+1`.
    pub fn path_num_segments(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr) -> Result<i32, Error> {
        call_suite_fn_single!(self, PF_PathNumSegments -> A_long, effect_ref.as_ptr(), path)
    }

    /// Retrieves the `PF_PathVertex` for the specified path. The range of points is `[0.num_segments]`;
    /// for closed paths, `vertex[0] == vertex[num_segments]`.
    pub fn path_vertex_info(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, which_point: i32) -> Result<PF_PathVertex, Error> {
        call_suite_fn_single!(self, PF_PathVertexInfo -> PF_PathVertex, effect_ref.as_ptr(), path, which_point)
    }

    /// This fairly counter-intuitive function informs After Effects that you’re going to ask for the
    /// length of a segment (using `path_get_seg_length` below), and it’d better get ready. `frequency`
    /// indicates how many times you’d like us to sample the length; our internal effects use 100.
    pub fn path_prepare_seg_length(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, which_seg: i32, frequency: i32) -> Result<PF_PathSegPrepPtr, Error> {
        call_suite_fn_single!(self, PF_PathPrepareSegLength -> PF_PathSegPrepPtr, effect_ref.as_ptr(), path, which_seg, frequency)
    }

    /// Retrieves the length of the given segment.
    pub fn path_get_seg_length(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, which_seg: i32, length_prep: &mut PF_PathSegPrepPtr) -> Result<f64, Error> {
        call_suite_fn_single!(self, PF_PathGetSegLength -> PF_FpLong, effect_ref.as_ptr(), path, which_seg, length_prep)
    }

    /// Retrieves the location of a point `length` along the given path segment.
    /// 
    /// Returns a tuple containing `(x, y)`
    pub fn path_eval_seg_length(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, length_prep: &mut PF_PathSegPrepPtr, which_seg: i32, length: f64) -> Result<(f64, f64), Error> {
        call_suite_fn_double!(self, PF_PathEvalSegLength -> PF_FpLong, PF_FpLong, effect_ref.as_ptr(), path, length_prep, which_seg, length)
    }

    /// Retrieves the location, and the first derivative, of a point `length` along the given path segment.
    /// If you’re not sure why you’d ever need this, don’t use it. Math is hard.
    /// 
    /// Returns a tuple containing `(x, y, deriv1x, deriv1y)`
    pub fn path_eval_seg_length_deriv1(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, length_prep: &mut PF_PathSegPrepPtr, which_seg: i32, length: f64) -> Result<(f64, f64, f64, f64), Error> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut deriv1x = 0.0;
        let mut deriv1y = 0.0;
        call_suite_fn!(self, PF_PathEvalSegLengthDeriv1, effect_ref.as_ptr(), path, length_prep, which_seg, length, &mut x, &mut y, &mut deriv1x, &mut deriv1y)?;
        Ok((x, y, deriv1x, deriv1y))
    }

    /// Call this when you’re finished evaluating that segment length, so After Effects
    /// can properly clean up the `PF_PathSegPrepPtr`.
    pub fn path_cleanup_seg_length(&self, effect_ref: impl AsPtr<PF_ProgPtr>, path: PF_PathOutlinePtr, which_seg: i32, length_prep: &mut PF_PathSegPrepPtr) -> Result<(), Error> {
        call_suite_fn!(self, PF_PathCleanupSegLength, effect_ref.as_ptr(), path, which_seg, length_prep)
    }

    /// Returns `true` if the path is inverted.
    pub fn path_is_inverted(&self, effect_ref: impl AsPtr<PF_ProgPtr>, unique_id: PF_PathID) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, PF_PathIsInverted -> PF_Boolean, effect_ref.as_ptr(), unique_id)? != 0)
    }

    /// Retrieves the mode for the given path.
    pub fn path_get_mask_mode(&self, effect_ref: impl AsPtr<PF_ProgPtr>, unique_id: PF_PathID) -> Result<MaskMode, Error> {
        Ok(call_suite_fn_single!(self, PF_PathGetMaskMode -> PF_MaskMode, effect_ref.as_ptr(), unique_id)?.into())
    }

    /// Retrieves the name of the path.
    pub fn path_get_name(&self, effect_ref: impl AsPtr<PF_ProgPtr>, unique_id: PF_PathID) -> Result<String, Error> {
        let mut name = [0; PF_MAX_PATH_NAME_LEN as usize + 1];
        call_suite_fn!(self, PF_PathGetName, effect_ref.as_ptr(), unique_id, name.as_mut_ptr())?;
        Ok(unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }.to_string_lossy().into_owned())
    }
}

define_enum! {
    PF_MaskMode,
    MaskMode {
        None       = ae_sys::PF_MaskMode_NONE,
        Add        = ae_sys::PF_MaskMode_ADD,
        Subtract   = ae_sys::PF_MaskMode_SUBTRACT,
        Intersect  = ae_sys::PF_MaskMode_INTERSECT,
        Lighten    = ae_sys::PF_MaskMode_LIGHTEN,
        Darken     = ae_sys::PF_MaskMode_DARKEN,
        Difference = ae_sys::PF_MaskMode_DIFFERENCE,
        Accum      = ae_sys::PF_MaskMode_ACCUM,
    }
}

/// The path from a text layer, shape layer or mask.
pub struct PathOutline {
    effect_ref: PF_ProgPtr,
    unique_id: PF_PathID,
    path: PF_PathOutlinePtr,
}

impl std::fmt::Debug for PathOutline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn e(r: &Result<impl std::fmt::Debug, Error>) -> &dyn std::fmt::Debug {
            match r {
                Ok(x) => x,
                Err(e) => e,
            }
        }

        f.debug_struct("PathOutline")
            .field("name", e(&self.name()))
            .field("id", &self.id())
            .field("num_segments", e(&self.num_segments()))
            .field("is_open", e(&self.is_open()))
            .field("is_inverted", e(&self.is_inverted()))
            .field("mask_mode", e(&self.mask_mode()))
            .finish()
    }
}

impl AsPtr<PF_PathOutlinePtr> for PathOutline {
    fn as_ptr(&self) -> PF_PathOutlinePtr {
        self.path
    }
}

impl PathOutline {
    pub fn from_raw(effect_ref: PF_ProgPtr, unique_id: PF_PathID, path: PF_PathOutlinePtr) -> Option<Self> {
        if path.is_null() {
            None
        } else {
            Some(Self {
                effect_ref,
                unique_id,
                path
            })
        }
    }

    /// Returns the ID of the path.
    pub fn id(&self) -> PF_PathID {
        self.unique_id
    }

    /// Returns `true` if the path is not closed (if the beginning and end vertex are not identical).
    pub fn is_open(&self) -> Result<bool, Error> {
        PathDataSuite::new()?.path_is_open(self.effect_ref, self.path)
    }

    /// Retrieves the number of segments in the path. N segments means there are segments `[0.N-1]`;
    /// segment J is defined by vertex `J` and `J+1`.
    pub fn num_segments(&self) -> Result<i32, Error> {
        PathDataSuite::new()?.path_num_segments(self.effect_ref, self.path)
    }

    /// Retrieves the `PF_PathVertex` for the specified path. The range of points is `[0.num_segments]`;
    /// for closed paths, `vertex[0] == vertex[num_segments]`.
    pub fn vertex(&self, which_point: i32) -> Result<PF_PathVertex, Error> {
        PathDataSuite::new()?.path_vertex_info(self.effect_ref, self.path, which_point)
    }

    /// This fairly counter-intuitive function informs After Effects that you’re going to ask for the
    /// length of a segment (using [`PathSegPrep::length`]), and it’d better get ready. `frequency`
    /// indicates how many times you’d like us to sample the length; our internal effects use 100.
    pub fn prepare_seg_length(&self, which_seg: i32, frequency: i32) -> Result<PathSegPrep, Error> {
        Ok(PathSegPrep {
            path: self,
            which_seg,
            length_prep: PathDataSuite::new()?.path_prepare_seg_length(self.effect_ref, self.path, which_seg, frequency)?,
        })
    }

    /// Returns `true` if the path is inverted.
    pub fn is_inverted(&self) -> Result<bool, Error> {
        PathDataSuite::new()?.path_is_inverted(self.effect_ref, self.unique_id)
    }

    /// Retrieves the mode for the path.
    pub fn mask_mode(&self) -> Result<MaskMode, Error> {
        PathDataSuite::new()?.path_get_mask_mode(self.effect_ref, self.unique_id)
    }

    /// Retrieves the name of the path.
    pub fn name(&self) -> Result<String, Error> {
        PathDataSuite::new()?.path_get_name(self.effect_ref, self.unique_id)
    }
}

impl Drop for PathOutline {
    fn drop(&mut self) {
        PathQuerySuite::new()
            .expect("Failed to acquire PathQuerySuite")
            .checkin_path(self.effect_ref, self.unique_id, false, self.path)
            .expect("Failed to check in PF_PathOutlinePtr");
    }
}

/// Information pertaining to the length of a segment in a [`PathOutline`].
pub struct PathSegPrep<'a> {
    path: &'a PathOutline,
    which_seg: i32,
    length_prep: PF_PathSegPrepPtr,
}

impl<'a> PathSegPrep<'a> {
    /// Retrieves the length of the segment.
    pub fn length(&mut self) -> Result<f64, Error> {
        PathDataSuite::new()?.path_get_seg_length(self.path.effect_ref, self.path.path, self.which_seg, &mut self.length_prep)
    }

    /// Retrieves the location of a point `length` along the segment.
    /// 
    /// Returns a tuple containing `(x, y)`
    pub fn eval(&mut self, length: f64) -> Result<(f64, f64), Error> {
        PathDataSuite::new()?.path_eval_seg_length(self.path.effect_ref, self.path.path, &mut self.length_prep, self.which_seg, length)
    }

    /// Retrieves the location, and the first derivative, of a point `length` along the segment.
    /// If you’re not sure why you’d ever need this, don’t use it. Math is hard.
    /// 
    /// Returns a tuple containing `(x, y, deriv1x, deriv1y)`
    pub fn eval_deriv1(&mut self, length: f64) -> Result<(f64, f64, f64, f64), Error> {
        PathDataSuite::new()?.path_eval_seg_length_deriv1(self.path.effect_ref, self.path.path, &mut self.length_prep, self.which_seg, length)
    }
}

impl<'a> Drop for PathSegPrep<'a> {
    fn drop(&mut self) {
        PathDataSuite::new()
            .expect("Failed to acquire PathDataSuite")
            .path_cleanup_seg_length(self.path.effect_ref, self.path.path, self.which_seg, &mut self.length_prep)
            .expect("Failed to clean up PF_PathSegPrepPtr");
    }
}
