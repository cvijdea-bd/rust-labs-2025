use core::str;
use std::borrow::Cow;
use std::env;
use std::ffi::CStr;
use std::fs;
use std::process;

/// Decodes a ULEB128-encoded integer from the given byte slice.
/// Returns the decoded value and the number of bytes read.
///
/// # Panics
///
/// Panics if the ULEB128 encoding is malformed (i.e., exceeds 10 bytes).
pub fn decode_uleb128(input: &[u8]) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    let mut count = 0;

    for byte in input {
        let value = (byte & 0x7F) as u64;
        result |= value << shift;

        count += 1;

        if (byte & 0x80) == 0 {
            return Some((result, count));
        }

        shift += 7;

        if shift >= 64 {
            // ULEB128 shouldn't be more than 10 bytes for u64
            break;
        }
    }

    None
}

#[allow(unused)]
#[derive(Debug)]
struct DexHeader {
    magic: [u8; 8],
    checksum: u32,
    signature: [u8; 20],
    file_size: u32,
    header_size: u32,
    endian_tag: u32,
    link_size: u32,
    link_off: u32,
    map_off: u32,
    string_ids_size: u32,
    string_ids_off: u32,
    type_ids_size: u32,
    type_ids_off: u32,
    proto_ids_size: u32,
    proto_ids_off: u32,
    field_ids_size: u32,
    field_ids_off: u32,
    method_ids_size: u32,
    method_ids_off: u32,
    class_defs_size: u32,
    class_defs_off: u32,
    data_size: u32,
    data_off: u32,
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn parse_dex_header(buffer: &[u8]) -> std::io::Result<DexHeader> {
    let buffer = buffer.get(0..112).ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Buffer too small to contain Dex header",
    ))?;

    let header = DexHeader {
        magic: buffer[0..8].try_into().unwrap(),
        checksum: read_u32_le(buffer, 8),
        signature: buffer[12..32].try_into().unwrap(),
        file_size: read_u32_le(buffer, 32),
        header_size: read_u32_le(buffer, 36),
        endian_tag: read_u32_le(buffer, 40),
        link_size: read_u32_le(buffer, 44),
        link_off: read_u32_le(buffer, 48),
        map_off: read_u32_le(buffer, 52),
        string_ids_size: read_u32_le(buffer, 56),
        string_ids_off: read_u32_le(buffer, 60),
        type_ids_size: read_u32_le(buffer, 64),
        type_ids_off: read_u32_le(buffer, 68),
        proto_ids_size: read_u32_le(buffer, 72),
        proto_ids_off: read_u32_le(buffer, 76),
        field_ids_size: read_u32_le(buffer, 80),
        field_ids_off: read_u32_le(buffer, 84),
        method_ids_size: read_u32_le(buffer, 88),
        method_ids_off: read_u32_le(buffer, 92),
        class_defs_size: read_u32_le(buffer, 96),
        class_defs_off: read_u32_le(buffer, 100),
        data_size: read_u32_le(buffer, 104),
        data_off: read_u32_le(buffer, 108),
    };

    Ok(header)
}

fn read_dex_strings<'a>(
    buffer: &'a [u8],
    header: &DexHeader,
) -> std::io::Result<Vec<Cow<'a, str>>> {
    let mut strings = Vec::with_capacity(header.string_ids_size as usize);
    let string_ids_off = header.string_ids_off as usize;

    // Read offsets to actual strings
    let mut string_offsets = Vec::with_capacity(header.string_ids_size as usize);
    for i in 0..header.string_ids_size as usize {
        let offset = string_ids_off + i * 4;
        let str_data_off = read_u32_le(buffer, offset) as usize;
        string_offsets.push(str_data_off);
    }

    for &offset in &string_offsets {
        let (utf16_len, len_size) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for string length",
            ))?;

        let string_bytes_offset = offset + len_size;
        let Ok(s) = CStr::from_bytes_until_nul(&buffer[string_bytes_offset..]) else {
            continue;
        };
        let s = String::from_utf8_lossy(s.to_bytes());
        if s.encode_utf16().count() as u64 != utf16_len {
            eprintln!("Warning: String length mismatch for string: {}", s);
        }
        strings.push(s);
    }

    Ok(strings)
}

fn parse_dex(path: &str) -> std::io::Result<()> {
    // Read the file content as bytes (binary)
    let bytes = fs::read(path)?;
    let header = parse_dex_header(&bytes)?;

    let string_ids_size = header.string_ids_size;
    let string_ids_off = header.string_ids_off;
    println!("string_ids_size: {}", string_ids_size);
    println!("string_ids_off: {}", string_ids_off);

    let strings = read_dex_strings(&bytes, &header)?;
    for (i, string) in strings.iter().enumerate() {
        println!("String {}: {:?}", i, string);
    }

    let class_defs = read_class_defs(&bytes, &header)?;

    let type_ids = read_type_ids(&bytes, &header, &strings)?;
    for (i, type_id) in type_ids.iter().enumerate() {
        println!("Type ID {}: {:?}", i, type_id.descriptor);
    }

    // let method_ids = read_method_ids(&bytes, &header)?;

    for (i, cd) in class_defs.iter().enumerate() {
        println!(
            "class #{}: class_idx={} {:?}",
            i, cd.class_idx, type_ids[cd.class_idx as usize].descriptor
        );
        println!("  class_data_off: {}", cd.class_data_off);
        let class_data = read_class_data_item(&bytes, cd.class_data_off)?;
        println!("  class_data: {:?}", class_data);

        // for (j, enc_method) in class_data
        //     .direct_methods
        //     .iter()
        //     .chain(class_data.virtual_methods.iter())
        //     .enumerate()
        // {
        //     // TODO: compute method index
        //     let method_idx: u32 = todo!();
        //     let method = method_ids[method_idx as usize];
        //     println!("  method #{j}: {}", strings[method.name_idx as usize]);
        // }
    }
    Ok(())
}

