use crate::utils::decode_uleb128;

/// https://source.android.com/docs/core/runtime/dex-format#encoded-field-format
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedField {
    /// index into the `field_ids` list for the identity of this field (includes the name and descriptor).
    pub field_idx: u64,
    /// access flags for the field (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    pub access_flags: u64,
}

impl EncodedField {
    pub fn parse_from_bytes_with_offset(
        buffer: &[u8],
        prev: u64,
        offset: &mut usize,
    ) -> std::io::Result<Self> {
        let (field_idx_diff, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for field index difference",
            ))?;
        *offset += bytes_used;

        let (access_flags, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for access flags",
            ))?;
        *offset += bytes_used;

        Ok(EncodedField {
            field_idx: prev + field_idx_diff,
            access_flags,
        })
    }
}

/// https://source.android.com/docs/core/runtime/dex-format#encoded-method
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedMethod {
    /// index into the `method_ids` list for the identity of this method (includes the name and descriptor).
    pub method_idx: u64,
    /// access flags for the method (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    pub access_flags: u64,
    /// offset from the start of the file to the code structure for this method, or `0` if this method is either `abstract` or `native`. The offset should be to a location in the data section. The format of the data is specified by "`code_item`" below.
    pub code_off: u64,
}

impl EncodedMethod {
    pub fn parse_from_bytes_with_offset(
        buffer: &[u8],
        prev: u64,
        offset: &mut usize,
    ) -> std::io::Result<Self> {
        let (method_idx_diff, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for method index difference",
            ))?;
        *offset += bytes_used;

        let (access_flags, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for access flags",
            ))?;
        *offset += bytes_used;

        let (code_off, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for code offset",
            ))?;
        *offset += bytes_used;

        Ok(EncodedMethod {
            method_idx: prev + method_idx_diff,
            access_flags,
            code_off,
        })
    }
}
