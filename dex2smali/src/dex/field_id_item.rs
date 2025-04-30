use crate::utils::read_u32_le;

#[allow(unused)]
pub struct FieldIdItem {
    /// index into the `type_ids` list for the definer of this field. This must be a class type, and not an array or primitive type.
    pub class_idx: u16,
    /// index into the `type_ids` list for the type of this field
    pub type_idx: u16,
    /// index into the string_ids list for the name of this field. The string must conform to the syntax for MemberName
    pub name_idx: u32,
}

impl FieldIdItem {
    pub fn parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small to read FieldIdItem",
            ));
        }

        let class_idx = u16::from_le_bytes([buffer[0], buffer[1]]);
        let type_idx = u16::from_le_bytes([buffer[2], buffer[3]]);
        let name_idx = read_u32_le(buffer, 4);

        Ok(FieldIdItem {
            class_idx,
            type_idx,
            name_idx,
        })
    }
}
