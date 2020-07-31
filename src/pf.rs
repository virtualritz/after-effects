pub use crate::*;
use aftereffects_sys as ae_sys;
use std::{convert::TryInto, ffi::CString, fmt::Debug, marker::PhantomData};
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntoPrimitive, UnsafeFromPrimitive)]
#[repr(i32)]
pub enum Err {
    None = ae_sys::PF_Err_NONE as i32,
    OutOfMemory = ae_sys::PF_Err_OUT_OF_MEMORY as i32,
    InternalStructDamaged = ae_sys::PF_Err_INTERNAL_STRUCT_DAMAGED as i32,
    // Out of range, or action not allowed on this index.
    InvalidIndex = ae_sys::PF_Err_INVALID_INDEX as i32,
    UnrecogizedParamType = ae_sys::PF_Err_UNRECOGNIZED_PARAM_TYPE as i32,
    InvalidCallback = ae_sys::PF_Err_INVALID_CALLBACK as i32,
    BadCallbackParam = ae_sys::PF_Err_BAD_CALLBACK_PARAM as i32,
    // Returned when user interrupts rendering.
    InterruptCancel = ae_sys::PF_Interrupt_CANCEL as i32,
    // Returned from PF_Arbitrary_SCAN_FUNC when effect cannot parse
    // arbitrary data from text
    CannonParseKeyframeText = ae_sys::PF_Err_CANNOT_PARSE_KEYFRAME_TEXT as i32,
}

/*
impl From<Err> for i32 {
    fn from(e: Err) -> Self {
        e as i32
    }
}*/

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pixel {
    pub alpha: ae_sys::A_u_char,
    pub red: ae_sys::A_u_char,
    pub green: ae_sys::A_u_char,
    pub blue: ae_sys::A_u_char,
}

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
    ColorBurn2 = PF_Xfer_COLOR_BURN2,

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
        debug_assert!(ratio.den != 0);
        ratio.num as Self / ratio.den as Self
    }
}

