macro_rules! ae_acquire_suite_ptr {
    ($pica:expr, $type:ident, $name:ident, $version:ident) => {{
        unsafe {
            let mut suite_ptr = std::mem::MaybeUninit::<*const after_effects_sys::$type>::uninit();

            let aquire_suite_func = (*($pica)).AcquireSuite.unwrap_or_else(|| unreachable!());
            match aquire_suite_func(
                after_effects_sys::$name.as_ptr() as *const i8,
                after_effects_sys::$version as i32,
                suite_ptr.as_mut_ptr() as *mut *const _ as _,
            ) as u32
            {
                after_effects_sys::kSPNoError => Ok(suite_ptr.assume_init()),
                _ => Err($crate::Error::MissingSuite),
            }
        }
    }};
}

macro_rules! ae_release_suite_ptr {
    ($pica:expr, $name:ident, $version:ident) => {{
        unsafe {
            let release_suite_func = (*($pica)).ReleaseSuite.unwrap_or_else(|| unreachable!());
            release_suite_func(
                after_effects_sys::$name.as_ptr() as *const i8,
                after_effects_sys::$version as i32,
            );
        }
    }};
}

macro_rules! ae_get_suite_fn {
    ($suite_ptr:expr, $function:ident ) => {{
        // Return an invocable function
        (*($suite_ptr)).$function.unwrap_or_else(|| unreachable!())
    }};
}

macro_rules! ae_call_suite_fn {
    ($suite_ptr:expr, $function:ident, $($arg:tt)* ) => {{
        let err = unsafe { ae_get_suite_fn!(($suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err(Error::from(err))
        }
    }};
}

// Call a function from a suite and return the value.
// Some new functions in AE_Scene3D_Private.h abandon suite API design
// practices and return a value instead of an error. E.g. the
// AEGP_*BufferElementSize() ones.
macro_rules! ae_call_suite_fn_no_err {
    ($suite_ptr:expr, $function:ident, $($arg:tt)* ) => {{
        unsafe {
            ae_get_suite_fn!(($suite_ptr), $function)($($arg)*)
        }
    }};
}

#[macro_export]
macro_rules! ae_acquire_suite_and_call_suite_fn_no_err {
    ($pica:expr, $type:ident, $name:ident, $version:ident, $function:ident, $($arg:tt)* ) => {{
        match ae_acquire_suite_ptr!( $pica, $type, $name, $version) {
            Ok(suite_ptr) =>
                ae_call_suite_fn_no_err!(suite_ptr, $function, $($arg)*),
            Err(e) => {
                Err(e)
            },
        }
    }};
}

#[macro_export]
macro_rules! ae_acquire_suite_and_call_suite_fn {
    ($pica:expr, $type:ident, $name:ident, $version:ident, $function:ident, $($arg:tt)* ) => {{
        match ae_acquire_suite_ptr!( $pica, $type, $name, $version) {
            Ok(suite_ptr) =>
                ae_call_suite_fn!(suite_ptr, $function, $($arg)*),
            Err(e) => {
                Err(e)
            },
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

macro_rules! define_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name(pub(crate) after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(raw_handle: after_effects_sys::$data_type) -> Self {
                Self(raw_handle)
            }

            pub fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
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
    };
}

macro_rules! define_struct_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone)]
        pub struct $wrapper_pretty_name(after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(ae_struct: after_effects_sys::$data_type) -> Self {
                Self(ae_struct)
            }

            pub fn into_raw(def: $wrapper_pretty_name) -> after_effects_sys::$data_type {
                def.0
            }
        }
        impl AsRef<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ref(&self) -> &after_effects_sys::$data_type {
                &self.0
            }
        }

        impl From<$wrapper_pretty_name> for after_effects_sys::$data_type {
            fn from(handle_wrapper: $wrapper_pretty_name) -> Self {
                handle_wrapper.0
            }
        }
    };
}

macro_rules! define_owned_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name(after_effects_sys::$data_type, bool);

        impl $wrapper_pretty_name {
            pub fn from_raw(raw_handle: after_effects_sys::$data_type) -> Self {
                Self(raw_handle, false)
            }

            pub fn as_ptr(&self) -> after_effects_sys::$data_type {
                self.0
            }

            pub fn owned(&mut self, is_owned: bool) {
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
    };
}

