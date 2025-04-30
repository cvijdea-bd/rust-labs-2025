use crate::utils::decode_uleb128;

use super::encoded::{EncodedField, EncodedMethod};

/// https://source.android.com/docs/core/runtime/dex-format#class-data-item
pub struct ClassDataItem {
    /// the defined static fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    static_fields: Vec<EncodedField>,
    /// the defined instance fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    instance_fields: Vec<EncodedField>,
    /// the defined direct (any of static, private, or constructor) methods, represented as a sequence of encoded elements. The methods must be sorted by `method_idx` in increasing order.
    direct_methods: Vec<EncodedMethod>,
    /// the defined virtual (none of `static`, `private`, or constructor) methods, represented as a sequence of encoded elements. This list should not include inherited methods unless overridden by the class that this item represents. The methods must be sorted by `method_idx` in increasing order. The `method_idx` of a virtual method must not be the same as any direct method.
    virtual_methods: Vec<EncodedMethod>,
}

impl ClassDataItem {
    pub fn parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;
        let (static_fields_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for static fields size",
            ))?;
        offset += bytes_used;

        let (instance_fields_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for instance fields size",
            ))?;
        offset += bytes_used;

        let (direct_methods_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for direct methods size",
            ))?;
        offset += bytes_used;

        let (virtual_methods_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for virtual methods size",
            ))?;

        Ok(ClassDataItem {
            static_fields: todo!(),
            instance_fields: todo!(),
            direct_methods: todo!(),
            virtual_methods: todo!(),
        })
    }
}
