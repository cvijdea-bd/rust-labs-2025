// https://doc.rust-lang.org/book/ch03-02-data-types.html

pub const SIGNED_BYTE: i8 = i8::MIN; // [-128, 127]
pub const UNSIGNED_BYTE: u8 = u8::MAX; // [0, 255]
pub const SIGNED_SHORT: i16 = i16::MIN; // [-32768, 32767]
pub const UNSIGNED_SHORT: u16 = u16::MAX; // [0, 65535]
pub const SIGNED_INT: i32 = i32::MIN; // [-2147483648, 2147483647]
pub const UNSIGNED_INT: u32 = u32::MAX; // [0, 4294967295]
pub const SIGNED_LONG: i64 = i64::MIN; // [-9223372036854775808, 9223372036854775807]
pub const UNSIGNED_LONG: u64 = u64::MAX; // [0, 18446744073709551615]

// he isize and usize types depend on the architecture of the computer your program is running on, 
// which is denoted in the table as “arch”: 64 bits 
// if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
pub const SIGNED_USIZE: isize = isize::MIN;
pub const UNSIGNED_USIZE: usize = usize::MAX;

// Number literals
pub const DECIMAL: i32 = -98_222;
pub const HEXADECIMAL: i32 = 0xFF; // 255
pub const OCTAL: i32 = 0o77; // 63
pub const BINARY: i32 = 0b1111_1111; // 255
pub const BYTE: u8 = b'A'; // 65

pub const FLOAT: f32 = f32::MIN; // [-3.4028235E38, 3.4028235E38]
pub const DOUBLE: f64 = f64::MIN; // [-1.7976931348623157E308, 1.7976931348623157E308]

pub const CHAR: char = 'A'; // [0, 65535]

pub const BOOLEAN: bool = true; // [false, true]

pub const STRING_SLICE: &str = "Hello, world!"; // [0, 65535]

pub const ARRAY: [i32; 5] = [1, 2, 3, 4, 5]; // [0, 65535]