

use byteorder::{ WriteBytesExt, BigEndian };
use std::io::Result;

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
