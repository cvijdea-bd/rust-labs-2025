use crate::{traits::parse::TryParseFromBytes, utils::read_u32_le};

use super::Dex;

#[allow(unused)]
pub struct ProtoIdItem {
    /// index into the string_ids list for the short-form descriptor string of this prototype. The string must conform to the syntax for ShortyDescriptor, and must correspond to the return type and parameters of this item.
    pub shorty_idx: u32,
    /// index into the `type_ids` list for the return type of this prototype
    pub return_type_idx: u32,
    /// offset from the start of the file to the list of parameter types for this prototype, or 0 if this prototype has no parameters. This offset, if non-zero, should be in the data section, and the data there should be in the format specified by "type_list" below. Additionally, there should be no reference to the type void in the list.
    pub parameters_off: u32,j 
}

impl ProtoIdItem {
    pub fn to_human_readable(&self, dex: &Dex) -> Result<String, crate::errors::TableIdxError> {
        let return_type = dex.types.get(self.return_type_idx as usize).ok_or(
            crate::errors::TableIdxError::Type(self.return_type_idx as usize),
        )?;

        Ok(format!("({}){return_type}", self.parameters_off))
    }
}

impl TryParseFromBytes for ProtoIdItem {
    const NAME: &'static str = "proto_id_item";
    const SIZE: usize = 12;

    fn parse_from_bytes(buffer: &[u8]) -> Self {
        let shorty_idx = read_u32_le(buffer, 0);
        let return_type_idx = read_u32_le(buffer, 4);
        let parameters_off = read_u32_le(buffer, 8);

        ProtoIdItem {
            shorty_idx,
            return_type_idx,
            parameters_off,
        }
    }
}
