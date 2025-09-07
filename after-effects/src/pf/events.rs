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
                Event::Click(ClickEventInfo::from_raw(unsafe { &self.as_ref().u.do_click as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_DRAG => {
                Event::Drag(ClickEventInfo::from_raw(unsafe { &self.as_ref().u.do_click as *const _ as *mut _ }))
            }
            ae_sys::PF_Event_DRAW => Event::Draw(DrawEventInfo::from_raw(unsafe { &self.as_ref().u.draw as *const _ as *mut _ })),
            ae_sys::PF_Event_DEACTIVATE => Event::Deactivate,
            ae_sys::PF_Event_CLOSE_CONTEXT => Event::CloseContext,
            ae_sys::PF_Event_IDLE => Event::Idle,
            ae_sys::PF_Event_ADJUST_CURSOR => {
                Event::AdjustCursor(AdjustCursorEventInfo::from_raw(unsafe {
                    &self.as_ref().u.adjust_cursor as *const _ as *mut _
                }))
            }
            ae_sys::PF_Event_KEYDOWN => {
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
            return unsafe { Modifiers::from_bits_truncate(self.as_ref().u.adjust_cursor.modifiers as _) }
        }

        unsafe { Modifiers::from_bits_truncate(self.as_ref().u.do_click.modifiers as _) }
    }

    pub fn set_continue_refcon(&mut self, index: usize, value: ae_sys::A_intptr_t) {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The continue_refcon() method is only valid if event() is Click or Drag."
        );
        debug_assert!(index < 4);
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
        unsafe { self.as_ref().u.do_click.continue_refcon[index] }
    }

    pub fn send_drag(&mut self) -> bool {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.as_ref().e_type),
            "The send_drag() method is only valid if event() is Click or Drag."
        );
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
        unsafe { self.as_ref().u.do_click.last_time != 0 }
    }

    pub fn screen_point(&self) -> Point {
        match self.as_ref().e_type {
            ae_sys::PF_Event_DO_CLICK | ae_sys::PF_Event_DRAG => {
                unsafe { self.as_ref().u.do_click.screen_point.into() }
            }
            ae_sys::PF_Event_ADJUST_CURSOR => {
                unsafe { self.as_ref().u.adjust_cursor.screen_point.into() }
            }
            ae_sys::PF_Event_KEYDOWN => {
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
        let ret = unsafe {
            ((*self.ptr).layer_to_comp.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn comp_to_layer(&self, curr_time: i32, time_scale: u32, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        let ret = unsafe {
            ((*self.ptr).comp_to_layer.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn source_to_frame(&self, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
        let ret = unsafe {
            ((*self.ptr).source_to_frame.unwrap())((*self.ptr).refcon, self.ctx, pt)
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }

    pub fn frame_to_source(&self, pt: &mut ae_sys::PF_FixedPoint) -> Result<(), Error> {
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
        let mut matrix: ae_sys::PF_FloatMatrix = unsafe { std::mem::zeroed() };
        let ret = unsafe {
            ((*self.ptr).get_comp2layer_xform.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, &mut exists, &mut matrix)
        };
        match ret {
            0 => Ok(if exists == 1 { Some(unsafe { std::mem::transmute(matrix) }) } else { None }),
            e => Err(Error::from(e))
        }
    }

    pub fn layer2comp_xform(&self, curr_time: i32, time_scale: u32) -> Result<Matrix3, Error> {
        let mut matrix: ae_sys::PF_FloatMatrix = unsafe { std::mem::zeroed() };
        let ret = unsafe {
            ((*self.ptr).get_layer2comp_xform.unwrap())((*self.ptr).refcon, self.ctx, curr_time, time_scale as _, &mut matrix)
        };
        match ret {
            0 => Ok(unsafe { std::mem::transmute(matrix) }),
            e => Err(Error::from(e))
        }
    }

    pub fn info_draw_color(&self, color: Pixel8) -> Result<(), Error> {
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
        let ret = unsafe {
            ((*self.ptr).info_draw_text.unwrap())((*self.ptr).refcon, text1.as_ptr(), text2.as_ptr())
        };
        match ret {
            0 => Ok(()),
            e => Err(Error::from(e))
        }
    }
}
