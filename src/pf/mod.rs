use crate::*;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    convert::TryFrom,
    fmt::{Debug, Write},
    marker::PhantomData,
};

mod in_data;
pub use in_data::*;
mod out_data;
pub use out_data::*;
mod parameters;
pub use parameters::*;
mod command;
pub use command::*;
mod layer;
pub use layer::*;
mod gpu;
pub use gpu::*;
mod render;
pub use render::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Pixel {
    pub alpha: ae_sys::A_u_char,
    pub red: ae_sys::A_u_char,
    pub green: ae_sys::A_u_char,
    pub blue: ae_sys::A_u_char,
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

impl From<ae_sys::PF_Pixel> for Pixel {
    fn from(pixel: ae_sys::PF_Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

impl From<Pixel> for ae_sys::PF_Pixel {
    fn from(pixel: Pixel) -> Self {
        Self {
            alpha: pixel.alpha,
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        }
    }
}

pub type Pixel8 = Pixel;

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
    // Sent when mouse moves over custom UI.
    AdjustCursor(AdjustCursorEventInfo),
    // Sends keycodes or unicode characters.
    Keydown(KeyDownEventInfo),
    // Notification that the mouse is no
    // longer over a specific view (layer or comp only).
    MouseExited,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(target_os = "macos", repr(u32))]
pub enum EffectArea {
    Mone = ae_sys::PF_EA_NONE,
    Title = ae_sys::PF_EA_PARAM_TITLE,
    Control = ae_sys::PF_EA_CONTROL,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum WindowType {
    Comp = ae_sys::PF_Window_COMP,
    Layer = ae_sys::PF_Window_LAYER,
    Effect = ae_sys::PF_Window_EFFECT,
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
        EffectArea::try_from(self.0.effect_win.area as EnumIntType).unwrap()
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel16 {
    pub alpha: ae_sys::A_u_short,
    pub red: ae_sys::A_u_short,
    pub green: ae_sys::A_u_short,
    pub blue: ae_sys::A_u_short,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel32 {
    pub alpha: ae_sys::PF_FpShort,
    pub red: ae_sys::PF_FpShort,
    pub green: ae_sys::PF_FpShort,
    pub blue: ae_sys::PF_FpShort,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(i32)]
pub enum TransferMode {
    None = ae_sys::PF_Xfer_NONE,
    Copy = ae_sys::PF_Xfer_COPY,
    Behind = ae_sys::PF_Xfer_BEHIND,
    InFront = ae_sys::PF_Xfer_IN_FRONT,
    Dissolve = ae_sys::PF_Xfer_DISSOLVE,
    Add = ae_sys::PF_Xfer_ADD,
    Mulitply = ae_sys::PF_Xfer_MULTIPLY,
    Screen = ae_sys::PF_Xfer_SCREEN,
    Overlay = ae_sys::PF_Xfer_OVERLAY,
    SoftLight = ae_sys::PF_Xfer_SOFT_LIGHT,
    HardLight = ae_sys::PF_Xfer_HARD_LIGHT,
    Darken = ae_sys::PF_Xfer_DARKEN,
    Lighten = ae_sys::PF_Xfer_LIGHTEN,
    Difference = ae_sys::PF_Xfer_DIFFERENCE,
    Hue = ae_sys::PF_Xfer_HUE,
    Saturation = ae_sys::PF_Xfer_SATURATION,
    Color = ae_sys::PF_Xfer_COLOR,
    Luminosity = ae_sys::PF_Xfer_LUMINOSITY,
    MultiplyAlpha = ae_sys::PF_Xfer_MULTIPLY_ALPHA,
    MultiplyAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_ALPHA_LUMA,
    MultiplyNotAlpha = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA,
    MultiplyNotAlphaLuma = ae_sys::PF_Xfer_MULTIPLY_NOT_ALPHA_LUMA,
    AddiditivePremul = ae_sys::PF_Xfer_ADDITIVE_PREMUL,
    AlphaAdd = ae_sys::PF_Xfer_ALPHA_ADD,
    ColorDodge = ae_sys::PF_Xfer_COLOR_DODGE,
    ColorBurn = ae_sys::PF_Xfer_COLOR_BURN,
    Exclusion = ae_sys::PF_Xfer_EXCLUSION,

    Difference2 = ae_sys::PF_Xfer_DIFFERENCE2,
    ColorDodge2 = ae_sys::PF_Xfer_COLOR_DODGE2,
    ColorBurn2 = ae_sys::PF_Xfer_COLOR_BURN2,

    LinearDodge = ae_sys::PF_Xfer_LINEAR_DODGE,
    LinearBurn = ae_sys::PF_Xfer_LINEAR_BURN,
    LinearLight = ae_sys::PF_Xfer_LINEAR_LIGHT,
    VividLight = ae_sys::PF_Xfer_VIVID_LIGHT,
    PinLight = ae_sys::PF_Xfer_PIN_LIGHT,

    HardMix = ae_sys::PF_Xfer_HARD_MIX,

    LighterColor = ae_sys::PF_Xfer_LIGHTER_COLOR,
    DarkerColor = ae_sys::PF_Xfer_DARKER_COLOR,

    Subtract = ae_sys::PF_Xfer_SUBTRACT,
    Divide = ae_sys::PF_Xfer_DIVIDE,

    Reserved0 = ae_sys::PF_Xfer_RESERVED0,
    Reserved1 = ae_sys::PF_Xfer_RESERVED1,

    NumModes = ae_sys::PF_Xfer_NUM_MODES,
}

pub type XferMode = TransferMode;

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct CompositeMode {
    pub xfer: TransferMode,
    // For TransferMode::DissolveRandomized.
    pub rand_seed: i32,
    // 0–255.
    pub opacity: u8,
    // Ignored TransferMode::MutiplyAlpha* modes.
    pub rgb_only: u8,
    // For deep color only.
    pub opacity_su: u16,
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct Point {
    pub h: i32,
    pub v: i32,
}
impl Into<ae_sys::PF_Point> for Point {
    fn into(self) -> ae_sys::PF_Point {
        ae_sys::PF_Point {
            h: self.h,
            v: self.v,
        }
    }
}
impl From<ae_sys::PF_Point> for Point {
    fn from(point: ae_sys::PF_Point) -> Self {
        Self {
            h: point.h,
            v: point.v,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
#[repr(C)]
pub struct RationalScale {
    pub num: ae_sys::A_long,
    pub den: ae_sys::A_u_long,
}

impl From<RationalScale> for ae_sys::PF_RationalScale {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<ae_sys::PF_RationalScale> for RationalScale {
    #[inline]
    fn from(ratio: ae_sys::PF_RationalScale) -> Self {
        Self {
            num: ratio.num,
            den: ratio.den,
        }
    }
}

impl From<RationalScale> for f64 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(
            ratio.den != 0,
            "Denominator is zero. This would lead to a division by zero."
        );
        ratio.num as Self / ratio.den as Self
    }
}

impl From<RationalScale> for f32 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(
            ratio.den != 0,
            "Denominator is zero. This would lead to a division by zero."
        );
        ratio.num as Self / ratio.den as Self
    }
}

pub type MaskFlags = u32;

#[derive(Debug)]
#[repr(C)]
pub struct MaskWorld {
    pub mask: EffectWorld,
    pub offset: Point,
    pub what_is_mask: MaskFlags,
}

#[derive(Copy, Clone, Debug, Hash)]
#[repr(i32)]
pub enum Quality {
    DrawingAudio = ae_sys::PF_Quality_DRAWING_AUDIO,
    Lo = ae_sys::PF_Quality_LO,
    Hi = ae_sys::PF_Quality_HI,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(target_os = "macos", repr(u32))]
pub enum ModeFlags {
    AlphaPremul = ae_sys::PF_MF_Alpha_PREMUL,
    AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(target_os = "macos", repr(u32))]
pub enum Field {
    Frame = ae_sys::PF_Field_FRAME,
    Upper = ae_sys::PF_Field_UPPER,
    Lower = ae_sys::PF_Field_LOWER,
}

// FIXME: wrap this nicely
/// An EffectWorld is a view on a WorldHandle that can be used to write to.
#[derive(Debug, Copy, Clone)]
pub struct EffectWorld {
    pub effect_world: ae_sys::PF_EffectWorld,
}

pub struct EffectWorldConst {
    pub effect_world: ae_sys::PF_EffectWorld,
}

unsafe impl Send for EffectWorldConst {}
unsafe impl Sync for EffectWorldConst {}

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables);

// FIXME: this is not safe.
// We just use EffectWorld responsibly but another user of this care may not.
unsafe impl Send for EffectWorld {}
unsafe impl Sync for EffectWorld {}

impl EffectWorld {
    #[inline]
    pub fn new(world_handle: aegp::WorldHandle) -> Result<Self, crate::Error> {
        aegp::WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
    }

    pub fn from_raw(effect_world_ptr: *const ae_sys::PF_EffectWorld) -> Result<Self, crate::Error> {
        if effect_world_ptr.is_null() {
            Err(crate::Error::Generic)
        } else {
            Ok(EffectWorld {
                effect_world: unsafe { *effect_world_ptr },
            })
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.effect_world.width as usize
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.effect_world.height as usize
    }

    #[inline]
    pub fn row_bytes(&self) -> usize {
        self.effect_world.rowbytes as usize
    }

    #[inline]
    pub fn data_as_ptr(&self) -> *const u8 {
        self.effect_world.data as *const u8
    }

    #[inline]
    pub fn data_as_ptr_mut(&self) -> *mut u8 {
        self.effect_world.data as *mut u8
    }

    #[inline]
    pub fn data_len(&self) -> usize {
        self.height() * self.row_bytes()
    }

    pub fn row_padding_bytes(&self) -> usize {
        self.row_bytes()
            - self.width()
                * 4
                * match self.world_type() {
                    aegp::WorldType::U15 => 2,
                    aegp::WorldType::U8 => 1,
                    aegp::WorldType::F32 => 4,
                    aegp::WorldType::None => panic!(),
                }
    }

    #[inline]
    pub fn as_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel8).add(x) }
    }

    #[inline]
    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        self.as_pixel8_mut(x, y)
    }

    #[inline]
    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel16).add(x) }
    }

    #[inline]
    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        self.as_pixel16_mut(x, y)
    }

    #[inline]
    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut Pixel32 {
        debug_assert!(
            x < self.width() && y < self.height(),
            "Coordinate is outside EffectWorld bounds."
        );
        unsafe { &mut *(self.data_as_ptr_mut().add(y * self.row_bytes()) as *mut Pixel32).add(x) }
    }

    #[inline]
    pub fn as_pixel32(&self, x: usize, y: usize) -> &Pixel32 {
        self.as_pixel32_mut(x, y)
    }

    #[inline]
    pub fn world_type(&self) -> aegp::WorldType {
        let flags = self.effect_world.world_flags as EnumIntType;
        // Most frequent case is 16bit integer.
        if ae_sys::PF_WorldFlag_DEEP & flags != 0 {
            aegp::WorldType::U15
        } else if ae_sys::PF_WorldFlag_RESERVED1 & flags != 0 {
            aegp::WorldType::F32
        } else {
            aegp::WorldType::U8
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ae_sys::PF_EffectWorld {
        &self.effect_world as *const ae_sys::PF_EffectWorld
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ae_sys::PF_EffectWorld {
        &mut self.effect_world as *mut ae_sys::PF_EffectWorld
    }
}

pub fn progress(in_data: InData, count: u16, total: u16) -> i32 {
    unsafe {
        (*in_data.as_ptr()).inter.progress.unwrap()(
            (*in_data.as_ptr()).effect_ref,
            count as i32,
            total as i32,
        )
    }
}

#[derive(Debug)]
pub struct Handle<'a, T: 'a> {
    suite_ptr: *const ae_sys::PF_HandleSuite1,
    handle: ae_sys::PF_Handle,
    _marker: PhantomData<&'a T>,
}

pub struct HandleLock<'a, T> {
    parent_handle: &'a Handle<'a, T>,
    ptr: *mut T,
}

impl<'a, T> HandleLock<'a, T> {
    pub fn as_ref(&self) -> Result<&'a T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn as_ref_mut(&self) -> Result<&'a mut T, Error> {
        if self.ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &mut *self.ptr })
        }
    }
}

