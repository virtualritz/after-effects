#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod resource;
pub use resource::*;

use byteorder::{ WriteBytesExt, LittleEndian };
use std::io::Result;

#[derive(Debug)]
pub enum PIPLType {
    General, Filter, Parser, ImageFormat, Extension, Acquire, Export, Selection, Picker, Actions, Test, MSPUtility, PsModernFilter,
    AEEffect, AEImageFormat, AEAccelerator, AEGeneral,
    PrEffect, PrVideoFilter, PrAudioFilter, PrEDLExport, PrDataExport, PrDevice, PrImporter, PrCompile, PrRecord, PrPlay,
    SweetPea, AIGeneral
}

impl PIPLType {
    pub fn as_u32(&self) -> u32 {
        match self {
            // Photoshop plug-in types
            Self::General        => u32::from_be_bytes(*b"8BPI"),
            Self::Filter         => u32::from_be_bytes(*b"8BFM"),
            Self::Parser         => u32::from_be_bytes(*b"8BYM"),
            Self::ImageFormat    => u32::from_be_bytes(*b"8BIF"),
            Self::Extension      => u32::from_be_bytes(*b"8BXM"),
            Self::Acquire        => u32::from_be_bytes(*b"8BAM"),
            Self::Export         => u32::from_be_bytes(*b"8BEM"),
            Self::Selection      => u32::from_be_bytes(*b"8BSM"),
            Self::Picker         => u32::from_be_bytes(*b"8BCM"),
            Self::Actions        => u32::from_be_bytes(*b"8LIZ"),
            Self::Test           => u32::from_be_bytes(*b"8BTS"),
            Self::MSPUtility     => u32::from_be_bytes(*b"8SPU"),
            Self::PsModernFilter => u32::from_be_bytes(*b"8BFm"),
            // After Effects plug-in types
            Self::AEEffect       => u32::from_be_bytes(*b"eFKT"),
            Self::AEImageFormat  => u32::from_be_bytes(*b"FXIF"),
            Self::AEAccelerator  => u32::from_be_bytes(*b"eFST"),
            Self::AEGeneral      => u32::from_be_bytes(*b"AEgp"),
            // Premiere plug-in types
            Self::PrEffect       => u32::from_be_bytes(*b"SPFX"),
            Self::PrVideoFilter  => u32::from_be_bytes(*b"VFlt"),
            Self::PrAudioFilter  => u32::from_be_bytes(*b"AFlt"),
            Self::PrEDLExport    => u32::from_be_bytes(*b"ExpM"),
            Self::PrDataExport   => u32::from_be_bytes(*b"ExpD"),
            Self::PrDevice       => u32::from_be_bytes(*b"DevC"),
            Self::PrImporter     => u32::from_be_bytes(*b"IMPT"),
            Self::PrCompile      => u32::from_be_bytes(*b"CMPM"),
            Self::PrRecord       => u32::from_be_bytes(*b"RECM"),
            Self::PrPlay         => u32::from_be_bytes(*b"PLYM"),
            // Illustrator/SweetPea plug-in types
            Self::SweetPea       => u32::from_be_bytes(*b"SPEA"),
            Self::AIGeneral      => u32::from_be_bytes(*b"ARPI")
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct OutFlags: u32 {
        const None = 0;
                                                // which PF_Cmds each flag is relevant for:
        const KeepResourceOpen             = 1 << 0;  // PF_Cmd_GLOBAL_SETUP
        const WideTimeInput                = 1 << 1;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const NonParamVary                 = 1 << 2;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const Reserved6                    = 1 << 3;
        const SequenceDataNeedsFlattening  = 1 << 4;  // PF_Cmd_GLOBAL_SETUP
        const IDoDialog                    = 1 << 5;  // PF_Cmd_GLOBAL_SETUP
        const UseOutputExtent              = 1 << 6;  // PF_Cmd_GLOBAL_SETUP
        const SendDoDialog                 = 1 << 7;  // PF_Cmd_SEQUENCE_SETUP
        const DisplayErrorMessage          = 1 << 8;  // all PF_Cmds
        const IExpandBuffer                = 1 << 9;  // PF_Cmd_GLOBAL_SETUP
        const PixIndependent               = 1 << 10; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const IWriteInputBuffer            = 1 << 11; // PF_Cmd_GLOBAL_SETUP
        const IShrinkBuffer                = 1 << 12; // PF_Cmd_GLOBAL_SETUP
        const WorksInPlace                 = 1 << 13; // PF_Cmd_GLOBAL_SETUP
        const Reserved8                    = 1 << 14;
        const CustomUI                     = 1 << 15; // PF_Cmd_GLOBAL_SETUP
        const Reserved7                    = 1 << 16;
        const RefreshUI                    = 1 << 17; // PF_Cmd_EVENT, PF_Cmd_RENDER, PF_Cmd_DO_DIALOG
        const NopRender                    = 1 << 18; // PF_Cmd_GLOBAL_SETUP
        const IUseShutterAngle             = 1 << 19; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const IUseAudio                    = 1 << 20; // PF_Cmd_GLOBAL_SETUP
        const IAmObsolete                  = 1 << 21; // PF_Cmd_GLOBAL_SETUP
        const ForceRerender                = 1 << 22; // PF_Cmd_EVENT, PF_Cmd_USER_CHANGED_PARAM, PF_Cmd_UPDATE_PARAMS_UI
        const PiplOverridesOutdataOutflags = 1 << 23; // PiPL-only-flag
        const IHaveExternalDependencies    = 1 << 24; // PF_Cmd_GLOBAL_SETUP
        const DeepColorAware               = 1 << 25; // PF_Cmd_GLOBAL_SETUP
        const SendUpdateParamsUI           = 1 << 26; // PF_Cmd_GLOBAL_SETUP

        // audio flags (pfOutflagAudio_EFFECT_TOO or PF_OutFlag_AUDIO_EFFECT_ONLY required for audio effects)
        const AudioFloatOnly               = 1 << 27; // PF_Cmd_GLOBAL_SETUP
        const AudioIir                     = 1 << 28; // PF_Cmd_GLOBAL_SETUP
        const ISynthesizeAudio             = 1 << 29; // PF_Cmd_GLOBAL_SETUP
        const AudioEffectToo               = 1 << 30; // PF_Cmd_GLOBAL_SETUP
        const AudioEffectOnly              = 1 << 31; // PF_Cmd_GLOBAL_SETUP
    }
}
bitflags::bitflags! {
    #[derive(Debug)]
    pub struct OutFlags2: u32 {
        const None = 0;
                                                       // which PF_Cmds each flag is relevant for:
        const SupportsQueryDynamicFlags           = 1 << 0;  // PF_Cmd_GLOBAL_SETUP
        const IUse3DCamera                        = 1 << 1;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const IUse3DLights                        = 1 << 2;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const ParamGroupStartCollapsedFlag        = 1 << 3;  // PF_Cmd_GLOBAL_SETUP
        const IAmThreadsafe                       = 1 << 4;  // PF_Cmd_GLOBAL_SETUP (unused)
        const CanCombineWithDestination           = 1 << 5;  // Premiere only (as of AE 6.0)
        const DoesntNeedEmptyPixels               = 1 << 6;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const RevealsZeroAlpha                    = 1 << 7;  // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const PreservesFullyOpaquePixels          = 1 << 8;  // Premiere only (as of AE 6.0)
        const SupportsSmartRender                 = 1 << 10; // PF_Cmd_GLOBAL_SETUP
        const Reserved9                           = 1 << 11; // PF_Cmd_GLOBAL_SETUP
        const FloatColorAware                     = 1 << 12; // PF_Cmd_GLOBAL_SETUP, may require PF_OutFlag2_SUPPORTS_SMART_RENDER
        const IUseColorspaceEnumeration           = 1 << 13; // PF_Cmd_GLOBAL_SETUP, not implemented in AE7 (may be impl in Premiere Pro)
        const IAmDeprecated                       = 1 << 14; // PF_Cmd_GLOBAL_SETUP
        const PproDoNotCloneSequenceDataForRender = 1 << 15; // PF_Cmd_GLOBAL_SETUP, Premiere only, CS4.1 and later
        const Reserved10                          = 1 << 16; // PF_Cmd_GLOBAL_SETUP
        const AutomaticWideTimeInput              = 1 << 17; // PF_Cmd_GLOBAL_SETUP, falls back to PF_OutFlag_WIDE_TIME_INPUT if not PF_OutFlag2_SUPPORTS_SMART_RENDER
        const IUseTimecode                        = 1 << 18; // PF_Cmd_GLOBAL_SETUP
        const DependsOnUnreferencedMasks          = 1 << 19; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const OutputIsWatermarked                 = 1 << 20; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_QUERY_DYNAMIC_FLAGS
        const IMixGuidDependencies                = 1 << 21; // PF_Cmd_GLOBAL_SETUP
        const Ae135Threadsafe                     = 1 << 22; // PF_Cmd_GLOBAL_SETUP (unused)
        const SupportsGetFlattenedSequenceData    = 1 << 23; // PF_Cmd_GLOBAL_SETUP, support required if both PF_OutFlag_SEQUENCE_DATA_NEEDS_FLATTENING and PF_OutFlag2_SUPPORTS_THREADED_RENDERING is set
        const CustomUIAsyncManager                = 1 << 24; // PF_Cmd_GLOBAL_SETUP
        const SupportsGpuRenderF32                = 1 << 25; // PF_Cmd_GLOBAL_SETUP, PF_Cmd_GPU_DEVICE_SETUP. Must also set PF_RenderOutputFlag_GPU_RENDER_POSSIBLE at pre-render to enable GPU rendering.
        const Reserved12                          = 1 << 26; // PF_Cmd_GLOBAL_SETUP
        const SupportsThreadedRendering           = 1 << 27; // PF_Cmd_GLOBAL_SETUP
        const MutableRenderSequenceDataSlower     = 1 << 28; // PF_Cmd_GLOBAL_SETUP
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct SupportedModes: u32 {
		const Bitmap           = 1 << 15;
		const GrayScale        = 1 << 14;
		const IndexedColor     = 1 << 13;
		const RGBColor         = 1 << 12;
		const CMYKColor        = 1 << 11;
		const HSLColor         = 1 << 10;
		const HSBColor         = 1 << 9;
		const Multichannel     = 1 << 8;
		const Duotone          = 1 << 7;
		const LABColor         = 1 << 6;
		const Gray16           = 1 << 5;
		const RGB48            = 1 << 4;
		const Lab48            = 1 << 3;
		const CMYK64           = 1 << 2;
		const DeepMultichannel = 1 << 1;
		const Duotone16        = 1 << 0;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FilterCaseInfoIn {
    CantFilter = 0,
    StraightData = 1,
    BlackMat = 2,
    GrayMat = 3,
    WhiteMat = 4,
    Defringe = 5,
    BlackZap = 6,
    GrayZap = 7,
    WhiteZap = 8,
    BackgroundZap = 10,
    ForegroundZap = 11
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FilterCaseInfoOut {
    CantFilter = 0,
    StraightData = 1,
    BlackMat = 2,
    GrayMat = 3,
    WhiteMat = 4,
    FillMask = 9
}
#[derive(Debug)]
pub struct FilterCaseInfoStruct {
    in_handling: FilterCaseInfoIn,
    out_handling: FilterCaseInfoOut,
    write_outside_selection: bool,
    filters_layer_masks: bool,
    works_with_blank_data: bool,
    copy_source_to_destination: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BitTypes {
    None       = 0x00,
    Top        = 0x01,
    Right      = 0x02,
    Bottom     = 0x04,
    Left       = 0x08,
    UpperRight = 0x10,
    LowerRight = 0x20,
    LowerLeft  = 0x40,
    UpperLeft  = 0x80,
}
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PixelAspectRatio {
	AnyPAR   = 0x10000,
	UnityPAR = 0x20000,
}

#[repr(u32)] #[derive(Debug, Clone, Copy)] pub enum AnimDataType { Opaque = 0, Char, Short, Long, UnsignedChar, UnsignedShort, UnsignedLong, Fixed, UnsignedFixed, Extended96, Double64, Float32, ColorRGB }
#[repr(u32)] #[derive(Debug, Clone, Copy)] pub enum AnimUIType { NoUI = 0, Angle, Slider, Point, Rect, ColorRGB, ColorCMYK, ColorLAB }
#[repr(u32)] #[derive(Debug, Clone, Copy)] pub enum ClassType { None = 0, Scanner, Camera, Video, Floppy, Cdrom, Internet }
#[derive(Debug)] pub enum ButtonIconType { None, MacCICN, WindowsICON }

pub const fn pf_version(vers: u32, subvers: u32, bugvers: u32, stage: u32, build: u32) -> u32 {
    const PF_VERS_BUILD_BITS   : u32 = 0x1ff;
    const PF_VERS_BUILD_SHIFT  : u32 = 0;
    const PF_VERS_STAGE_BITS   : u32 = 0x3;
    const PF_VERS_STAGE_SHIFT  : u32 = 9;
    const PF_VERS_BUGFIX_BITS  : u32 = 0xf;
    const PF_VERS_BUGFIX_SHIFT : u32 = 11;
    const PF_VERS_SUBVERS_BITS : u32 = 0xf;
    const PF_VERS_SUBVERS_SHIFT: u32 = 15;
    const PF_VERS_VERS_BITS    : u32 = 0x7;    // incomplete without high bits, below
    const PF_VERS_VERS_SHIFT   : u32 = 19;
    // skipping these bits for similarity to Up_Vers_ARCH_*, currently unused in PF
    const PF_VERS_VERS_HIGH_BITS : u32 = 0xf; // expand version max from 7 to 127
    const PF_VERS_VERS_HIGH_SHIFT: u32 = 26;
    // b/c we are stripping the stand alone vers value for two fields
    const PF_VERS_VERS_LOW_SHIFT: u32 = 3;

    (((vers >> PF_VERS_VERS_LOW_SHIFT) & PF_VERS_VERS_HIGH_BITS) << PF_VERS_VERS_HIGH_SHIFT) |
    ((vers & PF_VERS_VERS_BITS) << PF_VERS_VERS_SHIFT) |
    ((subvers & PF_VERS_SUBVERS_BITS) << PF_VERS_SUBVERS_SHIFT) |
    ((bugvers & PF_VERS_BUGFIX_BITS)  << PF_VERS_BUGFIX_SHIFT) |
    ((stage & PF_VERS_STAGE_BITS) << PF_VERS_STAGE_SHIFT) |
    ((build & PF_VERS_BUILD_BITS) << PF_VERS_BUILD_SHIFT)
}


#[derive(Debug)]
pub enum Property {
    Kind(PIPLType),
    Version((u32, u32, u32, u32, u32)),
    Priority(u32),
    RequiredHost(&'static [u8; 4]),
    Component((u32, &'static str)),
    Name(&'static str),
    Category(&'static str),
    Code68k((PIPLType, u16)),
    Code68kFPU((PIPLType, u16)),
    CodePowerPC((u32, u32, &'static str)),
    CodeCarbonPowerPC((u32, u32, &'static str)),
    CodeMachOPowerPC(&'static str),
    CodeMacIntel32(&'static str),
    CodeMacIntel64(&'static str),
    CodeMacARM64(&'static str),
    CodeWin32X86(&'static str),
    CodeWin64X86(&'static str),
    SupportedModes(SupportedModes),
    EnableInfo(&'static str),
    FilterCaseInfo(&'static [FilterCaseInfoStruct]),
    ExportFlags { supports_transparency: bool },
    FmtFileType((&'static [u8; 4], &'static [u8; 4])),
    ReadTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    WriteTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    FilteredTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ReadExtensions(&'static [&'static [u8; 4]]),
    WriteExtensions(&'static [&'static [u8; 4]]),
    FilteredExtensions(&'static [&'static [u8; 4]]),
    FormatFlags {
        saves_image_resources: bool,
        can_read: bool,
        can_write: bool,
        can_write_if_read: bool,
    },
    FormatMaxSize { width: u16, height: u16 },
    FormatMaxChannels(&'static [u16]),
    ParsableTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ParsableClipboardTypes(&'static [&'static [u8; 4]]),
    FilteredParsableTypes(&'static [(&'static [u8; 4], &'static [u8; 4])]),
    ParsableExtensions(&'static [&'static [u8; 4]]),
    FilteredParsableExtensions(&'static [&'static [u8; 4]]),
    PickerID(&'static str),
    HasTerminology {
        class_id: u32,
        event_id: u32,
        dictionary_resource_id: u16,
        unique_scope_string: &'static str
    },
    Persistent,
    AE_PiPL_Version { minor: u16, major: u16 },
    AE_Effect_Spec_Version { minor: u16, major: u16 },
    AE_Effect_Version((u32, u32, u32, u32, u32)),
    AE_Effect_Match_Name(&'static str),
    AE_Effect_Info_Flags(u32),
    AE_Effect_Global_OutFlags(OutFlags),
    AE_Effect_Global_OutFlags_2(OutFlags2),
    AE_Reserved(u32),
    AE_Reserved_Info(u32),
    AE_Effect_Support_URL(&'static str),
    AE_ImageFormat_Extension_Info {
        major_version: u16,
        minor_version: u16,
        has_options: bool,
        sequential_only: bool,
        must_interact: bool,
        has_interact_put: bool,
        has_interact_get: bool,
        has_time: bool,
        has_video: bool,
        still: bool,
        has_file: bool,
        output: bool,
        input: bool,
        signature: [u8; 4]
    },
    ANIM_FilterInfo {
        spec_version_major: u32,
        spec_version_minor: u32,
        filter_params_version: u32,
        unity_pixel_aspec_tratio: bool,
        any_pixel_aspect_ratio: bool,
        drive_me: bool,                 // ANIM_FF_DONT_DRIVE_ME (AE only)
        needs_dialog: bool,        // ANIM_FF_DOESNT_NEED_DLOG (AE only)
        params_pointer: bool,    // ANIM_FF_PARAMS_ARE PTR (AE only)
        params_handle: bool,        // ANIM_FF_PARAMS_ARE_HANDLE (AE only)
        params_mac_handle: bool,    // ANIM_FF_PARAMS_ARE_MAC_HANDLE (AE only)
        dialog_in_render: bool,    // ANIM_FF_DIALOG_IN_RENDER (AE only)
        params_in_globals: bool,    // ANIM_FF_PARAMS_IN_GLOBALS (AE only)
        bg_animatable: bool,        // ANIM_FF_BG_ANIMATABLE (AE only)
        fg_animatable: bool,        // ANIM_FF_FG_ANIMATABLE (AE only)
        geometric: bool,            // ANIM_FF_NON_GEOMETRIC (AE only)
        randomness: bool,            // ANIM_FF_HAS_RANDOMNESS (AE only)
        number_of_parameters: u32,
        match_name: &'static str, // 32 bytes
    },
    ANIM_ParamAtom {
        external_name: &'static str, // 32 bytes
        match_id: u32,
        data_type: AnimDataType,
        ui_type: AnimUIType,
        valid_min: f64,
        valid_max: f64,
        ui_min: f64,
        ui_max: f64,

        scale_ui_range: bool,
        animate_param: bool,
        restrict_bounds: bool,
        space_is_relative: bool,
        res_dependant: bool,

        property_size: u32, // size of property described in bytes (short = 2, long = 4, etc.)
    },
    Pr_Effect_Info {
        version: u32,
        valid_corners_mask: BitTypes,
        initial_corners: BitTypes,

        exclusive_dialog: bool,
        needs_callbacks_at_setup: bool,
        direct_comp_data: bool,
        want_initial_setup_call: bool,
        treat_as_transition: bool,
        has_custom_dialog: bool,
        highlight_opposite_corners: bool,
        exclusive: bool,
        reversible: bool,
        have_edges: bool,
        have_start_point: bool,
        have_end_point: bool,

        more_flags: u32
    },
    Pr_Effect_Description(&'static str),
    InterfaceVersion(u32),
    AdapterVersion(u32),
    SP_STSP(u32),
    InternalName(&'static str),
    Imports(&'static [(&'static str, u32)]), // suite name, version
    Exports(&'static [(&'static str, u32)]), // suite name, version
    Description(&'static str),
    Keywords(&'static [&'static str]),
    Title(&'static str),
    Messages {
        startup_required: bool,
        purge_cache: bool,
        shutdown_required: bool,
        accept_property: bool,
    },
    ButtonIcon {
        version: u32,
        mac_icon_type: ButtonIconType,
        win_icon_type: ButtonIconType,
        resource_id: u32,
        icon_name: &'static str,
    },
    Class {
        version: u32,
        class: ClassType
    },
    PreviewFile {
        version: u32,
        filename: &'static str
    }
}

pub fn build_pipl(properties: Vec<Property>) -> Result<Vec<u8>> {
	fn padding_4(x: u32) -> u32 { if x % 4 != 0 { 4 - x % 4 } else { 0 } }

	fn write(buffer: &mut Vec<u8>, type_: &[u8; 4], key: &[u8; 4], mut contents_fn: impl FnMut(&mut Vec<u8>) -> Result<()>) -> Result<()> {
		buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_))?;
		buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*key))?;
		buffer.write_u32::<LittleEndian>(0)?; // pad
		let len = buffer.len();
		buffer.write_u32::<LittleEndian>(0)?; // length placeholder
		contents_fn(buffer)?;
		let aligned_len = (buffer.len() - len - 4) as u32;
		// Overwrite the length
		buffer[len..len+4].clone_from_slice(&aligned_len.to_le_bytes());
        Ok(())
	}
	fn write_pstring(buffer: &mut Vec<u8>, s: &'static str) -> Result<()> { // Pascal string
		buffer.write_u8(s.len() as u8)?;
		buffer.extend(s.as_bytes());
		let padding = padding_4(s.len() as u32 + 1);
		for _ in 0..padding { buffer.write_u8(0)?; }
        Ok(())
	}
	fn write_cstring(buffer: &mut Vec<u8>, s: &'static str) -> Result<()> { // Long Word padded C String
		buffer.extend(s.as_bytes());
		buffer.push(0);
		let padding = padding_4(s.len() as u32 + 1);
		for _ in 0..padding { buffer.write_u8(0)?; }
        Ok(())
	}

    let mut buffer = Vec::new();
	buffer.write_u16::<LittleEndian>(1)?; // Reserved
    buffer.write_u32::<LittleEndian>(0)?; // kPIPropertiesVersion
    buffer.write_u32::<LittleEndian>(properties.len() as u32)?;
    for prop in properties {
        match prop {
            Property::Kind(x) => {
                write(&mut buffer, b"8BIM", b"kind", |buffer| {
					buffer.write_u32::<LittleEndian>(x.as_u32())
				})?;
            },
            Property::Version((a, b, c, d, e)) => {
                write(&mut buffer, b"8BIM", b"vers", |buffer| {
					buffer.write_u32::<LittleEndian>(pf_version(a, b, c, d, e))
				})?;
            },
            Property::Priority(x) => {
                write(&mut buffer, b"8BIM", b"prty", |buffer| {
					buffer.write_u32::<LittleEndian>(x)
				})?;
            },
            Property::Component((version, uuid)) => {
                write(&mut buffer, b"8BIM", b"cmpt", |buffer| {
					buffer.write_u32::<LittleEndian>(version)?;
					write_cstring(buffer, uuid)
				})?;
            },
            Property::RequiredHost(x) => {
                write(&mut buffer, b"8BIM", b"host", |buffer| {
					buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*x))
				})?;
			},
            Property::Name(x) => {
                write(&mut buffer, b"8BIM", b"name", |buffer| {
					write_pstring(buffer, x)
				})?;
			},
            Property::Category(x) => {
				// PSHelpMenu = "**Help**";
                write(&mut buffer, b"8BIM", b"catg", |buffer| {
					write_pstring(buffer, x)
				})?;
			},
            Property::Code68k((type_, x)) => {
                write(&mut buffer, b"8BIM", b"m68k", |buffer| {
					buffer.write_u32::<LittleEndian>(type_.as_u32())?;
					buffer.write_u16::<LittleEndian>(x)
				})?;
			},
            Property::Code68kFPU((type_, x)) => {
                write(&mut buffer, b"8BIM", b"68fp", |buffer| {
					buffer.write_u32::<LittleEndian>(type_.as_u32())?;
					buffer.write_u16::<LittleEndian>(x)
				})?;
			},
            Property::CodePowerPC((x, y, entry_point)) => {
                write(&mut buffer, b"8BIM", b"pwpc", |buffer| {
					buffer.write_u32::<LittleEndian>(x)?;
					buffer.write_u32::<LittleEndian>(y)?;
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeCarbonPowerPC((x, y, entry_point)) => {
                write(&mut buffer, b"8BIM", b"ppcb", |buffer| {
					buffer.write_u32::<LittleEndian>(x)?;
					buffer.write_u32::<LittleEndian>(y)?;
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeMachOPowerPC(entry_point) => {
                write(&mut buffer, b"8BIM", b"mach", |buffer| {
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeMacIntel32(entry_point) => {
                write(&mut buffer, b"8BIM", b"mi32", |buffer| {
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeMacIntel64(entry_point) => {
                write(&mut buffer, b"8BIM", b"mi64", |buffer| {
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeMacARM64(entry_point) => {
                write(&mut buffer, b"8BIM", b"ma64", |buffer| {
					write_pstring(buffer, entry_point)
				})?;
			},
            Property::CodeWin32X86(entry_point) => {
                write(&mut buffer, b"8BIM", b"wx86", |buffer| {
					write_cstring(buffer, entry_point)
				})?;
			},
            Property::CodeWin64X86(entry_point) => {
                write(&mut buffer, b"8BIM", b"8664", |buffer| {
					write_cstring(buffer, entry_point)
				})?;
			},
            Property::SupportedModes(flags) => {
                write(&mut buffer, b"8BIM", b"mode", |buffer| {
					buffer.write_u32::<LittleEndian>(flags.bits())
				})?;
			},
            Property::EnableInfo(condition) => {
                write(&mut buffer, b"8BIM", b"enbl", |buffer| {
					write_cstring(buffer, condition)
				})?;
			},
            //-------------------------------------------------------------------
            // Photoshop Filter PiPL properties
            //-------------------------------------------------------------------
            Property::FilterCaseInfo(infos) => {
                write(&mut buffer, b"8BIM", b"fici", |buffer| {
                    for i in 0..7 {
                        if let Some(info) = infos.get(i) {
                            buffer.write_u8(info.in_handling as u8)?;
                            buffer.write_u8(info.out_handling as u8)?;
                            let flags = if info.copy_source_to_destination { 0 } else { 1 << 0 } |
                                           if info.works_with_blank_data      { 1 << 1 } else { 0 } |
                                           if info.filters_layer_masks        { 1 << 2 } else { 0 } |
                                           if info.write_outside_selection    { 1 << 3 } else { 0 };
                            buffer.write_u8(flags)?;
                            buffer.write_u8(0)?;
                        } else {
                            buffer.write_u32::<LittleEndian>(0)?;
                        }
                    }
                    Ok(())
				})?;
			},
            //-------------------------------------------------------------------
            // Photoshop Export PiPL properties
            //-------------------------------------------------------------------
            Property::ExportFlags { supports_transparency } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"expf", |buffer| {
                    buffer.write_u8(if supports_transparency { 1 << 7 } else { 0 })?;
                    buffer.write_u24::<LittleEndian>(0)
				})?;
			},
            Property::FmtFileType((type_, creator)) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fmTC", |buffer| {
                    buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_))?;
                    buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*creator))
				})?;
			},
            // NOTE: If you specify you can READ type 'foo_', then you will never be called with a FilterFile for type 'foo_'.
            Property::ReadTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"RdTy", |buffer| {
                    for type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.0))?;
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.1))?;
                    }
                    Ok(())
				})?;
			},
            Property::WriteTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"WrTy", |buffer| {
                    for type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.0))?;
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.1))?;
                    }
                    Ok(())
				})?;
			},
            // NOTE: If you specify you want to filter type 'foo_' AND you specify you can read type 'foo_', you will never get a filter call.
            Property::FilteredTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fftT", |buffer| {
                    for type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.0))?;
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.1))?;
                    }
                    Ok(())
				})?;
			},
            // Macintosh plug-ins can use Windows file extensions to determine read/write/parseability.
            // NOTE: If you specify you READ extension '.foo' then you won't be called to Filter that type.
            Property::ReadExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"RdEx", |buffer| {
                    for &ext in exts {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*ext))?;
                    }
                    Ok(())
				})?;
			},
            Property::WriteExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"WrEx", |buffer| {
                    for &ext in exts {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*ext))?;
                    }
                    Ok(())
				})?;
			},
            // NOTE: If you specify you want to filter extension '.foo' AND you specify you can read extension '.foo', you will never get a filter call.
            Property::FilteredExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fftE", |buffer| {
                    for &ext in exts {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*ext))?;
                    }
                    Ok(())
				})?;
			},
            Property::FormatFlags { can_read, can_write, can_write_if_read, saves_image_resources } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"fmtf", |buffer| {
                    let flags = if can_write_if_read     { 1 << 3 } else { 0 } |
                                   if can_write             { 1 << 4 } else { 0 } |
                                   if can_read              { 1 << 5 } else { 0 } |
                                   if saves_image_resources { 1 << 6 } else { 0 };
                    buffer.write_u8(flags)?;
                    buffer.write_u24::<LittleEndian>(0)
				})?;
			},
            Property::FormatMaxSize { width, height } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"mxsz", |buffer| {
                    buffer.write_u16::<LittleEndian>(width)?;
                    buffer.write_u16::<LittleEndian>(height)
				})?;
			},
            Property::FormatMaxChannels(max_channels) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"mxch", |buffer| {
                    for ch in max_channels {
                        buffer.write_u16::<LittleEndian>(*ch)?;
                    }
                    for _ in 0..padding_4(max_channels.len() as u32 * 2) as usize { buffer.write_u8(0)?; }
                    Ok(())
				})?;
			},
            //-------------------------------------------------------------------
            // Photoshop Parser PiPL properties
            //-------------------------------------------------------------------
            // NOTE: If you specify you want to filter type 'foo_' and you specify you can parse type 'foo_', you will never get a filter call.
            Property::ParsableTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psTY", |buffer| {
                    for type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.0))?;
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.1))?;
                    }
                    Ok(())
				})?;
			},
            Property::ParsableClipboardTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psCB", |buffer| {
                    for &type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_))?;
                    }
                    Ok(())
				})?;
			},
            // NOTE: If you want to filter type 'foo_' and you specify you can parse type 'foo_', you will never get a filter call.
            Property::FilteredParsableTypes(types) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psTy", |buffer| {
                    for type_ in types {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.0))?;
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*type_.1))?;
                    }
                    Ok(())
				})?;
			},
            // Macintosh plug-ins can use Windows file extensions to determine read/write/parseability.
            // NOTE: If you want to filter extension '.foo' and you specify you can parse extension '.foo', you will never get a filter call.
            Property::ParsableExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psEX", |buffer| {
                    for &ext in exts {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*ext))?;
                    }
                    Ok(())
				})?;
			},
            Property::FilteredParsableExtensions(exts) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"psEx", |buffer| {
                    for &ext in exts {
                        buffer.write_u32::<LittleEndian>(u32::from_be_bytes(*ext))?;
                    }
                    Ok(())
				})?;
			},
            Property::PickerID(id) => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"pnme", |buffer| {
					write_pstring(buffer, id)
				})?;
			},
            //-------------------------------------------------------------------
            // Photoshop Actions/Scripting PiPL properties (Photoshop 4.0 and later)
            //-------------------------------------------------------------------
            Property::HasTerminology { class_id, event_id, dictionary_resource_id, unique_scope_string } => {
                // TODO: tests
                write(&mut buffer, b"8BIM", b"hstm", |buffer| {
                    buffer.write_u32::<LittleEndian>(0)?; // Version.
                    buffer.write_u32::<LittleEndian>(class_id)?; // Class ID, always required.  Can be Suite ID.
                    buffer.write_u32::<LittleEndian>(event_id)?; // Event ID, or typeNULL if not Filter/Color Picker/Selection.
                    buffer.write_u16::<LittleEndian>(dictionary_resource_id)?; // Dictionary ('AETE') resource ID.
					write_cstring(buffer, unique_scope_string)
                    // TODO: Padding?
				})?;
			},
            // If this property is present, then its on. No parameters are required:
            Property::Persistent => {
                write(&mut buffer, b"8BIM", b"prst", |buffer| {
                    buffer.write_u32::<LittleEndian>(1)
				})?;
			},
            //-------------------------------------------------------------------
            // After Effects and Premiere specific PiPL properties
            //-------------------------------------------------------------------
            Property::AE_PiPL_Version { major, minor } => {
				write(&mut buffer, b"8BIM", b"ePVR", |buffer| {
					buffer.write_u16::<LittleEndian>(major)?;
					buffer.write_u16::<LittleEndian>(minor)
				})?;
			},
            Property::AE_Effect_Spec_Version { major, minor } => {
				write(&mut buffer, b"8BIM", b"eSVR", |buffer| {
					buffer.write_u16::<LittleEndian>(major)?;
					buffer.write_u16::<LittleEndian>(minor)
				})?;
			},
            Property::AE_Effect_Version((a, b, c, d, e)) => {
				write(&mut buffer, b"8BIM", b"eVER", |buffer| {
					buffer.write_u32::<LittleEndian>(pf_version(a, b, c, d, e))
				})?;
			},
            Property::AE_Effect_Match_Name(name) => {
				write(&mut buffer, b"8BIM", b"eMNA", |buffer| {
					write_pstring(buffer, name)
				})?;
			},
            Property::AE_Effect_Support_URL(name) => {
				write(&mut buffer, b"8BIM", b"eURL", |buffer| {
					write_pstring(buffer, name)
				})?;
			},
            Property::AE_Effect_Info_Flags(x) => {
				write(&mut buffer, b"8BIM", b"eINF", |buffer| {
					buffer.write_u32::<LittleEndian>(x)
				})?;
			},
            Property::AE_Effect_Global_OutFlags(x) => {
				write(&mut buffer, b"8BIM", b"eGLO", |buffer| {
					buffer.write_u32::<LittleEndian>(x.bits())
				})?;
			},
            Property::AE_Effect_Global_OutFlags_2(x) => {
				write(&mut buffer, b"8BIM", b"eGL2", |buffer| {
					buffer.write_u32::<LittleEndian>(x.bits())
				})?;
			},
            Property::AE_Reserved(x) => {
				write(&mut buffer, b"8BIM", b"aeRD", |buffer| {
					buffer.write_u32::<LittleEndian>(x)
				})?;
			},
            Property::AE_Reserved_Info(x) => {
				write(&mut buffer, b"8BIM", b"aeFL", |buffer| {
					buffer.write_u32::<LittleEndian>(x)
				})?;
			},
            //-------------------------------------------------------------------
            // After Effects Image Format Extension PiPL properties
            //-------------------------------------------------------------------
            Property::AE_ImageFormat_Extension_Info { major_version, minor_version, has_options, sequential_only, must_interact, has_interact_put, has_interact_get, has_time, has_video, still, has_file, output, input, signature } => {
				write(&mut buffer, b"8BIM", b"FXMF", |buffer| {
					buffer.write_u16::<LittleEndian>(major_version)?;
					buffer.write_u16::<LittleEndian>(minor_version)?;
                    let flags: u32 = if input            { 1u32 << 0 } else { 0 } |
                                     if output           { 1u32 << 1 } else { 0 } |
                                     if has_file         { 1u32 << 2 } else { 0 } |
                                     if still            { 1u32 << 3 } else { 0 } |
                                     if has_video        { 1u32 << 4 } else { 0 } |
                                     if !has_time        { 1u32 << 5 } else { 0 } |
                                     if has_interact_get { 1u32 << 6 } else { 0 } |
                                     if has_interact_put { 1u32 << 7 } else { 0 } |
                                     if must_interact    { 1u32 << 8 } else { 0 } |
                                     if !sequential_only { 1u32 << 9 } else { 0 } |
                                     if !has_options     { 1u32 << 10 } else { 0 };
                    buffer.write_u32::<LittleEndian>(flags)?;
                    buffer.write_u32::<LittleEndian>(0)?; // Reserved.
                    buffer.write_u32::<LittleEndian>(u32::from_be_bytes(signature))
				})?;
			},
            //-------------------------------------------------------------------
            // After Effects and Premiere ANIM PiPL properties
            //-------------------------------------------------------------------
            Property::ANIM_FilterInfo { spec_version_major, spec_version_minor, filter_params_version, unity_pixel_aspec_tratio, any_pixel_aspect_ratio, drive_me, needs_dialog, params_pointer, params_handle, params_mac_handle, dialog_in_render, params_in_globals, bg_animatable, fg_animatable, geometric, randomness, number_of_parameters, match_name } => {
				write(&mut buffer, b"8BIM", b"aFLT", |buffer| {
                    buffer.write_u32::<LittleEndian>(spec_version_major)?;
                    buffer.write_u32::<LittleEndian>(spec_version_minor)?;
                    buffer.write_u32::<LittleEndian>(filter_params_version)?;
                    let flags: u32 = if randomness               { 1u32 << 0 } else { 0 } |  // ANIM_FF_HAS_RANDOMNESS (AE only)
                                     if !geometric               { 1u32 << 1 } else { 0 } |  // ANIM_FF_NON_GEOMETRIC (AE only)
                                     if fg_animatable            { 1u32 << 2 } else { 0 } |  // ANIM_FF_FG_ANIMATABLE (AE only)
                                     if bg_animatable            { 1u32 << 3 } else { 0 } |  // ANIM_FF_BG_ANIMATABLE (AE only)
                                     if params_in_globals        { 1u32 << 4 } else { 0 } |  // ANIM_FF_PARAMS_IN_GLOBALS (AE only)
                                     if dialog_in_render         { 1u32 << 5 } else { 0 } |  // ANIM_FF_DIALOG_IN_RENDER (AE only)
                                     if params_mac_handle        { 1u32 << 6 } else { 0 } |  // ANIM_FF_PARAMS_ARE_MAC_HANDLE (AE only)
                                     if params_handle            { 1u32 << 7 } else { 0 } |  // ANIM_FF_PARAMS_ARE_HANDLE (AE only)
                                     if params_pointer           { 1u32 << 8 } else { 0 } |  // ANIM_FF_PARAMS_ARE PTR (AE only)
                                     if !needs_dialog            { 1u32 << 9 } else { 0 } |  // ANIM_FF_DOESNT_NEED_DLOG (AE only)
                                     if !drive_me                { 1u32 << 10 } else { 0 } | // ANIM_FF_DONT_DRIVE_ME (AE only)
                                     if false                    { 1u32 << 11 } else { 0 } | // ANIM_FF_RESERVED0 (AE only)
                                     if false                    { 1u32 << 12 } else { 0 } | // ANIM_FF_RESERVED1 (AE only)
                                     if false                    { 1u32 << 13 } else { 0 } | // ANIM_FF_RESERVED2 (spare)
                                     if false                    { 1u32 << 14 } else { 0 } | // ANIM_FF_RESERVED3 (spare)
                                     if false                    { 1u32 << 15 } else { 0 } | // ANIM_FF_RESERVED4 (spare)
                                     if any_pixel_aspect_ratio   { 1u32 << 16 } else { 0 } | // ANIM_FF_ANY_PAR
                                     if unity_pixel_aspec_tratio { 1u32 << 17 } else { 0 };  // ANIM_FF_UNITY_PAR

                    buffer.write_u32::<LittleEndian>(flags)?;
                    buffer.write_u32::<LittleEndian>(number_of_parameters)?;

                    let match_name_buf = match_name.as_bytes();
                    assert!(match_name_buf.len() < 32);
                    buffer.extend(match_name_buf);
                    for _ in 0..(32 - match_name_buf.len()) { buffer.push(0); }

                    buffer.write_u32::<LittleEndian>(0)?; // Operates in place - not currently implemented
                    buffer.write_u32::<LittleEndian>(0)?; // reserved
                    buffer.write_u32::<LittleEndian>(0)?; // reserved
                    buffer.write_u32::<LittleEndian>(0)   // reserved
				})?;
			},
            Property::ANIM_ParamAtom { external_name, match_id, data_type, ui_type, valid_min, valid_max, ui_min, ui_max, scale_ui_range, animate_param, restrict_bounds, space_is_relative, res_dependant, property_size }  => {
				write(&mut buffer, b"8BIM", b"aPAR", |buffer| {
                    // TODO: MUST SPECIFY THE FIRST 0 u32 - buffer[4..8]

                    let external_name = external_name.as_bytes();
                    assert!(external_name.len() < 32);
                    buffer.extend(external_name);
                    for _ in 0..(32 - external_name.len()) { buffer.push(0); }
                    buffer.write_u32::<LittleEndian>(match_id)?;
                    buffer.write_u32::<LittleEndian>(data_type as u32)?; // obsolete, don't use OPAQUE with Premiere
                    buffer.write_u32::<LittleEndian>(ui_type as u32)?; // UI types are only used by AE
                    buffer.write_f64::<LittleEndian>(valid_min)?; // used for UI type slider - AE only
                    buffer.write_f64::<LittleEndian>(valid_max)?; // used for UI type slider - AE only
                    buffer.write_f64::<LittleEndian>(ui_min)?; // used for UI type slider - AE only
                    buffer.write_f64::<LittleEndian>(ui_max)?; // used for UI type slider - AE only

                    let flags: u32 = if res_dependant     { 1u32 << 0 } else { 0 } |
                                     if space_is_relative { 1u32 << 1 } else { 0 } |
                                     if restrict_bounds   { 1u32 << 2 } else { 0 } |
                                     if animate_param     { 1u32 << 3 } else { 0 } |
                                     if scale_ui_range    { 1u32 << 4 } else { 0 };
                    buffer.write_u32::<LittleEndian>(flags)?;

                    buffer.write_u32::<LittleEndian>(property_size)?; // size of property described in bytes (short = 2, long = 4, etc.)

                    buffer.write_u32::<LittleEndian>(0)?; // reserved0
                    buffer.write_u32::<LittleEndian>(0)?; // reserved1
                    buffer.write_u32::<LittleEndian>(0)?; // reserved2
                    buffer.write_u32::<LittleEndian>(0)   // reserved3
				})?;
			},
            //-------------------------------------------------------------------
            // Premiere Transition Effect PiPL properties
            //-------------------------------------------------------------------
            Property::Pr_Effect_Info { version, valid_corners_mask, initial_corners, exclusive_dialog, needs_callbacks_at_setup, direct_comp_data, want_initial_setup_call, treat_as_transition, has_custom_dialog, highlight_opposite_corners, exclusive, reversible, have_edges, have_start_point, have_end_point, more_flags }  => {
				write(&mut buffer, b"PrMr", b"pOPT", |buffer| {
                    buffer.write_u32::<LittleEndian>(version)?;

                    // Valid corners mask and initial corners (lsb to msb):
                    // bitTop | bitRight | bitBottom | bitLeft | bitUpperRight | bitLowerRight | bitLowerLeft | bitUpperLeft
                    buffer.write_u8(valid_corners_mask as u8)?;
                    buffer.write_u8(initial_corners as u8)?;
                    let flags: u8 = if highlight_opposite_corners { 1 << 0 } else { 0 } |
                                    if has_custom_dialog          { 1 << 1 } else { 0 } |
                                    if !treat_as_transition       { 1 << 2 } else { 0 } |
                                    if !want_initial_setup_call   { 1 << 3 } else { 0 } |
                                    if direct_comp_data           { 1 << 4 } else { 0 } |
                                    if needs_callbacks_at_setup   { 1 << 5 } else { 0 } |
                                    if exclusive_dialog           { 1 << 6 } else { 0 };
                    buffer.write_u8(flags)?;
                    buffer.write_u8(exclusive as u8)?;
                    buffer.write_u8(reversible as u8)?;
                    buffer.write_u8(have_edges as u8)?;
                    buffer.write_u8(have_start_point as u8)?;
                    buffer.write_u8(have_end_point as u8)?;

                    buffer.write_u32::<LittleEndian>(more_flags)
				})?;
			},
            // The text description of the transition.
            Property::Pr_Effect_Description(desc) => {
                write(&mut buffer, b"PrMr", b"TEXT", |buffer| {
                    write_pstring(buffer, desc)
                })?;
            },
            //-------------------------------------------------------------------
            // Illustrator/SweetPea PiPL properties
            //-------------------------------------------------------------------
            Property::InterfaceVersion(x) => {
                write(&mut buffer, b"ADBE", b"ivrs", |buffer| {
                    buffer.write_u32::<LittleEndian>(x)
                })?;
            },
            Property::AdapterVersion(x) => {
                write(&mut buffer, b"ADBE", b"adpt", |buffer| {
                    buffer.write_u32::<LittleEndian>(x)
                })?;
            },
            Property::SP_STSP(x) => {
                write(&mut buffer, b"ADBE", b"STSP", |buffer| {
                    buffer.write_u32::<LittleEndian>(x)
                })?;
            },
            Property::InternalName(name) => {
                write(&mut buffer, b"ADBE", b"pinm", |buffer| {
                    write_cstring(buffer, name)
                })?;
            },
            Property::Imports(imports) => {
                write(&mut buffer, b"ADBE", b"impt", |buffer| {
                    buffer.write_u32::<LittleEndian>(imports.len() as u32)?;
                    for import in imports {
                        let len = buffer.len();

                        buffer.write_u32::<LittleEndian>(0)?;
                        write_cstring(buffer, import.0)?;
                        buffer.write_u32::<LittleEndian>(import.1)?; // Suite version.

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len+4].clone_from_slice(&new_len.to_le_bytes());
                    }
                    Ok(())
                })?;
            },
            Property::Exports(exports) => {
                write(&mut buffer, b"ADBE", b"expt", |buffer| {
                    buffer.write_u32::<LittleEndian>(exports.len() as u32)?;
                    for export in exports {
                        let len = buffer.len();

                        buffer.write_u32::<LittleEndian>(0)?;
                        write_cstring(buffer, export.0)?;
                        buffer.write_u32::<LittleEndian>(export.1)?; // Suite version.

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len+4].clone_from_slice(&new_len.to_le_bytes());
                    }
                    Ok(())
                })?;
            },
            Property::Description(desc) => {
                write(&mut buffer, b"ADBE", b"desc", |buffer| {
                    write_cstring(buffer, desc)
                })?;
            },
            Property::Keywords(keywords) => {
                write(&mut buffer, b"ADBE", b"keyw", |buffer| {
                    buffer.write_u32::<LittleEndian>(keywords.len() as u32)?;
                    for keyword in keywords {
                        let len = buffer.len();

                        buffer.write_u32::<LittleEndian>(0)?;
                        write_cstring(buffer, keyword)?;

                        let new_len = (buffer.len() - len) as u32;
                        buffer[len..len+4].clone_from_slice(&new_len.to_le_bytes());
                    }
                    Ok(())
                })?;
            },
            Property::Title(title) => {
                write(&mut buffer, b"ADBE", b"titl", |buffer| {
                    write_cstring(buffer, title)
                })?;
            },
            Property::Messages { startup_required, purge_cache, shutdown_required, accept_property } => {
                write(&mut buffer, b"ADBE", b"AcpM", |buffer| {
                    let flags: u32 = if accept_property   { 1u32 << 0 } else { 0 } |
                                     if shutdown_required { 1u32 << 1 } else { 0 } | // Default is to give shutdown msg.
                                     if purge_cache       { 1u32 << 2 } else { 0 } |
                                     if startup_required  { 1u32 << 3 } else { 0 };
                    buffer.write_u32::<LittleEndian>(flags)
                })?;
            },
            //-------------------------------------------------------------------
            // PhotoDeluxe PiPL properties
            //-------------------------------------------------------------------
            Property::ButtonIcon { version, mac_icon_type, win_icon_type, resource_id, icon_name } => {
                write(&mut buffer, b"8BIM", b"btni", |buffer| {
                    buffer.write_u32::<LittleEndian>(version)?; // version
                    match mac_icon_type {
                        ButtonIconType::None    => buffer.write_u32::<LittleEndian>(0)?,
                        ButtonIconType::MacCICN => buffer.write_u32::<LittleEndian>(1)?,
                        _ => {}
                    }
                    match win_icon_type {
                        ButtonIconType::None        => buffer.write_u32::<LittleEndian>(0)?,
                        ButtonIconType::WindowsICON => buffer.write_u32::<LittleEndian>(1)?,
                        _ => {}
                    }
                    buffer.write_u32::<LittleEndian>(resource_id)?;
                    write_cstring(buffer, icon_name)
                })?;
            },
            //-------------------------------------------------------------------
            // PhotoDeluxe extension to Import plug-in PiPL properties
            //-------------------------------------------------------------------
            Property::Class { version, class } => {
                write(&mut buffer, b"8BIM", b"clas", |buffer| {
                    buffer.write_u32::<LittleEndian>(version)?; // version
                    buffer.write_u32::<LittleEndian>(class as u32)
                })?;
            },
            Property::PreviewFile { version, filename } => {
                write(&mut buffer, b"8BIM", b"prvw", |buffer| {
                    buffer.write_u32::<LittleEndian>(version)?; // version
                    write_cstring(buffer, filename)
                })?;
            }
        }
    }

	Ok(buffer)
}

