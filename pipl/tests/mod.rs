pub const PF_PLUG_IN_VERSION: u16 = 13;
pub const PF_PLUG_IN_SUBVERS: u16 = 28;

fn hex(s: &'static str) -> Vec<u8> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .into_bytes()
        .chunks(2)
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>()
}

fn ensure_eq(our: &[u8], expected: &'static str) {
    let expected = hex(expected);
    let pretty_our = pretty_hex::pretty_hex(&our);
    let pretty_expected = pretty_hex::pretty_hex(&expected);
    similar_asserts::assert_eq!(pretty_expected, pretty_our);
}

use pipl::*;

// The layout is chosen by the target (`CARGO_CFG_TARGET_OS` in a build script), not by the host, so
// the golden tests of both platforms run everywhere through `build_pipl_for`.
#[test]
fn skeleton_macos() {
    let pipl = build_pipl_for(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("Portable"),
        Property::Category("Sample Plug-ins"),
        //Property::CodeWin64X86("EffectMain"),
        Property::CodeMacIntel64("EffectMain"),
        Property::CodeMacARM64("EffectMain"),
        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version {
            major: PF_PLUG_IN_VERSION,
            minor: PF_PLUG_IN_SUBVERS,
        },
        Property::AE_Effect_Version {
            version: 1,
            subversion: 1,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(OutFlags::PixIndependent | OutFlags::UseOutputExtent),
        Property::AE_Effect_Global_OutFlags_2(OutFlags2::SupportsThreadedRendering),
        Property::AE_Effect_Match_Name("ADBE Portable"),
        Property::AE_Reserved_Info(0),
        Property::AE_Effect_Support_URL("https://www.adobe.com"),
    ], TargetOs::MacOs)
    .unwrap();

    ensure_eq(&pipl, "000000000000000E3842494D6B696E64000000000000000465464B543842494D6E616D65000000000000000908506F727461626C650000003842494D6361746700000000000000100F53
                      616D706C6520506C75672D696E733842494D6D693634000000000000000B0A4566666563744D61696E003842494D6D613634000000000000000B0A4566666563744D61696E003842494D
                      655056520000000000000004000200003842494D655356520000000000000004000D001C3842494D655645520000000000000004000880013842494D65494E4600000000000000020000
                      00003842494D65474C4F0000000000000004000004403842494D65474C320000000000000004080000003842494D654D4E41000000000000000E0D4144424520506F727461626C650000
                      3842494D6165464C0000000000000004000000003842494D6555524C00000000000000161568747470733A2F2F7777772E61646F62652E636F6D0000");

    #[rustfmt::skip]
    let rsrc = create_rsrc(&[
        (b"PiPL", &[
            (16000, &pipl)
        ])
    ]).unwrap();

    let hex_data = "0000010000000268000001680000003200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000000000000164000000000000000e3842494d6b696e64000000000000000465464b543842494d6e616d65
                    000000000000000908506f727461626c650000003842494d6361746700000000000000100f53616d706c6520506c75672d696e733842494d6d693634000000000000000b0a4566666563
                    744d61696e003842494d6d613634000000000000000b0a4566666563744d61696e003842494d655056520000000000000004000200003842494d655356520000000000000004000d001c
                    3842494d655645520000000000000004000880013842494d65494e460000000000000002000000003842494d65474c4f0000000000000004000004403842494d65474c32000000000000
                    0004080000003842494d654d4e41000000000000000e0d4144424520506f727461626c6500003842494d6165464c0000000000000004000000003842494d6555524c0000000000000016
                    1568747470733a2f2f7777772e61646f62652e636f6d0000000001000000026800000168000000320000000000000000001c003200005069504c0000000a3e80ffff0000000000000000";
    ensure_eq(&rsrc, hex_data);
}

#[test]
fn sdk_invert_proc_amp_windows() {
    #[rustfmt::skip]
    let pipl = build_pipl_for(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("SDK_Invert_ProcAmp"),
        Property::Category("Sample Plug-ins"),
        Property::CodeWin64X86("EffectMain"),
        // Property::CodeMacIntel64("EffectMain"),
        // Property::CodeMacARM64("EffectMain"),
        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version { major: PF_PLUG_IN_VERSION, minor: PF_PLUG_IN_SUBVERS },
        Property::AE_Effect_Version {
            version: 1,
            subversion: 1,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
            OutFlags::PixIndependent |
            OutFlags::DeepColorAware
        ),
        Property::AE_Effect_Global_OutFlags_2(
            OutFlags2::FloatColorAware |
            OutFlags2::SupportsSmartRender |
            OutFlags2::SupportsThreadedRendering |
            OutFlags2::SupportsGpuRenderF32,
        ),
        Property::AE_Effect_Match_Name("ADBE SDK_Invert_ProcAmp"),
        Property::AE_Reserved_Info(0),
        Property::AE_Effect_Support_URL("https://www.adobe.com"),
    ], TargetOs::Windows).unwrap();

    ensure_eq(&pipl, "0100000000000D0000004D494238646E696B0000000004000000544B46654D494238656D616E00000000140000001253444B5F496E766572745F50726F63416D70004D49423867746163
                      00000000100000000F53616D706C6520506C75672D696E734D49423834363638000000000C0000004566666563744D61696E00004D494238525650650000000004000000020000004D49
                      42385256536500000000040000000D001C004D494238524556650000000004000000018008004D494238464E49650000000004000000000000004D4942384F4C47650000000004000000
                      000400024D494238324C476500000000040000000014000A4D494238414E4D65000000001800000017414442452053444B5F496E766572745F50726F63416D704D4942384C4665610000
                      000004000000000000004D4942384C52556500000000180000001568747470733A2F2F7777772E61646F62652E636F6D0000");
}

#[test]
fn creating_rsrc() {
    let pipl = hex("000000000000000d3842494d6b696e64000000000000000465464b543842494d6e616d65000000000000001110526564756365204e6f697365207635200000003842494d63617467000000000
                    000000b0a4e65617420566964656f003842494d6d693634000000000000000f0e456e747279506f696e7446756e63003842494d6d613634000000000000000f0e456e747279506f696e7446756
                    e63003842494d655056520000000000000004000200003842494d655356520000000000000004000d001c3842494d655645520000000000000004002000013842494d65494e460000000000000
                    002000000003842494d65474c4f0000000000000004020000323842494d65474c32000000000000000408b294093842494d654d4e41000000000000000c0b4e656174566964656f35413842494
                    d6165464c000000000000000400000008");

    #[rustfmt::skip]
    let rsrc = create_rsrc(&[
        (b"PiPL", &[
            (16000, &pipl)
        ])
    ]).unwrap();

    ensure_eq(&rsrc, "0000010000000248000001480000003200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                      0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                      0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                      0000000000000000000000000000000000000000000000000000000000000000000000000144000000000000000D3842494D6B696E64000000000000000465464B543842494D6E616D65
                      000000000000001110526564756365204E6F697365207635200000003842494D63617467000000000000000B0A4E65617420566964656F003842494D6D693634000000000000000F0E45
                      6E747279506F696E7446756E63003842494D6D613634000000000000000F0E456E747279506F696E7446756E63003842494D655056520000000000000004000200003842494D65535652
                      0000000000000004000D001C3842494D655645520000000000000004002000013842494D65494E460000000000000002000000003842494D65474C4F0000000000000004020000323842
                      494D65474C32000000000000000408B294093842494D654D4E41000000000000000C0B4E656174566964656F35413842494D6165464C0000000000000004000000080000010000000248
                      00000148000000320000000000000000001C003200005069504C0000000A3E80FFFF0000000000000000");
}

// Reference bytes produced by Rez from the AE 27.0 SDK `AE_General.r` for the same properties.
#[test]
fn search_keywords_and_description_macos() {
    let pipl = build_pipl_for(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("Skeleton"),
        Property::Category("Sample Plug-ins"),
        Property::CodeMacIntel64("EffectMain"),
        Property::CodeMacARM64("EffectMain"),
        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version { major: 13, minor: 29 },
        Property::AE_Effect_Version {
            version: 1,
            subversion: 1,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(OutFlags::DeepColorAware),
        Property::AE_Effect_Global_OutFlags_2(OutFlags2::StringsAreUtf8),
        Property::AE_Effect_Match_Name("ADBE Skeleton"),
        Property::AE_Reserved_Info(0),
        Property::AE_Effect_Support_URL("https://www.adobe.com"),
        Property::AE_Effect_Search_Keywords(r#"{"en_US":["blur","soft"]}"#),
        Property::AE_Effect_Description("Skeleton effect."),
    ], TargetOs::MacOs)
    .unwrap();

    ensure_eq(&pipl, "00000000000000103842494D6B696E64000000000000000465464B543842494D6E616D65000000000000000908536B656C65746F6E0000003842494D6361746700000000000000100F5361
                      6D706C6520506C75672D696E733842494D6D693634000000000000000B0A4566666563744D61696E003842494D6D613634000000000000000B0A4566666563744D61696E003842494D6550
                      56520000000000000004000200003842494D655356520000000000000004000D001D3842494D655645520000000000000004000880013842494D65494E4600000000000000020000000038
                      42494D65474C4F0000000000000004020000003842494D65474C320000000000000004400000003842494D654D4E41000000000000000E0D4144424520536B656C65746F6E00003842494D
                      6165464C0000000000000004000000003842494D6555524C00000000000000161568747470733A2F2F7777772E61646F62652E636F6D00003842494D654B5744000000000000001A7B2265
                      6E5F5553223A5B22626C7572222C22736F6674225D7D0000003842494D654453430000000000000011536B656C65746F6E206566666563742E00000000");
}

fn skeleton_properties() -> Vec<Property> {
    vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("Skeleton"),
        Property::Category("Sample Plug-ins"),
        Property::CodeWin64X86("EffectMain"),
        Property::AE_Effect_Spec_Version { major: 13, minor: 29 },
        Property::AE_Effect_Global_OutFlags_2(OutFlags2::StringsAreUtf8),
        Property::AE_Effect_Search_Keywords("blur"),
    ]
}

// Outside a build script `CARGO_CFG_TARGET_OS` is unset, so `build_pipl` falls back to the host.
#[test]
fn build_pipl_defaults_to_the_host_layout() {
    let host = if cfg!(target_os = "windows") { TargetOs::Windows } else { TargetOs::MacOs };
    assert_eq!(TargetOs::current(), host);
    assert_eq!(build_pipl(skeleton_properties()).unwrap(), build_pipl_for(skeleton_properties(), host).unwrap());
}

#[test]
fn windows_and_macos_layouts_differ() {
    let windows = build_pipl_for(skeleton_properties(), TargetOs::Windows).unwrap();
    let macos = build_pipl_for(skeleton_properties(), TargetOs::MacOs).unwrap();

    // Windows: 2 reserved bytes, little-endian, four-character codes reversed.
    assert_eq!(&windows[..10], &[1, 0, 0, 0, 0, 0, 7, 0, 0, 0]);
    assert_eq!(&windows[10..18], b"MIB8dnik");
    // macOS: big-endian, four-character codes in natural order.
    assert_eq!(&macos[..8], &[0, 0, 0, 0, 0, 0, 0, 7]);
    assert_eq!(&macos[8..16], b"8BIMkind");
    // Windows pads C strings to 4 bytes inside the property: "blur\0" + 3, length 8.
    let ekwd = windows.windows(4).position(|w| w == b"DWKe").unwrap();
    assert_eq!(&windows[ekwd + 8..ekwd + 20], &[8, 0, 0, 0, b'b', b'l', b'u', b'r', 0, 0, 0, 0]);
    // macOS pads the whole property instead: length 5, then 3 padding bytes.
    let ekwd = macos.windows(4).position(|w| w == b"eKWD").unwrap();
    assert_eq!(&macos[ekwd + 8..ekwd + 20], &[0, 0, 0, 5, b'b', b'l', b'u', b'r', 0, 0, 0, 0]);
}