macro_rules! define_param_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident) => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $wrapper_pretty_name(after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn new() -> Self {
                Self(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })
            }

            pub fn from_raw(def: after_effects_sys::$data_type) -> Self {
                Self(def)
            }

            pub fn into_raw(def: $wrapper_pretty_name) -> after_effects_sys::$data_type {
                def.0
            }
        }
    };
}

macro_rules! define_param_basic_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident, $value_type:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn set_value(mut self, value: $value_type) -> $wrapper_pretty_name {
                self.0.value = value;
                self
            }

            pub fn set_default(mut self, default: $value_type_ui) -> $wrapper_pretty_name {
                self.0.dephault = default as _;
                self
            }
        }
    };
}

macro_rules! define_param_valid_min_max_wrapper {
    ($wrapper_pretty_name:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn set_valid_min(mut self, valid_min: $value_type_ui) -> $wrapper_pretty_name {
                self.0.valid_min = valid_min;
                self
            }

            pub fn set_valid_max(mut self, valid_max: $value_type_ui) -> $wrapper_pretty_name {
                self.0.valid_max = valid_max;
                self
            }
        }
    };
}

macro_rules! define_param_slider_min_max_wrapper {
    ($wrapper_pretty_name:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn set_slider_min(mut self, slider_min: $value_type_ui) -> $wrapper_pretty_name {
                self.0.slider_min = slider_min;
                self
            }

            pub fn set_slider_max(mut self, slider_max: $value_type_ui) -> $wrapper_pretty_name {
                self.0.slider_max = slider_max;
                self
            }
        }
    };
}

macro_rules! define_param_value_str_wrapper {
    ($wrapper_pretty_name:ident) => {
        impl $wrapper_pretty_name {
            pub fn set_value_str<'a>(
                &'a mut self,
                value_str: &str,
            ) -> &'a mut $wrapper_pretty_name {
                assert!(value_str.len() < 32);
                let value_cstr = CString::new(value_str).unwrap();
                let value_slice = value_cstr.to_bytes_with_nul();
                self.0.value_str[0..value_slice.len()]
                    .copy_from_slice(unsafe { std::mem::transmute(value_slice) });
                self
            }
        }
    };
}

macro_rules! define_param_value_desc_wrapper {
    ($wrapper_pretty_name:ident) => {
        impl $wrapper_pretty_name {
            pub fn set_value_desc<'a>(
                &'a mut self,
                value_desc: &str,
            ) -> &'a mut $wrapper_pretty_name {
                assert!(value_desc.len() < 32);
                let value_desc_cstr = CString::new(value_desc).unwrap();
                let value_desc_slice = value_desc_cstr.to_bytes_with_nul();
                self.0.value_desc[0..value_desc_slice.len()]
                    .copy_from_slice(unsafe { std::mem::transmute(value_desc_slice) });
                self
            }
        }
    };
}

macro_rules! define_suite {
    ($suite_pretty_name:ident, $suite_name:ident, $suite_name_string:ident, $suite_version:ident) => {
        #[derive(Clone, Debug, Hash)]
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

            fn from_raw(
                pica_basic_suite_ptr: *const after_effects_sys::SPBasicSuite,
            ) -> Result<Self, Error> {
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

#[macro_export]
macro_rules! add_param {
    (in_data: expr,
    index: expr,
    def: expr) => {
        in_data.inter.add_param.unwrap()(in_data.effect_ref, (index), &(def))
    };
}

#[macro_export]
macro_rules! assume {
    ($owner:ident, $var:pat => $out:expr, $ty:ty) => {
        impl AssumeFrom<$owner> for $ty {
            fn assume(x: &$owner) -> &$ty {
                use $owner::*;
                match x {
                    $var => $out,
                    _ => panic!(concat!("Assumed ", stringify!($var), " but was in {:?}"), x),
                }
            }

            fn assume_mut(x: &mut $owner) -> &mut $ty {
                use $owner::*;
                match x {
                    $var => $out,
                    _ => panic!(concat!("Assumed ", stringify!($var), " but was in {:?}"), x),
                }
            }
        }
    };
    ($owner:ident) => {
        impl $owner {
            pub fn assume<T: AssumeFrom<Self>>(&self) -> &T {
                T::assume(self)
            }

            pub fn assume_mut<T: AssumeFrom<Self>>(&mut self) -> &mut T {
                T::assume_mut(self)
            }
        }
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
