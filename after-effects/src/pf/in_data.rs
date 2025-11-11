use super::*;
use std::any::Any;

// struct PF_InData {
//     pub serial_num: A_long,                   // The serial number of the invoking application.
//     pub num_params: A_long,                   // Input parameter count.
//     pub shutter_angle: PF_Fixed,              // Motion blur shutter angle. Values range from 0 to 1, which represents 360 degrees. Will be zero unless motion blur is enabled and checked for the target layer. shutter_angle == 180 means the time interval between current_time and current_time + 1/2 time_step. Valid only if PF_OutFlag_I_USE_SHUTTER_ANGLE was set during [`Command::GlobalSetup`]. See the section on Motion Blur for details on how to implement motion blur in your effect.
//     pub start_sampL: A_long,                  // Starting sample number, relative to the start of the audio layer
//     pub dur_sampL: A_long,                    // Duration of audio, expressed as the number of samples. Audio-specific.
//     pub total_sampL: A_long,                  // Samples in the audio layer; equivalent to total_time expressed in samples.
//     pub src_snd: PF_SoundWorld,               // PF_SoundWorld describing the input sound. Audio-specific.
//     pub shutter_phase: PF_Fixed,              // Offset from frame time to shutter open time as a percentage of a frame duration.
//     ...
// }

#[derive(Clone, Copy, Debug)]
pub struct InData {
    pub(crate) ptr: *const ae_sys::PF_InData,
}

impl AsRef<ae_sys::PF_InData> for InData {
    fn as_ref(&self) -> &ae_sys::PF_InData {
        // SAFETY: Dereferencing self.ptr to create a reference.
        // Detailed explanation: (1) self.ptr is guaranteed non-null by from_raw(), (2) points to a valid PF_InData
        // structure provided by the Adobe AE SDK during plugin callbacks, (3) the SDK maintains ownership and validity
        // for the duration of the callback, ensuring the lifetime 'a doesn't outlive the pointed data.
        // Would be UB if: self.ptr was null, dangling, or the PF_InData was deallocated while the reference exists.
        unsafe { &*self.ptr }
    }
}

impl InData {
    pub fn from_raw(ptr: *const ae_sys::PF_InData) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_InData {
        self.ptr
    }

    /// The identifier of the invoking application.
    /// If your plug-in is running in After Effects, appl_id contains the application creator code 'FXTC'.
    /// If it is running in Premiere Pro & Other Hosts it will be 'PrMr'.
    /// Use this to test whether your plug-in, licensed for use with one application, is being used with another.
    pub fn application_id(&self) -> [u8; 4] {
        // SAFETY: Reading appl_id field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) appl_id is a plain
        // i32 field in the C struct so dereferencing and reading it is safe, (3) the value is copied (not borrowed)
        // so no lifetime concerns exist.
        // Would be UB if: self.ptr was null, dangling, or pointed to uninitialized memory.
        let bytes: [u8; 4] = unsafe { i32::to_ne_bytes((*self.ptr).appl_id) };
        [bytes[3], bytes[2], bytes[1], bytes[0]]
    }

    /// Checks if the plugin is running in Adobe Premiere Pro
    pub fn is_premiere(&self) -> bool {
        // SAFETY: Reading and comparing appl_id field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) accessing the appl_id
        // field of a properly aligned and initialized PF_InData struct is safe, (3) comparison operation doesn't create
        // any references, just copies the i32 value.
        // Would be UB if: self.ptr was null, dangling, or the appl_id field was uninitialized.
        unsafe { (*self.ptr).appl_id == i32::from_be_bytes(*b"PrMr") }
    }

    /// Checks if the plugin is running in Adobe After Effects
    pub fn is_after_effects(&self) -> bool {
        // SAFETY: Reading and comparing appl_id field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) accessing the appl_id
        // field of a properly aligned and initialized PF_InData struct is safe, (3) comparison operation doesn't create
        // any references, just copies the i32 value.
        // Would be UB if: self.ptr was null, dangling, or the appl_id field was uninitialized.
        unsafe { (*self.ptr).appl_id == i32::from_be_bytes(*b"FXTC") }
    }

