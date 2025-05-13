pub mod class_data_item;
pub mod class_def_item;
pub mod code_item;
pub mod encoded;
pub mod field_id_item;
pub mod header_item;
mod instruction;
pub mod method_id_item;
pub mod proto_id_item;
mod string;

use std::borrow::Cow;

use crate::errors::DexParseError;
use crate::traits::parse::TryParseFromBytes;
use crate::utils::read_u32_le;
use class_def_item::ClassDefItem;
use field_id_item::FieldIdItem;
use header_item::HeaderItem;
use method_id_item::MethodIdItem;
use proto_id_item::ProtoIdItem;

#[allow(unused)]
pub struct Dex<'a> {
    pub header_item: HeaderItem,
    pub strings: Vec<Cow<'a, str>>,
    pub types: Vec<Cow<'a, str>>,
    pub proto_ids: Vec<ProtoIdItem>,
    pub field_ids: Vec<FieldIdItem>,
    pub method_ids: Vec<MethodIdItem>,
    pub class_defs: Vec<ClassDefItem>,
}

impl<'a> Dex<'a> {
    fn read_strings(buffer: &'a [u8], header: &HeaderItem) -> Vec<Cow<'a, str>> {
        let string_ids_off = header.string_ids_off as usize;
        let string_ids_size = header.string_ids_size as usize;
        let mut strings = Vec::with_capacity(string_ids_size);
        for i in 0..string_ids_size {
            let string_data_off = read_u32_le(buffer, string_ids_off + i * 4) as usize;
            if let Ok(str) = string::read_string_from_bytes(buffer, string_data_off) {
                strings.push(str);
            }
        }
        strings
    }

    fn read_types(buffer: &'a [u8], header: &HeaderItem) -> Vec<Cow<'a, str>> {
        let type_ids_off = header.type_ids_off as usize;
        let type_ids_size = header.type_ids_size as usize;
        let mut types = Vec::with_capacity(type_ids_size);
        for i in 0..type_ids_size {
            let descriptor_idx = read_u32_le(buffer, type_ids_off + i * 4) as usize;
            if let Some(str) = Self::read_strings(buffer, header)
                .get(descriptor_idx)
                .cloned()
            {
                types.push(str);
            } else {
                eprintln!(
                    "Warning: Type ID {} out of bounds for string IDs",
                    descriptor_idx
                );
            }
        }
        types
    }

    fn read_proto_id_items(buffer: &[u8], header: &HeaderItem) -> Vec<ProtoIdItem> {
        let proto_ids_off = header.proto_ids_off as usize;
        let proto_ids_size = header.proto_ids_size as usize;

        let mut proto_ids = Vec::with_capacity(proto_ids_size);
        for i in 0..proto_ids_size {
            let offset = proto_ids_off + i * 12;
            match ProtoIdItem::try_parse_from_bytes(&buffer[offset..]) {
                Ok(proto_id) => proto_ids.push(proto_id),
                Err(e) => eprintln!("Failed to parse ProtoIdItem at offset {}: {}", offset, e),
            }
        }

        proto_ids
    }

    fn read_field_id_items(buffer: &[u8], header: &HeaderItem) -> Vec<FieldIdItem> {
        let field_ids_off = header.field_ids_off as usize;
        let field_ids_size = header.field_ids_size as usize;

        let mut field_ids = Vec::with_capacity(field_ids_size);
        for i in 0..field_ids_size {
            let offset = field_ids_off + i * 8;
            match FieldIdItem::try_parse_from_bytes(&buffer[offset..]) {
                Ok(field_id) => field_ids.push(field_id),
                Err(e) => eprintln!("Failed to parse FieldIdItem at offset {}: {}", offset, e),
            }
        }

        field_ids
    }

    fn read_method_id_items(buffer: &[u8], header: &HeaderItem) -> Vec<MethodIdItem> {
        let method_ids_off = header.method_ids_off as usize;
        let method_ids_size = header.method_ids_size as usize;

        let mut method_ids = Vec::with_capacity(method_ids_size);
        for i in 0..method_ids_size {
            let offset = method_ids_off + i * 8;
            match MethodIdItem::try_parse_from_bytes(&buffer[offset..offset + 8]) {
                Ok(method_id) => method_ids.push(method_id),
                Err(e) => eprintln!("Failed to parse MethodIdItem at offset {}: {}", offset, e),
            }
        }

        method_ids
    }

    fn read_class_def_items(buffer: &[u8], header: &HeaderItem) -> Vec<ClassDefItem> {
        let class_defs_off = header.class_defs_off as usize;
        let class_defs_size = header.class_defs_size as usize;

        let mut class_defs = Vec::with_capacity(class_defs_size);
        for i in 0..class_defs_size {
            let offset = class_defs_off + i * 32;
            match ClassDefItem::try_parse_from_bytes(&buffer[offset..]) {
                Ok(class_def) => class_defs.push(class_def),
                Err(e) => eprintln!("Failed to parse ClassDefItem at offset {}: {}", offset, e),
            }
        }

        class_defs
    }

    pub fn try_parse_from_bytes(buffer: &'a [u8]) -> Result<Self, DexParseError> {
        let header_item = HeaderItem::try_parse_from_bytes(buffer)?;

        let strings = Self::read_strings(buffer, &header_item);
        let types = Self::read_types(buffer, &header_item);
        let proto_ids = Self::read_proto_id_items(buffer, &header_item);
        let field_ids = Self::read_field_id_items(buffer, &header_item);
        let method_ids = Self::read_method_id_items(buffer, &header_item);
        let class_defs = Self::read_class_def_items(buffer, &header_item);

        Ok(Self {
            header_item,
            strings,
            types,
            proto_ids,
            field_ids,
            method_ids,
            class_defs,
        })
    }
}
