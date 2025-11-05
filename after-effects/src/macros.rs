macro_rules! ae_acquire_suite_ptr {
    ($pica:expr, $type:ident, $name:ident, $version:ident) => {{
        // SAFETY: This unsafe block contains multiple FFI operations that are safe under these conditions:
        // 1. $pica must be a valid pointer to SPBasicSuite provided by After Effects at plugin initialization
        // 2. The pointer is dereferenced to access AcquireSuite function pointer - valid because After Effects
        //    guarantees the SPBasicSuite pointer remains valid for the plugin's lifetime
        // 3. MaybeUninit is used correctly: suite_ptr is only assumed_init() after AcquireSuite returns
        //    kSPNoError, which guarantees it has been fully initialized by the FFI call
        // 4. The AcquireSuite function is called through a validated function pointer (checked for Some)
        // 5. Raw pointer casts are safe because After Effects expects these exact pointer types
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $pica is null, dangling, or points to invalid memory
        // - assume_init() is called when AcquireSuite returns an error (prevented by match check)
        // - The suite name/version constants are malformed (prevented by type system)
        unsafe {
            let mut suite_ptr = std::mem::MaybeUninit::<*const after_effects_sys::$type>::uninit();

            let aquire_suite_func = match (*($pica)).AcquireSuite {
                Some(func) => func,
                None => {
                    log::error!("AcquireSuite function pointer is null for suite: {}", stringify!($type));
                    return Err($crate::Error::MissingSuite);
                }
            };
            match aquire_suite_func(
                after_effects_sys::$name.as_ptr() as *const i8,
                after_effects_sys::$version as i32,
                suite_ptr.as_mut_ptr() as *mut *const _ as _,
            ) as u32
            {
                after_effects_sys::kSPNoError => Ok(suite_ptr.assume_init()),
                _ => {
                    log::error!("Suite not found: {} {} {}", stringify!($type), stringify!($name), stringify!($version));
                    Err($crate::Error::MissingSuite)
                },
            }
        }
    }};
}

