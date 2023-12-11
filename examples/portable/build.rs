use pipl::*;

const PF_PLUG_IN_VERSION: u16 = 13;
const PF_PLUG_IN_SUBVERS: u16 = 28;

fn main() {
    #[rustfmt::skip]
    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("Portable"),
        Property::Category("Sample Plug-ins"),

        #[cfg(target_os = "windows")]
        Property::CodeWin64X86("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EffectMain"),

        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version { major: PF_PLUG_IN_VERSION, minor: PF_PLUG_IN_SUBVERS },
        Property::AE_Effect_Version {
            version: 3,
            subversion: 3,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
            OutFlags::PixIndependent |
            OutFlags::UseOutputExtent
        ),
        Property::AE_Effect_Global_OutFlags_2(
            OutFlags2::SupportsThreadedRendering
        ),
        Property::AE_Effect_Match_Name("ADBE Portable"),
        Property::AE_Reserved_Info(0),
        Property::AE_Effect_Support_URL("https://www.adobe.com"),
    ])
}
