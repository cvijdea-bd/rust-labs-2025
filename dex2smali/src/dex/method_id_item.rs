use crate::utils::{read_u16_le, read_u32_le};

#[allow(unused)]
pub struct MethodIdItem {
    /// index into the `type_ids` list for the definer of this method. This must be a class or array type, and not a primitive type.
    pub class_idx: u16,
    /// index into the `proto_ids` list for the prototype of this method
    pub proto_idx: u16,
    /// index into the `string_ids` list for the name of this method. The string must conform to the syntax for MemberName.
    pub name_idx: u32,
}

impl MethodIdItem {
    pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small to read MethodIdItem",
            ));
        }

        let class_idx = read_u16_le(buffer, 0);
        let proto_idx = read_u16_le(buffer, 2);
        let name_idx = read_u32_le(buffer, 4);

        Ok(MethodIdItem {
            class_idx,
            proto_idx,
            name_idx,
        })
    }
}
