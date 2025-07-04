use crate::{
    errors::TableIdxError,
    traits::parse::TryParseFromBytes,
    utils::{read_u16_le, read_u32_le},
};

use super::Dex;

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
    pub fn to_human_readable(&self, dex: &Dex) -> Result<String, TableIdxError> {
        let class_name = dex
            .types
            .get(self.class_idx as usize)
            .ok_or(TableIdxError::Type(self.class_idx as usize))?;
        let type_name = dex
            .types
            .get(self.type_idx as usize)
            .ok_or(TableIdxError::Type(self.type_idx as usize))?;
        let field_name = dex
            .strings
            .get(self.name_idx as usize)
            .ok_or(TableIdxError::String(self.name_idx as usize))?;
        Ok(format!("{class_name}->{field_name}:{type_name}"))
    }
}

impl TryParseFromBytes for FieldIdItem {
    const NAME: &'static str = "field_id_item";
    const SIZE: usize = 8;

    fn parse_from_bytes(buffer: &[u8]) -> Self {
        let class_idx = read_u16_le(buffer, 0);
        let type_idx = read_u16_le(buffer, 2);
        let name_idx = read_u32_le(buffer, 4);
        Self {
            class_idx,
            type_idx,
            name_idx,
        }
    }
}
