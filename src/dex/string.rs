use std::ffi::CStr;

use crate::utils::decode_uleb128;

pub fn read_string_from_bytes(
    buffer: &[u8],
    offset: usize,
) -> Result<std::borrow::Cow<'_, str>, std::io::Error> {
    let (utf16_len, len_size) = decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Failed to decode ULEB128 for string length",
    ))?;
    let string_bytes_offset = offset + len_size;
    let cstr = CStr::from_bytes_until_nul(&buffer[string_bytes_offset..]).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse CStr: {e}"),
        )
    })?;
    let s = String::from_utf8_lossy(cstr.to_bytes());
    if s.encode_utf16().count() as u64 != utf16_len {
        eprintln!("Warning: String length mismatch for string: {}", s);
    }
    Ok(s)
}