    /// The current quality setting, either PF_Quality_HI or PF_Quality_LO. Effects should perform faster in LO, and more accurately in HI.
    /// The graphics utility callbacks perform differently between LO and HI quality; so should your effect! This field is defined during all frame and sequence selectors.
    pub fn quality(&self) -> Quality {
        // SAFETY: Reading quality field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) quality is a C enum
        // field that is properly initialized by the AE SDK, (3) the value is copied and converted to Rust Quality enum.
        // Would be UB if: self.ptr was null, dangling, or quality field contained an invalid enum discriminant.
        unsafe { (*self.ptr).quality.into() }
    }

    /// Valid only if PF_OutFlag_PIX_INDEPENDENT was set during [`Command::GlobalSetup`].
    /// Check this field to see if you can process just the upper or lower field.
    pub fn field(&self) -> Field {
        // SAFETY: Reading field enum from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) field is a C enum
        // properly initialized by the AE SDK, (3) the value is copied and converted to Rust Field enum.
        // Would be UB if: self.ptr was null, dangling, or field contained an invalid enum discriminant.
        unsafe { (*self.ptr).field.into() }
    }

    /// The intersection of the visible portions of the input and output layers; encloses the composition rectangle transformed into layer coordinates.
    /// Iterating over only this rectangle of pixels can speed your effect dramatically. See extent_hint Usage later in this chapter regarding proper usage.
    pub fn extent_hint(&self) -> Rect {
        // SAFETY: Reading extent_hint struct from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) extent_hint is a
        // properly initialized C struct (PF_Rect) provided by the AE SDK, (3) the value is copied into a Rust Rect.
        // Would be UB if: self.ptr was null, dangling, or extent_hint contained uninitialized or invalid data.
        Rect::from(unsafe { (*self.ptr).extent_hint })
    }

    /// Create the [`Effect`] instance to get access to various utility functions.
    pub fn effect(&self) -> Effect {
        // SAFETY: Reading effect_ref pointer field from PF_InData.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) effect_ref is an
        // opaque pointer provided and maintained by the AE SDK throughout the plugin callback, (3) the pointer value
        // is copied (not dereferenced here) and wrapped in Effect which will handle its own safety invariants.
        // Would be UB if: self.ptr was null or dangling; effect_ref validity is enforced by Effect::from_raw.
        Effect::from_raw(unsafe { (*self.ptr).effect_ref })
    }

    /// Opaque data that must be passed to most of the various callback routines.
    /// After Effects uses this to identify your plug-in.
    pub fn effect_ref(&self) -> EffectHandle {
        // SAFETY: Reading effect_ref pointer field from PF_InData.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) effect_ref is an
        // opaque handle provided by the AE SDK to identify the plugin instance, (3) the pointer value is copied and
        // wrapped in EffectHandle which maintains its own safety invariants.
        // Would be UB if: self.ptr was null or dangling; effect_ref validity is enforced by EffectHandle::from_raw.
        EffectHandle::from_raw(unsafe { (*self.ptr).effect_ref })
    }

    /// Returns the pointer to Pica Basic Suite
    pub fn pica_basic_suite_ptr(&self) -> *mut ae_sys::SPBasicSuite {
        // SAFETY: Reading pica_basicP pointer field from PF_InData.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) pica_basicP is a
        // function suite pointer provided by the AE SDK for accessing basic services, (3) returning the raw pointer
        // without dereferencing it, so caller is responsible for validating before use.
        // Would be UB if: self.ptr was null or dangling; the returned pointer's validity is caller's responsibility.
        unsafe { (*self.ptr).pica_basicP }
    }

    /// Width of the source layer, which are not necessarily the same as the width and height fields in the input image parameter.
    /// Buffer resizing effects can cause this difference. Not affected by downsampling.
    pub fn width(&self) -> i32 {
        // SAFETY: Reading width field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) width is a plain i32
        // field initialized by the AE SDK with the source layer dimensions, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or width was uninitialized.
        unsafe { (*self.ptr).width }
    }