impl From<RationalScale> for f32 {
    #[inline]
    fn from(ratio: RationalScale) -> Self {
        debug_assert!(ratio.den != 0);
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

#[repr(u32)]
pub enum ModeFlags {
    AlphaPremul = ae_sys::PF_MF_Alpha_PREMUL,
    AlphaStraight = ae_sys::PF_MF_Alpha_STRAIGHT,
}

#[repr(u32)]
pub enum Field {
    Frame = ae_sys::PF_Field_FRAME,
    Upper = ae_sys::PF_Field_UPPER,
    Lower = ae_sys::PF_Field_LOWER,
}

pub type Command = ae_sys::PF_Cmd;

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

define_handle_wrapper!(EffectBlendingTables, PF_EffectBlendingTables, blending_tabpe_ptr);

impl EffectWorld {
    #[inline]
    pub fn new(world_handle: WorldHandle) -> Result<Self, crate::Error> {
        WorldSuite::new()?.fill_out_pf_effect_world(world_handle)
    }

    pub fn from_raw(effect_world_ptr: *const ae_sys::PF_EffectWorld) -> Result<Self, crate::Error> {
        if std::ptr::null() == effect_world_ptr {
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
                    WorldType::Integer => 2,
                    WorldType::Byte => 1,
                    WorldType::Float => 4,
                    WorldType::None => panic!(),
                }
    }

    #[inline]
    pub fn as_pixel8_mut(&self, x: usize, y: usize) -> &mut Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel8).add(x) }
    }

    #[inline]
    pub fn as_pixel8(&self, x: usize, y: usize) -> &Pixel8 {
        debug_assert!(x < self.width() && y < self.height());
        self.as_pixel8_mut(x, y)
    }

    #[inline]
    pub fn as_pixel16_mut(&self, x: usize, y: usize) -> &mut Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel16).add(x) }
    }

    #[inline]
    pub fn as_pixel16(&self, x: usize, y: usize) -> &Pixel16 {
        debug_assert!(x < self.width() && y < self.height());
        self.as_pixel16_mut(x, y)
    }

    #[inline]
    pub fn as_pixel32_mut(&self, x: usize, y: usize) -> &mut Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &mut *(self.effect_world.data.add(y * self.row_bytes()) as *mut Pixel32).add(x) }
    }

    #[inline]
    pub fn as_pixel32(&self, x: usize, y: usize) -> &Pixel32 {
        debug_assert!(x < self.width() && y < self.height());
        unsafe { &*((self.effect_world.data as *const u8).add(y * self.row_bytes()) as *const Pixel32).add(x) }
    }

    #[inline]
    pub fn world_type(&self) -> WorldType {
        let flags = self.effect_world.world_flags;
        // Most frequent case is 16bit integer.
        if ae_sys::PF_WorldFlag_DEEP & flags as u32 != 0 {
            WorldType::Integer
        } else if ae_sys::PF_WorldFlag_RESERVED1 & flags as u32 != 0 {
            WorldType::Float
        } else {
            WorldType::Byte
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

#[macro_export]
macro_rules! add_param {
    (in_data: expr,
    index: expr,
    def: expr) => {
        in_data.inter.add_param.unwrap()(in_data.effect_ref, (index), &(def))
    };
}

pub fn progress(in_data: InDataHandle, count: u16, total: u16) -> i32 {
    unsafe { (*in_data.as_ptr()).inter.progress.unwrap()((*in_data.as_ptr()).effect_ref, count as i32, total as i32) }
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
    pub fn get(&self) -> Result<&'a T, Err> {
        if self.ptr.is_null() {
            Err(Err::InvalidIndex)
        } else {
            Ok(unsafe { &*self.ptr })
        }
    }

    pub fn get_mut(&self) -> Result<&'a mut T, Err> {
        if self.ptr.is_null() {
            Err(Err::InvalidIndex)
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
    pub fn new(value: T) -> Result<Handle<'a, T>, Err> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let handle: ae_sys::PF_Handle =
                    ae_call_suite_fn_no_err!(suite_ptr, host_new_handle, std::mem::size_of::<T>() as u64);
                if handle.is_null() {
                    return Err(Err::OutOfMemory);
                }
                let ptr = ae_call_suite_fn_no_err!(suite_ptr, host_lock_handle, handle) as *mut T;
                if ptr.is_null() {
                    return Err(Err::InvalidIndex);
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
            Err(_) => Err(Err::InvalidCallback),
        }
    }

    pub fn set(&mut self, value: T) {
        let ptr = ae_call_suite_fn_no_err!(self.suite_ptr, host_lock_handle, self.handle) as *mut T;
        if !ptr.is_null() {
            unsafe {
                // Run destructors, if any
                ptr.read()
            };
        }
        unsafe { ptr.write(value) };
        ae_call_suite_fn_no_err!(self.suite_ptr, host_unlock_handle, self.handle);
    }

    pub fn lock(&mut self) -> Result<HandleLock<T>, Err> {
        let ptr = ae_call_suite_fn_no_err!(self.suite_ptr, host_lock_handle, self.handle) as *mut T;
        if ptr.is_null() {
            Err(Err::InvalidIndex)
        } else {
            Ok(HandleLock {
                parent_handle: self,
                ptr,
            })
        }
    }

    pub fn get(&self) -> Result<&'a T, Err> {
        let ptr = unsafe { *(self.handle as *const *const T) };
        if ptr.is_null() {
            Err(Err::InvalidIndex)
        } else {
            Ok(unsafe { &(*ptr) })
        }
    }

    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(self.suite_ptr, host_get_handle_size, self.handle) as usize
    }

    /*
    pub fn resize(&mut self, size: usize) -> Result<(), Error> {
        ae_call_suite_fn!(self.suite_ptr, host_resize_handle, size as u64, &mut self.handle)
    }*/

    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<Handle<'a, T>, Err> {
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
            Err(_) => Err(Err::InvalidCallback),
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

