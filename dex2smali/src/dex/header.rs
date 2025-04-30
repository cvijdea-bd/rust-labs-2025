use crate::utils::read_u32_le;

/// https://source.android.com/docs/core/runtime/dex-format#header-item
#[allow(unused)]
#[derive(Debug)]
pub struct HeaderItem {
    /// magic value. See discussion above under "`DEX_FILE_MAGIC`" for more details.
    pub magic: [u8; 8],
    /// adler32 checksum of the rest of the file (everything but `magic` and this field); used to detect file corruption
    pub checksum: u32,
    /// SHA-1 signature (hash) of the rest of the file (everything but `magic`, checksum, and this field); used to uniquely identify files
    pub signature: [u8; 20],
    /// size of the entire file (including the header), in bytes(v40 or earlier)
    ///  
    /// distance in bytes from the start of this header to the next header or to the end of the whole file (the container). (v41 or later)
    pub file_size: u32,
    /// size of the header (this entire section), in bytes. This allows for at least a limited amount of backwards/forwards compatibility without invalidating the format.
    ///
    /// must be 0x70 (112) bytes(v40 or earlier)
    ///
    /// must be 0x78 (120) bytes (v41 or later)
    pub header_size: u32,
    /// endianness tag. See discussion above under "`ENDIAN_CONSTANT` and `REVERSE_ENDIAN_CONSTANT`" for more details.
    pub endian_tag: u32,
    /// size of the link section, or `0` if this file isn't statically linked
    pub link_size: u32,
    /// offset from the start of the file to the link section, or `0` if `link_size == 0`. The offset, if non-zero, should be to an offset into the `link_data` section. The format of the data pointed at is left unspecified by this document; this header field (and the previous) are left as hooks for use by runtime implementations.
    pub link_off: u32,
    /// offset from the start of the file to the map item. The offset, which must be non-zero, should be to an offset into the `data` section, and the data should be in the format specified by "`map_list`" below.
    pub map_off: u32,
    /// count of strings in the string identifiers list
    pub string_ids_size: u32,
    /// offset from the start of the file to the string identifiers list, or `0` if `string_ids_size == 0` (admittedly a strange edge case). The offset, if non-zero, should be to the start of the `string_ids` section.
    pub string_ids_off: u32,
    /// count of elements in the type identifiers list, at most 65535
    pub type_ids_size: u32,
    /// offset from the start of the file to the type identifiers list, or `0` if `type_ids_size == 0` (admittedly a strange edge case). The offset, if non-zero, should be to the start of the `type_ids` section.
    pub type_ids_off: u32,
    /// count of elements in the prototype identifiers list, at most 65535
    pub proto_ids_size: u32,
    /// offset from the start of the file to the prototype identifiers list, or `0` if `proto_ids_size == 0` (admittedly a strange edge case). The offset, if non-zero, should be to the start of the `proto_ids` section.
    pub proto_ids_off: u32,
    /// count of elements in the field identifiers list
    pub field_ids_size: u32,
    /// offset from the start of the file to the field identifiers list, or `0` if `field_ids_size == 0`. The offset, if non-zero, should be to the start of the `field_ids` section.
    pub field_ids_off: u32,
    /// count of elements in the method identifiers list
    pub method_ids_size: u32,
    /// offset from the start of the file to the method identifiers list, or `0` if `method_ids_size == 0`. The offset, if non-zero, should be to the start of the `method_ids` section.
    pub method_ids_off: u32,
    /// count of elements in the class definitions list
    pub class_defs_size: u32,
    /// offset from the start of the file to the class definitions list, or `0` if `class_defs_size == 0` (admittedly a strange edge case). The offset, if non-zero, should be to the start of the `class_defs` section.
    pub class_defs_off: u32,
    /// Size of data section in bytes. Must be an even multiple of sizeof(uint) (v40 or earlier)
    ///
    /// Unused (v41 or later)
    pub data_size: u32,
    /// offset from the start of the file to the start of the data section(v40 or earlier)
    ///
    /// Unused (v41 or later)
    pub data_off: u32,
    /// this field does not exist. It can be assumed to be equal to file_size. (v40 or earlier)
    ///
    /// size of the entire file (including other dex headers and their data). (v41 or later)
    containzer_size: u32,
    /// this field does not exist. It can be assumed to be equal to 0. (v40 or earlier)
    ///
    /// offset from the start of the file to the start of this header. (v41 or later)
    header_offset: u32,
}

impl HeaderItem {
    pub fn parse_from_bytes(buffer: &[u8]) -> std::io::Result<Self> {
        let buffer = buffer.get(0..112).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Buffer too small to contain Dex header",
        ))?;

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

        Ok(header)
    }
}
