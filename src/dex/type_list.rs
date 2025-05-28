use crate::{
    errors::DexParseError,
    traits::parse::TryParseFromBytes,
    utils::{read_u16_le, read_u32_le},
};

pub struct TypeList {
    /// elements of the list
    pub list: Vec<TypeItem>,
}

impl TypeList {
    const NAME: &'static str = "type_list";

    pub fn try_parse_from_bytes_unsized(buffer: &[u8]) -> Result<Self, DexParseError> {
        if buffer.len() < 2 {
            return Err(DexParseError::InvalidElementSize {
                field: Self::NAME,
                expected: 2,
                actual: buffer.len(),
            });
        }
        let size = read_u32_le(buffer, 0);
        if buffer.len() < (4 + size as usize * TypeItem::SIZE) {
            return Err(DexParseError::InvalidElementSize {
                field: Self::NAME,
                expected: 4 + size as usize * TypeItem::SIZE,
                actual: buffer.len(),
            });
        }

        let mut list = Vec::with_capacity(size as usize);
        for i in 0..size as usize {
            let offset = 4 + i * TypeItem::SIZE;
            let item = TypeItem::try_parse_from_bytes(&buffer[offset..])?;
            list.push(item);
        }

        Ok(Self { list })
    }
}

pub struct TypeItem {
    /// index into the `type_ids` list for this type
    pub type_idx: u16,
}

impl TryParseFromBytes for TypeItem {
    const NAME: &'static str = "type_item";

    const SIZE: usize = 2;

    fn parse_from_bytes(buffer: &[u8]) -> Self
    where
        Self: Sized,
    {
        Self {
            type_idx: read_u16_le(buffer, 0),
        }
    }
}