#[derive(Debug)]
pub struct FlatHandle<'a> {
    suite_ptr: *const ae_sys::PF_HandleSuite1,
    handle: ae_sys::PF_Handle,
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

                let ptr = ae_call_suite_fn_no_err!(suite_ptr, host_lock_handle, handle) as *mut u8;
                debug_assert!(!ptr.is_null());

                let dest = std::ptr::slice_from_raw_parts_mut(ptr, vector.len());
                unsafe {
                    (*dest).copy_from_slice(vector.as_slice());
                }
                ae_call_suite_fn_no_err!(suite_ptr, host_unlock_handle, handle);
                Ok(FlatHandle {
                    suite_ptr,
                    handle,
                    _marker: PhantomData,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn get(&self) -> &'a [u8] {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        debug_assert!(!ptr.is_null());
        unsafe { &*std::ptr::slice_from_raw_parts(ptr, self.size()) }
    }

    /*
    pub fn is_empty(&self) -> bool {
        let ptr = unsafe { *(self.handle as *const *const u8) };
        ptr.is_null()
    }*/

    pub fn size(&self) -> usize {
        ae_call_suite_fn_no_err!(self.suite_ptr, host_get_handle_size, self.handle) as usize
    }

    pub fn from_raw(handle: ae_sys::PF_Handle) -> Result<FlatHandle<'a>, Err> {
        match ae_acquire_suite_ptr!(
            borrow_pica_basic_as_ptr(),
            PF_HandleSuite1,
            kPFHandleSuite,
            kPFHandleSuiteVersion1
        ) {
            Ok(suite_ptr) => {
                let ptr = unsafe { *(handle as *const *const u8) };
                if ptr.is_null() {
                    Err(Err::InternalStructDamaged)
                } else {
                    Ok(FlatHandle {
                        suite_ptr,
                        handle,
                        //dispose: true,
                        _marker: PhantomData,
                    })
                }
            }
            Err(_) => Err(Err::InvalidCallback),
        }
    }

    /// Consumes the handle.
    pub fn into_raw(handle: FlatHandle) -> ae_sys::PF_Handle {
        let return_handle = handle.handle;
        // Handle is just on the stack so
        // we're not leaking anything here.
        // We need to call forget() or else
        // drop() will be called on handle
        // which will dispose the memory.
        std::mem::forget(handle);

        return_handle
    }
}

