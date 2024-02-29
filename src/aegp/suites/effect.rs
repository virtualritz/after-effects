use crate::*;
use crate::aegp::*;

define_suite!(
    /// Access the effects applied to a layer. This suite provides access to all parameter data streams.
    ///
    /// Use the [`suites::Stream`](aegp::suites::Stream) to work with those streams.
    ///
    /// An [`EffectRefHandle`] is a reference to an applied effect. An [`InstalledEffectKey`] is a reference to an installed effect, which may or may not be currently applied to anything.
    ///
    /// If Foobarocity is applied to a layer twice, there will be two distinct [`EffectRefHandle`]s, but they'll both return the same [`InstalledEffectKey`].
    EffectSuite,
    AEGP_EffectSuite4,
    kAEGPEffectSuite,
    kAEGPEffectSuiteVersion4
);

impl EffectSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Get the number of effects applied to a layer.
    pub fn layer_num_effects(&self, layer: &LayerHandle) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetLayerNumEffects -> ae_sys::A_long, layer.as_ptr())? as i32)
    }

    /// Retrieves (by index) a reference to an effect applied to the layer.
    pub fn layer_effect_by_index(&self, plugin_id: PluginId, layer: &LayerHandle, index: i32) -> Result<EffectRefHandle, Error> {
        Ok(EffectRefHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetLayerEffectByIndex -> ae_sys::AEGP_EffectRefH, plugin_id, layer.as_ptr(), index)?
        ))
    }

    /// Given an [`EffectRefHandle`], retrieves its associated [`InstalledEffectKey`].
    pub fn installed_key_from_layer_effect(&self, effect_ref: &EffectRefHandle) -> Result<InstalledEffectKey, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetInstalledKeyFromLayerEffect -> ae_sys::AEGP_InstalledEffectKey, effect_ref.as_ptr())?.into())
    }

    /// Returns description of effect parameter.
    ///
    /// Do not use the value(s) in the Param returned by this function (Use [`suites::Stream::new_stream_value()`](aegp::suites::Stream::new_stream_value) instead);
    /// it's provided so AEGPs can access parameter defaults, checkbox names, and pop-up strings.
    ///
    /// Use [`suites::Stream::effect_num_param_streams()`](aegp::suites::Stream::effect_num_param_streams) to get the stream count, useful for determining the maximum `param_index`.
    pub fn effect_param_union_by_index(&self, plugin_id: PluginId, effect_ref: &EffectRefHandle, param_index: i32) -> Result<pf::Param, Error> {
        let (param_type, u) = call_suite_fn_double!(self, AEGP_GetEffectParamUnionByIndex -> ae_sys::PF_ParamType, ae_sys::PF_ParamDefUnion, plugin_id, effect_ref.as_ptr(), param_index)?;

        unsafe {
            match param_type {
                ae_sys::PF_Param_ANGLE          => Ok(Param::Angle      (AngleDef      ::from_raw(u.ad))),
                ae_sys::PF_Param_ARBITRARY_DATA => Ok(Param::Arbitrary  (ArbitraryDef  ::from_raw(u.arb_d))),
                ae_sys::PF_Param_BUTTON         => Ok(Param::Button     (ButtonDef     ::from_raw(u.button_d))),
                ae_sys::PF_Param_CHECKBOX       => Ok(Param::CheckBox   (CheckBoxDef   ::from_raw(u.bd))),
                ae_sys::PF_Param_COLOR          => Ok(Param::Color      (ColorDef      ::from_raw(u.cd))),
                ae_sys::PF_Param_FLOAT_SLIDER   => Ok(Param::FloatSlider(FloatSliderDef::from_raw(u.fs_d))),
                ae_sys::PF_Param_POPUP          => Ok(Param::Popup      (PopupDef      ::from_raw(u.pd))),
                ae_sys::PF_Param_SLIDER         => Ok(Param::Slider     (SliderDef     ::from_raw(u.sd))),
                _ => Err(Error::InvalidParms),
            }
        }
    }

    /// Obtains the flags for the given [`EffectRefHandle`].
    pub fn effect_flags(&self, effect_ref: &EffectRefHandle) -> Result<EffectFlags, Error> {
        Ok(EffectFlags::from_bits_truncate(call_suite_fn_single!(self, AEGP_GetEffectFlags -> ae_sys::AEGP_EffectFlags, effect_ref.as_ptr())?))
    }

    /// Sets the flags for the given [`EffectRefHandle`], masked by a different set of effect flags.
    pub fn set_effect_flags(&self, effect_ref: &EffectRefHandle, set_mask: EffectFlags, flags: EffectFlags) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_SetEffectFlags, effect_ref.as_ptr(), set_mask.bits(), flags.bits())
    }

    /// Change the order of applied effects (pass the requested index).
    pub fn reorder_effect(&self, effect_ref: &EffectRefHandle, index: i32) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_ReorderEffect, effect_ref.as_ptr(), index)
    }

    /// Call an effect plug-in, and pass it a pointer to any data you like; the effect can modify it.
    ///
    /// This is how AEGPs communicate with effects.
    ///
    /// Pass [`Command::CompletelyGeneral`](crate::Command::CompletelyGeneral) for `command` to get the old behaviour.
    pub fn effect_call_generic<T: Sized>(&self, plugin_id: PluginId, effect_ref: &EffectRefHandle, time: Time, command: &pf::Command, extra_payload: Option<&T>) -> Result<(), Error> {
        // T is Sized so it can never be a fat pointer which means we are safe to transmute here.
        // Alternatively we could write extra_payload.map(|p| p as *const _).unwrap_or(core::ptr::null())
        call_suite_fn!(self, AEGP_EffectCallGeneric, plugin_id, effect_ref.as_ptr(), &time.into() as *const _, command.as_raw(), std::mem::transmute(extra_payload))
    }

    /// Disposes of an [`EffectRefHandle`]. Use this to dispose of any [`EffectRefHandle`] returned by After Effects.
    pub fn dispose_effect(&self, effect_ref: &EffectRefHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DisposeEffect, effect_ref.as_ptr())
    }

    /// Apply an effect to a given layer. Returns the newly-created [`EffectRefHandle`].
    pub fn apply_effect(&self, plugin_id: PluginId, layer: &LayerHandle, installed_effect_key: InstalledEffectKey) -> Result<EffectRefHandle, Error> {
        Ok(EffectRefHandle::from_raw(
            call_suite_fn_single!(self, AEGP_ApplyEffect -> ae_sys::AEGP_EffectRefH, plugin_id, layer.as_ptr(), installed_effect_key.into())?
        ))
    }

    /// Remove an applied effect.
    pub fn delete_layer_effect(&self, effect_ref: &EffectRefHandle) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_DeleteLayerEffect, effect_ref.as_ptr())
    }

    /// Returns the count of effects installed in After Effects.
    pub fn num_installed_effects(&self) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumInstalledEffects -> ae_sys::A_long)? as i32)
    }

    /// Returns the [`InstalledEffectKey`] of the next installed effect.
    ///
    /// Pass [`InstalledEffectKey::None`] as the first parameter to obtain the first [`InstalledEffectKey`].
    pub fn next_installed_effect(&self, installed_effect_key: InstalledEffectKey) -> Result<InstalledEffectKey, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNextInstalledEffect -> ae_sys::AEGP_InstalledEffectKey, installed_effect_key.into())?.into())
    }

    /// Get name of the effect. `name` can be up to `48` characters long.
    ///
    /// Note: use [`suites::DynamicStream::set_stream_name()`](aegp::suites::DynamicStream::set_stream_name) to change the display name of an effect.
    pub fn effect_name(&self, installed_effect_key: InstalledEffectKey) -> Result<String, Error> {
        let mut name = [0i8; ae_sys::AEGP_MAX_EFFECT_NAME_SIZE as usize + 1];
        call_suite_fn!(self, AEGP_GetEffectName, installed_effect_key.into(), name.as_mut_ptr() as _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Get match name of an effect (defined in PiPL). `match_name` up to `48` characters long.
    ///
    /// Match names are in 7-bit ASCII. UI names are in the current application runtime encoding;
    /// for example, ISO 8859-1 for most languages on Windows.
    pub fn effect_match_name(&self, installed_effect_key: InstalledEffectKey) -> Result<String, Error> {
        let mut name = [0i8; ae_sys::AEGP_MAX_EFFECT_MATCH_NAME_SIZE as usize + 1];
        call_suite_fn!(self, AEGP_GetEffectMatchName, installed_effect_key.into(), name.as_mut_ptr() as _)?;
        // TODO: It's not UTF-8
        Ok(unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Menu category of effect. `category` can be up to `48` characters long.
    pub fn effect_category(&self, installed_effect_key: InstalledEffectKey) -> Result<String, Error> {
        let mut name = [0i8; ae_sys::AEGP_MAX_EFFECT_CATEGORY_NAME_SIZE as usize + 1];
        call_suite_fn!(self, AEGP_GetEffectCategory, installed_effect_key.into(), name.as_mut_ptr() as _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Duplicates a given [`EffectRefHandle`]. Caller must dispose of duplicate when finished.
    pub fn duplicate_effect(&self, original_effect_ref: &EffectRefHandle) -> Result<EffectRefHandle, Error> {
        Ok(EffectRefHandle::from_raw(
            call_suite_fn_single!(self, AEGP_DuplicateEffect -> ae_sys::AEGP_EffectRefH, original_effect_ref.as_ptr())?
        ))
    }

    /// New in CC 2014. How many masks are on this effect?
    pub fn num_effect_mask(&self, effect_ref: &EffectRefHandle) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, AEGP_NumEffectMask -> ae_sys::A_u_long, effect_ref.as_ptr())? as usize)
    }

    /// New in CC 2014. For a given mask_indexL, returns the corresponding `AEGP_MaskIDVal` for use in uniquely identifying the mask.
    pub fn effect_mask_id(&self, effect_ref: &EffectRefHandle, mask_index: usize) -> Result<ae_sys::AEGP_MaskIDVal, Error> {
        call_suite_fn_single!(self, AEGP_GetEffectMaskID -> ae_sys::AEGP_MaskIDVal, effect_ref.as_ptr(), mask_index as ae_sys::A_u_long)
    }

    /// New in CC 2014. Add an effect mask, which may be created using the [`suites::Mask`](aegp::suites::Mask).
    ///
    /// Returns the local stream of the effect ref - useful if you want to add keyframes.
    ///
    /// Caller must dispose of [`StreamReferenceHandle`] when finished.
    ///
    /// Undoable.
    pub fn add_effect_mask(&self, effect_ref: &EffectRefHandle, id_val: ae_sys::AEGP_MaskIDVal) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle::from_raw(
            call_suite_fn_single!(self, AEGP_AddEffectMask -> ae_sys::AEGP_StreamRefH, effect_ref.as_ptr(), id_val)?
        ))
    }

    /// New in CC 2014. Remove an effect mask.
    ///
    /// Undoable.
    pub fn remove_effect_mask(&self, effect_ref: &EffectRefHandle, id_val: ae_sys::AEGP_MaskIDVal) -> Result<(), Error> {
        call_suite_fn!(self, AEGP_RemoveEffectMask, effect_ref.as_ptr(), id_val)
    }

    /// New in CC 2014. Set an effect mask on an existing index.
    ///
    /// Returns the local stream of the effect ref - useful if you want to add keyframes.
    ///
    /// Caller must dispose of [`StreamReferenceHandle`] when finished.
    ///
    /// Undoable.
    pub fn set_effect_mask(&self, effect_ref: &EffectRefHandle, mask_index: usize, id_val: ae_sys::AEGP_MaskIDVal) -> Result<StreamReferenceHandle, Error> {
        Ok(StreamReferenceHandle::from_raw(
            call_suite_fn_single!(self, AEGP_SetEffectMask -> ae_sys::AEGP_StreamRefH, effect_ref.as_ptr(), mask_index as ae_sys::A_u_long, id_val)?
        ))
    }
}

