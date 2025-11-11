use crate::*;
use bitflags::bitflags;

bitflags! {
    pub struct EventInFlags: ae_sys::A_long {
        const NONE       = ae_sys::PF_EI_NONE      as ae_sys::A_long;
        const DONT_DRAW  = ae_sys::PF_EI_DONT_DRAW as ae_sys::A_long;
    }
}

bitflags! {
    pub struct EventOutFlags: ae_sys::A_long {
        const NONE          = ae_sys::PF_EO_NONE          as ae_sys::A_long;
        const HANDLED_EVENT = ae_sys::PF_EO_HANDLED_EVENT as ae_sys::A_long;
        // Rerender the comp.
        const ALWAYS_UPDATE = ae_sys::PF_EO_ALWAYS_UPDATE as ae_sys::A_long;
        // Do not rerender the comp.
        const NEVER_UPDATE  = ae_sys::PF_EO_NEVER_UPDATE  as ae_sys::A_long;
        // Update the view immediately after the event returns when using pf::InvalidateRect.
        const UPDATE_NOW    = ae_sys::PF_EO_UPDATE_NOW    as ae_sys::A_long;
    }
}

define_struct_wrapper!(ClickEventInfo, PF_DoClickEventInfo);

impl ClickEventInfo {
    pub fn screen_point(&self) -> Point {
        self.as_ref().screen_point.into()
    }

    pub fn num_clicks(&self) -> u32 {
        self.as_ref().num_clicks as _
    }
}

impl std::fmt::Debug for ClickEventInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClickEventInfo")
    }
}

define_struct_wrapper!(DrawEventInfo, PF_DrawEventInfo);

impl std::fmt::Debug for DrawEventInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DrawEventInfo")
    }
}

define_struct_wrapper!(KeyDownEventInfo, PF_KeyDownEvent);

impl std::fmt::Debug for KeyDownEventInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("KeyDownEventInfo")
    }
}

define_struct_wrapper!(AdjustCursorEventInfo, PF_AdjustCursorEventInfo);

impl std::fmt::Debug for AdjustCursorEventInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AdjustCursorEventInfo")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    None,
    NewContext,
    Activate,
    Click(ClickEventInfo),
    Drag(ClickEventInfo),
    Draw(DrawEventInfo),
    Deactivate,
    CloseContext,
    Idle,
    AdjustCursor(AdjustCursorEventInfo), // Sent when mouse moves over custom UI.
    Keydown(KeyDownEventInfo),           // Sends keycodes or unicode characters.
    MouseExited,                         // Notification that the mouse is no longer over a specific view (layer or comp only).
}

define_struct_wrapper!(EventExtra, PF_EventExtra);

impl std::fmt::Debug for EventExtra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("EventExtra");
        dbg.field("event", &self.event());
        dbg.field("window_type", &self.window_type());
        dbg.field("param_index", &self.param_index());
        dbg.field("effect_area", &self.effect_area());
        dbg.field("current_frame", &self.current_frame());
        dbg.field("param_title_frame", &self.param_title_frame());
        dbg.field("horiz_offset", &self.horiz_offset());
        if [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG, ae_sys::PF_Event_ADJUST_CURSOR].contains(&self.as_ref().e_type) {
            dbg.field("modifiers", &self.modifiers());
        }
        if [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type) {
            dbg.field("last_time", &self.last_time());
        }
        dbg.finish()
    }
}

define_enum! {
    ae_sys::PF_EffectArea,
    EffectArea {
        Mone    = ae_sys::PF_EA_NONE,
        Title   = ae_sys::PF_EA_PARAM_TITLE,
        Control = ae_sys::PF_EA_CONTROL,
    }
}

define_enum! {
    ae_sys::PF_WindowType,
    WindowType {
        Comp   = ae_sys::PF_Window_COMP,
        Layer  = ae_sys::PF_Window_LAYER,
        Effect = ae_sys::PF_Window_EFFECT,
    }
}

impl EventExtra {
    pub fn context_handle(&self) -> ContextHandle {
        ContextHandle::from_raw(self.as_ref().contextH)
    }

