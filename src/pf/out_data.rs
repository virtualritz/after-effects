
use super::*;

// define_struct_wrapper!(OutData, PF_OutData);

// struct PF_OutData {
//     pub my_version: A_u_long,            // Set this flag (using the PF_VERSION macro) to the version of your plug-in code. After Effects uses this data to decide which of duplicate effects to load.
//     pub name: [A_char; 32usize],         // Unused.
//     pub global_data: PF_Handle,          // Handle which will be returned to you in PF_InData with every call. Use After Effects’ memory allocation functions.
//     pub num_params: A_long,              // After Effects checks this field against the number of calls made to PF_ADD_PARAM, as well as the implicit input layer.
//     pub sequence_data: PF_Handle,        // Allocatable upon receiving PF_Cmd_SEQUENCE_SETUP, this handle will be passed back to you in PF_InData during all subsequent calls.
//     pub flat_sdata_size: A_long,         // Unused (After Effects knows the size, because you used its allocation functions to get the memory in the first place).
//     pub frame_data: PF_Handle,           // Handle you (might have) allocated during PF_Cmd_FRAME_SETUP. This is never written to disk; it was used to pass information from your PF_Cmd_FRAME_SETUP response to your PF_Cmd_RENDER or PF_Cmd_FRAME_SETDOWN (which you must do if you resize the output buffer). Otherwise, this memory is rarely used.
//     pub width: A_long,                   // Set during PF_Cmd_FRAME_SETUP if the output image size differs from the input. width and height are the size of the output buffer, and origin is the point the input should map to in the output. To create a 5-pixel drop shadow up and left, set origin to (5, 5).
//     pub height: A_long,                  // --^
//     pub origin: PF_Point,                // --^
//     pub out_flags: PF_OutFlags,          // Send messages to After Effects. OR together multiple values.
//     pub return_msg: [A_char; 256usize],  // After Effects displays any C string you put here (checked and cleared after every command selector).
//     pub start_sampL: A_long,             // Used only for Audio commands
//     pub dur_sampL: A_long,               // --^
//     pub dest_snd: PF_SoundWorld,         // --^
//     pub out_flags2: PF_OutFlags2,        // Send messages to After Effects. OR together multiple values.
// }

#[derive(Clone, Copy, Debug)]
pub struct OutData {
    pub(crate) ptr: *mut ae_sys::PF_OutData,
}

impl AsRef<ae_sys::PF_OutData> for OutData {
    fn as_ref(&self) -> &ae_sys::PF_OutData {
        unsafe { &*self.ptr }
    }
}
impl AsMut<ae_sys::PF_OutData> for OutData {
    fn as_mut(&mut self) -> &mut ae_sys::PF_OutData {
        unsafe { &mut *self.ptr }
    }
}

impl OutData {
    pub fn from_raw(ptr: *mut ae_sys::PF_OutData) -> Self {
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *const ae_sys::PF_OutData {
        self.ptr
    }

    pub fn width(&self) -> u32 {
        self.as_ref().width as u32
    }
    pub fn set_width(&mut self, width: u32) {
        self.as_mut().width = width as ae_sys::A_long;
    }
    pub fn height(&self) -> u32 {
        self.as_ref().height as u32
    }
    pub fn set_height(&mut self, height: u32) {
        self.as_mut().height = height as ae_sys::A_long;
    }
    pub fn origin(&self) -> Point {
        self.as_ref().origin.into()
    }
    pub fn set_origin(&mut self, origin: Point) {
        self.as_mut().origin = origin.into();
    }
    pub fn set_return_msg(&mut self, msg: &str) {
        //let buf = std::ffi::CString::new(s).unwrap().into_bytes_with_nul();
        //self.return_msg[0..buf.len()].copy_from_slice(unsafe { std::mem::transmute(buf.as_slice()) });
        let msg = msg.as_bytes();
        assert!(msg.len() < 256);
        self.as_mut().return_msg[..msg.len()].copy_from_slice(unsafe { std::mem::transmute(msg) });
    }
    pub fn set_version(&mut self, v: u32) {
        self.as_mut().my_version = v as ae_sys::A_u_long;
    }
    pub fn set_out_flags(&mut self, v: i32) {
        self.as_mut().out_flags = v as ae_sys::PF_OutFlags;
    }
    pub fn set_out_flags2(&mut self, v: i32) {
        self.as_mut().out_flags2 = v as ae_sys::PF_OutFlags2;
    }
}
