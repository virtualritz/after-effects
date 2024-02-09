macro_rules! pr_acquire_suite_ptr {
    ($pica:expr, $type:ident, $name:ident, $version:ident) => {{
        unsafe {
            let mut suite_ptr = std::mem::MaybeUninit::<*const premiere_sys::$type>::uninit();

            if $pica.is_null() {
                return Err($crate::Error::NotImplemented);
            }

            let aquire_suite_func = (*($pica)).AcquireSuite.unwrap_or_else(|| unreachable!());
            match aquire_suite_func(
                premiere_sys::$name.as_ptr() as *const i8,
                premiere_sys::$version as i32,
                suite_ptr.as_mut_ptr() as *mut *const _ as _,
            ) as u32
            {
                premiere_sys::kSPNoError => Ok(suite_ptr.assume_init()),
                _ => Err($crate::Error::NotImplemented),
            }
        }
    }};
}

macro_rules! pr_release_suite_ptr {
    ($pica:expr, $name:ident, $version:ident) => {{
        unsafe {
            if $pica.is_null() {
                return;
            }
            let release_suite_func = (*($pica)).ReleaseSuite.unwrap_or_else(|| unreachable!());
            release_suite_func(
                premiere_sys::$name.as_ptr() as *const i8,
                premiere_sys::$version as i32,
            );
        }
    }};
}

macro_rules! pr_get_suite_fn {
    ($suite_ptr:expr, $function:ident ) => {{
        // Return an invocable function
        (*($suite_ptr)).$function.unwrap_or_else(|| unreachable!())
    }};
}

macro_rules! call_suite_fn {
    ($self:expr, $function:ident, $($arg:tt)* ) => {{
        let err = unsafe { pr_get_suite_fn!(($self.suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err(Error::from(err))
        }
    }};
}
macro_rules! call_suite_fn_single {
    ($self:expr, $function:ident -> $typ:ty, $($arg:tt)* ) => {{
        let mut val: $typ = unsafe { std::mem::zeroed() };
        let err = unsafe { pr_get_suite_fn!($self.suite_ptr, $function)($($arg)*, &mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
    ($self:expr, $function:ident -> $typ:ty) => {{
        let mut val: $typ = unsafe { std::mem::zeroed() };
        let err = unsafe { pr_get_suite_fn!($self.suite_ptr, $function)(&mut val) };

        match err {
            0 => Ok(val),
            _ => Err(Error::from(err))
        }
    }};
}

// Call a function from a suite and return the value.
macro_rules! call_suite_fn_no_err {
    ($self:expr, $function:ident, $($arg:tt)* ) => {{
        unsafe {
            pr_get_suite_fn!(($self.suite_ptr), $function)($($arg)*)
        }
    }};
}

macro_rules! define_enum {
    ($raw_type:ty, $name:ident { $( $variant:ident = $value:path ),*, }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl From<$name> for $raw_type {
            fn from(v: $name) -> Self {
                match v {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }
        impl From<$raw_type> for $name {
            fn from(v: $raw_type) -> Self {
                match v {
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
    ($suite_pretty_name:ident, $suite_name:ident, $suite_name_string:ident, $suite_version:ident) => {
        #[derive(Clone, Debug, Hash)]
        pub struct $suite_pretty_name {
            pica_basic_suite_ptr: *const premiere_sys::SPBasicSuite,
            suite_ptr: *const premiere_sys::$suite_name,
        }

        impl Suite for $suite_pretty_name {
            fn new() -> Result<Self, Error> {
                let pica_basic_suite_ptr = borrow_pica_basic_as_ptr();

                match pr_acquire_suite_ptr!(
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
                pr_release_suite_ptr!(
                    self.pica_basic_suite_ptr,
                    $suite_name_string,
                    $suite_version
                );
            }
        }
    };
}
