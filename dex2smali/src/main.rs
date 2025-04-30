use dex::Dex;

mod dex;
mod utils;

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a file path");
    let buffer = std::fs::read(&path).expect("Failed to read file");
    let dex = Dex::parse_from_bytes(&buffer).expect("Failed to parse DEX file");
    for (i, string) in dex.strings.iter().enumerate() {
        println!("String {}: {}", i, string);
    }
    for (i, string) in dex.types.iter().enumerate() {
        println!("Type {}: {}", i, string);
    }
}
