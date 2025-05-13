use crate::utils::{read_u16_le, read_u32_le};

use super::instruction::Instruction;

#[derive(Debug)]
pub struct CodeItem {
    /// the number of registers used by this code
    pub registers_size: u16,
    /// the number of words of incoming arguments to the method that this code is for
    pub ins_size: u16,
    /// the number of words of outgoing argument space required by this code for method invocation
    pub outs_size: u16,
    /// the number of `try_items` for this instance. If non-zero, then these appear as the `tries` array just after the insns in this instance.
    pub tries_size: u16,
    /// offset from the start of the file to the debug info (line numbers + local variable info) sequence for this code, or `0` if there simply is no information. The offset, if non-zero, should be to a location in the `data` section. The format of the data is specified by "`debug_info_item`" below.
    pub debug_info_off: u32,
    /// size of the instructions list, in 16-bit code units
    pub insns_size: u32,
    pub insns: Vec<Instruction>,
}

impl CodeItem {
    pub fn try_parse_from_bytes_unsized(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < 16 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small for CodeItem header",
            ));
        }

        let registers_size = read_u16_le(buffer, 0);
        let ins_size = read_u16_le(buffer, 2);
        let outs_size = read_u16_le(buffer, 4);
        let tries_size = read_u16_le(buffer, 6);
        let debug_info_off = read_u32_le(buffer, 8);
        let insns_size = read_u32_le(buffer, 12);

        let insns_bytes = insns_size as usize * 2;

        if buffer.len() < 16 + insns_bytes {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small for CodeItem instructions",
            ));
        }

        let mut insns = Vec::with_capacity(insns_size as usize);
        let mut total_size = 0;
        while total_size < insns_bytes {
            let offset = 16 + total_size;
            let insn = match Instruction::try_decode(&buffer[offset..]) {
                Ok(insn) => insn,
                Err(e) => {
                    println!(
                        "!!!!!!!! Failed to decode instruction at offset {}: {}",
                        offset, e
                    );
                    break;
                }
            };
            total_size += insn.size_bytes();
            insns.push(insn);
        }

        Ok(CodeItem {
            registers_size,
            ins_size,
            outs_size,
            tries_size,
            debug_info_off,
            insns_size,
            insns,
        })
    }
}
