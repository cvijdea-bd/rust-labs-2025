use std::env;
use std::fs;
use std::process;

/// Decodes a ULEB128-encoded integer from the given byte slice.
/// Returns the decoded value and the number of bytes read.
///
/// # Panics
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

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Read the file content as bytes (binary)
    let bytes = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to read file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    // count of string_id_item
    let string_ids_size = u32::from_le_bytes(bytes[56..60].try_into().unwrap());
    // offset in the file of array of string_id_item
    let string_ids_off = u32::from_le_bytes(bytes[60..64].try_into().unwrap());
    println!("string_ids_size: {}", string_ids_size);
    println!("string_ids_off: {}", string_ids_off);

    let string_ids =
        &bytes[string_ids_off as usize..(string_ids_off + string_ids_size * 4) as usize];
    for string_id in string_ids.chunks(4).take(15) {
        let string_id = u32::from_le_bytes(string_id.try_into().unwrap());
        println!("String ID: {}", string_id);

        let string_data_item = &bytes[string_id as usize..];
        let Some((string_len, count)) = decode_uleb128(string_data_item) else {
            eprintln!("Failed to decode ULEB128");
            continue;
        };
        println!(
            "String Data Item: {string_len} {:?}",
            String::from_utf8_lossy(&string_data_item[count..count + string_len as usize]),
        );
    }
}
