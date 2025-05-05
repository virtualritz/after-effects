use pipl::*;

#[rustfmt::skip]
fn main() {
    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEGeneral),
        Property::Name("AegpDemo"),
        Property::Category("General Plugin"),
        Property::Version {
                    version: 1,
                    subversion: 0,
                    bugversion: 0,
                    stage: Stage::Develop,
                    build: 0,
        },
        #[cfg(target_os = "windows")]
        #[cfg(target_arch = "x86_64")]
        Property::CodeWin64X86("EntryPointFunc"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EntryPointFunc"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EntryPointFunc"),
    ])
}
