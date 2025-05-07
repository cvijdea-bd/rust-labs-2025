# LAB 02 - Parsing DEX files

### Project structure
Instead of dumping the entire code in a single `main.rs` file, we will split our logic into multiple files. This is what we came up with:
```
dex2smali
├── Cargo.toml
└── src
    ├── dex                     < -- module containing Dex structure, parsing logic and all it's dependencies
    │   ├── class_data_item.rs
    │   ├── class_def_item.rs
    │   ├── encoded.rs
    │   ├── field_id_item.rs
    │   ├── header_item.rs
    │   ├── method_id_item.rs
    │   ├── mod.rs              < -- contains Dex struct
    │   ├── proto_id_item.rs    
    │   └── string.rs           < -- dex strings parsing 
    ├── main.rs                 < -- entry point
    └── utils.rs                < -- common utility functions like read_u32_le, read_u16_le, decode_uleb128 etc.
```

Feel free to modify the structure as you see fit, as long as as data is organized and readable.

### Files

#### src/utils.rs
The `src/utils.rs` file contains functions that we defined earlier, there is nothing much new here. Except we added a `read_u16_le` equivalent to `read_u32_le` that works with 2 bytes instead of 4.

```rust
/// Reads a 32-bit unsigned integer from the given byte slice at `offset` in little-endian order.
///
/// # Panics
///
/// Panics if the slice is not long enough to read 4 bytes.
pub fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

/// Reads a 16-bit unsigned integer from the given byte slice at `offset` in little-endian order.
///
/// # Panics
///
/// Panics if the slice is not long enough to read 2 bytes.
pub fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

/// Decodes a ULEB128-encoded integer from the given byte slice.
/// Returns the decoded value and the number of bytes read.
///
/// # Panics
/// 
/// Panics if the ULEB128 encoding is malformed (i.e., exceeds 10 bytes).
pub fn decode_uleb128(input: &[u8]) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    let mut count = 0;

    for byte in input {
        let value = (byte & 0x7F) as u64;
        result |= value << shift;

        count += 1;

        if (byte & 0x80) == 0 {
            return Some((result, count));
        }

        shift += 7;

        if shift >= 64 {
            // ULEB128 shouldn't be more than 10 bytes for u64
            break;
        }
    }

    None
}
```
#### src/dex/mod.rs
The `src/dex/mod.rs` file contains the newly added `Dex` struct and methods to parse members of the dex file.
We only added a subset of the fields and parsing logic for them, the rest will be added in the next labs.
```rust
mod class_data_item;
mod class_def_item;
mod encoded;
mod field_id_item;
mod header_item;
mod method_id_item;
mod proto_id_item;
mod string;

use std::borrow::Cow;

use crate::utils::read_u32_le;
use class_def_item::ClassDefItem;
use field_id_item::FieldIdItem;
use header_item::HeaderItem;
use method_id_item::MethodIdItem;
use proto_id_item::ProtoIdItem;

#[allow(unused)]
pub struct Dex<'a> {
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
            match ProtoIdItem::try_parse_from_bytes(&buffer[offset..offset + 12]) {
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
            match FieldIdItem::try_parse_from_bytes(&buffer[offset..offset + 8]) {
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
            match ClassDefItem::try_parse_from_bytes(&buffer[offset..offset + 32]) {
                Ok(class_def) => class_defs.push(class_def),
                Err(e) => eprintln!("Failed to parse ClassDefItem at offset {}: {}", offset, e),
            }
        }

        class_defs
    }

    pub fn try_parse_from_bytes(buffer: &'a [u8]) -> std::io::Result<Self> {
        let header_item = HeaderItem::try_parse_from_bytes(buffer)?;

        let strings = Self::read_strings(buffer, &header_item);
        let types = Self::read_types(buffer, &header_item);
        let proto_ids = Self::read_proto_id_items(buffer, &header_item);
        let field_ids = Self::read_field_id_items(buffer, &header_item);
        let method_ids = Self::read_method_id_items(buffer, &header_item);
        let class_defs = Self::read_class_def_items(buffer, &header_item);

        Ok(Self {
            strings,
            types,
            proto_ids,
            field_ids,
            method_ids,
            class_defs,
        })
    }
}
```