impl<'a> Drop for FlatHandle<'a> {
    fn drop(&mut self) {
        ae_call_suite_fn_no_err!(self.suite_ptr, host_dispose_handle, self.handle);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<ae_sys::PF_LRect> for Rect {
    fn from(rect: ae_sys::PF_LRect) -> Self {
        Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl From<Rect> for ae_sys::PF_LRect {
    fn from(rect: Rect) -> Self {
        ae_sys::PF_LRect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl Rect {
    pub fn is_empty(&self) -> bool {
        (self.left >= self.right) || (self.top >= self.bottom)
    }

    pub fn union<'a>(&'a mut self, other: &Rect) -> &'a mut Rect {
        if other.is_empty() {
            *self = *other;
        } else if !other.is_empty() {
            self.left = std::cmp::min(self.left, other.left);
            self.top = std::cmp::min(self.top, other.top);
            self.right = std::cmp::max(self.right, other.right);
            self.bottom = std::cmp::max(self.bottom, other.bottom);
        }
        self
    }

    pub fn is_edge_pixel(&self, x: i32, y: i32) -> bool {
        let mut x_hit = (x == self.left) || (x == self.right);
        let mut y_hit = (y == self.top) || (y == self.bottom);

        if x_hit {
            y_hit = (y >= self.top) && (y <= self.bottom);
        } else {
            if y_hit {
                x_hit = (x >= self.left) && (x <= self.right);
            }
        }
        x_hit && y_hit
    }
}

define_handle_wrapper!(ProgressInfo, PF_ProgPtr, prog_ptr);

#[derive(Copy, Clone, Debug)]
pub struct SmartRenderCallbacks {
    pub(crate) rc_ptr: *const ae_sys::PF_SmartRenderCallbacks,
}

impl SmartRenderCallbacks {
    pub fn from_raw(rc_ptr: *const ae_sys::PF_SmartRenderCallbacks) -> Self {
        Self { rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_SmartRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer_pixels(&self, effect_ref: ProgressInfo, checkout_id: u32) -> Result<EffectWorld, Err> {
        if let Some(checkout_layer_pixels) = unsafe { *self.rc_ptr }.checkout_layer_pixels {
            let mut effect_world_ptr = std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe {
                checkout_layer_pixels(effect_ref.as_ptr(), checkout_id as i32, effect_world_ptr.as_mut_ptr())
            } as u32
            {
                ae_sys::PF_Err_NONE => Ok(EffectWorld {
                    effect_world: unsafe { *effect_world_ptr.assume_init() },
                }),
                e => Err(unsafe { Err::from_unchecked(e as i32) }),
            }
        } else {
            Err(Err::InvalidCallback)
        }
    }

    pub fn checkin_layer_pixels(&self, effect_ref: ProgressInfo, checkout_id: u32) -> Result<(), Err> {
        if let Some(checkin_layer_pixels) = unsafe { *self.rc_ptr }.checkin_layer_pixels {
            match unsafe { checkin_layer_pixels(effect_ref.as_ptr(), checkout_id as i32) } as u32 {
                ae_sys::PF_Err_NONE => Ok(()),
                e => Err(unsafe { Err::from_unchecked(e as i32) }),
            }
        } else {
            Err(Err::InvalidCallback)
        }
    }

    pub fn checkout_output(&self, effect_ref: ProgressInfo) -> Result<EffectWorld, Err> {
        if let Some(checkout_output) = unsafe { *self.rc_ptr }.checkout_output {
            let mut effect_world_ptr = std::mem::MaybeUninit::<*mut ae_sys::PF_EffectWorld>::uninit();

            match unsafe { checkout_output(effect_ref.as_ptr(), effect_world_ptr.as_mut_ptr()) } as u32 {
                ae_sys::PF_Err_NONE => Ok(EffectWorld {
                    effect_world: unsafe { *effect_world_ptr.assume_init() },
                }),
                e => Err(unsafe { Err::from_unchecked(e as i32) }),
            }
        } else {
            Err(Err::InvalidCallback)
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ParamIndex {
    None = ae_sys::PF_ParamIndex_NONE,
    CheckAll = ae_sys::PF_ParamIndex_CHECK_ALL,
    CheckAllExceptLayerParams = ae_sys::PF_ParamIndex_CHECK_ALL_EXCEPT_LAYER_PARAMS,
    CheckAllHonorExclude = ae_sys::PF_ParamIndex_CHECK_ALL_HONOR_EXCLUDE,
}

#[derive(Copy, Clone, Debug)]
pub struct PreRenderCallbacks {
    pub(crate) rc_ptr: *const ae_sys::PF_PreRenderCallbacks,
}

impl PreRenderCallbacks {
    pub fn from_raw(rc_ptr: *const ae_sys::PF_PreRenderCallbacks) -> Self {
        Self { rc_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_PreRenderCallbacks {
        self.rc_ptr
    }

    pub fn checkout_layer(
        &self,
        effect_ref: ProgressInfo,
        index: i32,
        checkout_id: i32,
        // FIXME: warp this struct
        req: &PF_RenderRequest,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> Result<PF_CheckoutResult, Err> {
        if let Some(checkout_layer) = unsafe { *self.rc_ptr }.checkout_layer {
            let mut checkout_result = std::mem::MaybeUninit::<PF_CheckoutResult>::uninit();

            match unsafe {
                checkout_layer(
                    effect_ref.as_ptr(),
                    index as i32,
                    checkout_id as i32,
                    req,
                    what_time,
                    time_step,
                    time_scale,
                    checkout_result.as_mut_ptr(),
                )
            } as u32
            {
                ae_sys::PF_Err_NONE => Ok(unsafe { checkout_result.assume_init() }),
                e => Err(unsafe { Err::from_unchecked(e as i32) }),
            }
        } else {
            Err(Err::InvalidCallback)
        }
    }

    /* FIXME
    pub fn guid_mix_in_ptr(
            effect_ref: ProgressInfo,
            buf: [u8],
        ) -> PF_Err,
    >,*/
}
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InDataHandle {
    pub(crate) in_data_ptr: *const ae_sys::PF_InData,
}

impl InDataHandle {
    pub fn from_raw(in_data_ptr: *const ae_sys::PF_InData) -> Self {
        Self { in_data_ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_InData {
        self.in_data_ptr
    }

    pub fn is_null(&self) -> bool {
        self.in_data_ptr.is_null()
    }
}

pub type ProgPtr = PF_ProgPtr;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ParamType {
    Reserved = ae_sys::PF_Param_RESERVED,
    Layer = ae_sys::PF_Param_LAYER,
    Slider = ae_sys::PF_Param_SLIDER,
    FixSlider = ae_sys::PF_Param_FIX_SLIDER,
    Angle = ae_sys::PF_Param_ANGLE,
    CheckBox = ae_sys::PF_Param_CHECKBOX,
    Color = ae_sys::PF_Param_COLOR,
    Point = ae_sys::PF_Param_POINT,
    PopUp = ae_sys::PF_Param_POPUP,
    Custom = ae_sys::PF_Param_CUSTOM,
    NoData = ae_sys::PF_Param_NO_DATA,
    FloatSlider = ae_sys::PF_Param_FLOAT_SLIDER,
    ArbitraryData = ae_sys::PF_Param_ARBITRARY_DATA,
    Path = ae_sys::PF_Param_PATH,
    GroupStart = ae_sys::PF_Param_GROUP_START,
    GroupEnd = ae_sys::PF_Param_GROUP_END,
    Button = ae_sys::PF_Param_BUTTON,
    Reserved2 = ae_sys::PF_Param_RESERVED2,
    Reserved3 = ae_sys::PF_Param_RESERVED3,
    Point3D = ae_sys::PF_Param_POINT_3D,
}

bitflags! {
    pub struct ParamFlag: u32 {
        const RESERVED1 = ae_sys::PF_ParamFlag_RESERVED1;
        const CANNOT_TIME_VARY = ae_sys::PF_ParamFlag_CANNOT_TIME_VARY;
        const CANNOT_INTERP= ae_sys::PF_ParamFlag_CANNOT_INTERP;
        const RESERVED2= ae_sys::PF_ParamFlag_RESERVED2;
        const RESERVED3 = ae_sys::PF_ParamFlag_RESERVED3;
        const TWIRLY = ae_sys::PF_ParamFlag_COLLAPSE_TWIRLY;
        const SUPERVISE = ae_sys::PF_ParamFlag_SUPERVISE;
        const START_COLLAPSED = ae_sys::PF_ParamFlag_START_COLLAPSED;
        const USE_VALUE_FOR_OLD_PROJECTS = ae_sys::PF_ParamFlag_USE_VALUE_FOR_OLD_PROJECTS;
        const LAYER_PARAM_IS_TRACKMATTE = ae_sys::PF_ParamFlag_LAYER_PARAM_IS_TRACKMATTE;
        const EXCLUDE_FROM_HAVE_INPUTS_CHANGED = ae_sys::PF_ParamFlag_EXCLUDE_FROM_HAVE_INPUTS_CHANGED;
        const SKIP_REVEAL_WHEN_UNHIDDEN = ae_sys::PF_ParamFlag_SKIP_REVEAL_WHEN_UNHIDDEN;
    }
}

bitflags! {
    pub struct ValueDisplayFlag: u16 {
        const NONE = ae_sys::PF_ValueDisplayFlag_NONE as u16;
        const PERCENT = ae_sys::PF_ValueDisplayFlag_PERCENT as u16;
        const PIXEL = ae_sys::PF_ValueDisplayFlag_PIXEL as u16;
        const RESERVED = ae_sys::PF_ValueDisplayFlag_RESERVED1 as u16;
        const REVERSE = ae_sys::PF_ValueDisplayFlag_REVERSE as u16;
    }
}

//define_param_wrapper!(ButtonDef, PF_ButtonDef, button_def);

#[repr(C)]
#[derive(Clone)]
pub struct ButtonDef {
    button_def: ae_sys::PF_ButtonDef,
    label: CString,
}

impl ButtonDef {
    pub fn new() -> Self {
        Self {
            button_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            label: CString::new("").unwrap(),
        }
    }

    pub fn label<'a>(&'a mut self, label: &str) -> &'a mut Self {
        self.label = CString::new(label).unwrap();
        self.button_def.u.namesptr = self.label.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_BUTTON == param.param_def_boxed.param_type {
            Some(Self {
                button_def: unsafe { param.param_def_boxed.u.button_d },
                label: unsafe { CString::from_raw(param.param_def_boxed.u.button_d.u.namesptr as _) },
            })
        } else {
            None
        }
    }

    pub fn into_raw(def: ButtonDef) -> ae_sys::PF_ButtonDef {
        def.button_def
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PopupDef {
    popup_def: ae_sys::PF_PopupDef,
    names: CString,
}

define_param_basic_wrapper!(PopupDef, PF_PopupDef, popup_def, i32, u16);

impl PopupDef {
    pub fn new() -> Self {
        Self {
            popup_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            names: CString::new("").unwrap(),
        }
    }

    pub fn names<'a>(&'a mut self, names: Vec<&str>) -> &'a mut Self {
        // Build a string in the format "list|of|choices|", the
        // format Ae expects. Ae ignores the trailing '|'.
        let mut names_tmp = String::new();
        names.iter().for_each(|s| names_tmp += format!("{}|", *s).as_str());
        self.names = CString::new(names_tmp).unwrap();
        self.popup_def.u.namesptr = self.names.as_ptr();
        self.popup_def.num_choices = names.len().try_into().unwrap();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_POPUP == param.param_def_boxed.param_type {
            Some(Self {
                popup_def: unsafe { param.param_def_boxed.u.pd },
                names: CString::new("").unwrap(),
            })
        } else {
            None
        }
    }

    //pub fn check_out()

    pub fn get(&self) -> u16 {
        self.popup_def.value as u16
    }
}

define_param_wrapper!(AngleDef, PF_AngleDef, angle_def);
define_param_basic_wrapper!(AngleDef, PF_AngleDef, angle_def, i32, i32);

impl AngleDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_ANGLE == param.param_def_boxed.param_type {
            Some(Self {
                angle_def: unsafe { param.param_def_boxed.u.ad },
            })
        } else {
            None
        }
    }

    pub fn get(&self) -> i32 {
        self.angle_def.value
    }
}

define_param_wrapper!(ColorDef, PF_ColorDef, color_def);

impl ColorDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_COLOR == param.param_def_boxed.param_type {
            Some(Self {
                color_def: unsafe { param.param_def_boxed.u.cd },
            })
        } else {
            None
        }
    }

    pub fn get(&self) -> Pixel {
        Pixel::from(self.color_def.value)
    }

    pub fn value<'a>(&'a mut self, value: Pixel) -> &'a mut Self {
        self.color_def.value = ae_sys::PF_Pixel::from(value);
        self
    }

    pub fn default<'a>(&'a mut self, default: Pixel) -> &'a mut Self {
        self.color_def.dephault = ae_sys::PF_Pixel::from(default);
        self
    }

    pub fn into_raw(def: ColorDef) -> ae_sys::PF_ColorDef {
        def.color_def
    }
}

define_param_wrapper!(SliderDef, PF_SliderDef, slider_def);
define_param_basic_wrapper!(SliderDef, PF_SliderDef, slider_def, i32, i32);
define_param_valid_min_max_wrapper!(SliderDef, slider_def, i32);
define_param_slider_min_max_wrapper!(SliderDef, slider_def, i32);
define_param_value_str_wrapper!(SliderDef, slider_def);
define_param_value_desc_wrapper!(SliderDef, slider_def);

impl SliderDef {
    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_SLIDER == param.param_def_boxed.param_type {
            Some(Self {
                slider_def: unsafe { param.param_def_boxed.u.sd },
            })
        } else {
            None
        }
    }

    pub fn get(&self) -> i32 {
        self.slider_def.value
    }
}
/* Adobe recommends not useing fixed (point) sliders any more and
 * instead to use float sliders instead.

    define_param_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def);
    define_param_basic_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32, i32);
    define_param_slider_min_max_wrapper!(FixedSliderDef, PF_FixedSliderDef, slider_def, i32);
    define_param_value_str_wrapper!(FixedSliderDef, slider_def);
    define_param_value_desc_wrapper!(FixedSliderDef, slider_def);

    impl FixedSliderDef {
        pub fn precision<'a>(&'a mut self, precision: u16) -> &'a mut FixedSliderDef {
            self.slider_def.precision = precision as i16;
            self
        }

        pub fn display_flags<'a>(&'a mut self, display_flags: ValueDisplayFlag) -> &'a mut FixedSliderDef {
            self.slider_def.display_flags = display_flags.bits() as i16;
            self
        }

 *
}*/

// Float Slider
define_param_wrapper!(FloatSliderDef, PF_FloatSliderDef, slider_def);
define_param_basic_wrapper!(FloatSliderDef, PF_FloatSliderDef, slider_def, f64, f32);
define_param_valid_min_max_wrapper!(FloatSliderDef, slider_def, f32);
define_param_slider_min_max_wrapper!(FloatSliderDef, slider_def, f32);
define_param_value_desc_wrapper!(FloatSliderDef, slider_def);

impl FloatSliderDef {
    pub fn display_flags<'a>(&'a mut self, display_flags: ValueDisplayFlag) -> &'a mut Self {
        self.slider_def.display_flags = display_flags.bits() as i16;
        self
    }

    pub fn precision<'a>(&'a mut self, precision: u8) -> &'a mut Self {
        self.slider_def.precision = precision as i16;
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_FLOAT_SLIDER == param.param_def_boxed.param_type {
            Some(Self {
                slider_def: unsafe { param.param_def_boxed.u.fs_d },
            })
        } else {
            None
        }
    }

    pub fn get(&self) -> f64 {
        self.slider_def.value
    }
}

