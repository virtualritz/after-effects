use crate::*;
use bitflags::bitflags;

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
        self.0.screen_point.into()
    }

    pub fn num_clicks(&self) -> u32 {
        self.0.num_clicks as _
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
        f.debug_struct("EventExtra")
            .field("event", &self.event())
            .field("window_type", &self.window_type())
            .field("param_index", &self.param_index())
            .field("effect_area", &self.effect_area())
            .field("current_frame", &self.current_frame())
            .field("param_title_frame", &self.param_title_frame())
            .field("horiz_offset", &self.horiz_offset())
            .field("modifiers", &self.modifiers())
            .field("last_time", &self.last_time())
            .finish()
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
        ContextHandle::from_raw(self.0.contextH)
    }

    pub fn event(&self) -> Event {
        match self.0.e_type {
            ae_sys::PF_Event_NEW_CONTEXT => Event::NewContext,
            ae_sys::PF_Event_ACTIVATE => Event::Activate,
            ae_sys::PF_Event_DO_CLICK => {
                Event::Click(ClickEventInfo::from_raw(unsafe { self.0.u.do_click }))
            }
            ae_sys::PF_Event_DRAG => {
                Event::Drag(ClickEventInfo::from_raw(unsafe { self.0.u.do_click }))
            }
            ae_sys::PF_Event_DRAW => Event::Draw(DrawEventInfo::from_raw(unsafe { self.0.u.draw })),
            ae_sys::PF_Event_DEACTIVATE => Event::Deactivate,
            ae_sys::PF_Event_CLOSE_CONTEXT => Event::CloseContext,
            ae_sys::PF_Event_IDLE => Event::Idle,
            ae_sys::PF_Event_ADJUST_CURSOR => {
                Event::AdjustCursor(AdjustCursorEventInfo::from_raw(unsafe {
                    self.0.u.adjust_cursor
                }))
            }
            ae_sys::PF_Event_KEYDOWN => {
                Event::Keydown(KeyDownEventInfo::from_raw(unsafe { self.0.u.key_down }))
            }
            ae_sys::PF_Event_MOUSE_EXITED => Event::MouseExited,
            _ => unreachable!(),
        }
    }

    pub fn window_type(&self) -> WindowType {
        WindowType::try_from(unsafe { **self.0.contextH }.w_type).unwrap()
    }

    pub fn event_out_flags(&mut self, flags: EventOutFlags) {
        self.0.evt_out_flags = flags.bits() as _;
    }

    pub fn param_index(&self) -> usize {
        self.0.effect_win.index as _
    }

    pub fn effect_area(&self) -> EffectArea {
        self.0.effect_win.area.into()
    }

    pub fn current_frame(&self) -> Rect {
        unsafe { std::mem::transmute(self.0.effect_win.current_frame) }
    }

    pub fn param_title_frame(&self) -> Rect {
        unsafe { std::mem::transmute(self.0.effect_win.param_title_frame) }
    }

    pub fn horiz_offset(&self) -> i32 {
        self.0.effect_win.horiz_offset
    }

    pub fn modifiers(&self) -> Modifiers {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.0.e_type),
            "The modifiers() method is only valid if event() is Click or Drag."
        );
        unsafe { Modifiers::from_bits(self.0.u.do_click.modifiers as _).unwrap() }
    }

    pub fn set_continue_refcon(&mut self, index: usize, value: ae_sys::A_intptr_t) {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.0.e_type),
            "The continue_refcon() method is only valid if event() is Click or Drag."
        );
        debug_assert!(index < 4);
        unsafe {
            self.0.u.do_click.continue_refcon[index] = value;
        }
    }

    pub fn continue_refcon(&self, index: usize) -> ae_sys::A_intptr_t {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.0.e_type),
            "The get_continue_refcon() method is only valid if event() is Click or Drag."
        );
        debug_assert!(index < 4);
        unsafe { self.0.u.do_click.continue_refcon[index] }
    }

    pub fn send_drag(&mut self, send: bool) {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.0.e_type),
            "The send_drag() method is only valid if event() is Click or Drag."
        );
        self.0.u.do_click.send_drag = send as _;
    }

    pub fn last_time(&self) -> bool {
        debug_assert!(
            [ae_sys::PF_Event_DO_CLICK, ae_sys::PF_Event_DRAG].contains(&self.0.e_type),
            "The last_time() method is only valid if event() is Click or Drag."
        );
        unsafe { self.0.u.do_click.last_time != 0 }
    }
}