impl<'a, T> Drop for HandleLock<'a, T> {
    fn drop(&mut self) {
        ae_call_suite_fn_no_err!(
            self.parent_handle.suite_ptr,
            host_unlock_handle,
            self.parent_handle.handle
        );
    }
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(value: T) -> Result<Handle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let handle: ae_sys::PF_Handle = ae_call_suite_fn_no_err!(
                    suite_ptr,
                    host_new_handle,
                    std::mem::size_of::<T>() as u64
                );
                if handle.is_null() {
                    return Err(Error::OutOfMemory);
                }

                let ptr = ae_call_suite_fn_no_err!(suite_ptr, host_lock_handle, handle) as *mut T;
                if ptr.is_null() {
                    return Err(Error::InvalidIndex);
                }

                unsafe { ptr.write(value) };
                ae_call_suite_fn_no_err!(suite_ptr, host_unlock_handle, handle);
                Ok(Handle {
                    suite_ptr,
                    handle,
                    //dispose: true,
                    _marker: PhantomData,
                })
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    pub fn set(&mut self, value: T) {
        let ptr = ae_call_suite_fn_no_err!(self.suite_ptr, host_lock_handle, self.handle) as *mut T;
        if !ptr.is_null() {
            unsafe {
                // Run destructors, if any.
                ptr.read()
            };
        }
        unsafe { ptr.write(value) };
        ae_call_suite_fn_no_err!(self.suite_ptr, host_unlock_handle, self.handle);
    }