    /// Height of the source layer, which are not necessarily the same as the width and height fields in the input image parameter.
    /// Buffer resizing effects can cause this difference. Not affected by downsampling.
    pub fn height(&self) -> i32 {
        // SAFETY: Reading height field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) height is a plain i32
        // field initialized by the AE SDK with the source layer dimensions, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or height was uninitialized.
        unsafe { (*self.ptr).height }
    }

    /// The frame number of the current frame being rendered, valid during [`Command::Render`].
    /// This is the current frame in the layer, not in any composition.
    /// It's the result of `current_time()` divided by `time_step()`.
    pub fn current_frame(&self) -> f64 {
        // SAFETY: Reading time_step and current_time fields from PF_InData for frame calculation.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) both time_step and
        // current_time are plain i32 fields initialized by the AE SDK, (3) values are copied for computation with
        // division-by-zero protection, (4) no references or lifetimes involved.
        // Would be UB if: self.ptr was null, dangling, or these fields were uninitialized.
        unsafe {
            if (*self.ptr).time_step == 0 {
                0.0
            } else {
                (*self.ptr).current_time as f64 / (*self.ptr).time_step as f64
            }
        }
    }

    /// The frame number of the current frame being rendered, valid during [`Command::Render`].
    /// This is the current frame in the layer, not in any composition.
    /// It's the result of `current_time()` divided by `local_time_step()`.
    pub fn current_frame_local(&self) -> f64 {
        // SAFETY: Reading local_time_step and current_time fields from PF_InData for frame calculation.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) both local_time_step
        // and current_time are plain i32 fields initialized by the AE SDK, (3) values are copied for computation with
        // division-by-zero protection, (4) no references or lifetimes involved.
        // Would be UB if: self.ptr was null, dangling, or these fields were uninitialized.
        unsafe {
            if (*self.ptr).local_time_step == 0 {
                0.0
            } else {
                (*self.ptr).current_time as f64 / (*self.ptr).local_time_step as f64
            }
        }
    }

    /// The current timestamp (in seconds) of the current frame being rendered, valid during [`Command::Render`].
    /// This is the current time in the layer, not in any composition.
    /// It's the result of `current_time()` divided by `time_scale()`.
    pub fn current_timestamp(&self) -> f64 {
        // SAFETY: Reading current_time and time_scale fields from PF_InData for timestamp calculation.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) current_time is i32
        // and time_scale is u32, both initialized by the AE SDK, (3) values are copied and converted to f64 for
        // division, (4) time_scale is guaranteed non-zero by AE SDK contract.
        // Would be UB if: self.ptr was null, dangling, or these fields were uninitialized.
        unsafe { (*self.ptr).current_time as f64 / (*self.ptr).time_scale as f64 }
    }

    /// The time of the current frame being rendered, valid during [`Command::Render`].
    /// This is the current time in the layer, not in any composition.
    /// If a layer starts at other than time 0 or is time-stretched, layer time and composition time are distinct.
    /// The current frame number is current_time divided by time_step. The current time in seconds is current_time divided by `time_scale`.
    /// To handle time stretching, composition frame rate changes, and time remapping, After Effects may ask effects to render at non-integral times (between two frames).
    /// Be prepared for this; don't assume that you'll only be asked for frames on frame boundaries.
    /// NOTE: As of CS3 (8.0), effects may be asked to render at negative current times. Deal!
    pub fn current_time(&self) -> i32 {
        // SAFETY: Reading current_time field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) current_time is a
        // plain i32 field representing the current render time in time_scale units, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or current_time was uninitialized.
        unsafe { (*self.ptr).current_time }
    }

