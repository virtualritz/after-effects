use crate::*;

define_struct_wrapper!(ExternalDependenciesExtra, PF_ExtDependenciesExtra);

impl ExternalDependenciesExtra {
    pub fn check_type(&self) -> DepCheckType {
        unsafe { (*self.0).check_type.into() }
    }

    pub fn set_dependencies_str(&mut self, text: &str) -> Result<(), Error> {
        let suite = pf::suites::Handle::new()?;
        let text = std::ffi::CString::new(text).unwrap();
        let text = text.as_bytes_with_nul();
        let handle = suite.new_handle(text.len() as _);
        let ptr = suite.lock_handle(handle);
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