// Checkbox

// PF_CheckBoxDef does not implement Debug trait so we can't use
// the define_param_basic_wrapper!() macro.
#[repr(C)]
#[derive(Clone)]
pub struct CheckBoxDef {
    check_box_def: ae_sys::PF_CheckBoxDef,
    label: CString,
}

impl CheckBoxDef {
    pub fn new() -> Self {
        Self {
            check_box_def: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            label: CString::new("").unwrap(),
        }
    }

    pub fn label<'a>(&'a mut self, label: &str) -> &'a mut Self {
        self.label = CString::new(label).unwrap();
        self.check_box_def.u.nameptr = self.label.as_ptr();
        self
    }

    pub fn from(param: &ParamDef) -> Option<Self> {
        if ae_sys::PF_Param_CHECKBOX == param.param_def_boxed.param_type {
            Some(Self {
                check_box_def: unsafe { param.param_def_boxed.u.bd },
                label: unsafe { CString::from_raw(param.param_def_boxed.u.bd.u.nameptr as _) },
            })
        } else {
            None
        }
    }

    pub fn get(&self) -> bool {
        self.check_box_def.value != 0
    }
}

define_param_basic_wrapper!(CheckBoxDef, PF_CheckBoxDef, check_box_def, i32, bool);

#[repr(C)]
#[derive(Clone)]
pub enum ParamDefUnion {
    PopupDef(PopupDef),
    AngleDef(AngleDef),
    CheckBoxDef(CheckBoxDef),
    SliderDef(SliderDef),
    FloatSliderDef(FloatSliderDef),
    ColorDef(ColorDef),
    ButtonDef(ButtonDef),
    //FixedSliderDef(FixedSliderDef),
}