Our `Dex` implementation holds the following fields:
- `strings`: All of the strings parsed from the dex file, obtained by using the parsing function from the previous lab.
- `types`: All of the types parsed from the dex file, represented as strings. In the actual dex file, the `type_ids` table contains indices into the `string_ids` table, we store types as strings for easier access.
- `proto_ids`: All of the proto ids parsed from the dex file, represented as `ProtoIdItem` structs. `ProtoIdItem` struct contains information about the method signature, including the return type and parameter types.
- `field_ids`: All of the field ids parsed from the dex file, represented as `FieldIdItem` structs. `FieldIdItem` struct contains information about the fields in the dex file. A field is basically a variable inside the class, static or instance:
    ```java
    class MyClass {
        int myField; // instance field
        static int myStaticField; // static field
    }
    ```
- `method_ids`: All of the method ids parsed from the dex file, represented as `MethodIdItem` structs. `MethodIdItem` struct contains information about the methods in the dex file.
- `class_defs`: All of the class definitions parsed from the dex file, represented as `ClassDefItem` structs. `ClassDefItem` struct contains information about the classes in the dex file, including the class name, superclass name, and access flags (`public`, `private`, etc.).

The private `Dex` methods: `read_strings`, `read_types`, `read_proto_id_items`, `read_field_id_items`, `read_method_id_items` and `read_class_def_items` are used to parse the corresponding fields from the dex file. They take a byte slice and a reference to the header item, and return a vector of the corresponding items. You are encouraged to study the parsing logic in these methods, as well as parsing logic in `src/dex/*_item.rs` modules, but you can live without it. We will only cover a subset of the code in this lab.

#### src/dex/class_data_item.rs
We fully implemented the code for `ClassDataItem` struct parsing, as well as it's `EncodedField` and `EncodedMethod` dependencies. The struct contains references to the class fields and methods, which are represented as `EncodedField` and `EncodedMethod` structs.
```rust
use crate::utils::decode_uleb128;

use super::encoded::{EncodedField, EncodedMethod};

/// https://source.android.com/docs/core/runtime/dex-format#class-data-item
#[allow(unused)]
pub struct ClassDataItem {
    /// the defined static fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    static_fields: Vec<EncodedField>,
    /// the defined instance fields, represented as a sequence of encoded elements. The fields must be sorted by `field_idx` in increasing order.
    instance_fields: Vec<EncodedField>,
    /// the defined direct (any of static, private, or constructor) methods, represented as a sequence of encoded elements. The methods must be sorted by `method_idx` in increasing order.
    direct_methods: Vec<EncodedMethod>,
    /// the defined virtual (none of `static`, `private`, or constructor) methods, represented as a sequence of encoded elements. This list should not include inherited methods unless overridden by the class that this item represents. The methods must be sorted by `method_idx` in increasing order. The `method_idx` of a virtual method must not be the same as any direct method.
    virtual_methods: Vec<EncodedMethod>,
}

impl ClassDataItem {
    fn read_encoded_fields(
        buffer: &[u8],
        offset: &mut usize,
        size: usize,
    ) -> std::io::Result<Vec<EncodedField>> {
        let mut encoded_fields = Vec::with_capacity(size);
        let mut prev = 0;
        for _ in 0..size {
            let encoded_field = EncodedField::try_parse_from_bytes_with_offset(buffer, prev, offset)?;
            prev = encoded_field.field_idx;
            encoded_fields.push(encoded_field);
        }
        Ok(encoded_fields)
    }

    fn read_encoded_methods(
        buffer: &[u8],
        offset: &mut usize,
        size: usize,
    ) -> std::io::Result<Vec<EncodedMethod>> {
        let mut encoded_methods = Vec::with_capacity(size);
        let mut prev = 0;
        for _ in 0..size {
            let encoded_method = EncodedMethod::try_parse_from_bytes_with_offset(buffer, prev, offset)?;
            prev = encoded_method.method_idx;
            encoded_methods.push(encoded_method);
        }
        Ok(encoded_methods)
    }

    pub fn try_parse_from_bytes_unsized(buffer: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;
        let (static_fields_size, bytes_used) =
            decode_uleb128(&buffer).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for static fields size",
            ))?;
        offset += bytes_used;

        let (instance_fields_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for instance fields size",
            ))?;
        offset += bytes_used;

        let (direct_methods_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for direct methods size",
            ))?;
        offset += bytes_used;

        let (virtual_methods_size, bytes_used) =
            decode_uleb128(&buffer[offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for virtual methods size",
            ))?;
        offset += bytes_used;

        let static_fields =
            Self::read_encoded_fields(buffer, &mut offset, static_fields_size as usize)?;

        let instance_fields =
            Self::read_encoded_fields(buffer, &mut offset, instance_fields_size as usize)?;

        let direct_methods =
            Self::read_encoded_methods(buffer, &mut offset, direct_methods_size as usize)?;

        let virtual_methods =
            Self::read_encoded_methods(buffer, &mut offset, virtual_methods_size as usize)?;

        Ok(ClassDataItem {
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }
}
```
The public `try_parse_from_bytes_unsized` method is used as a constructor for the `ClassDataItem` struct. It takes a byte slice and returns a `ClassDataItem` struct. We added the `_unsized` suffix to indicate that `class_data_item` has no fixed size within the dex file. The method uses `ULEB128` to read the sizes of the fields and methods, and then calls the private `read_encoded_fields` and `read_encoded_methods` methods to read the corresponding items. The private methods take a byte slice, a mutable offset and a size as arguments, and return a vector of the corresponding items. They use the `EncodedField` and `EncodedMethod` structs to parse the items.

