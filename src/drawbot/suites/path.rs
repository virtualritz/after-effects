use crate::*;

define_suite!(
    /// Calls to draw paths.
    PathSuite,
    DRAWBOT_PathSuite1,
    kDRAWBOT_PathSuite,
    kDRAWBOT_PathSuite_Version1
);

// +--------------+---------------------------------------------------------+
// | **Function** |                       **Purpose**                       |
// +==============+=========================================================+
// | ``MoveTo``   | Move to a point.                                        |
// +--------------+---------------------------------------------------------+
// | ``LineTo``   | Add a line to the path.                                 |
// +--------------+---------------------------------------------------------+
// | ``BezierTo`` | Add a cubic bezier to the path.                         |
// +--------------+---------------------------------------------------------+
// | ``AddRect``  | Add a rect to the path.                                 |
// +--------------+---------------------------------------------------------+
// | ``AddArc``   | Add a arc to the path. Zero start degrees == 3 o'clock. |
// |              | Sweep is clockwise. Units for angle are in degrees.     |
// +--------------+---------------------------------------------------------+
// | ``Close``    | Close the path.                                         |
// +--------------+---------------------------------------------------------+
// fn MoveTo(in_path_ref: DRAWBOT_PathRef, in_x: f32, in_y: f32) -> SPErr,
// fn LineTo(in_path_ref: DRAWBOT_PathRef, in_x: f32, in_y: f32) -> SPErr,
// fn BezierTo(in_path_ref: DRAWBOT_PathRef,in_pt1P: *const DRAWBOT_PointF32,in_pt2P: *const DRAWBOT_PointF32,in_pt3P: *const DRAWBOT_PointF32) -> SPErr,
// fn AddRect(in_path_ref: DRAWBOT_PathRef,in_rectPR: *const DRAWBOT_RectF32) -> SPErr,
// fn AddArc(in_path_ref: DRAWBOT_PathRef,in_centerP: *const DRAWBOT_PointF32,in_radius: f32,in_start_angle: f32,in_sweep: f32) -> SPErr,
// fn Close: ::std::option::Option<unsafe extern "C" fn(in_path_ref: DRAWBOT_PathRef) -> SPErr>,
