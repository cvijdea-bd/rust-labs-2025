use dex::{Dex, class_data_item::ClassDataItem, encoded::EncodedField};

mod dex;
mod traits;
mod utils;

fn print_field(dex: &Dex, field: &EncodedField) {
    let field_id = &dex.field_ids[field.field_idx as usize];
    let field_class = &dex.types[field_id.class_idx as usize];
    let field_type = &dex.types[field_id.type_idx as usize];
    let field_name = &dex.strings[field_id.name_idx as usize];

    println!("      Field: {} {} {}", field_class, field_type, field_name);
}

fn print_method(dex: &Dex, method: &dex::encoded::EncodedMethod) {
    let method_id = &dex.method_ids[method.method_idx as usize];
    let method_class = &dex.types[method_id.class_idx as usize];
    let proto_id = &dex.proto_ids[method_id.proto_idx as usize];

    let shorty = &dex.strings[proto_id.shorty_idx as usize];
    let return_type = &dex.types[proto_id.return_type_idx as usize];
    let method_proto = format!("{} {}", return_type, shorty);

    let method_name = &dex.strings[method_id.name_idx as usize];

    println!(
        "      Method: {} {} {}",
        method_class, method_proto, method_name
    );
}

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a file path");
    let buffer = std::fs::read(&path).expect("Failed to read file");
    let dex = Dex::try_parse_from_bytes(&buffer).expect("Failed to parse DEX file");
    for class in &dex.class_defs {
        let class_name = &dex.types[class.class_idx as usize];
        let class_data_offset = class.class_data_off as usize;
        match ClassDataItem::try_parse_from_bytes_unsized(&buffer[class_data_offset..]) {
            Ok(class_data_item) => {
                println!("Class: {}", class_name);
                println!("  Static Fields:");
                for encoded_field in &class_data_item.static_fields {
                    print_field(&dex, encoded_field);
                }
                println!("  Instance Fields:");
                for encoded_field in &class_data_item.instance_fields {
                    print_field(&dex, encoded_field);
                }
                println!("  Direct Methods:");
                for encoded_method in &class_data_item.direct_methods {
                    print_method(&dex, encoded_method);
                }
                println!("  Virtual Methods:");
                for encoded_method in &class_data_item.virtual_methods {
                    print_method(&dex, encoded_method);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse ClassDataItem for {}: {}", class_name, e);
            }
        }
    }
}
