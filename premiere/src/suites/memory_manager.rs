use crate::*;

define_suite!(MemoryManagerSuite, PrSDKMemoryManagerSuite, kPrSDKMemoryManagerSuite, kPrSDKMemoryManagerSuiteVersion);

impl MemoryManagerSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// Set the memory reserve size in bytes for the plugin with the specified ID.
    /// @param  inPluginID  The ID of the plugin.
    /// @param  inSize      The size in bytes to reserve.
    pub fn reserve_memory(&self, plugin_id: u32, size: u32) -> Result<(), Error> {
        pr_call_suite_fn!(self.suite_ptr, ReserveMemory, plugin_id, size)?;
        Ok(())
    }
}