pub fn plugin_build(properties: Vec<Property>) {
    for prop in properties.iter() {
        match prop {
            Property::Kind(x) => {
                println!("cargo:rustc-env=PIPL_KIND={}", x.as_u32());
            },
            Property::Name(x) => {
                println!("cargo:rustc-env=PIPL_NAME={x}");
            },
            Property::Category(x) => {
                println!("cargo:rustc-env=PIPL_CATEGORY={x}");
            },
            Property::AE_Effect_Match_Name(x) => {
                println!("cargo:rustc-env=PIPL_MATCH_NAME={x}");
            },
            Property::AE_Effect_Support_URL(x) => {
                println!("cargo:rustc-env=PIPL_SUPPORT_URL={x}");
            },
            Property::CodeWin64X86(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            },
            Property::CodeMacIntel64(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            },
            Property::CodeMacARM64(x) => {
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            },
            Property::AE_Effect_Spec_Version { major, minor } => {
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MAJOR={major}");
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MINOR={minor}");
            },
            Property::AE_Reserved_Info(x) => {
                println!("cargo:rustc-env=PIPL_AE_RESERVED={}", x);
            },
            Property::AE_Effect_Version((a, b, c, d, e)) => {
                println!("cargo:rustc-env=PIPL_VERSION={}", pf_version(*a, *b, *c, *d, *e));
            },
            Property::AE_Effect_Global_OutFlags(x) => {
                println!("cargo:rustc-env=PIPL_OUTFLAGS={}", x.bits());
            },
            Property::AE_Effect_Global_OutFlags_2(x) => {
                println!("cargo:rustc-env=PIPL_OUTFLAGS2={}", x.bits());
            },
            _ => { }
        }
    }
    let pipl = build_pipl(properties).unwrap();

	resource::produce_resource(&pipl, Some(&format!("{}/../../../{}.rsrc", std::env::var("OUT_DIR").unwrap(), std::env::var("CARGO_PKG_NAME").unwrap())));
}