#### src/dex/encoded.rs
The `src/dex/encoded.rs` file contains the `EncodedField` and `EncodedMethod` structs, which are used to represent the fields and methods in the dex file. The structs contain information about the fields and methods, including their indices and access flags. The parsing logic for these structs is implemented in the `try_parse_from_bytes_with_offset` method, which takes a byte slice, a previous index and a mutable offset as arguments. The method returns an `EncodedField` or `EncodedMethod` struct, depending on the type of item being parsed.

```rust
use crate::utils::decode_uleb128;

/// https://source.android.com/docs/core/runtime/dex-format#encoded-field-format
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedField {
    /// index into the `field_ids` list for the identity of this field (includes the name and descriptor).
    pub field_idx: u64,
    /// access flags for the field (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    pub access_flags: u64,
}

impl EncodedField {
    pub fn try_parse_from_bytes_with_offset(buffer: &[u8], prev: u64, offset: &mut usize) -> std::io::Result<Self> {
        let (field_idx_diff, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for field index difference",
            ))?;
        *offset += bytes_used;

        let (access_flags, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for access flags",
            ))?;
        *offset += bytes_used;

        Ok(
            EncodedField {
                field_idx: prev + field_idx_diff,
                access_flags,
            },
        )
    }
}

/// https://source.android.com/docs/core/runtime/dex-format#encoded-method
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedMethod {
    /// index into the `method_ids` list for the identity of this method (includes the name and descriptor).
    pub method_idx: u64,
    /// access flags for the method (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    pub access_flags: u64,
    /// offset from the start of the file to the code structure for this method, or `0` if this method is either `abstract` or `native`. The offset should be to a location in the data section. The format of the data is specified by "`code_item`" below.
    pub code_off: u64,
}

impl EncodedMethod {
    pub fn try_parse_from_bytes_with_offset(buffer: &[u8], prev: u64, offset: &mut usize) -> std::io::Result<Self> {
        let (method_idx_diff, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for method index difference",
            ))?;
        *offset += bytes_used;

        let (access_flags, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for access flags",
            ))?;
        *offset += bytes_used;

        let (code_off, bytes_used) =
            decode_uleb128(&buffer[*offset..]).ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode ULEB128 for code offset",
            ))?;
        *offset += bytes_used;

        Ok(
            EncodedMethod {
                method_idx: prev + method_idx_diff,
                access_flags,
                code_off,
            },
        )
    }
}
```
The important thing to mention that the original `encoded_field` and `encoded_method` structs contain a field called `field_idx_diff` and `method_idx_diff`, which are used to store the difference between the current index and the previous index. This is done to save space in the dex file, as the indices are stored as `ULEB128` encoded values. The `try_parse_from_bytes_with_offset` method takes care of this by adding the difference to the previous index and returning the result. An original `encoded_*` sequence would look something like this:
```
[
    {
        (field/method)_idx_diff: 0x2025  // Because it is the first element, the index will always be a value that you can look up directly in the field/method id table
        access_flags: 0x1 <-- ignore this for now (or do not, you can read more about access flags here https://source.android.com/docs/core/runtime/dex-format#access-flags)
    }
    {
        (field/method)_idx_diff: 0x3  // Because this is not the first element, this must be treated as a difference from the previous element, so if you'd like to look up the field/method id, you must add this to the actual index value of the previous element, that would be 0x2025 + 0x3 = 0x2028
        access_flags: 0x2
    }
    {
        (field/method)_idx_diff: 0x7 // Same as above, this is a difference from the previous element, so if you'd like to look up the field/method id, you must add this to the actual index value of the previous element, that would be 0x2028 + 0x7 = 0x202F
        access_flags: 0x4
    }
]
```
### Assignment
#### Print out all the class data info available
Now that we have the implementation for `ClassDataItem` and all lookup tables it might use, we can print out most of the class data. Analyze how you can access human readable names of the class, fields and methods. *HINT* Use the indexes the encoded items point to until you reach the string representation of elements. Your output must look something like this:
```
Class: Ljava/TestClass0;
  Static Fields:
      Field: Ljava/TestClass0; I MAX
  Instance Fields:
      Field: Ljava/TestClass0; I age
      Field: Ljava/TestClass0; Ljava/lang/String; name
  Direct Methods:
      Method: Ljava/TestClass0; V V <init>
      Method: Ljava/TestClass0; I I foo
  Virtual Methods:
      Method: Ljava/TestClass0; V V greet
Class: Ljava/TestClass1;
  Static Fields:
      Field: Ljava/TestClass1; I staticCounter
  Instance Fields:
      Field: Ljava/TestClass1; I id
      Field: Ljava/TestClass1; Ljava/lang/String; name
  Direct Methods:
      Method: Ljava/TestClass1; V V <clinit>
      Method: Ljava/TestClass1; V VIL <init>
      Method: Ljava/TestClass1; V V incrementCounter
      Method: Ljava/TestClass1; V V logInternal
  Virtual Methods:
      Method: Ljava/TestClass1; V V displayInfo
      Method: Ljava/TestClass1; Ljava/lang/String; L getName
      Method: Ljava/TestClass1; V V resetName
```
Based on the input java classes:
```java
public class TestClass0 {
    static final int MAX = 1000;

    public static int foo() {
        return MAX;
    }

    String name;
    int age;

    public void greet() {
        System.out.println("Hello, " + name + "!");
    }
}
```

