use dex::Dex;

mod dex;
mod utils;

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a file path");
    let buffer = std::fs::read(&path).expect("Failed to read file");
    let dex = Dex::try_parse_from_bytes(&buffer).expect("Failed to parse DEX file");
    for (i, string) in dex.strings.iter().enumerate() {
        println!("String {i}: {string}");
    }
    println!();
    for (i, t) in dex.types.iter().enumerate() {
        println!("Type {i}: {t}");
    }
    println!();
    for (i, method_id) in dex.method_ids.iter().enumerate() {
        let t = &dex.types[method_id.class_idx as usize];
        let n = &dex.strings[method_id.name_idx as usize];
        println!("Method {i}: {t}{n}");
    }
}
