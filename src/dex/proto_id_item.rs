use crate::utils::read_u32_le;

#[allow(unused)]
pub struct ProtoIdItem {
    /// index into the string_ids list for the short-form descriptor string of this prototype. The string must conform to the syntax for ShortyDescriptor, and must correspond to the return type and parameters of this item.
    pub shorty_idx: u32,
    /// index into the `type_ids` list for the return type of this prototype
    pub return_type_idx: u32,
    /// offset from the start of the file to the list of parameter types for this prototype, or 0 if this prototype has no parameters. This offset, if non-zero, should be in the data section, and the data there should be in the format specified by "type_list" below. Additionally, there should be no reference to the type void in the list.
    pub parameters_off: u32,
}

impl ProtoIdItem {
    pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < 12 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer too short to parse ProtoIdItem",
            ));
        }
        let shorty_idx = read_u32_le(buffer, 0);
        let return_type_idx = read_u32_le(buffer, 4);
        let parameters_off = read_u32_le(buffer, 8);

        Ok(ProtoIdItem {
            shorty_idx,
            return_type_idx,
            parameters_off,
        })
    }
}
