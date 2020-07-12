#[macro_export]
macro_rules! ae_acquire_suite_ptr {
    ($pica:expr, $type:ident, $name:ident, $version:ident) => {{
        unsafe {
            let mut suite_ptr = std::mem::MaybeUninit::<*const aftereffects_sys::$type>::uninit();

            let aquire_suite_func = (*($pica)).AcquireSuite.unwrap_or_else(|| unreachable!());
            match aquire_suite_func(
                aftereffects_sys::$name.as_ptr() as *const i8,
                aftereffects_sys::$version as i32,
                suite_ptr.as_mut_ptr() as *mut *const _ as _,
            ) as u32
            {
                ae_sys::kSPNoError => Ok(suite_ptr.assume_init()),
                _ => Err($crate::Error::MissingSuite),
            }
        }
    }};
}

#[macro_export]
macro_rules! ae_release_suite_ptr {
    ($pica:expr, $name:ident, $version:ident) => {{
        unsafe {
            let release_suite_func = (*($pica)).ReleaseSuite.unwrap_or_else(|| unreachable!());
            release_suite_func(
                aftereffects_sys::$name.as_ptr() as *const i8,
                aftereffects_sys::$version as i32,
            );
        }
    }};
}

#[macro_export]
macro_rules! ae_get_suite_fn {
    ($suite_ptr:expr, $function:ident ) => {{
        // Return an invocable function
        (*($suite_ptr)).$function.unwrap_or_else(|| unreachable!())
    }};
}

#[macro_export]
macro_rules! ae_call_suite_fn {
    ($suite_ptr:expr, $function:ident, $($arg:tt)* ) => {{
        let err = unsafe { ae_get_suite_fn!(($suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err( unsafe { $crate::Error::from_unchecked(err) } )
        }
    }};
}

// Call a function from a suite and return the value.
// Some new functions in AE_Scene3D_Private.h abandon suite API design
// practices and return a value instead of an error. E.g. the
// AEGP_*BufferElementSize() ones.
#[macro_export]
macro_rules! ae_call_suite_fn_no_err {
    ($suite_ptr:expr, $function:ident, $($arg:tt)* ) => {{
        unsafe {
            ae_get_suite_fn!(($suite_ptr), $function)($($arg)*)
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
            snake!($wrapper_pretty_name): ae_sys::$data_type,
        }

        impl $wrapper_pretty_name {
            pub fn from_raw(ptr: ae_sys::$data_type) -> $wrapper_pretty_name {
                $wrapper_pretty_name {
                    snake!($wrapper_pretty_name): ptr,
                }
            }

            pub fn as_ptr(&self) -> ae_sys::$data_type {
                &self.snake!($wrapper_pretty_name)
            }
        }
    };
}*/

macro_rules! define_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident, $data_name:ident) => {
        #[derive(Copy, Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name {
            pub(crate) $data_name: ae_sys::$data_type,
        }

        impl $wrapper_pretty_name {
            pub fn from_raw($data_name: ae_sys::$data_type) -> Self {
                Self { $data_name }
            }

            pub fn as_ptr(&self) -> ae_sys::$data_type {
                self.$data_name
            }

            pub fn is_null(&self) -> bool {
                self.$data_name.is_null()
            }
        }
    };
}

macro_rules! _define_owned_handle_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident, $data_name:ident) => {
        #[derive(Clone, Debug, Hash)]
        pub struct $wrapper_pretty_name {
            $data_name: ae_sys::$data_type,
            is_owned: bool,
        }

        impl $wrapper_pretty_name {
            pub fn from_raw($data_name: ae_sys::$data_type) -> Self {
                Self {
                    $data_name,
                    is_owned: false,
                }
            }

            pub fn as_ptr(&self) -> ae_sys::$data_type {
                self.$data_name
            }
        }
    };
}

macro_rules! define_param_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident, $data_name:ident) => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $wrapper_pretty_name {
            pub(crate) $data_name: ae_sys::$data_type,
        }
        impl $wrapper_pretty_name {
            pub fn new() -> Self {
                Self {
                    $data_name: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
                }
            }
        }
    };
}

macro_rules! define_param_basic_wrapper {
    ($wrapper_pretty_name:ident, $data_type:ident, $data_name:ident, $value_type:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn value<'a>(&'a mut self, value: $value_type) -> &'a mut $wrapper_pretty_name {
                self.$data_name.value = value;
                self
            }

            pub fn default<'a>(&'a mut self, default: $value_type_ui) -> &'a mut $wrapper_pretty_name {
                self.$data_name.dephault = default as _;
                self
            }

            pub fn into_raw(def: $wrapper_pretty_name) -> $data_type {
                def.$data_name
            }
        }
    };
}

macro_rules! define_param_valid_min_max_wrapper {
    ($wrapper_pretty_name:ident, $data_name:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn valid_min<'a>(&'a mut self, valid_min: $value_type_ui) -> &'a mut $wrapper_pretty_name {
                self.$data_name.valid_min = valid_min;
                self
            }

            pub fn valid_max<'a>(&'a mut self, valid_max: $value_type_ui) -> &'a mut $wrapper_pretty_name {
                self.$data_name.valid_max = valid_max;
                self
            }
        }
    };
}

macro_rules! define_param_slider_min_max_wrapper {
    ($wrapper_pretty_name:ident, $data_name:ident, $value_type_ui:ident) => {
        impl $wrapper_pretty_name {
            pub fn slider_min<'a>(&'a mut self, slider_min: $value_type_ui) -> &'a mut $wrapper_pretty_name {
                self.$data_name.slider_min = slider_min;
                self
            }

            pub fn slider_max<'a>(&'a mut self, slider_max: $value_type_ui) -> &'a mut $wrapper_pretty_name {
                self.$data_name.slider_max = slider_max;
                self
            }
        }
    };
}

macro_rules! define_param_value_str_wrapper {
    ($wrapper_pretty_name:ident, $data_name:ident) => {
        impl $wrapper_pretty_name {
            pub fn value_str<'a>(&'a mut self, value_str: &str) -> &'a mut $wrapper_pretty_name {
                assert!(value_str.len() < 32);
                let value_cstr = CString::new(value_str).unwrap();
                let value_slice = value_cstr.to_bytes_with_nul();
                self.$data_name.value_str[0..value_slice.len()]
                    .copy_from_slice(unsafe { std::mem::transmute(value_slice) });
                self
            }
        }
    };
}

macro_rules! define_param_value_desc_wrapper {
    ($wrapper_pretty_name:ident, $data_name:ident) => {
        impl $wrapper_pretty_name {
            pub fn value_desc<'a>(&'a mut self, value_desc: &str) -> &'a mut $wrapper_pretty_name {
                assert!(value_desc.len() < 32);
                let value_desc_cstr = CString::new(value_desc).unwrap();
                let value_desc_slice = value_desc_cstr.to_bytes_with_nul();
                self.$data_name.value_desc[0..value_desc_slice.len()]
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
            pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
            suite_ptr: *const aftereffects_sys::$suite_name,
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

            fn from_raw(pica_basic_suite_ptr: *const $crate::ae_sys::SPBasicSuite) -> Result<Self, Error> {
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
                ae_release_suite_ptr!(self.pica_basic_suite_ptr, $suite_name_string, $suite_version);
            }
        }
    };
}
