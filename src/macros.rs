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

macro_rules! call_suite_fn {
    ($self:expr, $function:ident, $($arg:tt)* ) => {{
        let err = unsafe { ae_get_suite_fn!(($self.suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err(Error::from(err))
        }
    }};
}
macro_rules! call_suite_fn_single {
    ($self:expr,  $function:ident -> $typ:ty, $($arg:tt)* ) => {{
        let mut val: $typ = unsafe { std::mem::zeroed() };
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)($($arg)*, &mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
    ($self:expr,  $function:ident -> $typ:ty) => {{
        let mut val: $typ = unsafe { std::mem::zeroed() };
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)(&mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
}
macro_rules! call_suite_fn_double {
    ($self:expr,  $function:ident -> $typ1:ty, $typ2:ty, $($arg:tt)* ) => {{
        let mut v1: $typ1 = unsafe { std::mem::zeroed() };
        let mut v2: $typ2 = unsafe { std::mem::zeroed() };
        let err = unsafe { ae_get_suite_fn!($self.suite_ptr, $function)($($arg)*, &mut v1, &mut v2) };

        match err {
            0 => Ok((v1, v2)),
            _ => Err(Error::from(err))
        }
    }};
    ($self:expr,  $function:ident -> $typ1:ty, $typ2:ty) => {{
        let mut v1: $typ1 = unsafe { std::mem::zeroed() };
        let mut v2: $typ2 = unsafe { std::mem::zeroed() };
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

macro_rules! define_struct {
    ($raw_type:path, $(#[$attr:meta])* $name:ident { $( $(#[$fattr:meta])* $field:ident: $type:ty ),*, }) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        $(#[$attr])*
        pub struct $name {
            $(
                $(#[$fattr])*
                $field: $type,
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
    ($raw_type:ty, $name:ident { $( $(#[$attr:meta])* $variant:ident = $value:path ),*, }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$attr])*
                $variant,
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
            pub fn handle(&self) -> $handle_type {
                self.handle
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
                        suite.$suite_fn(self.handle, $($arg, )*).map(Into::into)
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
        impl Into<$name> for $handle_type {
            fn into(self) -> $name {
                $name::from_handle(self, false)
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
