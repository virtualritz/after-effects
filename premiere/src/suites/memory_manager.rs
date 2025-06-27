use crate::*;

define_suite!(
    MemoryManagerSuite,
    PrSDKMemoryManagerSuite,
    kPrSDKMemoryManagerSuite,
    kPrSDKMemoryManagerSuiteVersion
);

impl MemoryManagerSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Set the memory reserve size in bytes for the plugin with the specified ID.
    /// * `plugin_id` - The ID of the plugin.
    /// * `size` - The size in bytes to reserve.
    pub fn reserve_memory(&self, plugin_id: u32, size: u32) -> Result<(), Error> {
        call_suite_fn!(self, ReserveMemory, plugin_id, size)
    }

    /// Get the current size of the media cache in this process.
    /// Returns the size of the memory manager in bytes.
    pub fn memory_manager_size(&self) -> Result<u64, Error> {
        call_suite_fn_single!(self, GetMemoryManagerSize -> u64)
    }

    /// Add a block of memory to management. This block should not be something entered
    /// in any of the suite cache, and it must be purgeable. The purge function you pass in may be called on any thread.
    /// * `size` - The size in bytes of the item in question.
    /// * `purge_function` - The function that will be called to purge the item.
    ///
    /// Returns the id the host will use for this item.
    pub fn add_block<F: FnOnce(u32) + Send + Sync + 'static>(&self, size: u64, purge_function: F) -> Result<u32, Error> {
        unsafe extern "C" fn purge_fn(data: *mut std::ffi::c_void, memory_id: u32) {
            let cb = unsafe { Box::<Box<dyn FnOnce(u32) + Send + Sync + 'static>>::from_raw(data as *mut _) };
            cb(memory_id);
        }
        let cb = Box::new(Box::new(purge_function));

        call_suite_fn_single!(self, AddBlock -> u32, size, Some(purge_fn), Box::into_raw(cb) as *mut _)
    }

    /// Each time you use a block of memory, you should call this function. This pushes its
    /// priority up in the cache, making a purge less likely.
    /// * `id` - The id of the block to touch.
    pub fn touch_block(&self, id: u32) -> Result<(), Error> {
        call_suite_fn!(self, TouchBlock, id)
    }

    /// You can manually expire an item from the cache with this function. Note that the purge function
    /// on the item will not be called.
    /// * `id` - The id of the block to touch.
    pub fn remove_block(&self, id: u32) -> Result<(), Error> {
        call_suite_fn!(self, RemoveBlock, id)
    }

    pub fn new_ptr_clear(&self, byte_count: u32) -> pr_sys::PrMemoryPtr {
        call_suite_fn_no_err!(self, NewPtrClear, byte_count)
    }

    pub fn new_ptr(&self, byte_count: u32) -> pr_sys::PrMemoryPtr {
        call_suite_fn_no_err!(self, NewPtr, byte_count)
    }

    pub fn ptr_size(&self, p: pr_sys::PrMemoryPtr) -> u32 {
        call_suite_fn_no_err!(self, GetPtrSize, p)
    }

    pub fn set_ptr_size(&self, p: *mut pr_sys::PrMemoryPtr, new_size: u32) {
        call_suite_fn_no_err!(self, SetPtrSize, p, new_size)
    }

    pub fn new_handle(&self, byte_count: u32) -> pr_sys::PrMemoryHandle {
        call_suite_fn_no_err!(self, NewHandle, byte_count)
    }

    pub fn new_handle_clear(&self, byte_count: u32) -> pr_sys::PrMemoryHandle {
        call_suite_fn_no_err!(self, NewHandleClear, byte_count)
    }

    pub fn dispose_ptr(&self, p: pr_sys::PrMemoryPtr) {
        call_suite_fn_no_err!(self, PrDisposePtr, p)
    }

    pub fn dispose_handle(&self, h: pr_sys::PrMemoryHandle) {
        call_suite_fn_no_err!(self, DisposeHandle, h)
    }

    pub fn set_handle_size(&self, h: pr_sys::PrMemoryHandle, new_size: u32) -> i16 {
        call_suite_fn_no_err!(self, SetHandleSize, h, new_size)
    }

    pub fn handle_size(&self, h: pr_sys::PrMemoryHandle) -> u32 {
        call_suite_fn_no_err!(self, GetHandleSize, h)
    }

    pub fn adjust_reserved_memory_size(&self, plugin_id: u32, size: i64) -> Result<(), Error> {
        call_suite_fn!(self, AdjustReservedMemorySize, plugin_id, size)
    }
}