#[derive(Clone)]
#[repr(C)]
pub struct ParamDef {
    param_def_boxed: std::mem::ManuallyDrop<Box<ae_sys::PF_ParamDef>>,
    drop: bool,
    in_data_ptr: *const PF_InData,
}

impl ParamDef {
    pub fn new(in_data_handle: InDataHandle) -> Self {
        Self {
            param_def_boxed: std::mem::ManuallyDrop::new(unsafe { Box::new_zeroed().assume_init() }),
            drop: true,
            in_data_ptr: in_data_handle.as_ptr(),
        }
    }

    pub fn from_raw(in_data_ptr: *const PF_InData, param_def: *mut ae_sys::PF_ParamDef) -> Self {
        Self {
            param_def_boxed: unsafe { std::mem::ManuallyDrop::new(Box::from_raw(param_def)) },
            drop: false,
            in_data_ptr,
        }
    }

    pub fn add(&mut self, index: i32) {
        unsafe {
            (*self.in_data_ptr).inter.add_param.unwrap()(
                (*self.in_data_ptr).effect_ref,
                index,
                &mut **self.param_def_boxed,
            );
        }
        // Parameters we just added are not checked out
        // so they do not need to be checked in.
        self.drop = false;
    }

    pub fn checkout(
        in_data_handle: InDataHandle,
        index: i32,
        what_time: i32,
        time_step: i32,
        time_scale: u32,
    ) -> ParamDef {
        let mut param_def_boxed =
            std::mem::ManuallyDrop::new(unsafe { Box::<ae_sys::PF_ParamDef>::new_zeroed().assume_init() });
        let in_data_ptr = in_data_handle.as_ptr();
        unsafe {
            (*in_data_ptr).inter.checkout_param.unwrap()(
                (*in_data_ptr).effect_ref,
                index,
                what_time,
                time_step,
                time_scale,
                &mut **param_def_boxed,
            );
        }
        ParamDef {
            param_def_boxed,
            drop: true,
            in_data_ptr,
        }
    }

