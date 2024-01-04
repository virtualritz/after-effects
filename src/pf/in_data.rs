use super::*;
use std::any::Any;

// struct PF_InData {
//     pub inter: PF_InteractCallbacks,          // Callbacks used for user interaction, adding parameters, checking whether the user has interrupted the effect, displaying a progress bar, and obtaining source frames and parameter values at times other than the current time being rendered. This very useful function suite is described in Interaction Callback Functions.
//     pub utils: *mut _PF_UtilCallbacks,        // Graphical and mathematical callbacks. This pointer is defined at all times.
//     pub effect_ref: PF_ProgPtr,               // Opaque data that must be passed to most of the various callback routines. After Effects uses this to identify your plug-in.
//     pub quality: PF_Quality,                  // The current quality setting, either PF_Quality_HI or PF_Quality_LO. Effects should perform faster in LO, and more accurately in HI. The graphics utility callbacks perform differently between LO and HI quality; so should your effect! This field is defined during all frame and sequence selectors.
//     pub version: PF_SpecVersion,              // Effects specification version, Indicate the version you need to run successfully during PF_Cmd_GLOBAL_SETUP.
//     pub serial_num: A_long,                   // The serial number of the invoking application.
//     pub appl_id: A_long,                      // The identifier of the invoking application. If your plug-in is running in After Effects, appl_id contains the application creator code ‘FXTC’. If it is running in Premiere Pro & Other Hosts it will be ‘PrMr’. Use this to test whether your plug-in, licensed for use with one application, is being used with another.
//     pub num_params: A_long,                   // Input parameter count.
//     pub reserved: A_long,                     //
//     pub what_cpu: A_long,                     // Under macOS this contains the Gestalt value for CPU type (see Inside Macintosh, volume 6). Undefined on Windows.
//     pub what_fpu: A_long,                     // Under macOS this contains the Gestalt value for FPU type. Undefined on Windows.
//     pub current_time: A_long,                 // The time of the current frame being rendered, valid during PF_Cmd_RENDER. This is the current time in the layer, not in any composition. If a layer starts at other than time 0 or is time-stretched, layer time and composition time are distinct. The current frame number is current_time divided by time_step. The current time in seconds is current_time divided by time_scale. To handle time stretching, composition frame rate changes, and time remapping, After Effects may ask effects to render at non-integral times (between two frames). Be prepared for this; don’t assume that you’ll only be asked for frames on frame boundaries. NOTE: As of CS3 (8.0), effects may be asked to render at negative current times. Deal!
//     pub time_step: A_long,                    // The duration of the current source frame being rendered. In several situations with nested compositions, this source frame duration may be different than the time span between frames in the layer (local_time_step). This value can be converted to seconds by dividing by time_scale. When calculating other source frame times, such as for PF_CHECKOUT_PARAM, use this value rather than local_time_step. Can be negative if the layer is time-reversed. Can vary from one frame to the next if time remapping is applied on a nested composition. Can differ from local_time_step when source material is stretched or remapped in a nested composition. For example, this could occur when an inner composition is nested within an outer composition with a different frame rate, or time remapping is applied to the outer composition. This value will be 0 during PF_Cmd_SEQUENCE_SETUP if it is not constant for all frames. It will be set correctly during PF_Cmd_FRAME_SETUP and PF_Cmd_FRAME_SETDOWN selectors. WARNING: This can be zero, so check it before you divide.
//     pub total_time: A_long,                   // Duration of the layer. If the layer is time-stretched longer than 100%, the value will be adjusted accordingly; but if the layer is time-stretched shorter, the value will not be affected. If time remapping is enabled, this value will be the duration of the composition. This value can be converted to seconds by dividing by time_scale.
//     pub local_time_step: A_long,              // Time difference between frames in the layer. Affected by any time stretch applied to a layer. Can be negative if the layer is time-reversed. Unlike time_step, this value is constant from one frame to the next. This value can be converted to seconds by dividing by time_scale. For a step value that is constant over the entire frame range of the layer, use local_time_step, which is based on the composition’s framerate and layer stretch.
//     pub time_scale: A_u_long,                 // The units per second that current_time, time_step, local_time_step and total_time are in. If time_scale is 30, then the units of current_time, time_step, local_time_step and total_time are in 30ths of a second. The time_step might then be 3, indicating that the sequence is actually being rendered at 10 frames per second. total_time might be 105, indicating that the sequence is 3.5 seconds long.
//     pub field: PF_Field,                      // Valid only if PF_OutFlag_PIX_INDEPENDENT was set during PF_Cmd_GLOBAL_SETUP. Check this field to see if you can process just the upper or lower field.
//     pub shutter_angle: PF_Fixed,              // Motion blur shutter angle. Values range from 0 to 1, which represents 360 degrees. Will be zero unless motion blur is enabled and checked for the target layer. shutter_angle == 180 means the time interval between current_time and current_time + 1/2 time_step. Valid only if PF_OutFlag_I_USE_SHUTTER_ANGLE was set during PF_Cmd_GLOBAL_SETUP. See the section on Motion Blur for details on how to implement motion blur in your effect.
//     pub width: A_long,                        // Dimensions of the source layer, which are not necessarily the same as the width and height fields in the input image parameter. Buffer resizing effects can cause this difference. Not affected by downsampling.
//     pub height: A_long,                       //
//     pub extent_hint: PF_Rect,                 // The intersection of the visible portions of the input and output layers; encloses the composition rectangle transformed into layer coordinates. Iterating over only this rectangle of pixels can speed your effect dramatically. See extent_hint Usage later in this chapter regarding proper usage.
//     pub output_origin_x: A_long,              // The origin of the output buffer in the input buffer. Non-zero only when the effect changes the origin.
//     pub output_origin_y: A_long,              //
//     pub downsample_x: PF_RationalScale,       // Point control parameters and layer parameter dimensions are automatically adjusted to compensate for a user telling After Effects to render only every nth pixel. Effects need the downsampling factors to interpret scalar parameters representing pixel distances in the image (like sliders). For example, a blur of 4 pixels should be interpreted as a blur of 2 pixels if the downsample factor is 1/2 in each direction (downsample factors are represented as ratios.) Valid only during PF_Cmd_SEQUENCE_SETUP, PF_Cmd_SEQUENCE_RESETUP, PF_Cmd_FRAME_SETUP, PF_Cmd_RENDER
//     pub downsample_y: PF_RationalScale,       //
//     pub pixel_aspect_ratio: PF_RationalScale, // Pixel aspect ratio (width over height).
//     pub in_flags: PF_InFlags,                 // Unused.
//     pub global_data: PF_Handle,               // Data stored by your plug-in during other selectors. Locked and unlocked by After Effects before and after calling the plug-in.
//     pub sequence_data: PF_Handle,             //
//     pub frame_data: PF_Handle,                //
//     pub start_sampL: A_long,                  // Starting sample number, relative to the start of the audio layer
//     pub dur_sampL: A_long,                    // Duration of audio, expressed as the number of samples. Audio-specific.
//     pub total_sampL: A_long,                  // Samples in the audio layer; equivalent to total_time expressed in samples.
//     pub src_snd: PF_SoundWorld,               // PF_SoundWorld describing the input sound. Audio-specific.
//     pub pica_basicP: *mut SPBasicSuite,       // Pointer to the PICA Basic suite, used to acquire other suites.
//     pub pre_effect_source_origin_x: A_long,   // Origin of the source image in the input buffer. Valid only when sent with a frame selector. Non-zero only if one or more effects that preceded this effect on the same layer resized the output buffer and moved the origin. Check for both the resize and the new origin to determine output area. This is useful for effects which have implicit spatial operations (other than point controls), like flipping a file around an image’s center. NOTE: Checked-out point parameters are adjusted for the pre-effect origin at the current time, not the time being checked out.
//     pub pre_effect_source_origin_y: A_long,   //
//     pub shutter_phase: PF_Fixed,              // Offset from frame time to shutter open time as a percentage of a frame duration.
// }

