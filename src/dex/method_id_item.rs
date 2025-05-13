use crate::{
    traits::parse::TryParseFromBytes,
    utils::{read_u16_le, read_u32_le},
};

#[allow(unused)]
pub struct MethodIdItem {
    /// index into the `type_ids` list for the definer of this method. This must be a class or array type, and not a primitive type.
    pub class_idx: u16,
    /// index into the `proto_ids` list for the prototype of this method
    pub proto_idx: u16,
    /// index into the `string_ids` list for the name of this method. The string must conform to the syntax for MemberName.
    pub name_idx: u32,
}

impl TryParseFromBytes for MethodIdItem {
    const NAME: &str = "method_id_item";
    const SIZE: usize = 8;

    fn parse_from_bytes(buffer: &[u8]) -> Self {
        let class_idx = read_u16_le(buffer, 0);
        let proto_idx = read_u16_le(buffer, 2);
        let name_idx = read_u32_le(buffer, 4);
        Self {
            class_idx,
            proto_idx,
            name_idx,
        }
    }
}