    pub fn lock(&mut self) -> Result<HandleLock<T>, Error> {
        let ptr = ae_call_suite_fn_no_err!(self.suite_ptr, host_lock_handle, self.handle) as *mut T;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(HandleLock {
                parent_handle: self,
                ptr,
            })
        }
    }

    pub fn as_ref(&self) -> Result<&'a T, Error> {
        let ptr = unsafe { *(self.handle as *const *const T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &(*ptr) })
        }
    }

    pub fn as_mut(&self) -> Result<&'a mut T, Error> {
        let ptr = unsafe { *(self.handle as *mut *mut T) };
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(unsafe { &mut (*ptr) })
        }
    }

    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(self.suite_ptr, host_get_handle_size, self.handle) as usize
    }

    /*
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        ae_call_suite_fn!(self.suite_ptr, host_resize_handle, size as u64, &mut self.handle)
    }*/

    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<Handle<'a, T>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => Ok(Handle {
                suite_ptr,
                handle,
                //dispose: true,
                _marker: PhantomData,
            }),
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: Handle<T>) -> ae_sys::PF_Handle {
        //let us = crate::aegp::UtilitySuite::new().unwrap();
        //us.write_to_os_console("Handle::into_raw()").unwrap();

        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here
        std::mem::forget(handle);
        // Make sure drop(Handle) does *not*
        // actually drop anything since we're
        // passing ownership.
        //handle.dispose = false;
        return_handle
        // drop(handle) gets called.
    }

    /// Returns the raw handle.
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
}

