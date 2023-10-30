#![allow(non_camel_case_types)]
#![allow(dead_code)]

use byteorder::{ WriteBytesExt, LittleEndian, BigEndian };
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
#[derive(Debug)]
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
#[derive(Debug)]
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
    works_with_blan_data: bool,
    copy_source_to_destination: bool,
}

#[repr(u8)]
#[derive(Debug)]
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
#[derive(Debug)]
pub enum PixelAspectRatio {
	AnyPAR   = 0x10000,
	UnityPAR = 0x20000,
}

#[repr(u32)] #[derive(Debug)] pub enum AnimDataType { Opaque = 0, Char, Short, Long, UnsignedChar, UnsignedShort, UnsignedLong, Fixed, UnsignedFixed, Extended96, Double64, Float32, ColorRGB }
#[repr(u32)] #[derive(Debug)] pub enum AnimUIType { NoUI = 0, Angle, Slider, Point, Rect, ColorRGB, ColorCMYK, ColorLAB }
#[repr(u32)] #[derive(Debug)] pub enum ClassType { None = 0, Scanner, Camera, Video, Floppy, Cdrom, Internet }
#[derive(Debug)] pub enum ButtonIconType { Mac, Windows }

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
    RequiredHost([u8; 4]),
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
    FmtFileType((u32, u32)),
    ReadTypes(&'static [(u32, u32)]),
    WriteTypes(&'static [(u32, u32)]),
    FilteredTypes(&'static [(u32, u32)]),
    ReadExtensions(&'static [[u8; 4]]),
    WriteExtensions(&'static [[u8; 4]]),
    FilteredExtensions(&'static [[u8; 4]]),
    FormatFlags {
        saves_image_resources: bool,
        can_read: bool,
        can_write: bool,
        can_write_if_read: bool,
    },
    FormatMaxSize { width: u16, height: u16 },
    FormatMaxChannels(&'static [u16]),
    ParsableTypes(&'static [([u8; 4], [u8; 4])]),
    ParsableClipboardTypes(&'static [[u8; 4]]),
    FilteredParsableTypes(&'static [([u8; 4], [u8; 4])]),
    ParsableExtensions(&'static [[u8; 4]]),
    FilteredParsableExtensions(&'static [[u8; 4]]),
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
        signature: [char; 4]
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
        icon_type: ButtonIconType,
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
					buffer.write_u32::<LittleEndian>(u32::from_be_bytes(x))
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
            /*
            // Photoshop Filter PiPL properties
            //-------------------------------------------------------------------
            case FilterCaseInfo:
                longint = '8BIM';
                key longint = 'fici';
                longint = 0;
                longint = 28;
                array [7]
                    {
                    byte inCantFilter = 0,
                         inStraightData = 1,
                         inBlackMat = 2,
                         inGrayMat = 3,
                         inWhiteMat = 4,
                         inDefringe = 5,
                         inBlackZap = 6,
                         inGrayZap = 7,
                         inWhiteZap = 8,
                         inBackgroundZap = 10,
                         inForegroundZap = 11;
                    byte outCantFilter = 0,
                         outStraightData = 1,
                         outBlackMat = 2,
                         outGrayMat = 3,
                         outWhiteMat = 4,
                         outFillMask = 9;
                    fill bit [4];
                    boolean doNotWriteOutsideSelection, writeOutsideSelection;
                    boolean doesNotFilterLayerMasks, filtersLayerMasks;
                    boolean doesNotWorkWithBlankData, worksWithBlankData;
                    boolean copySourceToDestination, doNotCopySourceToDestination;
                    fill byte;
                    };

            //-------------------------------------------------------------------
            // Photoshop Export PiPL properties
            //-------------------------------------------------------------------
            case ExportFlags:
                longint = '8BIM';
                key longint = 'expf';
                longint = 0;
                #if DeRez
                    fill long;
                #else
                    longint = (expFlagsEnd[$$ArrayIndex(properties)] - expFlagsStart[$$ArrayIndex(properties)]) / 8;
                #endif
             expFlagsStart:
                boolean expDoesNotSupportTransparency, expSupportsTransparency;
                fill bit[7];
              expFlagsEnd:
                align long;

            //-------------------------------------------------------------------
            // Photoshop File Format PiPL properties
            //-------------------------------------------------------------------
            case FmtFileType:
                longint = '8BIM';
                key longint = 'fmTC';
                longint = 0;
                longint = 8;
                literal longint; // Default file type.
                literal longint; // Default file creator.

            // NOTE: If you specify you can READ type 'foo_', then you
            // will never be called with a FilterFile for type 'foo_'.
            case ReadTypes:
                longint = '8BIM';
                key longint = 'RdTy';
                longint = 0;
                longint = $$CountOf(ReadableTypes) * 8;
                wide array ReadableTypes { literal longint; literal longint; } ;

            case WriteTypes:
                longint = '8BIM';
                key longint = 'WrTy';
                longint = 0;
                longint = $$CountOf(WritableTypes) * 8;
                wide array WritableTypes { literal longint; literal longint; } ;

            // NOTE: If you specify you want to filter type 'foo_' AND you
            // specify you can read type 'foo_', you will never get
            // a filter call.
            case FilteredTypes:
                longint = '8BIM';
                key longint = 'fftT';
                longint = 0;
                longint = $$CountOf(FilteredTypes) * 8;
                wide array FilteredTypes { literal longint; literal longint; } ;

            // Macintosh plug-ins can use Windows file extensions
            // to determine read/write/parseability.
            //
            // NOTE: If you specify you READ extension '.foo' then you
            // won't be called to Filter that type.
            case ReadExtensions:
                longint = '8BIM';
                key longint = 'RdEx';
                longint = 0;
                longint = $$CountOf(ReadableExtensions) * 4;
                wide array ReadableExtensions { literal longint; } ;

            case WriteExtensions:
                longint = '8BIM';
                key longint = 'WrEx';
                longint = 0;
                longint = $$CountOf(WriteableExtensions) * 4;
                wide array WriteableExtensions { literal longint; } ;

            // NOTE: If you specify you want to filter extension '.foo'
            // AND you specify you can read extension '.foo', you will
            // never get a filter call.
            case FilteredExtensions:
                longint = '8BIM';
                key longint = 'fftE';
                longint = 0;
                longint = $$CountOf(FilteredExtensions) * 4;
                wide array FilteredExtensions { literal longint; } ;

            case FormatFlags:
                longint = '8BIM';
                key longint = 'fmtf';
                longint = 0;
                longint = (fmtFlagsEnd[$$ArrayIndex(properties)] - fmtFlagsStart[$$ArrayIndex(properties)]) / 8;
             fmtFlagsStart:
                boolean = false; // Obsolete.
                boolean fmtDoesNotSaveImageResources, fmtSavesImageResources;
                boolean fmtCannotRead, fmtCanRead;
                boolean fmtCannotWrite, fmtCanWrite;
                boolean fmtWritesAll, fmtCanWriteIfRead;
                fill bit[3];
              fmtFlagsEnd:
                align long;

            case FormatMaxSize:
                longint = '8BIM';
                key longint = 'mxsz';
                longint = 0;
                longint = 4;
                Point;

            case FormatMaxChannels:
                longint = '8BIM';
                key longint = 'mxch';
                longint = 0;
                longint = $$CountOf(ChannelsSupported) * 2;
                wide array ChannelsSupported { integer; } ;
                align long;

            //-------------------------------------------------------------------
            // Photoshop Parser PiPL properties
            //-------------------------------------------------------------------
            // NOTE: If you specify you want to filter type 'foo_' and you
            // specify you can parse type 'foo_', you will never get a
            // filter call.
            case ParsableTypes:
                longint = '8BIM';
                key longint = 'psTY';
                longint = 0;
                longint = $$CountOf(ParsableTypes) * 8;
                wide array ParsableTypes { literal longint; literal longint; } ;

            case ParsableClipboardTypes:
                longint = '8BIM';
                key longint = 'psCB';
                longint = 0;
                longint = $$CountOf(ParsableClipboardTypes) * 4;
                wide array ParsableClipboardTypes { literal longint; };

            // NOTE: If you want to filter type 'foo_' and you specify you
            // can parse type 'foo_', you will never get a filter call.
            case FilteredParsableTypes:
                longint = '8BIM';
                key longint = 'psTy';
                longint = 0;
                longint = $$CountOf(ParsableTypes) * 8;
                wide array ParsableTypes { literal longint; literal longint; } ;


            // Macintosh plug-ins can use Windows file extensions
            // to determine read/write/parseability.
            //
            // NOTE: If you want to filter extension '.foo' and you
            // specify you can parse extension '.foo', you will
            // never get a filter call.
            case ParsableExtensions:
                longint = '8BIM';
                key longint = 'psEX';
                longint = 0;
                longint = $$CountOf(ParsableExtensions) * 4;
                wide array ParsableExtensions { literal longint; };

            case FilteredParsableExtensions:
                longint = '8BIM';
                key longint = 'psEx';
                longint = 0;
                longint = $$CountOf(ParsableExtensions) * 4;
                wide array ParsableExtensions { literal longint; };

            //-------------------------------------------------------------------
            // Photoshop Parser PiPL properties
            //-------------------------------------------------------------------
            case PickerID:
                longint = '8BIM';
                key longint = 'pnme';
                longint = 0;
                #if DeRez
                    fill long;
                #else
                    longint = (PickerIDEnd[$$ArrayIndex(properties)] - PickerIDStart[$$ArrayIndex(properties)]) / 8;
                #endif
              PickerIDStart:
                pstring;            // Unique ID string.
              PickerIDEnd:
                align long;

            //-------------------------------------------------------------------
            // Photoshop Actions/Scripting PiPL properties
            // (Photoshop 4.0 and later)
            //-------------------------------------------------------------------
            case HasTerminology:
                longint = '8BIM';
                key longint = 'hstm';
                longint = 0;
                longint = (hasTermEnd[$$ArrayIndex(properties)] - hasTermStart[$$ArrayIndex(properties)]) / 8;
            hasTermStart:
                longint = 0;    // Version.
                longint;        // Class ID, always required.  Can be Suite ID.
                longint;        // Event ID, or typeNULL if not Filter/Color Picker/Selection.
                integer;        // Dictionary ('AETE') resource ID.
                cstring;        // Unique scope string.  Always required in Photoshop 5.0 and later.
            hasTermEnd:
                align long;

            // If this property is present, then its on.  No parameters
            // are required:
            case Persistent:
                longint = '8BIM';
                key longint = 'prst';
                longint = 0;    // Index.
                longint = 4;     // Length.
                literal longint = 1;    // If specified, always on.
*/
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
            /*
            // After Effects Image Format Extension PiPL properties
            //-------------------------------------------------------------------
            case AE_ImageFormat_Extension_Info:
                longint = '8BIM';
                key longint = 'FXMF';
                longint = 0;
                longint = 16;
                integer;        // Major version.
                integer;        // Minor version.
                fill bit[21];
                boolean hasOptions, hasNoOptions;
                boolean sequentialOnly, nonSequentialOk;
                boolean noInteractRequired, mustInteract;
                boolean noInteractPut, hasInteractPut;
                boolean noInteractGet, hasInteractGet;
                boolean hasTime, hasNoTime;
                boolean noVideo, hasVideo;
                boolean noStill, still;
                boolean noFile, hasFile;
                boolean noOutput, output;
                boolean noInput, input;

                longint = 0;        // Reserved.
                literal longint;    // Signature.

            //-------------------------------------------------------------------
            // After Effects and Premiere ANIM PiPL properties
            //-------------------------------------------------------------------
            case ANIM_FilterInfo:
                longint = '8BIM';
                key longint = 'aFLT';
                longint = 0;    // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (animFilterEnd[$$ArrayIndex(properties)] - animFilterStart[$$ArrayIndex(properties)]) / 8;
                #endif

              animFilterStart:
                  longint=1;        // spec_version_major (AE & PrMr)
                  longint=1;        // spec_version_minor (AE & PrMr)
                  longint;        // filter_params_version (AE only)

#ifdef PiPLVer2p3
                fill bit[14];
                boolean notUnityPixelAspectRatio, unityPixelAspectRatio; // ANIM_FF_UNITY_PAR
                boolean notAnyPixelAspectRatio, anyPixelAspectRatio; // ANIM_FF_ANY_PAR
                boolean reserved4False, reserved4True;         // ANIM_FF_RESERVED4 (spare)
                boolean reserved3False, reserved3True;         // ANIM_FF_RESERVED3 (spare)
                boolean reserved2False, reserved2True;         // ANIM_FF_RESERVED2 (spare)
#else
                fill bit[19];
#endif
                boolean reserved1False, reserved1True;         // ANIM_FF_RESERVED1 (AE only)
                boolean reserved0False, reserved0True;         // ANIM_FF_RESERVED0 (AE only)
                boolean driveMe, dontDriveMe;                 // ANIM_FF_DONT_DRIVE_ME (AE only)
                boolean needsDialog, doesntNeedDialog;        // ANIM_FF_DOESNT_NEED_DLOG (AE only)
                boolean paramsNotPointer, paramsPointer;    // ANIM_FF_PARAMS_ARE PTR (AE only)
                boolean paramsNotHandle, paramsHandle;        // ANIM_FF_PARAMS_ARE_HANDLE (AE only)
                boolean paramsNotMacHandle,paramsMacHandle;    // ANIM_FF_PARAMS_ARE_MAC_HANDLE (AE only)
                boolean dialogNotInRender, dialogInRender;    // ANIM_FF_DIALOG_IN_RENDER (AE only)
                boolean paramsNotInGlobals,paramsInGlobals;    // ANIM_FF_PARAMS_IN_GLOBALS (AE only)
                boolean bgNotAnimatable, bgAnimatable;        // ANIM_FF_BG_ANIMATABLE (AE only)
                boolean fgNotAnimatable, fgAnimatable;        // ANIM_FF_FG_ANIMATABLE (AE only)
                boolean geometric, notGeometric;            // ANIM_FF_NON_GEOMETRIC (AE only)
                boolean noRandomness, randomness;            // ANIM_FF_HAS_RANDOMNESS (AE only)

                longint;        // number of parameters

                cstring[32];    // match name

                  longint=0;        // Operates in place - not currently implemented
                  longint=0;        // reserved
                  longint=0;        // reserved
                  longint=0;        // reserved
              animFilterEnd:

            case ANIM_ParamAtom:
                longint = '8BIM';
                key longint = 'aPAR';
                longint;        // property id *NOTE: Breaks model -- MUST SPECIFY.
                #if DeRez
                    fill long;
                #else
                    longint = (animParamEnd[$$ArrayIndex(properties)] - animParamStart[$$ArrayIndex(properties)]) / 8;
                #endif

              animParamStart:
                cstring[32];                    // external name

                  longint;                        // match id

                  longint ANIM_DT_OPAQUE,         // obsolete, don't use OPAQUE with Premiere
                          ANIM_DT_CHAR,
                        ANIM_DT_SHORT,
                        ANIM_DT_LONG,
                        ANIM_DT_UNSIGNED_CHAR,
                        ANIM_DT_UNSIGNED_SHORT,
                        ANIM_DT_UNSIGNED_LONG,
                        ANIM_DT_FIXED,
                        ANIM_DT_UNSIGNED_FIXED,
                        ANIM_DT_EXTENDED_96,
                        ANIM_DT_DOUBLE_64,
                        ANIM_DT_FLOAT_32,
                        ANIM_DT_COLOR_RGB;

                  longint ANIM_UI_NO_UI,            // UI types are only used by AE
                          ANIM_UI_ANGLE,
                        ANIM_UI_SLIDER,
                        ANIM_UI_POINT,
                        ANIM_UI_RECT,
                        ANIM_UI_COLOR_RGB,
                        ANIM_UI_COLOR_CMYK,
                        ANIM_UI_COLOR_LAB;

                // These next four sets of longints are IEEE 64-bit doubles.  To store
                // them correctly, you must specify them as hexidecimal numbers.  To
                // find the correct hexidecimal number, you must convert your decimal
                // number to a double.
                  hex longint;        // low long, valid_min (used for UI type slider - AE only)
                  hex longint;        // high long, valid_min (64-bit double)

                  hex longint;        // low long, valid_max (used for UI type slider - AE only)
                  hex longint;        // high long, valid_max (64-bit double)

                  hex longint;        // low long, ui_min (used for UI type slider - AE only)
                  hex longint;        // high long, ui_min (64-bit double)

                  hex longint;        // low long, ui_max (used for UI type slider - AE only)
                  hex longint;        // high long, ui_max (64-bit double)

#ifdef PiPLVer2p3
                fill bit[27];        // ANIM_ParamFlags
                boolean dontScaleUIRange, scaleUIRange;        // ANIM_PF_SCALE_UI_RANGE (Premiere 6.0)
#else
                  fill bit[28];        // ANIM_ParamFlags
#endif
                  boolean dontAnimateParam, animateParam;        // ANIM_PR_DONT_ANIMATE (PrMr)
                  boolean dontRestrictBounds, restrictBounds;    // ANIM_PF_RESTRICT_BOUNDS (AE only)
                  boolean    spaceIsAbsolute, spaceIsRelative;    // ANIM_PF_SPACE_IS_RELATIVE (AE only)
                  boolean resIndependent, resDependant;        // ANIM_PF_IS_RES_DEPENDENT (AE only)

                  longint;            // size of property described in bytes (short = 2, long = 4, etc.)

                  longint=0;            // reserved0
                  longint=0;            // reserved1
                  longint=0;            // reserved2
                  longint=0;            // reserved3
              animParamEnd:

            //-------------------------------------------------------------------
            // Premiere Transition Effect PiPL properties
            //-------------------------------------------------------------------
            case Pr_Effect_Info:        // Mirrors the old Premiere 'Fopt' resource
                longint = 'PrMr';        // Premiere host.
                key longint = 'pOPT';
                longint = 0;            // Index.
                longint = 16;            // Length.
#ifdef PiPLVer2p2
                longint;                // Version of this property
#else
                longint = 0;
#endif

                // Valid corners mask and initial corners (lsb to msb):
                // bitTop | bitRight | bitBottom | bitLeft | bitUpperRight |
                // bitLowerRight | bitLowerLeft | bitUpperLeft
                byte;                    // Valid corners mask.
                byte;                    // Initial corners.
#ifdef PiPLVer2p2
                boolean;                                        // Premiere 5.1
                boolean noExclusiveDialog, exclusiveDialog;        // Premiere 5.1
                boolean doesNotNeedCallbacksAtSetup, needsCallbacksAtSetup;
                boolean noDirectCompData, directCompData;        // Premiere 5.1
#else
                fill bit[2];
                boolean doesNotNeedCallbacksAtSetup, needsCallbacksAtSetup;
                boolean;
#endif
                boolean wantInitialSetupCall, dontWantInitialSetupCall;
                boolean treatAsTransition, treatAsTwoInputFilter;
                boolean noCustomDialog, hasCustomDialog;
                boolean dontHighlightOppositeCorners, highlightOppositeCorners;

                // These should be changed to booleans:
                byte notExclusive = 0, exclusive = 1;
                byte notReversible = 0, reversible = 1;
                byte doesNotHaveEdges = 0, haveEdges = 1;
                byte doesNotHaveStartPoint = 0, haveStartPoint = 1;
                byte doesNotHaveEndPoint = 0, haveEndPoint = 1;

#ifdef PiPLVer2p3
                longint;                // more flags - Premiere 6.0
#else
                longint = 0;            // Reserved.
#endif

            case Pr_Effect_Description:    // The text description of the transition.
                longint = 'PrMr';        // Premiere host.
                key longint = 'TEXT';    // This should be changed to 'pDES'.
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (descEnd[$$ArrayIndex(properties)] - descStart[$$ArrayIndex(properties)]) / 8;
                #endif
              descStart:
                pstring;
              descEnd:
                align long;

            //-------------------------------------------------------------------
            // Illustrator/SweetPea PiPL properties
            //-------------------------------------------------------------------
            case InterfaceVersion:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'ivrs';
                longint = 0;            // Index.
                longint = 4;            // Length.
                longint;                // Version.

            case AdapterVersion:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'adpt';
                longint = 0;            // Index.
                longint = 4;            // Length.
                longint;                // Version.

            case SP_STSP:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'STSP';
                longint = 0;            // Index.
                longint = 4;            // Length.
                longint;

            case InternalName:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'pinm';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (plugInNameEnd[$$ArrayIndex(properties)] -
                               plugInNameStart[$$ArrayIndex(properties)]) / 8;
                #endif
                plugInNameStart:
                    cstring;
                plugInNameEnd:
                    align long;

            case Imports:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'impt';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (importsEnd[$$ArrayIndex(properties)] -
                               importsStart[$$ArrayIndex(properties)]) / 8;
                #endif
                importsStart:
                    longint = $$CountOf(ImportSuites);
                    wide array ImportSuites
                    {
                        isuitesStart:
                            // Length (including this long):
                            #if DeRez
                                fill long;
                            #else
                                longint = ((isuitesEnd[$$ArrayIndex(properties), $$ArrayIndex(ImportSuites)] -
                                            isuitesStart[$$ArrayIndex(properties), $$ArrayIndex(ImportSuites)]) / 8);
                            #endif

                            cstring;
                            align long;
                            longint;            // Suite version.
                        isuitesEnd:
                    };
                importsEnd:

            case Exports:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'expt';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (exportsEnd[$$ArrayIndex(properties)] -
                               exportsStart[$$ArrayIndex(properties)]) / 8;
                #endif
                exportsStart:
                    longint = $$CountOf(ExportSuites);
                    wide array ExportSuites
                    {
                        esuitesStart:
                            // Length (including this long):
                            #if DeRez
                                fill long;
                            #else
                                longint = ((esuitesEnd[$$ArrayIndex(properties), $$ArrayIndex(ExportSuites)] -
                                            esuitesStart[$$ArrayIndex(properties), $$ArrayIndex(ExportSuites)]) / 8);
                            #endif

                            cstring;
                            align long;
                            longint;            // Suite version.
                        esuitesEnd:
                    };
                exportsEnd:

            case Description:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'desc';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (descriptionEnd[$$ArrayIndex(properties)] -
                               descriptionStart[$$ArrayIndex(properties)]) / 8;
                #endif
                descriptionStart:
                    cstring;
                descriptionEnd:
                    align long;

            case Keywords:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'keyw';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (keywordsEnd[$$ArrayIndex(properties)] -
                               keywordsStart[$$ArrayIndex(properties)]) / 8;
                #endif
                keywordsStart:
                    longint = $$CountOf(KeywordsArray);
                    wide array KeywordsArray
                    {
                        keywordsArrayStart:
                            // Length (including this long):
                            #if DeRez
                                fill long;
                            #else
                                longint = ((keywordsArrayEnd[$$ArrayIndex(properties), $$ArrayIndex(KeywordsArray)] -
                                            keywordsArrayStart[$$ArrayIndex(properties), $$ArrayIndex(KeywordsArray)]) / 8);
                            #endif

                            cstring;
                        keywordsArrayEnd:
                    };
                keywordsEnd:
                    align long;

            case Title:
                longint = 'ADBE';        // SweetPea/Illustrator host.
                key longint = 'titl';
                longint = 0;            // Index.
                #if DeRez
                    fill long;
                #else
                    longint = (titleEnd[$$ArrayIndex(properties)] -
                               titleStart[$$ArrayIndex(properties)]) / 8;
                #endif
                titleStart:
                    cstring;
                titleEnd:
                    align long;

            case Messages:
                longint = 'ADBE';        // SweetPea/Illustrator host
                key longint = 'AcpM';
                longint = 0;            // Index.
                longint = 4;            // Length.
                fill bit[28];            // Reserved.

                boolean startupRequired, noStartupRequired;
                boolean doesNotPurgeCache, purgeCache;
                boolean shutdownRequired, noShutdownRequired;    // Default is to give shutdown msg.
                boolean doNotAcceptProperty, acceptProperty;

            //-------------------------------------------------------------------
            // PhotoDeluxe PiPL properties
            //-------------------------------------------------------------------
            case ButtonIcon:
                longint = '8BIM';
                key longint = 'btni';
                longint = 0;        // pad
                #if DeRez
                    fill long;
                #else
                    longint = (buttonIconEnd[$$ArrayIndex(properties)] - buttonIconStart[$$ArrayIndex(properties)]) / 8; // length
                #endif
            buttonIconStart:
                longint = 0;        // version
                longint none = 0,
                         cicn = 1;    // Macintosh icon type
                longint none = 0,
                         ICON = 1;    // Windows icon type
                longint;            // Icon resource ID
                cstring;            // Button icon name
            buttonIconEnd:
                align long;

            //-------------------------------------------------------------------
            // PhotoDeluxe extension to Import plug-in PiPL properties
            //-------------------------------------------------------------------
            case Class:
                longint = '8BIM';
                key longint = 'clas';
                longint = 0;    // pad
                longint = 8;    // length
                longint = 0;    // version
                longint none = 0,
                        scanner = 1,
                        camera = 2,
                        video = 3,
                        floppy = 4,
                        cdrom = 5,
                        internet = 6;

            case PreviewFile:
                longint = '8BIM';
                key longint = 'prvw';
                longint = 0;    // pad
                #if DeRez
                    fill long;
                #else
                    longint = (previewFileEnd[$$ArrayIndex(properties)] - previewFileStart[$$ArrayIndex(properties)]) / 8; // length
                #endif
            previewFileStart:
                longint = 0;    // version
                cstring;        // preview filename
            previewFileEnd:
                align long;
            };
*/
            _ => panic!("Property not implemented: {prop:?}")
        }
    }

	Ok(buffer)
}

pub fn plugin_build(properties: Vec<Property>) {
    for prop in properties.iter() {
        match prop {
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

	produce_resource(&pipl, Some(&format!("{}/../../../{}.rsrc", std::env::var("OUT_DIR").unwrap(), std::env::var("CARGO_PKG_NAME").unwrap())));
}

pub fn produce_resource(pipl: &[u8], _macos_rsrc_path: Option<&str>) {
    #[cfg(target_os = "windows")]
    {
        fn to_seq(bytes: &[u8]) -> String {
            bytes.iter().fold(String::new(), |mut s, b| { s.push_str(&format!("\\x{b:02x}")); s })
        }

        let mut res = winres::WindowsResource::new();
        res.append_rc_content(&format!("16000 PiPL DISCARDABLE BEGIN \"{}\" END", to_seq(&pipl)));
        res.compile().unwrap();
    }
    #[cfg(target_os = "macos")]
    if let Some(rsrc_path) = _macos_rsrc_path {
        let rsrc_content = create_rsrc(&[
            (b"PiPL", &[
                (16001, pipl)
            ])
        ]).unwrap();
        std::fs::write(rsrc_path, rsrc_content).unwrap();
    }
}

// Reference: https://github.com/dgelessus/python-rsrcfork/blob/master/src/rsrcfork/api.py#L14
// Reference: https://github.com/andrews05/ResForge/blob/master/ResForge/Formats/ClassicFormat.swift#L114
pub type RSRCResource<'a> = (i16, &'a [u8]); // id, data

pub fn create_rsrc(resources: &[(&[u8; 4], &[RSRCResource])]) -> Result<Vec<u8>> {
    const DATA_OFFSET         : u32 = 256;
    const DATA_SIZE_MASK      : u32 = (1 << 24) - 1;
    const MAP_HEADER_LENGTH   : u32 = 24;
    const TYPE_INFO_LENGTH    : u32 = 8;
    const RESOURCE_INFO_LENGTH: u32 = 12;

    let mut buffer = Vec::new();

    let num_types = resources.len() as u32;
    let num_resources: u32 = resources.iter().map(|x| x.1.len() as u32).sum();
    let type_list_offset = MAP_HEADER_LENGTH + 4;
    let name_list_offset: u32 = type_list_offset + 2 + (num_types * TYPE_INFO_LENGTH) + (num_resources * RESOURCE_INFO_LENGTH);

    for _ in 0..DATA_OFFSET { buffer.push(0); } // Fill header with 0 for now

    // Write resource data
    let mut resource_offsets = Vec::new();
    for (_, resources) in resources {
        for resource in resources.iter() {
            let offset = buffer.len() as u32 - DATA_OFFSET;
            if offset > DATA_SIZE_MASK {
                panic!("File too big");
            }
            resource_offsets.push(offset);
            buffer.write_u32::<BigEndian>(resource.1.len() as u32)?;
            buffer.extend(resource.1);
        }
    }

    let map_offset = buffer.len() as u32;
    for _ in 0..MAP_HEADER_LENGTH { buffer.push(0); } // Fill map header with 0 for now

    buffer.write_u16::<BigEndian>(type_list_offset as u16)?;
    buffer.write_u16::<BigEndian>(name_list_offset as u16)?;

    // Write types
    buffer.write_u16::<BigEndian>(num_types as u16 - 1)?;
    let mut resource_list_offset = 2 + (num_types * TYPE_INFO_LENGTH);
    for (type_, resources) in resources {
        buffer.write_u32::<BigEndian>(u32::from_be_bytes(**type_))?;
        buffer.write_u16::<BigEndian>(resources.len() as u16 - 1)?;
        buffer.write_u16::<BigEndian>(resource_list_offset as u16)?;
        resource_list_offset += resources.len() as u32 * RESOURCE_INFO_LENGTH;
    }

    // Write resources
    // let mut name_list = Vec::<u8>::new();
    resource_offsets.reverse();
    for (_, resources) in resources {
        for resource in resources.iter() {
            buffer.write_i16::<BigEndian>(resource.0)?;
            if true { // empty name
                buffer.write_u16::<BigEndian>(std::u16::MAX)?;
            } else {
                // buffer.write_u16::<BigEndian>(nameList.len());
                // buffer.write_u8(name.len());
                // buffer.extend(name.as_bytes());
            }

            let resource_data_offset = resource_offsets.pop().unwrap();
            let attributes = 0u32;
            let atts_and_offset = attributes << 24 | resource_data_offset;
            buffer.write_u32::<BigEndian>(atts_and_offset)?;

            buffer.write_u32::<BigEndian>(0)?; // Skip handle to next resource
        }
    }

    // buffer.extend(name_list);

    assert!(buffer.len() < DATA_SIZE_MASK as usize);

    // Go back and write headers
    let mut header = Vec::new();
    let data_length = map_offset - DATA_OFFSET;
    let map_length = buffer.len() as u32 - map_offset;
    header.write_u32::<BigEndian>(DATA_OFFSET)?;
    header.write_u32::<BigEndian>(map_offset)?;
    header.write_u32::<BigEndian>(data_length)?;
    header.write_u32::<BigEndian>(map_length)?;

    buffer[0..16].copy_from_slice(&header);
    buffer[map_offset as usize..map_offset as usize + 16].copy_from_slice(&header);

    Ok(buffer)
}
