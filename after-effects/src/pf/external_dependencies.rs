use crate::*;

define_struct_wrapper!(ExternalDependenciesExtra, PF_ExtDependenciesExtra);

impl ExternalDependenciesExtra {
    pub fn check_type(&self) -> DepCheckType {
        // SAFETY: Dereferencing the raw pointer to the PF_ExtDependenciesExtra FFI struct.
        // Detailed explanation: (1) self.0 points to a valid PF_ExtDependenciesExtra instance provided by the Adobe AE SDK,
        // (2) the pointer's lifetime is tied to the ExternalDependenciesExtra wrapper which maintains validity,
        // (3) the check_type field is a plain C enum that can be safely read.
        // Would be UB if: self.0 is null, dangling, or points to uninitialized/freed memory.
        unsafe { (*self.0).check_type.into() }
    }

    pub fn set_dependencies_str(&mut self, text: &str) -> Result<(), Error> {
        let suite = pf::suites::Handle::new()?;
        let text = std::ffi::CString::new(text).unwrap();
        let text = text.as_bytes_with_nul();
        let handle = suite.new_handle(text.len() as _);
        let ptr = suite.lock_handle(handle);
        // SAFETY: Copying string data to locked handle memory and assigning handle to FFI struct.
        // Detailed explanation: (1) ptr comes from lock_handle and points to valid, writable memory of at least text.len() bytes,
        // (2) text.as_ptr() is valid for reading text.len() bytes from the CString's buffer,
        // (3) the source and destination regions do not overlap as they are from different allocations,
        // (4) both pointers are properly aligned for u8 access,
        // (5) self.0 points to a valid PF_ExtDependenciesExtra instance that can have its dependencies_strH field written.
        // Would be UB if: ptr is null/invalid, text.as_ptr() is invalid, regions overlap, handle size is insufficient, or self.0 is invalid.
        unsafe {
            std::ptr::copy_nonoverlapping(text.as_ptr(), ptr as *mut u8, text.len());
            (*self.0).dependencies_strH = handle;
        }
        suite.unlock_handle(handle);
        Ok(())
    }

    pub fn set_dependencies_data(&mut self, data: Vec<u8>) -> Result<(), Error> {
        let suite = pf::suites::Handle::new()?;
        let handle = suite.new_handle(data.len() as _);
        let ptr = suite.lock_handle(handle);
        // SAFETY: Copying binary data to locked handle memory and assigning handle to FFI struct.
        // Detailed explanation: (1) ptr comes from lock_handle and points to valid, writable memory of at least data.len() bytes,
        // (2) data.as_ptr() is valid for reading data.len() bytes from the Vec's buffer,
        // (3) the source and destination regions do not overlap as they are from different allocations,
        // (4) both pointers are properly aligned for u8 access,
        // (5) self.0 points to a valid PF_ExtDependenciesExtra instance that can have its dependencies_strH field written.
        // Would be UB if: ptr is null/invalid, data.as_ptr() is invalid, regions overlap, handle size is insufficient, or self.0 is invalid.
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut u8, data.len());
            (*self.0).dependencies_strH = handle;
        }
        suite.unlock_handle(handle);
        Ok(())
    }
}

impl std::fmt::Debug for ExternalDependenciesExtra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("ExternalDependenciesExtra");
        dbg.field("check_type", &self.check_type());
        dbg.finish()
    }
}

define_enum! {
    ae_sys::PF_DepCheckType,
    DepCheckType {
        None                = ae_sys::PF_DepCheckType_NONE,
        AllDependencies     = ae_sys::PF_DepCheckType_ALL_DEPENDENCIES,
        MissingDependencies = ae_sys::PF_DepCheckType_MISSING_DEPENDENCIES,
    }
}