    /// The duration of the current source frame being rendered. In several situations with nested compositions, this source frame duration may be
    /// different than the time span between frames in the layer (`local_time_step`).
    /// This value can be converted to seconds by dividing by `time_scale`.
    /// When calculating other source frame times, such as for PF_CHECKOUT_PARAM, use this value rather than `local_time_step`.
    /// Can be negative if the layer is time-reversed. Can vary from one frame to the next if time remapping is applied on a nested composition.
    /// Can differ from `local_time_step` when source material is stretched or remapped in a nested composition.
    /// For example, this could occur when an inner composition is nested within an outer composition with a different frame rate,
    /// or time remapping is applied to the outer composition. This value will be 0 during [`Command::SequenceSetup`] if it is not constant for all frames.
    /// It will be set correctly during [`Command::FrameSetup`] and [`Command::FrameSetdown`] selectors.
    /// WARNING: This can be zero, so check it before you divide.
    pub fn time_step(&self) -> i32 {
        // SAFETY: Reading time_step field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) time_step is a plain
        // i32 field representing frame duration, initialized by the AE SDK, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or time_step was uninitialized.
        unsafe { (*self.ptr).time_step }
    }

    /// Time difference between frames in the layer. Affected by any time stretch applied to a layer.
    /// Can be negative if the layer is time-reversed.
    /// Unlike time_step, this value is constant from one frame to the next.
    /// This value can be converted to seconds by dividing by `time_scale`.
    /// For a step value that is constant over the entire frame range of the layer, use `local_time_step`, which is based on the composition's framerate and layer stretch.
    pub fn local_time_step(&self) -> i32 {
        // SAFETY: Reading local_time_step field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) local_time_step is a
        // plain i32 field representing constant frame-to-frame time difference, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or local_time_step was uninitialized.
        unsafe { (*self.ptr).local_time_step }
    }

    /// The units per second that `current_time`, `time_step`, `local_time_step` and `total_time` are in.
    /// If `time_scale` is 30, then the units of `current_time`, `time_step`, `local_time_step` and `total_time` are in 30ths of a second.
    /// The `time_step` might then be 3, indicating that the sequence is actually being rendered at 10 frames per second. `total_time` might be 105, indicating that the sequence is 3.5 seconds long.
    pub fn time_scale(&self) -> u32 {
        // SAFETY: Reading time_scale field from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) time_scale is a plain
        // u32 field representing time units per second, initialized by the AE SDK, (3) the value is copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or time_scale was uninitialized.
        unsafe { (*self.ptr).time_scale }
    }

    /// Origin of the source image in the input buffer.
    /// Valid only when sent with a frame selector.
    /// Non-zero only if one or more effects that preceded this effect on the same layer resized the output buffer and moved the origin.
    /// Check for both the resize and the new origin to determine output area.
    /// This is useful for effects which have implicit spatial operations (other than point controls), like flipping a file around an image's center.
    /// NOTE: Checked-out point parameters are adjusted for the pre-effect origin at the current time, not the time being checked out.
    pub fn pre_effect_source_origin(&self) -> Point {
        Point {
            // SAFETY: Reading pre_effect_source_origin_x field from PF_InData pointer.
            // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) pre_effect_source_origin_x
            // is a plain i32 field for horizontal origin offset, (3) the value is copied (Copy type).
            // Would be UB if: self.ptr was null, dangling, or this field was uninitialized.
            h: unsafe { (*self.ptr).pre_effect_source_origin_x },
            // SAFETY: Reading pre_effect_source_origin_y field from PF_InData pointer.
            // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) pre_effect_source_origin_y
            // is a plain i32 field for vertical origin offset, (3) the value is copied (Copy type).
            // Would be UB if: self.ptr was null, dangling, or this field was uninitialized.
            v: unsafe { (*self.ptr).pre_effect_source_origin_y },
        }
    }

    /// The origin of the output buffer in the input buffer. Non-zero only when the effect changes the origin.
    pub fn output_origin(&self) -> Point {
        Point {
            // SAFETY: Reading output_origin_x field from PF_InData pointer.
            // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) output_origin_x is
            // a plain i32 field for horizontal output origin, (3) the value is copied (Copy type).
            // Would be UB if: self.ptr was null, dangling, or this field was uninitialized.
            h: unsafe { (*self.ptr).output_origin_x },
            // SAFETY: Reading output_origin_y field from PF_InData pointer.
            // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) output_origin_y is
            // a plain i32 field for vertical output origin, (3) the value is copied (Copy type).
            // Would be UB if: self.ptr was null, dangling, or this field was uninitialized.
            v: unsafe { (*self.ptr).output_origin_y },
        }
    }

