use crate::*;

define_suite!(
    HashSuite,
    AEGP_HashSuite1,
    kAEGPHashSuite,
    kAEGPHashSuiteVersion1
);

/// A GUID used as a hash key for the compute cache.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Guid(pub ae_sys::AEGP_GUID);

impl Guid {
    pub fn new() -> Self { Self(ae_sys::AEGP_GUID { bytes: [0; 4] }) }

    pub fn as_mut_ptr(&mut self) -> *mut ae_sys::AEGP_GUID { &mut self.0 }

    pub fn as_raw(&self) -> ae_sys::AEGP_GUID { self.0 }
}

impl Default for Guid {
    fn default() -> Self { Self::new() }
}

impl From<ae_sys::AEGP_GUID> for Guid {
    fn from(guid: ae_sys::AEGP_GUID) -> Self { Self(guid) }
}

impl From<Guid> for ae_sys::AEGP_GUID {
    fn from(guid: Guid) -> Self { guid.0 }
}

impl HashSuite {
    /// Call this to begin creating the hash which will be returned in hashP
    /// that can be used for returning from generate_key.
    pub fn create_hash_from_ptr(&self, data: &[u8]) -> Result<Guid, Error> {
        let mut hash = Guid::new();
        call_suite_fn!(
            self,
            AEGP_CreateHashFromPtr,
            data.len() as ae_sys::A_u_longlong,
            data.as_ptr() as *const _,
            hash.as_mut_ptr()
        )?;
        Ok(hash)
    }

    /// Call this for each effect parameter, layer checkout hash or other data
    /// that would be used in calculating a cache entry.
    pub fn hash_mix_in_ptr(&self, data: &[u8], hash: &mut Guid) -> Result<(), Error> {
        call_suite_fn!(
            self,
            AEGP_HashMixInPtr,
            data.len() as ae_sys::A_u_longlong,
            data.as_ptr() as *const _,
            hash.as_mut_ptr()
        )
    }
}
