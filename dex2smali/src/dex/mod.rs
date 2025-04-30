mod class_data_item;
mod class_def_item;
mod encoded;
mod header;
mod method_id_item;
mod string;

use std::borrow::Cow;

use crate::utils::read_u32_le;
use class_def_item::ClassDefItem;
use header::HeaderItem;
use method_id_item::MethodIdItem;

#[allow(unused)]
pub struct Dex<'a> {
    pub strings: Vec<Cow<'a, str>>,
    pub types: Vec<Cow<'a, str>>,
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

    fn read_method_id_items(buffer: &'a [u8], header: &HeaderItem) -> Vec<MethodIdItem> {
        let method_ids_off = header.method_ids_off as usize;
        let method_ids_size = header.method_ids_size as usize;

        let mut method_ids = Vec::with_capacity(method_ids_size);
        for i in 0..method_ids_size {
            let offset = method_ids_off + i * 8;
            match MethodIdItem::parse_from_bytes(&buffer[offset..offset + 8]) {
                Ok(method_id) => method_ids.push(method_id),
                Err(e) => eprintln!("Failed to parse MethodIdItem at offset {}: {}", offset, e),
            }
        }

        method_ids
    }

    fn read_class_def_items(buffer: &'a [u8], header: &HeaderItem) -> Vec<ClassDefItem> {
        let class_defs_off = header.class_defs_off as usize;
        let class_defs_size = header.class_defs_size as usize;

        let mut class_defs = Vec::with_capacity(class_defs_size);
        for i in 0..class_defs_size {
            let offset = class_defs_off + i * 32;
            match ClassDefItem::parse_from_bytes(&buffer[offset..offset + 32]) {
                Ok(class_def) => class_defs.push(class_def),
                Err(e) => eprintln!("Failed to parse ClassDefItem at offset {}: {}", offset, e),
            }
        }

        class_defs
    }

    pub fn parse_from_bytes(buffer: &'a [u8]) -> std::io::Result<Self> {
        let header_item = HeaderItem::parse_from_bytes(buffer)?;

        let strings = Self::read_strings(buffer, &header_item);
        let types = Self::read_types(buffer, &header_item);
        // let proto_id_items = todo!();
        // let field_id_items = todo!();

        let method_ids = Self::read_method_id_items(buffer, &header_item);

        let class_defs = Self::read_class_def_items(buffer, &header_item);

        Ok(Self {
            strings,
            types,
            method_ids,
            class_defs,
        })
    }
}
