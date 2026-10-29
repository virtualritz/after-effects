use byteorder::{BigEndian, WriteBytesExt};
use std::io::Result;

/// Embeds (Windows) or writes (macOS, to `macos_rsrc_path`) the PiPL resource for the target being
/// built (see [`TargetOs::current`](crate::TargetOs::current)).
pub fn produce_resource(pipl: &[u8], macos_rsrc_path: Option<&str>) {
    produce_resource_for(crate::TargetOs::current(), pipl, macos_rsrc_path)
}

pub(crate) fn produce_resource_for(target: crate::TargetOs, pipl: &[u8], macos_rsrc_path: Option<&str>) {
    match target {
        crate::TargetOs::Windows => {
            // An RC string literal is code-page converted by the resource
            // compiler, which turns every byte >= 0x80 into `?` (0x3F). The
            // global out-flags word crosses that line as soon as
            // `PF_OutFlag_CUSTOM_UI` (bit 15) is set, and After Effects then
            // refuses the plug-in with a global out-flags mismatch. A file
            // reference is copied into the resource verbatim, so the PiPL bytes
            // are written next to the generated .rc and referenced instead.
            let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set for build scripts");
            std::fs::write(std::path::Path::new(&out_dir).join("pipl.bin"), pipl).unwrap();

            let mut res = winres::WindowsResource::new();
            res.append_rc_content("16000 PiPL DISCARDABLE \"pipl.bin\"");
            if cfg!(windows) {
                res.compile().unwrap();
            } else {
                // Off Windows, winres only knows `rc.exe`/`windres` from fixed locations, so a missing
                // resource compiler must not fail e.g. `cargo check --target x86_64-pc-windows-msvc`.
                // The .rc and pipl.bin are still written to OUT_DIR.
                if let Err(e) = res.compile() {
                    println!(
                        "cargo:warning=The Windows PiPL resource was not compiled ({e}); the plugin won't load in After Effects without it. The resource script is {out_dir}/resource.rc"
                    );
                }
            }
        }
        crate::TargetOs::MacOs => {
            if let Some(rsrc_path) = macos_rsrc_path {
                #[rustfmt::skip]
                let rsrc_content = create_rsrc(&[
                    (b"PiPL", &[
                        (16000, pipl)
                    ])
                ]).unwrap();
                std::fs::write(rsrc_path, rsrc_content).unwrap();
            }
        }
    }
}

// Reference: https://github.com/dgelessus/python-rsrcfork/blob/master/src/rsrcfork/api.py#L14
// Reference: https://github.com/andrews05/ResForge/blob/master/ResForge/Formats/ClassicFormat.swift#L114
pub type RSRCResource<'a> = (i16, &'a [u8]); // id, data

pub fn create_rsrc(resources: &[(&[u8; 4], &[RSRCResource])]) -> Result<Vec<u8>> {
    const DATA_OFFSET: u32 = 256;
    const DATA_SIZE_MASK: u32 = (1 << 24) - 1;
    const MAP_HEADER_LENGTH: u32 = 24;
    const TYPE_INFO_LENGTH: u32 = 8;
    const RESOURCE_INFO_LENGTH: u32 = 12;

    let mut buffer = Vec::new();

    let num_types = resources.len() as u32;
    let num_resources: u32 = resources.iter().map(|x| x.1.len() as u32).sum();
    let type_list_offset = MAP_HEADER_LENGTH + 4;
    let name_list_offset: u32 = type_list_offset
        + 2
        + (num_types * TYPE_INFO_LENGTH)
        + (num_resources * RESOURCE_INFO_LENGTH);

    // Fill header with 0 for now
    buffer.extend(std::iter::repeat_n(0, DATA_OFFSET as usize));

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
    // Fill map header with 0 for now
    buffer.extend(std::iter::repeat_n(0, MAP_HEADER_LENGTH as usize));

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
            // empty name
            if true {
                buffer.write_u16::<BigEndian>(u16::MAX)?;
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