    pub fn event(&self) -> Event {
        match self.as_ref().e_type {
            ae_sys::PF_Event_NEW_CONTEXT => Event::NewContext,
            ae_sys::PF_Event_ACTIVATE => Event::Activate,
            ae_sys::PF_Event_DO_CLICK => {
                // SAFETY: Accessing union field do_click based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_DO_CLICK guarantees do_click variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra owned by AE SDK,
                // (3) creating mutable pointer from const reference is safe as from_raw only reads the pointer.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                Event::Click(ClickEventInfo::from_raw(unsafe { &self.as_ref().u.do_click as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_DRAG => {
                // SAFETY: Accessing union field do_click based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_DRAG guarantees do_click variant is active (shared with DO_CLICK),
                // (2) self.as_ref() returns valid reference to PF_EventExtra owned by AE SDK,
                // (3) creating mutable pointer from const reference is safe as from_raw only reads the pointer.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                Event::Drag(ClickEventInfo::from_raw(unsafe { &self.as_ref().u.do_click as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_DRAW => {
                // SAFETY: Accessing union field draw based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_DRAW guarantees draw variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra owned by AE SDK,
                // (3) creating mutable pointer from const reference is safe as from_raw only reads the pointer.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                Event::Draw(DrawEventInfo::from_raw(unsafe { &self.as_ref().u.draw as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_DEACTIVATE => Event::Deactivate,
            ae_sys::PF_Event_CLOSE_CONTEXT => Event::CloseContext,
            ae_sys::PF_Event_IDLE => Event::Idle,
            ae_sys::PF_Event_ADJUST_CURSOR => {
                // SAFETY: Accessing union field adjust_cursor based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_ADJUST_CURSOR guarantees adjust_cursor variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra owned by AE SDK,
                // (3) creating mutable pointer from const reference is safe as from_raw only reads the pointer.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                Event::AdjustCursor(AdjustCursorEventInfo::from_raw(unsafe {
                    &self.as_ref().u.adjust_cursor as *const _ as *mut _
                }))
            }
            ae_sys::PF_Event_KEYDOWN => {
                // SAFETY: Accessing union field key_down based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_KEYDOWN guarantees key_down variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra owned by AE SDK,
                // (3) creating mutable pointer from const reference is safe as from_raw only reads the pointer.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                Event::Keydown(KeyDownEventInfo::from_raw(unsafe { &self.as_ref().u.key_down as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_MOUSE_EXITED => Event::MouseExited,
            _ => unreachable!(),
        }
    }

    pub fn set_cursor(&mut self, cursor: CursorType) {
        self.as_mut().u.adjust_cursor.set_cursor = cursor.into();
    }

    pub fn window_type(&self) -> WindowType {
        // SAFETY: Double pointer dereference of AE SDK context handle.
        // Detailed explanation: (1) contextH is a valid PF_ContextH pointer provided by AE SDK,
        // (2) the first dereference yields a valid pointer to PF_Context,
        // (3) the second dereference accesses the PF_Context structure which remains valid for the lifetime of the event.
        // Would be UB if: contextH is null or points to freed/invalid memory, or if PF_Context has been deallocated.
        unsafe { **self.as_ref().contextH }.w_type.into()
    }

    pub fn set_event_out_flags(&mut self, flags: EventOutFlags) {
        self.as_mut().evt_out_flags = flags.bits() as _;
    }

    pub fn param_index(&self) -> usize {
        self.as_ref().effect_win.index as _
    }

    pub fn effect_area(&self) -> EffectArea {
        self.as_ref().effect_win.area.into()
    }

    pub fn current_frame(&self) -> Rect {
        self.as_ref().effect_win.current_frame.into()
    }

    pub fn param_title_frame(&self) -> Rect {
        self.as_ref().effect_win.param_title_frame.into()
    }

    pub fn horiz_offset(&self) -> i32 {
        self.as_ref().effect_win.horiz_offset
    }

    pub fn modifiers(&self) -> Modifiers {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG, ae_sys::PF_Event_ADJUST_CURSOR].contains(&self.as_ref().e_type),
            "The modifiers() method is only valid if event() is Click, Drag or AdjustCursor."
        );
        if self.as_ref().e_type == ae_sys::PF_Event_ADJUST_CURSOR {
            // SAFETY: Accessing union field adjust_cursor based on discriminant e_type.
            // Detailed explanation: (1) e_type == PF_Event_ADJUST_CURSOR guarantees adjust_cursor variant is active,
            // (2) self.as_ref() returns valid reference to PF_EventExtra,
            // (3) reading modifiers field directly from the active union variant.
            // Would be UB if: e_type discriminant doesn't match the active union variant.
            return unsafe { Modifiers::from_bits_truncate(self.as_ref().u.adjust_cursor.modifiers as _) }
        }

        // SAFETY: Accessing union field do_click based on debug_assert check.
        // Detailed explanation: (1) debug_assert verifies e_type is DO_CLICK or DRAG (both use do_click variant),
        // (2) ADJUST_CURSOR case already handled above,
        // (3) reading modifiers field directly from the active union variant.
        // Would be UB if: e_type is not DO_CLICK, DRAG, or ADJUST_CURSOR, or if debug assertions are disabled and invariant is violated.
        unsafe { Modifiers::from_bits_truncate(self.as_ref().u.do_click.modifiers as _) }
    }

    pub fn set_continue_refcon(&mut self, index: usize, value: ae_sys::A_intptr_t) {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The continue_refcon() method is only valid if event() is Click or Drag."
        );
        debug_assert!(index < 4);
        // SAFETY: Accessing and mutating union field do_click array element.
        // Detailed explanation: (1) debug_assert verifies e_type is DO_CLICK or DRAG (both use do_click variant),
        // (2) debug_assert verifies index < 4 ensuring bounds safety for continue_refcon array,
        // (3) self.as_mut() returns valid mutable reference to PF_EventExtra.
        // Would be UB if: e_type is not DO_CLICK or DRAG, or if index >= 4 and bounds check is disabled.
        unsafe {
            self.as_mut().u.do_click.continue_refcon[index] = value;
        }
    }

    pub fn continue_refcon(&self, index: usize) -> ae_sys::A_intptr_t {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The get_continue_refcon() method is only valid if event() is Click or Drag."
        );
        debug_assert!(index < 4);
        // SAFETY: Accessing union field do_click array element.
        // Detailed explanation: (1) debug_assert verifies e_type is DO_CLICK or DRAG (both use do_click variant),
        // (2) debug_assert verifies index < 4 ensuring bounds safety for continue_refcon array,
        // (3) self.as_ref() returns valid reference to PF_EventExtra.
        // Would be UB if: e_type is not DO_CLICK or DRAG, or if index >= 4 and bounds check is disabled.
        unsafe { self.as_ref().u.do_click.continue_refcon[index] }
    }

    pub fn send_drag(&mut self) -> bool {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The send_drag() method is only valid if event() is Click or Drag."
        );
        // SAFETY: Accessing union field do_click for reading send_drag.
        // Detailed explanation: (1) debug_assert verifies e_type is DO_CLICK or DRAG (both use do_click variant),
        // (2) self.as_mut() returns valid mutable reference to PF_EventExtra (mutability for method signature consistency),
        // (3) reading send_drag field directly from the active union variant.
        // Would be UB if: e_type is not DO_CLICK or DRAG and debug assertions are disabled.
        unsafe { self.as_mut().u.do_click.send_drag != 0 }
    }

    pub fn set_send_drag(&mut self, send: bool) {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The send_drag() method is only valid if event() is Click or Drag."
        );
        self.as_mut().u.do_click.send_drag = send as _;
    }

    pub fn last_time(&self) -> bool {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The last_time() method is only valid if event() is Click or Drag."
        );
        // SAFETY: Accessing union field do_click for reading last_time.
        // Detailed explanation: (1) debug_assert verifies e_type is DO_CLICK or DRAG (both use do_click variant),
        // (2) self.as_ref() returns valid reference to PF_EventExtra,
        // (3) reading last_time field directly from the active union variant.
        // Would be UB if: e_type is not DO_CLICK or DRAG and debug assertions are disabled.
        unsafe { self.as_ref().u.do_click.last_time != 0 }
    }

    pub fn screen_point(&self) -> Point {
        match self.as_ref().e_type {
            ae_sys::PF_Event_DO_CLICK | ae_sys::PF_Event_DRAG => {
                // SAFETY: Accessing union field do_click based on discriminant e_type.
                // Detailed explanation: (1) e_type is DO_CLICK or DRAG (both use do_click variant),
                // (2) self.as_ref() returns valid reference to PF_EventExtra,
                // (3) reading screen_point field directly from the active union variant.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                unsafe { self.as_ref().u.do_click.screen_point.into() }
            }
            ae_sys::PF_Event_ADJUST_CURSOR => {
                // SAFETY: Accessing union field adjust_cursor based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_ADJUST_CURSOR guarantees adjust_cursor variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra,
                // (3) reading screen_point field directly from the active union variant.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                unsafe { self.as_ref().u.adjust_cursor.screen_point.into() }
            }
            ae_sys::PF_Event_KEYDOWN => {
                // SAFETY: Accessing union field key_down based on discriminant e_type.
                // Detailed explanation: (1) e_type == PF_Event_KEYDOWN guarantees key_down variant is active,
                // (2) self.as_ref() returns valid reference to PF_EventExtra,
                // (3) reading screen_point field directly from the active union variant.
                // Would be UB if: e_type discriminant doesn't match the active union variant.
                unsafe { self.as_ref().u.key_down.screen_point.into() }
            }
            _ => {
                panic!("The screen_point() method is only valid if event() is Click, Drag, AdjustCursor or Keydown.")
            }
        }
    }

    pub fn in_flags(&self) -> EventInFlags {
        EventInFlags::from_bits_truncate(self.as_ref().evt_in_flags as _)
    }

    pub fn callbacks(&self) -> EventCallbacks<'_> {
        EventCallbacks {
            ptr: &self.as_ref().cbs,
            ctx: self.as_ref().contextH
        }
    }
}

