use crate::utils::decode_uleb128;

use super::encoded::{EncodedField, EncodedMethod};

/// https://source.android.com/docs/core/runtime/dex-format#class-data-item
#[allow(unused)]
pub struct ClassDataItem {
    /// the defined static fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    pub static_fields: Vec<EncodedField>,
    /// the defined instance fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    pub instance_fields: Vec<EncodedField>,
    /// the defined direct (any of static, private, or constructor) methods, represented as a sequence of encoded elements. The methods must be sorted by `method_idx` in increasing order.
    pub direct_methods: Vec<EncodedMethod>,
    /// the defined virtual (none of `static`, `private`, or constructor) methods, represented as a sequence of encoded elements. This list should not include inherited methods unless overridden by the class that this item represents. The methods must be sorted by `method_idx` in increasing order. The `method_idx` of a virtual method must not be the same as any direct method.
    pub virtual_methods: Vec<EncodedMethod>,
}

impl ClassDataItem {
    fn read_encoded_fields(
        buffer: &[u8],
        offset: &mut usize,
        size: usize,
    ) -> std::io::Result<Vec<EncodedField>> {
        let mut encoded_fields = Vec::with_capacity(size);
        let mut prev = 0;
        for _ in 0..size {
            let encoded_field = EncodedField::parse_from_bytes_with_offset(buffer, prev, offset)?;
            prev = encoded_field.field_idx;
            encoded_fields.push(encoded_field);
        }
        Ok(encoded_fields)
    }

    fn read_encoded_methods(
        buffer: &[u8],
        offset: &mut usize,
        size: usize,
    ) -> std::io::Result<Vec<EncodedMethod>> {
        let mut encoded_methods = Vec::with_capacity(size);
        let mut prev = 0;
        for _ in 0..size {
            let encoded_method = EncodedMethod::parse_from_bytes_with_offset(buffer, prev, offset)?;
            prev = encoded_method.method_idx;
            encoded_methods.push(encoded_method);
        }
        Ok(encoded_methods)
    }

    pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;
        let (static_fields_size, bytes_used) =
            decode_uleb128(&buffer).ok_or(std::io::Error::new(
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
        offset += bytes_used;

        let static_fields =
            Self::read_encoded_fields(buffer, &mut offset, static_fields_size as usize)?;

        let instance_fields =
            Self::read_encoded_fields(buffer, &mut offset, instance_fields_size as usize)?;

        let direct_methods =
            Self::read_encoded_methods(buffer, &mut offset, direct_methods_size as usize)?;

        let virtual_methods =
            Self::read_encoded_methods(buffer, &mut offset, virtual_methods_size as usize)?;

        Ok(ClassDataItem {
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }
}