#[derive(Clone, Copy, Debug)]
pub struct InData {
    pub(crate) ptr: *const ae_sys::PF_InData,
}

impl AsRef<ae_sys::PF_InData> for InData {
    fn as_ref(&self) -> &ae_sys::PF_InData {
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

    #[inline]
    pub fn application_id(&self) -> [u8; 4] {
        let bytes: [u8; 4] = unsafe { std::mem::transmute((*self.ptr).appl_id) };
        [bytes[3], bytes[2], bytes[1], bytes[0]]
    }

    #[inline]
    pub fn extent_hint(&self) -> Rect {
        Rect::from(unsafe { (*self.ptr).extent_hint })
    }

    #[inline]
    pub fn effect_ref(&self) -> ProgressInfo {
        pf::ProgressInfo(unsafe { (*self.ptr).effect_ref })
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.ptr).width }
    }
    pub fn height(&self) -> i32 {
        unsafe { (*self.ptr).height }
    }
    pub fn current_frame(&self) -> f32 {
        unsafe { (*self.ptr).current_time as f32 / (*self.ptr).time_step as f32 }
    }
    pub fn current_timestamp(&self) -> f32 {
        unsafe { (*self.ptr).current_time as f32 / (*self.ptr).time_scale as f32 }
    }
    pub fn current_time(&self) -> i32 {
        unsafe { (*self.ptr).current_time }
    }
    pub fn time_step(&self) -> i32 {
        unsafe { (*self.ptr).time_step }
    }
    pub fn time_scale(&self) -> u32 {
        unsafe { (*self.ptr).time_scale }
    }

    #[inline]
    pub fn version(&self) -> (i16, i16) {
        unsafe { ((*self.ptr).version.major, (*self.ptr).version.minor) }
    }

    pub fn frame_data_mut<'a, T: Any>(&'a mut self) -> Option<&'a mut T> {
        assert!(!self.ptr.is_null());
        if unsafe { (*self.ptr).frame_data.is_null() } {
            return None;
        }
        let data = unsafe { Box::<Box<dyn Any>>::from_raw((*self.ptr).frame_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_mut::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for frame_data"),
        }
    }
    pub fn frame_data<'a, T: Any>(&'a self) -> Option<&'a T> {
        assert!(!self.ptr.is_null());
        if unsafe { (*self.ptr).frame_data.is_null() } {
            return None;
        }
        let data = unsafe { Box::<Box<dyn Any>>::from_raw((*self.ptr).frame_data as *mut _) };
        let data = Box::<Box<dyn Any>>::leak(data);
        match data.downcast_ref::<T>() {
            Some(data) => Some(data),
            None => panic!("Invalid type for frame_data"),
        }
    }
    // Only valid at Command::FrameSetdown
    pub fn destroy_frame_data<T: Any>(&mut self) {
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
}