```java
public class TestClass1 {
    // Static field (class variable)
    public static int staticCounter = 0;

    // Instance fields (object variables)
    private int id;
    protected String name;

    // Constructor (direct method)
    public TestClass1(int id, String name) {
        this.id = id;
        this.name = name;
    }

    // Static method (does not depend on an instance)
    public static void incrementCounter() {
        staticCounter++;
    }

    // Instance method (virtual method if not private/final)
    public void displayInfo() {
        System.out.println("ID: " + id + ", Name: " + name);
    }

    // Private method (direct method)
    private void logInternal() {
        System.out.println("Internal log for ID: " + id);
    }

    // Final method (still virtual but not overridable)
    public final String getName() {
        return name;
    }

    // Protected method (virtual method)
    protected void resetName() {
        this.name = "Unnamed";
    }
}
```
Start out by parsing the `ClassDataItem` from the raw dex buffer and iterating through all classes. From this point on, you start manipulating various indexes until you reach the underlying string data and print it out.

Bonus points for printing out the method signatures in a human readable format.

#### DRY
"Don't repeat yourself" (DRY) is a software development principle aimed at reducing the repetition of code patterns. If you've investigated the code and implementations for various structs, you probably noticed that a lot of structs use a common pattern for parsing:
```rust
// src/dex/field_it_item.rs
pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
    if buffer.len() < 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer too small to read FieldIdItem",
        ));
    }
    ...
}

// src/dex/header_item.rs
pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
    if buffer.len() < 112 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer too small to read HeaderItem",
        ));
    }
    ...
}

// src/dex/method_id_item.rs
pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
    if buffer.len() < 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer too small to read MethodIdItem",
        ));
    }
}

// src/dex/proto_id_item.rs
pub fn try_parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
    if buffer.len() < 12 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Buffer too short to parse ProtoIdItem",
        ));
    }
}
```
Wouldn't it be nice to provide an implementation for length checking once and rewrite our code so that it is used automatically for all structs?

