use std::{fs::File, path::Path};
use rayon::prelude::*;
use dex::{class_data_item::ClassDataItem, code_item::CodeItem, Dex};

mod dex;
mod errors;
mod traits;
mod utils;

fn write_class<W: std::io::Write>(
    writer: &mut W,
    dex: &Dex,
    class_name: &str,
    superclass_name: &str,
    class_data_item: &ClassDataItem,
) -> Result<(), std::io::Error> {
    writeln!(writer, ".class {}", class_name)?;
    writeln!(writer, ".super {}", superclass_name)?;

    let fields = class_data_item
        .static_fields
        .iter()
        .chain(class_data_item.instance_fields.iter());

    for field in fields {
        let field_id = &dex.field_ids[field.field_idx as usize];
        let field_type = &dex.types[field_id.type_idx as usize];
        let field_name = &dex.strings[field_id.name_idx as usize];

        writeln!(writer)?;
        writeln!(writer, ".field {field_name}:{field_type}")?;
    }

    let methods = class_data_item
        .direct_methods
        .iter()
        .chain(class_data_item.virtual_methods.iter());

    for method in methods {
        let method_id = &dex.method_ids[method.method_idx as usize];
        let proto_id = &dex.proto_ids[method_id.proto_idx as usize];

        let return_type = &dex.types[proto_id.return_type_idx as usize];
        let method_name = &dex.strings[method_id.name_idx as usize];

        writeln!(writer)?;
        writeln!(writer, ".method {method_name}(){return_type}")?;

        let code_item =
            match CodeItem::try_parse_from_bytes_unsized(&dex.raw[method.code_off as usize..]) {
                Ok(code_item) => code_item,
                Err(e) => {
                    eprintln!("Failed to parse CodeItem for {}: {}", method_name, e);
                    continue;
                }
            };

        for insns in &code_item.insns {
            match insns.to_human_readable(dex) {
                Ok(repr) => writeln!(writer, "    {repr}")?,
                Err(e) => {
                    eprintln!("Failed to write instruction: {e}");
                    continue;
                }
            }
        }

        writeln!(writer, ".end method")?;
    }

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a file path");
    let buffer = std::fs::read(&path).expect("Failed to read file");
    let dex = Dex::try_parse_from_bytes(&buffer).expect("Failed to parse DEX file");

    let out_path = Path::new("out-smali");
    if let Err(e) = std::fs::remove_dir_all(out_path) {
        eprint!("Failed to remove directory: {e}");
    }
    std::fs::create_dir(out_path).unwrap();

    let start_time = std::time::Instant::now();

    dex.class_defs.par_iter().for_each(|class| {
        let class_name = &dex.types[class.class_idx as usize];
        let superclass_name = &dex.types[class.superclass_idx as usize];
        let class_data_offset = class.class_data_off as usize;
        let class_data_item =
            match ClassDataItem::try_parse_from_bytes_unsized(&buffer[class_data_offset..]) {
                Ok(class_data_item) => class_data_item,
                Err(e) => {
                    eprintln!("Failed to parse ClassDataItem for {}: {}", class_name, e);
                    return; // Skip this class if parsing fails
                }
            };

        let class_name_stripped = &class_name[1..class_name.len() - 1]; // Remove 'L' and ';'

        let class_out_path =
            out_path.join(format!("{}.smali", class_name_stripped.replace('/', "_")));
        
        let mut class_out_file = File::create(&class_out_path)
            .unwrap_or_else(|_| panic!("Failed to create file: {}", class_out_path.display()));

        if let Err(e) = write_class(
            &mut class_out_file,
            &dex,
            class_name_stripped,
            superclass_name,
            &class_data_item,
        ) {
            eprintln!("Failed to write class {}: {}", class_name, e);
        }
    });

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time: {} seconds",
        elapsed_time.as_secs_f32()
    );
}