    /// Pixel aspect ratio (width over height).
    pub fn pixel_aspect_ratio(&self) -> RationalScale {
        // SAFETY: Reading pixel_aspect_ratio struct from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) pixel_aspect_ratio is
        // a C struct (PF_Rational) properly initialized by the AE SDK, (3) the value is copied and converted to RationalScale.
        // Would be UB if: self.ptr was null, dangling, or pixel_aspect_ratio was uninitialized.
        unsafe { (*self.ptr).pixel_aspect_ratio.into() }
    }

    /// Point control parameters and layer parameter dimensions are automatically adjusted to compensate for a user telling After Effects to render only every nth pixel.
    /// Effects need the downsampling factors to interpret scalar parameters representing pixel distances in the image (like sliders).
    /// For example, a blur of 4 pixels should be interpreted as a blur of 2 pixels if the downsample factor is 1/2 in each direction (downsample factors are represented as ratios.)
    /// Valid only during [`Command::SequenceSetup`], [`Command::SequenceResetup`], [`Command::FrameSetup`], [`Command::Render`]
    pub fn downsample_x(&self) -> RationalScale {
        // SAFETY: Reading downsample_x struct from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) downsample_x is a C
        // struct (PF_Rational) representing horizontal downsampling factor, (3) the value is copied and converted to RationalScale.
        // Would be UB if: self.ptr was null, dangling, or downsample_x was uninitialized.
        unsafe { (*self.ptr).downsample_x.into() }
    }

    /// Point control parameters and layer parameter dimensions are automatically adjusted to compensate for a user telling After Effects to render only every nth pixel.
    /// Effects need the downsampling factors to interpret scalar parameters representing pixel distances in the image (like sliders).
    /// For example, a blur of 4 pixels should be interpreted as a blur of 2 pixels if the downsample factor is 1/2 in each direction (downsample factors are represented as ratios.)
    /// Valid only during [`Command::SequenceSetup`], [`Command::SequenceResetup`], [`Command::FrameSetup`], [`Command::Render`]
    pub fn downsample_y(&self) -> RationalScale {
        // SAFETY: Reading downsample_y struct from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) downsample_y is a C
        // struct (PF_Rational) representing vertical downsampling factor, (3) the value is copied and converted to RationalScale.
        // Would be UB if: self.ptr was null, dangling, or downsample_y was uninitialized.
        unsafe { (*self.ptr).downsample_y.into() }
    }

    /// Effects specification version, Indicate the version you need to run successfully during [`Command::GlobalSetup`].
    #[inline]
    pub fn version(&self) -> (i16, i16) {
        // SAFETY: Reading version struct fields from PF_InData pointer.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) version is a C struct
        // with major and minor i16 fields initialized by the AE SDK, (3) both values are copied (Copy type).
        // Would be UB if: self.ptr was null, dangling, or version fields were uninitialized.
        unsafe { ((*self.ptr).version.major, (*self.ptr).version.minor) }
    }

