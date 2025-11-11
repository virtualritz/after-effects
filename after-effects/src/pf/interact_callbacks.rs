use crate::*;

/// Effects modules use callbacks to define their parameters.
/// When invoked, they will be given the parameters values at the particular invocation moment,
/// but some effects may need to ask for the parameter values at other times (notably of layer parameters for, say, a visual echo).
///
/// While running, effects modules are responsible for checking for user interrupts.
/// This checking can be done with either the abort callback, which will return a value indicating if the user has taken any action,
/// or with the progress callback, which performs user interrupt checking just like the abort callback, and also displays a progress display.
///
/// At the bottom of this section are macros for accessing these callback routines.
/// The first parameter to each macro is a pointer to a PF_InData structure, defined below.
/// This pointer will be passed to your effect.
pub struct InteractCallbacks(InData);

impl InteractCallbacks {
    pub fn new(in_data: InData) -> Self {
        Self(in_data)
    }

    /// The checkout_param callback allows you to inquire param values at times other than the current one, and allows you to access layer params other
    /// than the default input layer and the output layer. See the notes on the "params" structure at the end of this file.
    ///
    /// If you checkout a layer parameter and the layer popup is currently set
    /// to `<none>`, the return value will be filled with zeros. You can check the "data" pointer. If it is `NULL`, then the layer param is set to `<none>`
    /// and you should do something like faking an all alpha zero layer or some such nonsense.
    ///
    /// IMPORTANT: Due to 13.5 threading changes, checking out a layer param that is not `<none>` inside of `UPDATE_PARAMS_UI` will return
    /// a frame with black pixels to avoid render requests and possible deadlock.
    /// In other selectors the actual render will be triggered as it did before.
    pub fn checkout_param(&self, index: i32, what_time: i32, time_step: i32, time_scale: u32) -> Result<ae_sys::PF_ParamDef, Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Zero-initializing PF_ParamDef FFI type for use as an out-parameter.
        // Detailed explanation: (1) PF_ParamDef is a repr(C) FFI struct from Adobe After Effects SDK where all-zero bytes represent a valid initial state, (2) we immediately set param_type to PF_Param_RESERVED before passing to FFI, (3) the zeroed value is then passed as a mutable reference to the checkout_param FFI function which fully initializes it before we read from it, (4) the value is only returned when the FFI call succeeds (no error).
        // Would be UB if: PF_ParamDef contained types with invalid zero-bit patterns (like non-nullable references), or if we read from uninitialized fields before the FFI call completes, or if the FFI call failed but we used the value anyway.
        let mut param: ae_sys::PF_ParamDef = unsafe { std::mem::zeroed() };
        param.param_type = ae_sys::PF_Param_RESERVED;
        // SAFETY: Calling Adobe After Effects SDK checkout_param FFI callback through function pointer.
        // Detailed explanation: (1) in_data.inter.checkout_param is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) all arguments are valid: effect_ref is SDK-provided, index/time parameters are primitive types, and param is a valid mutable reference, (4) the function follows the Adobe SDK's callback convention of returning 0 on success or an error code.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid or pointed to freed code, effect_ref was invalid, or param pointer was dangling.
        match unsafe { in_data.inter.checkout_param.unwrap()(in_data.effect_ref, index as _, what_time as _, time_step as _, time_scale as _, &mut param) } {
            0 => Ok(param),
            e => Err(Error::from(e)),
        }
    }

    /// When you have called `checkout_param`, you must call `checkin_param` when you are done, so After Effects can clean up after itself and you.
    /// This is very important for smooth functioning and also to save memory where possible.
    /// Once checked in, the fields in the `PF_ParamDef` will no longer be valid.
    pub fn checkin_param(&self, param: &ae_sys::PF_ParamDef) -> Result<(), Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Calling Adobe After Effects SDK checkin_param FFI callback through function pointer with const-to-mut cast.
        // Detailed explanation: (1) in_data.inter.checkin_param is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) effect_ref is SDK-provided and valid, (4) param is a valid reference to a PF_ParamDef that was previously checked out, (5) the const-to-mut cast is safe because the Adobe SDK's checkin_param function doesn't actually modify the param data despite the signature (it's a legacy C API quirk), (6) the function follows the Adobe SDK's callback convention of returning 0 on success or an error code.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid, effect_ref was invalid, param pointer was dangling, or if the param was not previously checked out.
        match unsafe { in_data.inter.checkin_param.unwrap()(in_data.effect_ref, param as *const _ as *mut _) } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    /// When given the [`Command::ParamsSetup`] message, the effect will generally make a series of calls to the `add_param` routine
    /// to define the interface that the After Effects user will see. See the `PF_ParamDefs` defined above.
    /// Currently you can only add params at the end, and only at [`Command::ParamsSetup`] time.
    pub fn add_param(&self, index: i32, def: &ae_sys::PF_ParamDef) -> Result<(), Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Calling Adobe After Effects SDK add_param FFI callback through function pointer with const-to-mut cast.
        // Detailed explanation: (1) in_data.inter.add_param is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) effect_ref is SDK-provided and valid, (4) index is a primitive type parameter, (5) def is a valid reference to a properly initialized PF_ParamDef, (6) the const-to-mut cast is required by the Adobe SDK's C API signature though the function may internally copy the param definition, (7) the function follows the Adobe SDK's callback convention of returning 0 on success or an error code.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid, effect_ref was invalid, or def pointer was dangling.
        match unsafe { in_data.inter.add_param.unwrap()(in_data.effect_ref, index as _, def as *const _ as *mut _) } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    /// Periodically, you should check if the user wants to interrupt the current processing.
    /// The abort proc here will return non-zero if the effects module should suspend its current processing.
    /// If you call this routine and it returns a value other than zero, you should return that value when your effect returns.
    /// That will let us know if the effect completed rendering or not.
    pub fn abort(&self) -> Result<(), Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Calling Adobe After Effects SDK abort FFI callback through function pointer.
        // Detailed explanation: (1) in_data.inter.abort is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) effect_ref is SDK-provided and valid, identifying the current effect instance, (4) the function checks for user interrupt and follows the Adobe SDK's callback convention of returning 0 if no interrupt or an error code if user requested abort.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid or pointed to freed code, or effect_ref was invalid.
        match unsafe { in_data.inter.abort.unwrap()(in_data.effect_ref) } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    /// Alternatively, you may wish to display a progress bar while you are processing the image.
    /// This routine combines the abort proc user interrupt checking with code that will display a progress bar for you.
    /// The current and total params represent a fraction (current/total) that describes how far you are along in your processing.
    ///
    /// Current should equal total when done.
    ///
    /// Additionally, this routine will return non-zero if you should suspend/abort your current processing.
    /// You should probably try not to call this too frequently (e.g. at every pixel).
    /// It is better to call it, say, once per scanline, unless your filter is really really slow.
    pub fn progress(&self, current: i32, total: i32) -> Result<(), Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Calling Adobe After Effects SDK progress FFI callback through function pointer.
        // Detailed explanation: (1) in_data.inter.progress is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) effect_ref is SDK-provided and valid, (4) current and total are primitive integer parameters representing the progress fraction, (5) the function displays a progress bar and checks for user interrupt, following the Adobe SDK's callback convention of returning 0 if no interrupt or an error code if user requested abort.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid or pointed to freed code, or effect_ref was invalid.
        match unsafe { in_data.inter.progress.unwrap()(in_data.effect_ref, current as _, total as _) } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    /// Register a custom user interface element. See [Effect UI and events](https://ae-plugins.docsforadobe.dev/effect-ui-events/effect-ui-events.html). Note: The `PF_UIAlignment` flags are not honored.
    pub fn register_ui(&self, custom_ui_info: CustomUIInfo) -> Result<(), Error> {
        // SAFETY: Dereferencing InData pointer to access PF_InData structure.
        // Detailed explanation: (1) self.0.as_ptr() returns a non-null pointer guaranteed by InData::from_raw which asserts non-null, (2) the pointer points to a valid PF_InData structure owned by the Adobe After Effects SDK for the duration of the plugin callback, (3) the reference is only used within this function scope and doesn't outlive the SDK-managed data.
        // Would be UB if: the InData pointer was null (prevented by assertion in from_raw), dangling, or if the PF_InData structure was deallocated during this function call.
        let in_data = unsafe { &(*self.0.as_ptr()) };
        // SAFETY: Calling Adobe After Effects SDK register_ui FFI callback through function pointer.
        // Detailed explanation: (1) in_data.inter.register_ui is guaranteed to be a valid function pointer provided by the Adobe SDK during plugin initialization, (2) unwrap() is safe because the SDK always initializes these function pointers before invoking plugin callbacks, (3) effect_ref is SDK-provided and valid, (4) custom_ui_info.as_ptr() returns a valid pointer to a properly initialized CustomUIInfo structure, (5) the function registers a custom UI element and follows the Adobe SDK's callback convention of returning 0 on success or an error code on failure.
        // Would be UB if: the function pointer was null (prevented by unwrap), the function pointer was invalid or pointed to freed code, effect_ref was invalid, or custom_ui_info pointer was dangling.
        match unsafe { in_data.inter.register_ui.unwrap()(in_data.effect_ref, custom_ui_info.as_ptr() as _) } {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }

    // fn checkout_layer_audio(effect_ref: PF_ProgPtr, index: PF_ParamIndex, start_time: A_long, duration: A_long, time_scale: A_u_long, rate: PF_UFixed, bytes_per_sample: A_long, num_channels: A_long, fmt_signed: A_long, audio: *mut PF_LayerAudio) -> PF_Err,
    // fn checkin_layer_audio(effect_ref: PF_ProgPtr, audio: PF_LayerAudio) -> PF_Err,
    // fn audio_data(effect_ref: PF_ProgPtr, audio: PF_LayerAudio, data0: *mut PF_SndSamplePtr, num_samples0: *mut A_long, rate0: *mut PF_UFixed, bytes_per_sample0: *mut A_long, num_channels0: *mut A_long, fmt_signed0: *mut A_long) -> PF_Err,
}
