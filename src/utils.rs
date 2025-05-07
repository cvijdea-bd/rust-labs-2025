/// Reads a 32-bit unsigned integer from the given byte slice at `offset` in little-endian order.
///
/// # Panics
///
/// Panics if the slice is not long enough to read 4 bytes.
pub fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

/// Reads a 16-bit unsigned integer from the given byte slice at `offset` in little-endian order.
///
/// # Panics
///
/// Panics if the slice is not long enough to read 2 bytes.
pub fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

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

/// Converts a byte to a tuple of two nibbles (4-bit values): (lo, hi).
pub const fn to_nibbles(byte: u8) -> (u8, u8) {
    (byte & 0x0F, byte >> 4)
}