You can do this in many ways including macros, traits or even generics. We will use a [trait](https://doc.rust-lang.org/rust-by-example/trait.html) for this. Your job is to write a trait called `TryParseFromBytes` that share this code pattern for checking the known fixed size of the element. Think of were would you start starting the trait (create a new module or use an existing one), and start writing.

##### Final result
What you should end up with is a convenient way of parsing items:
```rust
// src/dex/header_item.rs
impl TryParseFromBytes for HeaderItem {
    const SIZE: usize = 112;  // <-- We provide the size as a constant trait field

    fn parse_from_bytes(buffer: &[u8]) -> Self {
        let header = Self {
            magic: buffer[0..8].try_into().unwrap(),
            checksum: read_u32_le(buffer, 8),
            signature: buffer[12..32].try_into().unwrap(),
            file_size: read_u32_le(buffer, 32),
            header_size: read_u32_le(buffer, 36),
            endian_tag: read_u32_le(buffer, 40),
            link_size: read_u32_le(buffer, 44),
            link_off: read_u32_le(buffer, 48),
            map_off: read_u32_le(buffer, 52),
            string_ids_size: read_u32_le(buffer, 56),
            string_ids_off: read_u32_le(buffer, 60),
            type_ids_size: read_u32_le(buffer, 64),
            type_ids_off: read_u32_le(buffer, 68),
            proto_ids_size: read_u32_le(buffer, 72),
            proto_ids_off: read_u32_le(buffer, 76),
            field_ids_size: read_u32_le(buffer, 80),
            field_ids_off: read_u32_le(buffer, 84),
            method_ids_size: read_u32_le(buffer, 88),
            method_ids_off: read_u32_le(buffer, 92),
            class_defs_size: read_u32_le(buffer, 96),
            class_defs_off: read_u32_le(buffer, 100),
            data_size: read_u32_le(buffer, 104),
            data_off: read_u32_le(buffer, 108),
            containzer_size: 0, // We will ignore this
            header_offset: 0,   // We will ignore this
        };

        header
    }
}
```

As you can see, when using trait functionality, the only thing we need to do is to provide the element size and then we would be able to call the constructor like this:
```rust
let header_item = HeaderItem::try_parse_from_bytes(buffer).unwrap();
```
The `try_parse_from_bytes` will be a default method definition inside the trait itself, which will handle the length checking and call the `parse_from_bytes` method, which will be implemented in each struct that implements the trait. The `parse_from_bytes` method will contain the actual parsing logic for individual items.