pub struct EventCallbacks<'a> {
    ptr: &'a ae_sys::PF_EventCallbacks,
    ctx: ae_sys::PF_ContextH
}

impl<'a> EventCallbacks<'a> {
    pub fn layer_to_comp(&self, curr_time: i32, time_scale: u32, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access layer_to_comp function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable point reference,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, layer_to_comp is null, ctx is invalid, or pt points to invalid memory.
        let ret = unsafe {
            ((*self.ptr).layer_to_comp.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn comp_to_layer(&self, curr_time: i32, time_scale: u32, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access comp_to_layer function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable point reference,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, comp_to_layer is null, ctx is invalid, or pt points to invalid memory.
        let ret = unsafe {
            ((*self.ptr).comp_to_layer.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn source_to_frame(&self, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access source_to_frame function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable point reference,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, source_to_frame is null, ctx is invalid, or pt points to invalid memory.
        let ret = unsafe {
            ((*self.ptr).source_to_frame.unwrap())((*self.ptr).refcon, self.ctx, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn frame_to_source(&self, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access frame_to_source function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable point reference,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, frame_to_source is null, ctx is invalid, or pt points to invalid memory.
        let ret = unsafe {
            ((*self.ptr).frame_to_source.unwrap())((*self.ptr).refcon, self.ctx, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn comp2layer_xform(&self, curr_time: i32, time_scale: u32) -> Result<Option<Matrix3>, Error> {
        let mut exists: ae_sys::A_long = 0;
        // SAFETY: Zero-initializing PF_FloatMatrix structure.
        // Detailed explanation: (1) PF_FloatMatrix is a POD (Plain Old Data) struct of floats,
        // (2) zeroed float values (0.0) are valid and represent a zero matrix,
        // (3) the matrix will be properly initialized by the subsequent AE SDK callback.
        // Would be UB if: PF_FloatMatrix contained non-POD types or had validity constraints on zero values.
        let mut matrix: ae_sys::PF_FloatMatrix = unsafe { std::mem::zeroed() };
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access get_comp2layer_xform function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable references to exists and matrix,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, get_comp2layer_xform is null, ctx is invalid, or exists/matrix point to invalid memory.
        let ret = unsafe {
            ((*self.ptr).get_comp2layer_xform.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, &mut exists, &mut matrix)
        };
        match ret {
            // SAFETY: Transmuting PF_FloatMatrix to Matrix3.
            // Detailed explanation: (1) both types have identical memory layout (3x3 float arrays),
            // (2) Matrix3 is a repr(C) or transparent wrapper around the same data structure,
            // (3) AE SDK has successfully populated the matrix with valid float values.
            // Would be UB if: PF_FloatMatrix and Matrix3 have different memory layouts or alignment requirements.
            0 => Ok(if exists == 1 { Some(unsafe { std::mem::transmute(matrix) }) } else { None }),
            e => Err(Error::from(e))
        }
    }

    pub fn layer2comp_xform(&self, curr_time: i32, time_scale: u32) -> Result<Matrix3, Error> {
        // SAFETY: Zero-initializing PF_FloatMatrix structure.
        // Detailed explanation: (1) PF_FloatMatrix is a POD (Plain Old Data) struct of floats,
        // (2) zeroed float values (0.0) are valid and represent a zero matrix,
        // (3) the matrix will be properly initialized by the subsequent AE SDK callback.
        // Would be UB if: PF_FloatMatrix contained non-POD types or had validity constraints on zero values.
        let mut matrix: ae_sys::PF_FloatMatrix = unsafe { std::mem::zeroed() };
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access get_layer2comp_xform function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon, context handle, and mutable reference to matrix,
        // (4) all parameters have correct types per AE SDK contract.
        // Would be UB if: self.ptr is invalid, get_layer2comp_xform is null, ctx is invalid, or matrix points to invalid memory.
        let ret = unsafe {
            ((*self.ptr).get_layer2comp_xform.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, &mut matrix)
        };
        match ret {
            // SAFETY: Transmuting PF_FloatMatrix to Matrix3.
            // Detailed explanation: (1) both types have identical memory layout (3x3 float arrays),
            // (2) Matrix3 is a repr(C) or transparent wrapper around the same data structure,
            // (3) AE SDK has successfully populated the matrix with valid float values.
            // Would be UB if: PF_FloatMatrix and Matrix3 have different memory layouts or alignment requirements.
            0 => Ok(unsafe { std::mem::transmute(matrix) }),
            e => Err(Error::from(e))
        }
    }

    pub fn info_draw_color(&self, color: Pixel8) -> Result<(), Error> {
        // SAFETY: FFI call to AE SDK callback function.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access info_draw_color function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon and color value,
        // (4) Pixel8 is passed by value and has a valid representation per AE SDK contract.
        // Would be UB if: self.ptr is invalid, info_draw_color is null, or refcon is invalid.
        let ret = unsafe {
            ((*self.ptr).info_draw_color.unwrap())((*self.ptr).refcon, color)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn info_draw_text(&self, text1: &str, text2: &str) -> Result<(), Error> {
        let text1 = std::ffi::CString::new(text1).unwrap();
        let text2 = std::ffi::CString::new(text2).unwrap();
        // SAFETY: FFI call to AE SDK callback function with C string pointers.
        // Detailed explanation: (1) self.ptr is a valid reference to PF_EventCallbacks from AE SDK,
        // (2) dereferencing self.ptr to access info_draw_text function pointer which is guaranteed non-null by unwrap(),
        // (3) calling through function pointer with valid refcon and null-terminated C string pointers,
        // (4) CString::as_ptr() returns valid pointers to null-terminated strings that remain alive for the call duration.
        // Would be UB if: self.ptr is invalid, info_draw_text is null, refcon is invalid, or the C strings are deallocated during the call.
        let ret = unsafe {
            ((*self.ptr).info_draw_text.unwrap())((*self.ptr).refcon, text1.as_ptr(), text2.as_ptr())
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }
}