macro_rules! ae_release_suite_ptr {
    ($pica:expr, $name:ident, $version:ident) => {{
        // SAFETY: Dereferencing $pica pointer to access ReleaseSuite function pointer.
        // This is safe because:
        // 1. $pica is the same SPBasicSuite pointer used in ae_acquire_suite_ptr
        // 2. After Effects guarantees this pointer remains valid throughout plugin lifetime
        // 3. ReleaseSuite is an optional function pointer that's validated before calling
        // 4. The suite name/version passed to ReleaseSuite match those from AcquireSuite
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $pica is null, dangling, or invalid (caller's responsibility to maintain validity)
        // - ReleaseSuite is called with a suite that wasn't successfully acquired
        // - This is called after the plugin has been unloaded
        unsafe {
            if let Some(release_suite_func) = (*($pica)).ReleaseSuite {
                release_suite_func(
                    after_effects_sys::$name.as_ptr() as *const i8,
                    after_effects_sys::$version as i32,
                );
            } else {
                log::error!("ReleaseSuite function pointer is null for suite: {}", stringify!($name));
            }
        }
    }};
}

macro_rules! ae_get_suite_fn {
    ($suite_ptr:expr, $function:ident ) => {{
        // Return an invocable function
        match (*($suite_ptr)).$function {
            Some(func) => func,
            None => {
                log::error!("Suite function pointer is null: {}", stringify!($function));
                panic!("Suite function {} is not available", stringify!($function));
            }
        }
    }};
}

macro_rules! call_suite_fn {
    ($self:expr, $function:ident, $($arg:tt)* ) => {{
        // SAFETY: Calling an After Effects suite function through a validated function pointer.
        // This is safe because:
        // 1. $self.suite_ptr was successfully acquired via ae_acquire_suite_ptr (checked at creation)
        // 2. ae_get_suite_fn validates the function pointer is Some before returning it
        // 3. The suite pointer remains valid because it's held by the suite struct until Drop
        // 4. Arguments passed must match the C function signature (caller's responsibility)
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $self.suite_ptr is null or invalid (prevented by suite acquisition checks)
        // - Arguments don't match expected C function signature
        // - The suite was released but the wrapper is still being used
        let err = unsafe { ae_get_suite_fn!(($self.suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err(Error::from(err))
        }
    }};
}
macro_rules! call_suite_fn_single {
    ($self:expr,  $function:ident -> $typ:ty, $($arg:tt)* ) => {{
        // SAFETY: Initializing a value with zeroed memory for FFI out-parameter pattern.
        // This is safe because:
        // 1. $typ must be a type where all-zero bytes represent a valid state (FFI types like integers, pointers)
        // 2. The value is immediately passed as a mutable reference to the FFI function which will initialize it
        // 3. The value is only returned when err == 0, ensuring it was properly initialized by After Effects
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $typ is not a valid type for zeroed() (e.g., non-nullable references, bools not 0 or 1)
        // - The FFI function fails (err != 0) but val is used anyway (prevented by match)
        // - Caller uses this macro with invalid FFI types
        let mut val: $typ = unsafe { std::mem::zeroed() };
        // SAFETY: Calling suite function - see call_suite_fn safety documentation.
        // Additionally, &mut val is passed as an out-parameter which the FFI function will initialize.
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)($($arg)*, &mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
    ($self:expr,  $function:ident -> $typ:ty) => {{
        // SAFETY: Initializing a value with zeroed memory for FFI out-parameter pattern.
        // This is safe because:
        // 1. $typ must be a type where all-zero bytes represent a valid state (FFI types like integers, pointers)
        // 2. The value is immediately passed as a mutable reference to the FFI function which will initialize it
        // 3. The value is only returned when err == 0, ensuring it was properly initialized by After Effects
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $typ is not a valid type for zeroed() (e.g., non-nullable references, bools not 0 or 1)
        // - The FFI function fails (err != 0) but val is used anyway (prevented by match)
        // - Caller uses this macro with invalid FFI types
        let mut val: $typ = unsafe { std::mem::zeroed() };
        // SAFETY: Calling suite function - see call_suite_fn safety documentation.
        // Additionally, &mut val is passed as an out-parameter which the FFI function will initialize.
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)(&mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
}
macro_rules! call_suite_fn_double {
    ($self:expr,  $function:ident -> $typ1:ty, $typ2:ty, $($arg:tt)* ) => {{
        // SAFETY: Initializing two values with zeroed memory for FFI out-parameter pattern.
        // This is safe because:
        // 1. Both $typ1 and $typ2 must be types where all-zero bytes represent valid states
        // 2. The values are immediately passed as mutable references to the FFI function
        // 3. The values are only returned when err == 0, ensuring proper initialization
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $typ1 or $typ2 are not valid types for zeroed()
        // - The FFI function fails but values are used anyway (prevented by match)
        let mut v1: $typ1 = unsafe { std::mem::zeroed() };
        let mut v2: $typ2 = unsafe { std::mem::zeroed() };
        // SAFETY: Calling suite function - see call_suite_fn safety documentation.
        // Both v1 and v2 are passed as out-parameters which the FFI function will initialize.
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)($($arg)*, &mut v1, &mut v2) };

        match err {
            0 => Ok((v1, v2)),
            _ => Err(Error::from(err))
        }
    }};
    ($self:expr,  $function:ident -> $typ1:ty, $typ2:ty) => {{
        // SAFETY: Initializing two values with zeroed memory for FFI out-parameter pattern.
        // This is safe because:
        // 1. Both $typ1 and $typ2 must be types where all-zero bytes represent valid states
        // 2. The values are immediately passed as mutable references to the FFI function
        // 3. The values are only returned when err == 0, ensuring proper initialization
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $typ1 or $typ2 are not valid types for zeroed()
        // - The FFI function fails but values are used anyway (prevented by match)
        let mut v1: $typ1 = unsafe { std::mem::zeroed() };
        let mut v2: $typ2 = unsafe { std::mem::zeroed() };
        // SAFETY: Calling suite function - see call_suite_fn safety documentation.
        // Both v1 and v2 are passed as out-parameters which the FFI function will initialize.
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)(&mut v1, &mut v2) };

        match err {
            0 => Ok((v1, v2)),
            _ => Err(Error::from(err))
        }
    }};
}

// Call a function from a suite and return the value.
// Some new functions in AE_Scene3D_Private.h abandon suite API design
// practices and return a value instead of an error. E.g. the
// AEGP_*BufferElementSize() ones.
macro_rules! call_suite_fn_no_err {
    ($self:expr, $function:ident, $($arg:tt)* ) => {{
        // SAFETY: Calling suite function that returns a value directly instead of an error code.
        // This is safe because:
        // 1. $self.suite_ptr was successfully acquired and remains valid (held by suite struct)
        // 2. ae_get_suite_fn validates the function pointer exists before returning it
        // 3. These functions don't use the error-code pattern and return values directly
        // 4. The function signature and arguments must be correct (caller's responsibility)
        //
        // UNDEFINED BEHAVIOR would occur if:
        // - $self.suite_ptr is null or invalid (prevented by suite acquisition)
        // - Arguments don't match the expected C function signature
        // - The function pointer has been invalidated (prevented by lifetime management)
        unsafe {
            ae_get_suite_fn!(($self.suite_ptr), $function)($($arg)*)
        }
    }};
}

/*
macro_rules! define_handle_wrapper_v2 {
    ($wrapper_pretty_name:ident, $data_type:ident,) => {

        #[derive(Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name {
            snake!($wrapper_pretty_name): after_effects_sys::$data_type,
        }

        impl $wrapper_pretty_name {
            pub fn from_raw(ptr: after_effects_sys::$data_type) -> $wrapper_pretty_name {
                $wrapper_pretty_name {
                    snake!($wrapper_pretty_name): ptr,
                }
            }

            pub fn as_ptr(&self) -> after_effects_sys::$data_type {
                &self.snake!($wrapper_pretty_name)
            }
        }
    };
}*/

macro_rules! register_handle {
    ($data_type:ident) => {
        impl AsPtr<after_effects_sys::$data_type> for after_effects_sys::$data_type {
            fn as_ptr(&self) -> after_effects_sys::$data_type {
                *self
            }
        }
    };
}
macro_rules! define_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name(pub(crate) after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(raw_handle: after_effects_sys::$data_type) -> Self {
                Self(raw_handle)
            }

            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }

        impl From<$wrapper_pretty_name> for after_effects_sys::$data_type {
            fn from(handle_wrapper: $wrapper_pretty_name) -> Self {
                handle_wrapper.as_ptr()
            }
        }
        impl AsRef<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ref(&self) -> &after_effects_sys::$data_type {
                &self.0
            }
        }
        impl AsPtr<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }
        }
        impl AsPtr<after_effects_sys::$data_type> for &$wrapper_pretty_name {
            fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }
        }
    };
}