// ――――――――――――――――――――――――――――――――――――――― Types ――――――――――――――――――――――――――――――――――――――――

define_handle_wrapper!(EffectRefHandle, AEGP_EffectRefH);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstalledEffectKey {
    None,
    Key(i32)
}
impl From<ae_sys::AEGP_InstalledEffectKey> for InstalledEffectKey {
    fn from(key: ae_sys::AEGP_InstalledEffectKey) -> Self {
        if key == ae_sys::AEGP_InstalledEffectKey_NONE as ae_sys::AEGP_InstalledEffectKey  {
            InstalledEffectKey::None
        } else {
            InstalledEffectKey::Key(key as _)
        }
    }
}
impl Into<ae_sys::AEGP_InstalledEffectKey> for InstalledEffectKey {
    fn into(self) -> ae_sys::AEGP_InstalledEffectKey {
        match self {
            InstalledEffectKey::None     => ae_sys::AEGP_InstalledEffectKey_NONE as ae_sys::AEGP_InstalledEffectKey,
            InstalledEffectKey::Key(key) => key as ae_sys::AEGP_InstalledEffectKey,
        }
    }
}

bitflags::bitflags! {
    pub struct EffectFlags: ae_sys::A_long {
        const NONE       = ae_sys::AEGP_EffectFlags_NONE       as ae_sys::A_long;
        const ACTIVE     = ae_sys::AEGP_EffectFlags_ACTIVE     as ae_sys::A_long;
        const AUDIO_ONLY = ae_sys::AEGP_EffectFlags_AUDIO_ONLY as ae_sys::A_long;
        const AUDIO_TOO  = ae_sys::AEGP_EffectFlags_AUDIO_TOO  as ae_sys::A_long;
        const MISSING    = ae_sys::AEGP_EffectFlags_MISSING    as ae_sys::A_long;
    }
}
