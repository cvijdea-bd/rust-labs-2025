use crate::{traits::parse::TryParseFromBytes, utils::read_u16_le};

#[allow(unused)]
#[derive(Debug)]
pub struct MethodHandleItem {
    /// type of the method handle; see [Method handle type codes](https://source.android.com/docs/core/runtime/dex-format#method-handle-type-codes)
    pub method_handle_type: u16,
    /// Field or method id depending on whether the method handle type is an accessor or a method invoker
    pub field_or_method_id: u16,
}

impl TryParseFromBytes for MethodHandleItem {
    const NAME: &'static str = "method_handle_item";
    const SIZE: usize = 8;

    fn parse_from_bytes(buffer: &[u8]) -> Self {
        let method_handle_type = read_u16_le(buffer, 0);
        let field_or_method_id = read_u16_le(buffer, 4);
        MethodHandleItem {
            method_handle_type,
            field_or_method_id,
        }
    }
}
