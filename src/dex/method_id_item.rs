use crate::{
    errors::TableIdxError,
    traits::parse::TryParseFromBytes,
    utils::{read_u16_le, read_u32_le},
};

use super::Dex;

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
    pub fn to_human_readable(&self, dex: &Dex) -> Result<String, TableIdxError> {
        let class_name = dex
            .types
            .get(self.class_idx as usize)
            .ok_or(TableIdxError::Type(self.class_idx as usize))?;
        let proto = dex
            .proto_ids
            .get(self.proto_idx as usize)
            .ok_or(TableIdxError::ProtoId(self.proto_idx as usize))?;
        let method_name = dex
            .strings
            .get(self.name_idx as usize)
            .ok_or(TableIdxError::String(self.name_idx as usize))?;
        Ok(format!(
            "{class_name}->{method_name}{}",
            proto.to_human_readable(dex)?
        ))
    }
}

impl TryParseFromBytes for MethodIdItem {
    const NAME: &'static str = "method_id_item";
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
