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
        pub struct $wrapper_pretty_name(*mut after_effects_sys::$data_type);

        impl $wrapper_pretty_name {
            pub fn from_raw(ae_struct: *mut after_effects_sys::$data_type) -> Self {
                assert!(!ae_struct.is_null());
                Self(ae_struct)
            }
        }
        impl AsRef<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_ref(&self) -> &after_effects_sys::$data_type {
                unsafe { &*self.0 }
            }
        }
        impl AsMut<after_effects_sys::$data_type> for $wrapper_pretty_name {
            fn as_mut(&mut self) -> &mut after_effects_sys::$data_type {
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
    $(impl $impl_name:tt: $impl_type:tt, )*) => {
        $(#[$attr])*
        pub struct $name<'parent> {
            pub(crate) def: Ownership<'parent, after_effects_sys::$sys_type>,
            pub(crate) change_flags: Option<&'parent mut ae_sys::PF_ChangeFlags>,
            $($field_name: $field_type, )*
        }
        impl<'parent> $name<'parent> {
            pub fn new() -> Self {
                Self {
                    def: Ownership::Rust(unsafe { std::mem::zeroed() }),
                    change_flags: None,
                    $($field_name: Default::default(), )*
                }
            }
            pub fn setup<F: FnOnce(&mut Self)>(cb: F) -> Self {
                let mut ret = Self::new();
                cb(&mut ret);
                ret
            }
            pub fn from_mut(def: &'parent mut after_effects_sys::$sys_type, change_flags: &'parent mut ae_sys::PF_ChangeFlags) -> Self {
                Self {
                    def: Ownership::AfterEffectsMut(def),
                    change_flags: Some(change_flags),
                    $($field_name: Default::default(), )*
                }
            }
            pub fn from_ref(def: &'parent after_effects_sys::$sys_type) -> Self {
                Self {
                    def: Ownership::AfterEffects(def),
                    change_flags: None,
                    $($field_name: Default::default(), )*
                }
            }
            pub fn from_owned(def: after_effects_sys::$sys_type) -> Self {
                Self {
                    def: Ownership::Rust(def),
                    change_flags: None,
                    $($field_name: Default::default(), )*
                }
            }
            pub fn set_value_changed(&mut self) {
                if let Some(ref mut change_flags) = self.change_flags {
                    **change_flags = ChangeFlag::CHANGED_VALUE.bits();
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
            unsafe { CStr::from_ptr(self.def.u.namesptr).to_str().unwrap() }
        }
    };
    (impl ShortString, $name:ident) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, v: &str) -> &mut Self {
                assert!(v.len() < 32);
                let cstr = CString::new(v).unwrap();
                let slice = cstr.to_bytes_with_nul();
                self.def.$name[0..slice.len()].copy_from_slice(unsafe { std::mem::transmute(slice) });
                self
            }
        }
        pub fn $name(&self) -> &str {
            let ptr = self.def.$name;
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