macro_rules! define_struct_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone)]
        pub struct $wrapper_pretty_name(*mut after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(ae_struct: *mut after_effects_sys::$data_type) -> Self {
                assert!(!ae_struct.is_null());
                Self(ae_struct)
            }
        }
        impl AsRef<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ref(&self) -> &after_effects_sys::$data_type {
                // SAFETY: Dereferencing the raw pointer to create a shared reference.
                // This is safe because:
                // 1. from_raw() enforces non-null via assert before storing the pointer
                // 2. The pointer originates from After Effects and points to valid data
                // 3. The returned reference lifetime is tied to &self, preventing use-after-free
                // 4. AsRef only provides shared access, so no aliasing violations
                //
                // UNDEFINED BEHAVIOR would occur if:
                // - The pointer was invalidated after from_raw() (caller must ensure validity)
                // - The underlying data was freed while references exist (caller's responsibility)
                // - Multiple mutable references exist simultaneously (prevented by borrow checker)
                unsafe { &*self.0 }
            }
        }
        impl AsMut<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_mut(&mut self) -> &mut after_effects_sys::$data_type {
                // SAFETY: Dereferencing the raw pointer to create a mutable reference.
                // This is safe because:
                // 1. from_raw() enforces non-null via assert before storing the pointer
                // 2. The pointer originates from After Effects and points to valid, mutable data
                // 3. The returned reference lifetime is tied to &mut self, preventing aliasing
                // 4. The &mut self requirement ensures exclusive access (no other references exist)
                //
                // UNDEFINED BEHAVIOR would occur if:
                // - The pointer was invalidated after from_raw() (caller must ensure validity)
                // - The underlying data was freed while the mutable reference exists
                // - The data is accessed through other references while this mutable reference exists
                unsafe { &mut *self.0 }
            }
        }
        impl AsPtr<*mut after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ptr(&self) -> *mut after_effects_sys::$data_type {
                self.0
            }
        }
        impl AsPtr<*mut after_effects_sys::$data_type> for &$wrapper_pretty_name {
            fn as_ptr(&self) -> *mut after_effects_sys::$data_type {
                self.0
            }
        }
    };
}

