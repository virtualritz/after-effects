#![feature(proc_macro_hygiene)]

use std::mem::MaybeUninit;

macro_rules! ae_acquire_suite_ptr {
    ($pica:expr, $type:ident, $name:ident, $version:ident) => {{
        #[allow(deprecated)]
        unsafe {
            let mut suite_ptr: *const aftereffects_sys::$type =
                std::mem::uninitialized();
            let suite_ptr_ptr: *mut *const aftereffects_sys::$type =
                &mut suite_ptr;

            let aquire_suite_func = (*($pica))
                .AcquireSuite
                .unwrap_or_else(|| unreachable!());
            aquire_suite_func(
                aftereffects_sys::$name.as_ptr() as *const i8,
                aftereffects_sys::$version as i32,
                suite_ptr_ptr as *mut *const _, /* as *mut *const
                                                 * libc::c_void, */
            );

            //suite_ptr

            if std::ptr::null() == suite_ptr {
                Err(crate::Error::MissingSuite)
            } else {
                Ok(suite_ptr)
            }
        }
    }};
}

macro_rules! ae_release_suite_ptr {
    ($pica:expr, $name:ident, $version:ident) => {{
        #[allow(deprecated)]
        unsafe {
            let release_suite_func = (*($pica))
                .ReleaseSuite
                .unwrap_or_else(|| unreachable!());
            release_suite_func(
                aftereffects_sys::$name.as_ptr() as *const i8,
                aftereffects_sys::$version as i32,
            );
        }
    }};
}

macro_rules! ae_get_suite_fn {
    ($suite_ptr:expr, $function:ident ) => {{
        // return an invocable function
        (*($suite_ptr)).$function.unwrap_or_else(|| unreachable!()) //expect("Could not call function!") //unwrap_or_else(|| unreachable!())
    }};
}

macro_rules! ae_call_suite_fn {
    ($suite_ptr:expr, $function:ident, $($arg:tt)* ) => {{
        use std::convert::TryInto;
        let err = unsafe { ae_get_suite_fn!(($suite_ptr), $function)($($arg)*) };

        match err {
            0 => Ok(()),
            _ => Err( unsafe { crate::Error::from_unchecked(err) } )
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

macro_rules! ae_aquire_suite_and_call_suite_fn {
    ($pica:ident, $type:ident, $name:ident, $version:ident, $function:ident, $($arg:tt)* ) => {{
        #[allow(deprecated)]

        //let suite_ptr: *mut *const aftereffects_sys::$type =
        match ae_aquire_suite_ptr!( $pica, $type, $name, $version) {
            Ok(suite_ptr) =>
                ae_call_suite_fn!(suite_ptr, $function, $($arg)*),
            Err(err_str) => {
                Err(err_str)
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
            $data_name: ae_sys::$data_type,
        }

        impl $wrapper_pretty_name {
            pub fn from_raw(
                $data_name: ae_sys::$data_type,
            ) -> $wrapper_pretty_name {
                $wrapper_pretty_name { $data_name }
            }

            pub fn as_ptr(&self) -> ae_sys::$data_type {
                self.$data_name
            }
        }
    };
}

macro_rules! define_suite{
    ($suite_pretty_name:ident, $suite_name:ident, $suite_name_string:ident, $suite_version:ident) => {
        #[allow(deprecated)]
        #[derive(Clone, Debug, Hash)]
        pub struct $suite_pretty_name {
            pica_basic_suite_ptr: *const ae_sys::SPBasicSuite,
            suite_ptr: *const aftereffects_sys::$suite_name,
        }

        impl Suite for $suite_pretty_name {
            fn new(
                pica_basic_suite: &crate::PicaBasicSuiteHandle,
            ) -> $suite_pretty_name {
                $suite_pretty_name {
                    pica_basic_suite_ptr: pica_basic_suite.as_ptr(),
                    suite_ptr: {
                        let suite_ptr = pica_basic_suite.as_ptr();
                        ae_acquire_suite_ptr!(
                            suite_ptr,
                            $suite_name,
                            $suite_name_string,
                            $suite_version
                        )
                        .expect(concat!("Could not aquire ", stringify!($suite_name), "."))
                    },
                }
            }

            fn from_raw(
                pica_basic_suite_raw_ptr: *const crate::ae_sys::SPBasicSuite,
            ) -> $suite_pretty_name {
                $suite_pretty_name {
                    pica_basic_suite_ptr: pica_basic_suite_raw_ptr,
                    suite_ptr: ae_acquire_suite_ptr!(
                        pica_basic_suite_raw_ptr,
                        $suite_name,
                        $suite_name_string,
                        $suite_version
                    )
                    .expect(concat!("Could not aquire ", stringify!($suite_name), "."))
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
