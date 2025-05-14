use std::path::Path;

use dex::{class_data_item::ClassDataItem, code_item::CodeItem, encoded::EncodedField, Dex};
use std::io::Write;

mod dex;
mod errors;
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

    let out_path = Path::new("out-smali");
    std::fs::remove_dir_all(out_path);
    std::fs::create_dir(out_path);

    for class in &dex.class_defs {
        let class_name = &dex.types[class.class_idx as usize];
        let superclass_name = &dex.types[class.superclass_idx as usize];
        let class_data_offset = class.class_data_off as usize;
        let class_data_item =
            match ClassDataItem::try_parse_from_bytes_unsized(&buffer[class_data_offset..]) {
                Ok(class_data_item) => class_data_item,
                Err(e) => {
                    eprintln!("Failed to parse ClassDataItem for {}: {}", class_name, e);
                    continue; // Skip this class if parsing fails
                }
            };

        let class_name_stripped = &class_name[1..class_name.len() - 1]; // Remove 'L' and ';'
        println!("Class: {}", class_name);
        let class_out_path =
            out_path.join(format!("{}.smali", class_name_stripped.replace('/', "_")));
        let mut class_out_file = std::fs::File::create(class_out_path).unwrap();
        writeln!(class_out_file, ".class {}", class_name);
        writeln!(class_out_file, ".super {}", superclass_name).unwrap();

        let fields = class_data_item
            .static_fields
            .iter()
            .chain(class_data_item.instance_fields.iter());
        let methods = class_data_item
            .direct_methods
            .iter()
            .chain(class_data_item.virtual_methods.iter());
        for field in fields {
            let field_id = &dex.field_ids[field.field_idx as usize];
            let field_type = &dex.types[field_id.type_idx as usize];
            let field_name = &dex.strings[field_id.name_idx as usize];

            writeln!(class_out_file, "").unwrap();
            writeln!(class_out_file, ".field {field_name}:{field_type}").unwrap();
        }
        for method in methods {
            let method_id = &dex.method_ids[method.method_idx as usize];
            let proto_id = &dex.proto_ids[method_id.proto_idx as usize];

            let return_type = &dex.types[proto_id.return_type_idx as usize];
            let method_name = &dex.strings[method_id.name_idx as usize];

            writeln!(class_out_file, "").unwrap();
            writeln!(class_out_file, ".method {method_name}(){return_type}").unwrap();

            println!(
                "  Method: {method_name}(){return_type} code_off={}",
                method.code_off
            );
            let code_item =
                CodeItem::try_parse_from_bytes_unsized(&buffer[method.code_off as usize..])
                    .unwrap();
            println!("");

            for insns in &code_item.insns {
                writeln!(class_out_file, "    {:?}", insns).unwrap();
            }

            writeln!(class_out_file, ".end method").unwrap();
        }
    }
}
