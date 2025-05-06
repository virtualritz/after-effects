use pipl::*;

#[rustfmt::skip]
fn main() {
    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEGP),
        Property::Name("AegpDemo"),
        Property::Category("General Plugin"),
        #[cfg(target_os = "windows")]
        #[cfg(target_arch = "x86_64")]
        Property::CodeWin64X86("EntryPointFunc"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EntryPointFunc"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EntryPointFunc"),
    ])
}