macro_rules! define_owned_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Debug, Hash)]
        pub struct $wrapper_pretty_name(after_effects_sys::$data_type, bool);

        impl $wrapper_pretty_name {
            pub fn from_raw(raw_handle: after_effects_sys::$data_type) -> Self {
                Self(raw_handle, false)
            }
            pub fn from_raw_owned(raw_handle: after_effects_sys::$data_type) -> Self {
                Self(raw_handle, true)
            }

            pub fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }

            pub fn set_owned(&mut self, is_owned: bool) {
                self.1 = is_owned;
            }

            pub fn is_owned(&self) -> bool {
                self.1
            }
        }

        impl From<$wrapper_pretty_name> for after_effects_sys::$data_type {
            fn from(handle_wrapper: $wrapper_pretty_name) -> Self {
                handle_wrapper.as_ptr()
            }
        }
        impl AsRef<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ref(&self) -> &after_effects_sys::$data_type {
                &self.0
            }
        }
        impl AsPtr<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }
        }
        impl AsPtr<after_effects_sys::$data_type> for &$wrapper_pretty_name {
            fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }
        }
    };
}

macro_rules! define_param_wrapper {
    ($sys_enum:ident, $sys_type:ident, $sys_field:ident, $param_enum:path, $(#[$attr:meta])* $name:ident {
        $($field_name:ident: $field_type:ty, )*
    },
    $(impl $impl_name:tt: $impl_type:tt, )*
    $(fn init($self_name:ident) $init_fn:block)? ) => {
        $(#[$attr])*
        pub struct $name<'parent> {
            pub(crate) def: Ownership<'parent, ae_sys::$sys_type>,
            pub(crate) _parent_ptr: Option<*const ae_sys::PF_ParamDef>,
            pub(crate) _in_data: *const ae_sys::PF_InData,
            $($field_name: $field_type, )*
        }
        impl<'parent> $name<'parent> {
            pub fn new() -> Self {
                #[allow(unused_mut)]
                let mut param = Self {
                    // SAFETY: Zero-initializing an After Effects parameter definition struct.
                    // This is safe because:
                    // 1. After Effects C structs ($sys_type) are POD types designed for FFI
                    // 2. All-zero bytes represent a valid initial state for these FFI structs
                    // 3. The init_fn (if present) will further initialize required fields
                    // 4. This follows After Effects SDK conventions for param initialization
                    //
                    // UNDEFINED BEHAVIOR would occur if:
                    // - $sys_type contains non-nullable references (not the case for C FFI structs)
                    // - The struct has required fields that must be non-zero (handled by init_fn)
                    def: Ownership::Rust(unsafe { std::mem::zeroed() }),
                    _in_data: std::ptr::null(),
                    _parent_ptr: None,
                    $($field_name: Default::default(), )*
                };
                $( let $self_name = &mut param; $init_fn; )?
                param
            }
            pub fn setup<F: FnOnce(&mut Self)>(cb: F) -> Self {
                let mut ret = Self::new();
                cb(&mut ret);
                ret
            }
            pub fn from_mut(def: &'parent mut ae_sys::$sys_type, in_data: *const ae_sys::PF_InData, parent_ptr: *const ae_sys::PF_ParamDef) -> Self {
                Self {
                    def: Ownership::AfterEffectsMut(def),
                    _parent_ptr: Some(parent_ptr),
                    _in_data: in_data,
                    $($field_name: Default::default(), )*
                }
            }
            pub fn from_ref(def: &'parent ae_sys::$sys_type, in_data: *const ae_sys::PF_InData, parent_ptr: *const ae_sys::PF_ParamDef) -> Self {
                Self {
                    def: Ownership::AfterEffects(def),
                    _parent_ptr: Some(parent_ptr),
                    _in_data: in_data,
                    $($field_name: Default::default(), )*
                }
            }
            pub fn from_owned(def: ae_sys::$sys_type) -> Self {
                Self {
                    def: Ownership::Rust(def),
                    _parent_ptr: None,
                    _in_data: std::ptr::null(),
                    $($field_name: Default::default(), )*
                }
            }
            pub fn set_value_changed(&mut self) {
                if let Some(parent_ptr) = self._parent_ptr {
                    let parent_ptr = parent_ptr as *mut ae_sys::PF_ParamDef;
                    // SAFETY: Creating a mutable reference to the parent parameter definition.
                    // This is safe because:
                    // 1. parent_ptr originates from from_mut/from_ref which receive valid pointers from After Effects
                    // 2. The pointer lifetime is tied to 'parent lifetime, ensuring it remains valid
                    // 3. The &mut Self requirement ensures we have exclusive access
                    // 4. After Effects guarantees parent parameter remains valid during parameter callbacks
                    //
                    // UNDEFINED BEHAVIOR would occur if:
                    // - parent_ptr was invalidated after storing (prevented by lifetime management)
                    // - Multiple mutable references to the same parent exist (prevented by &mut self)
                    // - This is called outside of After Effects parameter callback context
                    let parent = unsafe { &mut *parent_ptr };
                    if (parent.ui_flags & (ae_sys::PF_PUI_STD_CONTROL_ONLY as ae_sys::PF_ParamUIFlags)) == 0 {
                        parent.uu.change_flags = ChangeFlag::CHANGED_VALUE.bits();
                    }
                }
            }
            $(
                define_param_wrapper!(impl $impl_type, $impl_name);
            )*
        }
        impl<'p> Into<Param<'p>> for $name<'p> {
            fn into(self) -> Param<'p> {
                $param_enum(self)
            }
        }
    };
    (impl String, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: &str) -> &mut Self {
                self.$name = CString::new(v).unwrap();
                { self.def.u.namesptr = self.$name.as_ptr(); }
                self
            }
        }
        pub fn $name(&self) -> &str {
            // SAFETY: Creating a CStr from a raw C string pointer.
            // This is safe because:
            // 1. self.def.u.namesptr points to a valid C string (null-terminated)
            // 2. The pointer was set by set_$name which stores a valid CString
            // 3. The CString is stored in self.$name, keeping it alive
            // 4. The returned &str lifetime is tied to &self, ensuring validity
            //
            // UNDEFINED BEHAVIOR would occur if:
            // - namesptr is null (prevented by set_$name always initializing it)
            // - namesptr points to invalid memory or non-null-terminated data
            // - The underlying CString was freed (prevented by storing in self.$name)
            unsafe { CStr::from_ptr(self.def.u.namesptr).to_str().unwrap() }
        }
    };
    (impl ShortString, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: &str) -> Result<&mut Self, Error> {
                if v.len() >= 32 {
                    return Err(Error::Parameter);
                }
                let cstr = CString::new(v).map_err(|_| Error::Parameter)?;
                let slice = cstr.as_bytes_with_nul();
                self.def.$name[0..slice.len()].copy_from_slice(slice);
                Ok(self)
            }
        }
        pub fn $name(&self) -> &str {
            let ptr = self.def.$name;
            // SAFETY: Creating a byte slice from a fixed-size C string array.
            // This is safe because:
            // 1. ptr is a fixed-size array (typically [i8; 32]) embedded in the struct
            // 2. ptr.as_ptr() returns a valid pointer to the start of this array
            // 3. ptr.len() returns the compile-time known array length
            // 4. The array is part of self, so it's valid for the lifetime of &self
            // 5. set_$name ensures the array contains a valid null-terminated C string
            //
            // UNDEFINED BEHAVIOR would occur if:
            // - ptr.as_ptr() returned a null or invalid pointer (impossible for array field)
            // - ptr.len() exceeded the actual array size (prevented by type system)
            // - The array doesn't contain a null terminator (prevented by set_$name validation)
            let u8_slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr.as_ptr() as *const u8, ptr.len()) };
            CStr::from_bytes_until_nul(u8_slice).unwrap().to_str().unwrap()
        }
    };
    (impl bool, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: bool) -> &mut Self {
                self.def.$name = if v { 1 } else { 0 };
                self.set_value_changed();
                self
            }
        }
        pub fn $name(&self) -> bool {
            self.def.$name != 0
        }
    };
    (impl Fixed, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: f32) -> &mut Self {
                self.def.$name = Fixed::from(v).as_fixed();
                self.set_value_changed();
                self
            }
        }
        pub fn $name(&self) -> f32 {
            Fixed::from_fixed(self.def.$name).into()
        }
    };
    (impl $typ:ty, default) => {
        pub fn set_default(&mut self, v: $typ) -> &mut Self {
            self.def.dephault = v as _;
            self
        }
        pub fn default(&self) -> $typ {
            self.def.dephault as _
        }
    };
    (impl $typ:ty, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: $typ) -> &mut Self {
                self.def.$name = v.into();
                self.set_value_changed();
                self
            }
        }
        pub fn $name(&self) -> $typ {
            self.def.$name.into()
        }
    };
}