impl<'a, T: 'a> Drop for Handle<'a, T> {
    fn drop(&mut self) {
        let ptr = unsafe { *(self.handle as *const *const T) };
        if !ptr.is_null() {
            unsafe { ptr.read() };
        }

        ae_call_suite_fn_no_err!(self.suite_ptr, host_dispose_handle, self.handle);
    }
}

pub struct FlatHandleLock<'a, 'b: 'a> {
    parent_handle: &'a FlatHandle<'b>,
}

impl<'a, 'b> Drop for FlatHandleLock<'a, 'b> {
    fn drop(&mut self) {
        ae_call_suite_fn_no_err!(
            self.parent_handle.suite_ptr,
            host_unlock_handle,
            self.parent_handle.handle
        );
    }
}

/// A flat handle takes a [Vec<u8>] as data. This is useful when data it passed
/// to Ae permanently or between runs of your plug-in.
/// You can use something like [bincode::serialize()] to serialize your data
/// structure into a flat [Vec<u8>].
#[derive(Debug)]
pub struct FlatHandle<'a> {
    suite_ptr: *const ae_sys::PF_HandleSuite1,
    handle: ae_sys::PF_Handle,
    is_owned: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FlatHandle<'a> {
    pub fn new(slice: impl Into<Vec<u8>>) -> Result<FlatHandle<'a>, Error> {
        let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

        match ae_acquire_suite_ptr!(
            pica_basic_suite_ptr,
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let vector = slice.into();

                let handle: ae_sys::PF_Handle =
                    ae_call_suite_fn_no_err!(suite_ptr, host_new_handle, vector.len() as u64);

                if handle.is_null() {
                    return Err(Error::OutOfMemory);
                }

                let ptr = ae_call_suite_fn_no_err!(suite_ptr, host_lock_handle, handle) as *mut u8;

                if ptr.is_null() {
                    return Err(Error::OutOfMemory);
                }

                let dest = std::ptr::slice_from_raw_parts_mut(ptr, vector.len());

                unsafe {
                    (*dest).copy_from_slice(vector.as_slice());
                }

                ae_call_suite_fn_no_err!(suite_ptr, host_unlock_handle, handle);
                Ok(Self {
                    suite_ptr,
                    handle,
                    is_owned: true,
                    _marker: PhantomData,
                })
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        match ae_call_suite_fn_no_err!(
            self.suite_ptr,
            host_resize_handle,
            size as _,
            &mut self.handle as _
        ) as EnumIntType
        {
            ae_sys::PF_Err_NONE => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    #[inline]
    pub fn lock<'b: 'a>(&'b self) -> Result<FlatHandleLock, Error> {
        let ptr =
            ae_call_suite_fn_no_err!(self.suite_ptr, host_lock_handle, self.handle) as *mut u8;
        if ptr.is_null() {
            Err(Error::InvalidIndex)
        } else {
            Ok(FlatHandleLock {
                parent_handle: self,
            })
        }
    }

    #[inline]
    pub fn as_slice(&'a self) -> Option<&'a [u8]> {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*std::ptr::slice_from_raw_parts(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_slice_mut(&'a self) -> Option<&'a mut [u8]> {
        let ptr = unsafe { *(self.handle as *const *mut u8) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *std::ptr::slice_from_raw_parts_mut(ptr, self.size()) })
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { *(self.handle as *const *const u8) }
    }

    #[inline]
    pub fn as_ptr_mut(&self) -> *mut u8 {
        unsafe { *(self.handle as *const *mut u8) }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        if ptr.is_null() {
            Vec::new()
        } else {
            unsafe {
                &*std::ptr::slice_from_raw_parts(*(self.handle as *const *const u8), self.size())
            }
            .to_vec()
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(self.suite_ptr, host_get_handle_size, self.handle) as usize
    }

    #[inline]
    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                if handle.is_null() {
                    Err(Error::Generic)
                } else {
                    let ptr = unsafe { *(handle as *const *const u8) };
                    if ptr.is_null() {
                        Err(Error::InternalStructDamaged)
                    } else {
                        Ok(Self {
                            suite_ptr,
                            handle,
                            is_owned: false,
                            _marker: PhantomData,
                        })
                    }
                }
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    #[inline]
    pub fn from_raw_owned(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Error> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                if handle.is_null() {
                    Err(Error::Generic)
                } else {
                    let ptr = unsafe { *(handle as *const *const u8) };
                    if ptr.is_null() {
                        Err(Error::InternalStructDamaged)
                    } else {
                        Ok(Self {
                            suite_ptr,
                            handle,
                            is_owned: true,
                            _marker: PhantomData,
                        })
                    }
                }
            }
            Err(_) => Err(Error::InvalidCallback),
        }
    }

    /// Turns the handle into and owned one
    #[inline]
    pub fn into_owned(mut handle: Self) -> Self {
        handle.is_owned = true;
        handle
    }

    /// Consumes the handle.
    #[inline]
    pub fn into_raw(handle: Self) -> ae_sys::PF_Handle {
        let return_handle = handle.handle;
        // We need to call forget() or else
        // drop() will be called on handle
        // which will dispose the memory.
        // Handle is just on the stack so
        // we're not leaking anything here.
        std::mem::forget(handle);

        return_handle
    }

    #[inline]
    pub fn as_raw(&self) -> ae_sys::PF_Handle {
        self.handle
    }
}
/*
impl<'a> Clone for FlatHandle<'a> {
    fn clone(&self) -> FlatHandle<'a> {
        Self::new(self.as_slice()).unwrap()
    }
}*/

impl<'a> Drop for FlatHandle<'a> {
    #[inline]
    fn drop(&mut self) {
        if self.is_owned {
            ae_call_suite_fn_no_err!(self.suite_ptr, host_dispose_handle, self.handle);
        }
    }
}

define_handle_wrapper!(ProgressInfo, PF_ProgPtr);

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ParamIndex {
    None = ae_sys::PF_ParamIndex_NONE,
    CheckAll = ae_sys::PF_ParamIndex_CHECK_ALL,
    CheckAllExceptLayerParams = ae_sys::PF_ParamIndex_CHECK_ALL_EXCEPT_LAYER_PARAMS,
    CheckAllHonorExclude = ae_sys::PF_ParamIndex_CHECK_ALL_HONOR_EXCLUDE,
}

pub type ProgPtr = ae_sys::PF_ProgPtr;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct CustomEventFlags: ae_sys::A_long {
        const NONE    = ae_sys::PF_CustomEFlag_NONE    as ae_sys::A_long;
        const COMP    = ae_sys::PF_CustomEFlag_COMP    as ae_sys::A_long;
        const LAYER   = ae_sys::PF_CustomEFlag_LAYER   as ae_sys::A_long;
        const EFFECT  = ae_sys::PF_CustomEFlag_EFFECT  as ae_sys::A_long;
        const PREVIEW = ae_sys::PF_CustomEFlag_PREVIEW as ae_sys::A_long;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    struct _UIAlignment: ae_sys::A_long {
        // No values other than PF_UIAlignment_NONE are honored, in Ae or PPro.
        const NONE   = ae_sys::PF_UIAlignment_NONE   as ae_sys::A_long;
        const TOP    = ae_sys::PF_UIAlignment_TOP    as ae_sys::A_long;
        const LEFT   = ae_sys::PF_UIAlignment_LEFT   as ae_sys::A_long;
        const BOTTOM = ae_sys::PF_UIAlignment_BOTTOM as ae_sys::A_long;
        const RIGHT  = ae_sys::PF_UIAlignment_RIGHT  as ae_sys::A_long;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Modifiers: ae_sys::A_long {
        const NONE            = ae_sys::PF_Mod_NONE            as ae_sys::A_long;
        /// Cmd on macOS, Ctrl on Windows.
        const CMD_CTRL_KEY    = ae_sys::PF_Mod_CMD_CTRL_KEY    as ae_sys::A_long;
        const SHIFT_KEY       = ae_sys::PF_Mod_SHIFT_KEY       as ae_sys::A_long;
        const CAPS_LOCK_KEY   = ae_sys::PF_Mod_CAPS_LOCK_KEY   as ae_sys::A_long;
        // Option on macOS, alt on Windows.
        const OPT_ALT_KEY     = ae_sys::PF_Mod_OPT_ALT_KEY     as ae_sys::A_long;
        // Mac control key only
        const MAC_CONTROL_KEY = ae_sys::PF_Mod_MAC_CONTROL_KEY as ae_sys::A_long;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CustomUIInfo(ae_sys::PF_CustomUIInfo);

impl CustomUIInfo {
    pub fn new() -> Self {
        Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_CustomUIInfo {
        &self.0
    }

    pub fn events(&mut self, events: CustomEventFlags) -> &mut Self {
        self.0.events = events.bits() as _;
        self
    }

    pub fn comp_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.comp_ui_width = width as _;
        self
    }

    pub fn comp_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.comp_ui_height = height as _;
        self
    }

    pub fn layer_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.layer_ui_width = width as _;
        self
    }

    pub fn layer_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.layer_ui_height = height as _;
        self
    }

    pub fn preview_ui_width(&mut self, width: u16) -> &mut Self {
        self.0.preview_ui_width = width as _;
        self
    }

    pub fn preview_ui_height(&mut self, height: u16) -> &mut Self {
        self.0.preview_ui_height = height as _;
        self
    }

    pub fn finalize(self) -> Self {
        self
    }
}

pub struct InteractCallbacks(InData);

impl InteractCallbacks {
    pub fn new(in_data: InData) -> Self {
        Self(in_data)
    }

    pub fn register_ui(&self, custom_ui_info: CustomUIInfo) -> Result<(), Error> {
        match unsafe {
            (*self.0.as_ptr()).inter.register_ui.unwrap()(
                (*self.0.as_ptr()).effect_ref,
                custom_ui_info.as_ptr() as _,
            )
        } as EnumIntType
        {
            ae_sys::PF_Err_NONE => Ok(()),
            e => Err(Error::from(e)),
        }
    }
}

define_handle_wrapper!(ContextHandle, PF_ContextH);

define_suite!(
    EffectCustomUISuite,
    PF_EffectCustomUISuite1,
    kPFEffectCustomUISuite,
    kPFEffectCustomUISuiteVersion1
);

impl EffectCustomUISuite {
    pub fn drawing_reference(
        &self,
        context_handle: &ContextHandle,
    ) -> Result<drawbot::DrawRef, Error> {
        let mut draw_reference = std::mem::MaybeUninit::<ae_sys::DRAWBOT_DrawRef>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetDrawingReference,
            context_handle.as_ptr(),
            draw_reference.as_mut_ptr()
        ) {
            Ok(()) => Ok(drawbot::DrawRef::from_raw(unsafe {
                draw_reference.assume_init()
            })),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    EffectCustomUIOverlayThemeSuite,
    PF_EffectCustomUIOverlayThemeSuite1,
    kPFEffectCustomUIOverlayThemeSuite,
    kPFEffectCustomUIOverlayThemeSuiteVersion1
);

impl EffectCustomUIOverlayThemeSuite {
    pub fn preferred_foreground_color(&self) -> Result<drawbot::ColorRGBA, Error> {
        let mut color = std::mem::MaybeUninit::<drawbot::ColorRGBA>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredForegroundColor,
            color.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { color.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn preferred_shadow_color(&self) -> Result<drawbot::ColorRGBA, Error> {
        let mut color = std::mem::MaybeUninit::<drawbot::ColorRGBA>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredShadowColor,
            color.as_mut_ptr() as _,
        ) {
            Ok(()) => Ok(unsafe { color.assume_init() }),
            Err(e) => Err(e),
        }
    }

    //PF_GetPreferredShadowOffset

    pub fn preferred_stroke_width(&self) -> Result<f32, Error> {
        let mut width = std::mem::MaybeUninit::<f32>::uninit();

        match ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPreferredStrokeWidth,
            width.as_mut_ptr(),
        ) {
            Ok(()) => Ok(unsafe { width.assume_init() }),
            Err(e) => Err(e),
        }
    }

    pub fn preferred_vertex_size(&self) -> Result<f32, Error> {
        let mut size = std::mem::MaybeUninit::<f32>::uninit();

        match ae_call_suite_fn!(self.suite_ptr, PF_GetPreferredVertexSize, size.as_mut_ptr(),) {
            Ok(()) => Ok(unsafe { size.assume_init() }),
            Err(e) => Err(e),
        }
    }
}

define_suite!(
    IterateFloatSuite,
    PF_iterateFloatSuite2,
    kPFIterateFloatSuite,
    kPFIterateFloatSuiteVersion2
);

impl IterateFloatSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_PixelFloat,
                out: *mut ae_sys::PF_PixelFloat,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}
define_suite!(
    Iterate16Suite,
    PF_Iterate16Suite2,
    kPFIterate16Suite,
    kPFIterate16SuiteVersion2
);

impl Iterate16Suite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_Pixel16,
                out: *mut ae_sys::PF_Pixel16,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}

define_suite!(
    Iterate8Suite,
    PF_Iterate8Suite2,
    kPFIterate8Suite,
    kPFIterate8SuiteVersion2
);

impl Iterate8Suite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    pub fn iterate(
        &self,
        in_data: InData,
        progress_base: i32,
        progress_final: i32,
        src: EffectWorld,
        area: Option<Rect>,
        refcon: *const std::ffi::c_void,
        pix_fn: Option<
            unsafe extern "C" fn(
                refcon: *mut std::ffi::c_void,
                x: i32,
                y: i32,
                in_: *mut ae_sys::PF_Pixel8,
                out: *mut ae_sys::PF_Pixel8,
            ) -> ae_sys::PF_Err,
        >,
        dst: EffectWorld,
    ) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            iterate,
            in_data.as_ptr() as *mut _,
            progress_base,
            progress_final,
            src.as_ptr() as *mut _,
            if let Some(area) = area {
                &area.into()
            } else {
                std::ptr::null()
            },
            refcon as *mut _,
            pix_fn,
            dst.as_ptr() as *mut _,
        )
    }
}