#[allow(unused)]
#[derive(Debug)]
struct TypeId<'a> {
    descriptor: &'a str,
}

#[allow(unused)]
#[derive(Debug)]
struct ClassDefItem {
    class_idx: u32,
    access_flags: u32,
    superclass_idx: u32,
    interfaces_off: u32,
    source_file_idx: u32,
    annotations_off: u32,
    class_data_off: u32,
    static_values_off: u32,
}

#[allow(unused)]
#[derive(Debug)]
struct EncodedField {
    field_idx_diff: u32,
    access_flags: u32,
}

#[allow(unused)]
#[derive(Debug)]
struct EncodedMethod {
    method_idx_diff: u32,
    access_flags: u32,
    code_off: u32,
}

#[allow(unused)]
#[derive(Debug)]
struct ClassDataItem {
    static_fields: Vec<EncodedField>,
    instance_fields: Vec<EncodedField>,
    direct_methods: Vec<EncodedMethod>,
    virtual_methods: Vec<EncodedMethod>,
}

#[allow(unused)]
#[derive(Debug)]
struct MethodIdItem {
    class_idx: u16,
    proto_idx: u16,
    name_idx: u32,
}

fn read_type_ids<'a>(
    buffer: &'a [u8],
    header: &DexHeader,
    strings: &'a [Cow<'a, str>],
) -> std::io::Result<Vec<TypeId<'a>>> {
    let type_ids_off = header.type_ids_off as usize;
    let type_ids_size = header.type_ids_size as usize;

    let mut type_ids = Vec::new();
    let mut offset = type_ids_off;
    for _ in 0..type_ids_size {
        let descriptor_idx = read_u32_le(buffer, offset) as usize;
        offset += 4;

        let descriptor = strings
            .get(descriptor_idx)
            .map(|c| c.as_ref())
            .unwrap_or("wrong descriptor index");
        type_ids.push(TypeId { descriptor });
    }

    Ok(type_ids)
}

fn read_class_defs(buffer: &[u8], header: &DexHeader) -> std::io::Result<Vec<ClassDefItem>> {
    let class_defs_off = header.class_defs_off as usize;
    let class_defs_size = header.class_defs_size as usize;

    let mut class_defs = Vec::new();
    let mut offset = class_defs_off;
    for _ in 0..class_defs_size {
        let class_idx = read_u32_le(buffer, offset);
        let access_flags = read_u32_le(buffer, offset + 4);
        let superclass_idx = read_u32_le(buffer, offset + 8);
        let interfaces_off = read_u32_le(buffer, offset + 12);
        let source_file_idx = read_u32_le(buffer, offset + 16);
        let annotations_off = read_u32_le(buffer, offset + 20);
        let class_data_off = read_u32_le(buffer, offset + 24);
        let static_values_off = read_u32_le(buffer, offset + 28);
        offset += 32;

        class_defs.push(ClassDefItem {
            class_idx,
            access_flags,
            superclass_idx,
            interfaces_off,
            source_file_idx,
            annotations_off,
            class_data_off,
            static_values_off,
        });
    }

    Ok(class_defs)
}

fn read_class_data_item(buffer: &[u8], mut class_data_off: u32) -> std::io::Result<ClassDataItem> {
    let (static_fields_size, bytes_used) = decode_uleb128(&buffer[class_data_off as usize..])
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to decode ULEB128 for static fields size",
        ))?;
    class_data_off += bytes_used as u32;

    let (instance_fields_size, bytes_used) = decode_uleb128(&buffer[class_data_off as usize..])
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to decode ULEB128 for instance fields size",
        ))?;
    class_data_off += bytes_used as u32;

    let (direct_methods_size, bytes_used) = decode_uleb128(&buffer[class_data_off as usize..])
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to decode ULEB128 for direct methods size",
        ))?;
    class_data_off += bytes_used as u32;

    let (virtual_methods_size, bytes_used) = decode_uleb128(&buffer[class_data_off as usize..])
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to decode ULEB128 for virtual methods size",
        ))?;
    class_data_off += bytes_used as u32;

    println!(
        "static_fields_size: {}, instance_fields_size: {}, direct_methods_size: {}, virtual_methods_size: {}",
        static_fields_size, instance_fields_size, direct_methods_size, virtual_methods_size
    );

    Ok(ClassDataItem {
        static_fields: vec![],
        instance_fields: vec![], // TODO: Implement reading instance fields
        direct_methods: vec![],  // TODO: Implement reading direct methods
        virtual_methods: vec![], // TODO: Implement reading virtual methods
    })
}

fn read_method_ids(buffer: &[u8], header: &DexHeader) -> std::io::Result<Vec<MethodIdItem>> {
    todo!()
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match parse_dex(file_path) {
        Ok(_) => println!("DEX file parsed successfully."),
        Err(err) => eprintln!("Error parsing DEX file: {}", err),
    }
}