    pub fn do_not_checkin(&mut self) {
        self.drop = false;
    }

    pub fn param<'a>(&'a mut self, param: ParamDefUnion) -> &'a mut ParamDef {
        match param {
            ParamDefUnion::PopupDef(pd) => {
                self.param_def_boxed.u.pd = PopupDef::into_raw(pd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_POPUP;
            }
            ParamDefUnion::AngleDef(ad) => {
                self.param_def_boxed.u.ad = AngleDef::into_raw(ad);
                self.param_def_boxed.param_type = ae_sys::PF_Param_ANGLE;
            }
            ParamDefUnion::CheckBoxDef(bd) => {
                self.param_def_boxed.u.bd = CheckBoxDef::into_raw(bd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_BUTTON;
            }
            ParamDefUnion::ColorDef(cd) => {
                self.param_def_boxed.u.cd = ColorDef::into_raw(cd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_COLOR;
            }
            ParamDefUnion::SliderDef(sd) => {
                self.param_def_boxed.u.sd = SliderDef::into_raw(sd);
                self.param_def_boxed.param_type = ae_sys::PF_Param_SLIDER;
            }
            ParamDefUnion::FloatSliderDef(fs_d) => {
                self.param_def_boxed.u.fs_d = FloatSliderDef::into_raw(fs_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_FLOAT_SLIDER;
            } /*ParamDefUnion::FixedSliderDef(sd) => {
            self.param_def_boxed.u.fd = FixedSliderDef::into_raw(sd);
            self.param_def_boxed.param_type = ae_sys::PF_Param_FIX_SLIDER;
            }*/
            ParamDefUnion::ButtonDef(button_d) => {
                self.param_def_boxed.u.button_d = ButtonDef::into_raw(button_d);
                self.param_def_boxed.param_type = ae_sys::PF_Param_BUTTON;
            }
        }
        self
    }

    pub fn param_type<'a>(&'a mut self, param_type: ParamType) -> &'a mut ParamDef {
        self.param_def_boxed.param_type = param_type as i32;
        self
    }

    pub fn name<'a>(&'a mut self, name: &str) -> &'a mut ParamDef {
        assert!(name.len() < 32);
        let name_cstr = CString::new(name).unwrap();
        let name_slice = name_cstr.to_bytes_with_nul();
        self.param_def_boxed.name[0..name_slice.len()].copy_from_slice(unsafe { std::mem::transmute(name_slice) });
        self
    }

    pub fn ui_flags<'a>(&'a mut self, flags: i32) -> &'a mut ParamDef {
        self.param_def_boxed.ui_flags = flags;
        self
    }

    pub fn ui_width<'a>(&'a mut self, width: u16) -> &'a mut ParamDef {
        self.param_def_boxed.ui_width = width as i16;
        self
    }

    pub fn ui_height<'a>(&'a mut self, height: u16) -> &'a mut ParamDef {
        self.param_def_boxed.ui_height = height as i16;
        self
    }

    pub fn flags<'a>(&'a mut self, flags: ParamFlag) -> &'a mut ParamDef {
        self.param_def_boxed.flags = flags.bits() as i32;
        self
    }
}

impl Drop for ParamDef {
    fn drop(&mut self) {
        if self.drop {
            unsafe {
                (*self.in_data_ptr).inter.checkin_param.unwrap()(
                    (*self.in_data_ptr).effect_ref,
                    // ManuallyDrop ensures we do not double free the memory
                    // after passing the pointer to the box contents to the FFI.
                    &mut **self.param_def_boxed,
                );
            }
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.param_def_boxed);
        }
    }
}
