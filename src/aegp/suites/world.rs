use crate::*;
use crate::aegp::*;

define_suite!(
    /// [`World`]s are the common format used throughout the AEGP APIs to describe frames of pixels.
    WorldSuite,
    AEGP_WorldSuite3,
    kAEGPWorldSuite,
    kAEGPWorldSuiteVersion3
);

impl WorldSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Returns an allocated, initialized [`WorldHandle`].
    pub fn new_world(&self, plugin_id: PluginID, world_type: WorldType, width: u32, height: u32) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_New -> ae_sys::AEGP_WorldH, plugin_id, world_type.into(), width as _, height as _)?
        ))
    }

    /// Disposes of an [`WorldHandle`]. Use this on every world you allocate.
    pub fn dispose_world(&self, world: WorldHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_Dispose, world.as_ptr())
    }

    /// Returns the type of a given [`WorldHandle`]
    pub fn get_type(&self, world: WorldHandle) -> Result<WorldType, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetType -> ae_sys::AEGP_WorldType, world.as_ptr())?.into())
    }

    /// Returns the width and height of the given [`WorldHandle`].
    pub fn get_size(&self, world: WorldHandle) -> Result<(i32, i32), Error> {
        let (width, height) = call_suite_fn_double!(self, AEGP_GetSize -> ae_sys::A_long, ae_sys::A_long, world.as_ptr())?;
        Ok((
            width as i32,
            height as i32
        ))
    }

    /// Returns the rowbytes for the given [`WorldHandle`].
    pub fn get_row_bytes(&self, world: WorldHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetRowBytes -> ae_sys::A_u_long, world.as_ptr())? as usize)
    }

    /// Returns the base address of the [`WorldHandle`] for use in pixel iteration functions.
    ///
    /// Will return an error if used on a non-8bpc world.
    pub fn get_base_addr8(&self, world_handle: WorldHandle) -> Result<*mut pf::Pixel8, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetBaseAddr8 -> *mut ae_sys::PF_Pixel8, world_handle.as_ptr())? as _)
    }

    /// Returns the base address of the [`WorldHandle`] for use in pixel iteration functions.
    ///
    /// Will return an error if used on a non-16bpc world.
    pub fn get_base_addr16(&self, world_handle: WorldHandle) -> Result<*mut pf::Pixel16, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetBaseAddr16 -> *mut ae_sys::PF_Pixel16, world_handle.as_ptr())? as _)
    }

    /// Returns the base address of the [`WorldHandle`] for use in pixel iteration functions.
    ///
    /// Will return an error if used on a non-32bpc world.
    pub fn get_base_addr32(&self, world_handle: WorldHandle) -> Result<*mut pf::Pixel32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetBaseAddr32 -> *mut ae_sys::PF_PixelFloat, world_handle.as_ptr())? as _)
    }

    /// Populates and returns a [`EffectWorld`] representing the given [`WorldHandle`], for use with numerous pixel processing callbacks.
    ///
    /// NOTE: This does not give your plug-in ownership of the world referenced; destroy the source [`WorldHandle`] only if you allocated it.
    /// It just fills out the provided [`EffectWorld`] to point to the same pixel buffer.
    pub fn fill_out_pf_effect_world(&self, world: WorldHandle) -> Result<EffectWorld, Error> {
        Ok(EffectWorld {
            effect_world: call_suite_fn_single!(self, AEGP_FillOutPFEffectWorld -> ae_sys::PF_EffectWorld, world.as_ptr())?
        })
    }

    /// Performs a fast blur on a given [`WorldHandle`].
    pub fn fast_blur(&self, world: WorldHandle, radius: f64, mode: ModeFlags, quality: Quality) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_FastBlur, radius, mode.into(), quality.into(), world.as_ptr())
    }

    /// Creates a new [`PlatformWorldHandle`] (a pixel world native to the execution platform).
    pub fn new_platform_world(&self, plugin_id: PluginID, world_type: WorldType, width: i32, height: i32) -> Result<PlatformWorldHandle, Error> {
        Ok(PlatformWorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_NewPlatformWorld -> ae_sys::AEGP_PlatformWorldH, plugin_id, world_type.into(), width, height)?
        ))
    }

    /// Disposes of an [`PlatformWorldHandle`].
    pub fn dispose_platform_world(&self, world: PlatformWorldHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposePlatformWorld, world.as_ptr())
    }

    /// Retrieves an [`WorldHandle`] referring to the given [`PlatformWorldHandle`].
    ///
    /// NOTE: This doesn't allocate a new world, it simply provides a reference to an existing one.
    pub fn new_reference_from_platform_world(&self, plugin_id: PluginID, platform_world: PlatformWorldHandle) -> Result<WorldHandle, Error> {
        Ok(WorldHandle::from_raw(
            call_suite_fn_single!(self, AEGP_NewReferenceFromPlatformWorld -> ae_sys::AEGP_WorldH, plugin_id, platform_world.as_ptr())?
        ))
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_enum! {
    ae_sys::AEGP_WorldType,
    WorldType {
        None = ae_sys::AEGP_WorldType_NONE,
        U8   = ae_sys::AEGP_WorldType_8,
        /// Yes, Ae's 16bit color type is actually just 15bits!
        /// The underlying data type is ofc. an [`u16`].
        U15 = ae_sys::AEGP_WorldType_16,
        F32 = ae_sys::AEGP_WorldType_32,
    }
}

define_handle_wrapper!(WorldHandle, AEGP_WorldH);
define_handle_wrapper!(PlatformWorldHandle, AEGP_PlatformWorldH);

define_suite_item_wrapper!(
    ae_sys::AEGP_WorldH, WorldHandle,
    suite: WorldSuite,
    /// [`World`]s are the common format used throughout the AEGP APIs to describe frames of pixels.
    World {
        dispose: suite.dispose_world;

        /// Returns the type of this world
        r#type() -> WorldType => suite.get_type,

        /// Returns the width and height of this world
        size() -> (i32, i32) => suite.get_size,

        /// Returns the rowbytes of this world
        row_bytes() -> usize => suite.get_row_bytes,

        /// Returns the base address of this world for use in pixel iteration functions.
        ///
        /// Will return an error if used on a non-8bpc world.
        base_addr8() -> *mut pf::Pixel8 => suite.get_base_addr8,

        /// Returns the base address of this world for use in pixel iteration functions.
        ///
        /// Will return an error if used on a non-16bpc world.
        base_addr16() -> *mut pf::Pixel16 => suite.get_base_addr16,

        /// Returns the base address of this world for use in pixel iteration functions.
        ///
        /// Will return an error if used on a non-32bpc world.
        base_addr32() -> *mut pf::Pixel32 => suite.get_base_addr32,

        /// Performs a fast blur on this world.
        fast_blur(radius: f64, mode: ModeFlags, quality: Quality) -> () => suite.fast_blur,

        /// Returns a [`EffectWorld`] representing this world, for use with numerous pixel processing callbacks.
        ///
        /// NOTE: This does not give your plug-in ownership of the world referenced; destroy the source [`WorldHandle`] only if you allocated it.
        /// It just fills out the provided [`EffectWorld`] to point to the same pixel buffer.
        pf_effect_world() -> EffectWorld => suite.fill_out_pf_effect_world,
    }
);

impl World {
    pub fn new(plugin_id: PluginID, world_type: WorldType, width: u32, height: u32) -> Result<Self, Error> {
        let suite = WorldSuite::new()?;
        Ok(Self {
            handle: suite.new_world(plugin_id, world_type, width, height)?,
            suite,
            is_owned: true,
        })
    }
}