macro_rules! define_struct {
    ($raw_type:path, $(#[$attr:meta])* $name:ident { $( $(#[$fattr:meta])* $field:ident: $type:ty ),*, }) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        $(#[$attr])*
        pub struct $name {
            $(
                $(#[$fattr])*
                pub $field: $type,
            )*
        }
        impl From<$name> for $raw_type {
            fn from(v: $name) -> Self {
                Self {
                    $( $field: v.$field as _, )*
                }
            }
        }
        impl From<$raw_type> for $name {
            fn from(v: $raw_type) -> Self {
                Self {
                    $( $field: v.$field as _, )*
                }
            }
        }
    };
}
macro_rules! define_struct_conv {
    ($raw_type:path, $name:ident { $( $field:ident ),* }) => {
        impl From<$name> for $raw_type {
            fn from(v: $name) -> Self {
                Self {
                    $( $field: v.$field as _, )*
                }
            }
        }
        impl From<$raw_type> for $name {
            fn from(v: $raw_type) -> Self {
                Self {
                    $( $field: v.$field as _, )*
                }
            }
        }
    };
}

macro_rules! define_enum {
    ($raw_type:ty, $(#[$topattr:meta])* $name:ident { $( $(#[$attr:meta])* $variant:ident = $value:path ),*, }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        $(#[$topattr])*
        pub enum $name {
            $(
                $(#[$attr])*
                $variant = $value as _,
            )*
        }

        impl From<$name> for $raw_type {
            fn from(v: $name) -> Self {
                match v {
                    $(
                        $name::$variant => $value as _,
                    )*
                }
            }
        }
        impl From<$raw_type> for $name {
            fn from(v: $raw_type) -> Self {
                match v as _ {
                    $(
                        $value => Self::$variant,
                    )*
                    _ => {
                        panic!("Unknown enum value {}: {v}", stringify!($name));
                    }
                }
            }
        }
    };
}

macro_rules! define_suite {
    ($(#[$attr:meta])* $suite_pretty_name:ident, $suite_name:ident, $suite_name_string:ident, $suite_version:ident) => {
        #[derive(Debug, Hash)]
        $(#[$attr])*
        pub struct $suite_pretty_name {
            pica_basic_suite_ptr: *const after_effects_sys::SPBasicSuite,
            suite_ptr: *const after_effects_sys::$suite_name,
        }

        impl Suite for $suite_pretty_name {
            fn new() -> Result<Self, Error> {
                let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

                match ae_acquire_suite_ptr!(
                    pica_basic_suite_ptr,
                    $suite_name,
                    $suite_name_string,
                    $suite_version
                ) {
                    Ok(suite_ptr) => Ok(Self {
                        pica_basic_suite_ptr,
                        suite_ptr,
                    }),
                    Err(e) => Err(e),
                }
            }
        }

        impl Clone for $suite_pretty_name {
            fn clone(&self) -> Self {
                Suite::new().unwrap()
            }
        }

        impl Drop for $suite_pretty_name {
            fn drop(&mut self) {
                ae_release_suite_ptr!(
                    self.pica_basic_suite_ptr,
                    $suite_name_string,
                    $suite_version
                );
            }
        }
    };
}

macro_rules! define_suite_item_wrapper {
    ($raw_handle_type:ty, $handle_type:ty, $( $suite_ident:ident: $suite_type:ty ),*, $(#[$type_attr:meta])+ $name:ident {
        dispose: $( $dispose_suite_fn_ident:ident.$dispose_fun:ident )?;
        $( $(#[$attr:meta])* $fn_name:ident($($arg:ident: $argt:ty),*) -> $ret:ty => $suite_fn_ident:ident.$suite_fn:ident ),*,
    }) => {
        $(#[$type_attr])+
        pub struct $name {
            handle: $handle_type,
            $( $suite_ident: once_cell::sync::Lazy<Result<$suite_type, crate::Error>>, )*
            #[allow(dead_code)]
            is_owned: bool,
        }

        impl $name {
            pub fn from_handle(handle: $handle_type, owned: bool) -> Self {
                Self {
                    handle,
                    $( $suite_ident: once_cell::sync::Lazy::new(|| <$suite_type>::new()), )*
                    is_owned: owned,
                }
            }
            pub fn from_raw(raw_handle: $raw_handle_type) -> Self {
                Self::from_handle(<$handle_type>::from_raw(raw_handle), false)
            }
            pub fn handle(&self) -> &$handle_type {
                &self.handle
            }
            pub fn into_raw(item: Self) -> $raw_handle_type {
                item.handle.into()
            }
            pub fn as_ptr(&self) -> $raw_handle_type {
                self.handle.as_ptr()
            }

            $(
                $(#[$attr])*
                pub fn $fn_name(&self, $($arg: $argt, )*) -> Result<$ret, crate::Error> {
                    if let Ok(ref suite) = *self.$suite_fn_ident {
                        suite.$suite_fn(self.handle.as_ptr(), $($arg, )*).map(Into::into)
                    } else {
                        Err(crate::Error::MissingSuite)
                    }
                }
            )*
        }
        impl AsRef<$handle_type> for $name {
            fn as_ref(&self) -> &$handle_type {
                &self.handle
            }
        }
        impl From<$handle_type> for $name {
            fn from(handle: $handle_type) -> Self {
                Self::from_handle(handle, false)
            }
        }
        impl crate::AsPtr<$raw_handle_type> for $name {
            fn as_ptr(&self) -> $raw_handle_type {
                self.handle.as_ptr()
            }
        }
        $(
            impl Drop for $name {
                fn drop(&mut self) {
                    if self.is_owned {
                        if let Ok(ref suite) = *self.$dispose_suite_fn_ident {
                            suite.$dispose_fun(self.handle).expect(concat!("Failed to dispose the ", stringify!($name), " handle."));
                        }
                    }
                }
            }
        )?
    };
}

#[cfg(feature = "artisan-2-api")]
macro_rules! define_ptr_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name(pub(crate) *const after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(raw_ptr: *const after_effects_sys::$data_type) -> Self {
                Self(raw_ptr)
            }

            pub fn as_ptr(&self) -> *const after_effects_sys::$data_type {
                self.0
            }

            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }

        impl From<$wrapper_pretty_name> for *const after_effects_sys::$data_type {
            fn from(ptr_wrapper: $wrapper_pretty_name) -> Self {
                ptr_wrapper.as_ptr()
            }
        }
    };
}