    /// Frame data stored by your plug-in during other selectors. Locked and unlocked by After Effects before and after calling the plug-in.
    pub fn frame_data_mut<'a, T: Any>(&'a mut self) -> Option<&'a mut T> {
        assert!(!self.ptr.is_null());
        // SAFETY: Reading frame_data pointer field to check if it's null.
        // Detailed explanation: (1) self.ptr is non-null (just asserted), (2) frame_data is a void pointer field that
        // may be null or point to plugin-allocated data, (3) only checking null status, not dereferencing.
        // Would be UB if: self.ptr was null or dangling.
        if unsafe { (*self.ptr).frame_data.is_null() } {
            return None;
        }
        // SAFETY: Reconstructing Box from frame_data raw pointer to access plugin's stored data.
        // Detailed explanation: (1) frame_data is non-null (checked above), (2) points to a Box<Box<dyn Any>> that was
        // previously leaked via Box::leak in a prior call, (3) immediately leak it again to avoid double-free since
        // the plugin maintains ownership, (4) the AE SDK guarantees frame_data persists throughout frame processing.
        // Would be UB if: frame_data wasn't originally created via Box::into_raw of Box<Box<dyn Any>>, was already freed,
        // or pointed to invalid memory.
        let data = unsafe { Box::<Box<dyn Any>>::from_raw((*self.ptr).frame_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_mut::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for frame_data"),
        }
    }

    /// Frame data stored by your plug-in during other selectors. Locked and unlocked by After Effects before and after calling the plug-in.
    pub fn frame_data<'a, T: Any>(&'a self) -> Option<&'a T> {
        assert!(!self.ptr.is_null());
        // SAFETY: Reading frame_data pointer field to check if it's null.
        // Detailed explanation: (1) self.ptr is non-null (just asserted), (2) frame_data is a void pointer field that
        // may be null or point to plugin-allocated data, (3) only checking null status, not dereferencing.
        // Would be UB if: self.ptr was null or dangling.
        if unsafe { (*self.ptr).frame_data.is_null() } {
            return None;
        }
        // SAFETY: Reconstructing Box from frame_data raw pointer to access plugin's stored data.
        // Detailed explanation: (1) frame_data is non-null (checked above), (2) points to a Box<Box<dyn Any>> that was
        // previously leaked via Box::leak in a prior call, (3) immediately leak it again to avoid double-free since
        // the plugin maintains ownership, (4) the AE SDK guarantees frame_data persists throughout frame processing.
        // Would be UB if: frame_data wasn't originally created via Box::into_raw of Box<Box<dyn Any>>, was already freed,
        // or pointed to invalid memory.
        let data = unsafe { Box::<Box<dyn Any>>::from_raw((*self.ptr).frame_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_ref::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for frame_data"),
        }
    }

    /// Only valid at [`Command::FrameSetdown`]
    pub fn destroy_frame_data<T: Any>(&self) {
        // SAFETY: Deallocating frame_data at the end of frame processing.
        // Detailed explanation: (1) self.ptr is non-null and valid (enforced by from_raw), (2) checking if frame_data
        // is non-null before attempting deallocation, (3) reconstructing the Box<Box<dyn Any>> from the raw pointer
        // to properly drop and deallocate it, (4) this is only called during Command::FrameSetdown when the AE SDK
        // expects the plugin to release frame-specific resources, (5) the Box goes out of scope and is properly dropped.
        // Would be UB if: self.ptr was null/dangling, frame_data wasn't created via Box::into_raw, was already freed,
        // or this is called at the wrong time (not during FrameSetdown).
        unsafe {
            if !(*self.ptr).frame_data.is_null() {
                let data = Box::<Box<dyn Any>>::from_raw((*self.ptr).frame_data as *mut _);
                match data.downcast::<T>() {
                    Ok(_) => {}
                    Err(e) => panic!("Invalid type for frame_data: {e:?}"),
                }
            }
        }
    }

    /// Allows access to functions in the [`InteractCallbacks`] struct.
    pub fn interact(&self) -> InteractCallbacks {
        InteractCallbacks::new(*self)
    }

    /// Allows access to functions in the [`UtilCallbacks`] struct.
    pub fn utils(&self) -> UtilCallbacks {
        UtilCallbacks::new(*self)
    }
}

impl AsPtr<*const ae_sys::PF_InData> for *const ae_sys::PF_InData {
    fn as_ptr(&self) -> *const ae_sys::PF_InData {
        *self
    }
}
impl AsPtr<*const ae_sys::PF_InData> for InData {
    fn as_ptr(&self) -> *const ae_sys::PF_InData {
        self.ptr
    }
}
impl AsPtr<*const ae_sys::PF_InData> for &InData {
    fn as_ptr(&self) -> *const ae_sys::PF_InData {
        self.ptr
    }
}