define_suite!(
    PixelFormatSuite,
    PF_PixelFormatSuite1,
    kPFPixelFormatSuite,
    kPFPixelFormatSuiteVersion1
);

impl PixelFormatSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn clear_supported_pixel_formats(&self, effect_ref: ProgressInfo) -> Result<(), Error> {
        ae_call_suite_fn!(
            self.suite_ptr,
            ClearSupportedPixelFormats,
            effect_ref.as_ptr()
        )
    }
    pub fn add_supported_pixel_format(
        &self,
        effect_ref: ProgressInfo,
        pixel_format: pf::PixelFormat,
    ) -> Result<(), Error> {
        let pixel_format: ae_sys::PF_PixelFormat = pixel_format.into();
        ae_call_suite_fn!(
            self.suite_ptr,
            AddSupportedPixelFormat,
            effect_ref.as_ptr(),
            pixel_format as EnumIntType
        )
    }
    pub fn add_pr_supported_pixel_format(
        &self,
        effect_ref: ProgressInfo,
        pixel_format: pr::PixelFormat,
    ) -> Result<(), Error> {
        let pixel_format: ae_sys::PrPixelFormat = pixel_format.into();
        ae_call_suite_fn!(
            self.suite_ptr,
            AddSupportedPixelFormat,
            effect_ref.as_ptr(),
            pixel_format as EnumIntType
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PixelFormat {
    Argb32,
    Argb64,
    Argb128,
    GpuBgra128,
    Reserved,
    Bgra32,
    Vuya32,
    NtscDv25,
    PalDv25,
    Invalid,
    ForceLongInt,
}
impl From<after_effects_sys::PF_PixelFormat> for PixelFormat {
    #[rustfmt::skip]
    fn from(x: after_effects_sys::PF_PixelFormat) -> Self {
        match x {
            ae_sys::PF_PixelFormat_ARGB32         => PixelFormat::Argb32,
            ae_sys::PF_PixelFormat_ARGB64         => PixelFormat::Argb64,
            ae_sys::PF_PixelFormat_ARGB128        => PixelFormat::Argb128,
            ae_sys::PF_PixelFormat_GPU_BGRA128    => PixelFormat::GpuBgra128,
            ae_sys::PF_PixelFormat_RESERVED       => PixelFormat::Reserved,
            ae_sys::PF_PixelFormat_BGRA32         => PixelFormat::Bgra32,
            ae_sys::PF_PixelFormat_VUYA32         => PixelFormat::Vuya32,
            ae_sys::PF_PixelFormat_NTSCDV25       => PixelFormat::NtscDv25,
            ae_sys::PF_PixelFormat_PALDV25        => PixelFormat::PalDv25,
            ae_sys::PF_PixelFormat_INVALID        => PixelFormat::Invalid,
            ae_sys::PF_PixelFormat_FORCE_LONG_INT => PixelFormat::ForceLongInt,
            _ => PixelFormat::Invalid,
        }
    }
}
impl Into<after_effects_sys::PF_PixelFormat> for PixelFormat {
    #[rustfmt::skip]
    fn into(self) -> after_effects_sys::PF_PixelFormat {
        match self {
            PixelFormat::Argb32       => ae_sys::PF_PixelFormat_ARGB32,
            PixelFormat::Argb64       => ae_sys::PF_PixelFormat_ARGB64,
            PixelFormat::Argb128      => ae_sys::PF_PixelFormat_ARGB128,
            PixelFormat::GpuBgra128   => ae_sys::PF_PixelFormat_GPU_BGRA128,
            PixelFormat::Reserved     => ae_sys::PF_PixelFormat_RESERVED,
            PixelFormat::Bgra32       => ae_sys::PF_PixelFormat_BGRA32,
            PixelFormat::Vuya32       => ae_sys::PF_PixelFormat_VUYA32,
            PixelFormat::NtscDv25     => ae_sys::PF_PixelFormat_NTSCDV25,
            PixelFormat::PalDv25      => ae_sys::PF_PixelFormat_PALDV25,
            PixelFormat::Invalid      => ae_sys::PF_PixelFormat_INVALID,
            PixelFormat::ForceLongInt => ae_sys::PF_PixelFormat_FORCE_LONG_INT,
        }
    }
}

define_suite!(
    WorldSuite2,
    PF_WorldSuite2,
    kPFWorldSuite,
    kPFWorldSuiteVersion2
);

impl WorldSuite2 {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_pixel_format(&self, effect_world: EffectWorld) -> Result<PixelFormat, Error> {
        let mut pixel_format = ae_sys::PF_PixelFormat_INVALID;

        ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetPixelFormat,
            effect_world.as_ptr(),
            &mut pixel_format
        )?;
        Ok(pixel_format.into())
    }
}

define_suite!(
    EffectSequenceDataSuite1,
    PF_EffectSequenceDataSuite1,
    kPFEffectSequenceDataSuite,
    kPFEffectSequenceDataSuiteVersion1
);
impl EffectSequenceDataSuite1 {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_const_sequence_data(
        &self,
        in_data_handle: InData,
    ) -> Result<ae_sys::PF_ConstHandle, Error> {
        let mut data: ae_sys::PF_ConstHandle = std::ptr::null_mut();

        ae_call_suite_fn!(
            self.suite_ptr,
            PF_GetConstSequenceData,
            in_data_handle.effect_ref().as_ptr(),
            &mut data
        )?;
        Ok(data)
    }
}

/*
pub enum GameState {
    FreePlacement(FreePlacement),
    Play(PlayState),
    Scoring(ScoringState),
    Done(ScoringState),
}

assume!(GameState);
assume!(GameState, Play(x) => x, PlayState);
assume!(GameState, Scoring(x) => x, ScoringState);
assume!(GameState, FreePlacement(x) => x, FreePlacement);

// and then I can:

state.assume::<PlayState>()
*/

pub trait AssumeFrom<T> {
    fn assume(x: &T) -> &Self;
    fn assume_mut(x: &mut T) -> &mut Self;
}

define_suite!(
    UtilitySuite,
    PF_UtilitySuite,
    kPFUtilitySuite,
    kPFUtilitySuiteVersion9
);
impl UtilitySuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn filter_instance_id(&self, in_data_handle: InData) -> Result<i32, Error> {
        let mut instance_id: ae_sys::A_long = 0;

        ae_call_suite_fn!(
            self.suite_ptr,
            GetFilterInstanceID,
            in_data_handle.effect_ref().as_ptr(),
            &mut instance_id
        )?;
        Ok(instance_id as i32)
    }
}
