use crate::{
    errors::InstructionError,
    utils::{read_u16_le, read_u32_le, read_u64_le, to_nibbles},
};

use super::{size::instruction_size_bytes, Instruction};

impl Instruction {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, InstructionError> {
        if buffer.is_empty() {
            return Err(InstructionError::EmptyBuffer);
        }

        let opcode = buffer[0];
        let expected = instruction_size_bytes(opcode)?;

        if buffer.len() < expected {
            return Err(InstructionError::Size {
                opcode,
                expected,
                actual: buffer.len(),
            });
        }

        let inst = match opcode {
            // 00-0D: Basic operations
            0x00 => Instruction::Nop,
            0x01 => {
                let (dst, src) = to_nibbles(buffer[1]);
                Instruction::Move { dst, src }
            }
            0x02 => {
                let dst = buffer[1];
                let src = read_u16_le(buffer, 2);
                Instruction::MoveFrom16 { dst, src }
            }
            0x03 => {
                let dst = read_u16_le(buffer, 2);
                let src = read_u16_le(buffer, 4);
                Instruction::Move16 { dst, src }
            }
            0x04 => {
                let (dst, src) = to_nibbles(buffer[1]);
                Instruction::MoveWide { dst, src }
            }
            0x05 => {
                let dst = buffer[1];
                let src = read_u16_le(buffer, 2);
                Instruction::MoveWideFrom16 { dst, src }
            }
            0x06 => {
                let dst = read_u16_le(buffer, 2);
                let src = read_u16_le(buffer, 4);
                Instruction::MoveWide16 { dst, src }
            }
            0x07 => {
                let (dst, src) = to_nibbles(buffer[1]);
                Instruction::MoveObject { dst, src }
            }
            0x08 => {
                let dst = buffer[1];
                let src = read_u16_le(buffer, 2);
                Instruction::MoveObjectFrom16 { dst, src }
            }
            0x09 => {
                let dst = read_u16_le(buffer, 2);
                let src = read_u16_le(buffer, 4);
                Instruction::MoveObject16 { dst, src }
            }
            0x0A => Instruction::MoveResult { dst: buffer[1] },
            0x0B => Instruction::MoveResultWide { dst: buffer[1] },
            0x0C => Instruction::MoveResultObject { dst: buffer[1] },
            0x0D => Instruction::MoveException { dst: buffer[1] },

            // 0E-11: Returns
            0x0E => Instruction::ReturnVoid,
            0x0F => Instruction::Return { value: buffer[1] },
            0x10 => Instruction::ReturnWide { value: buffer[1] },
            0x11 => Instruction::ReturnObject { value: buffer[1] },

            // 12-1F: Constants and checks
            0x12 => {
                let (dst, value) = to_nibbles(buffer[1]);
                Instruction::Const4 {
                    dst,
                    value: value as i8,
                }
            }
            0x13 => {
                let dst = buffer[1];
                let value = read_u16_le(buffer, 2) as i16;
                Instruction::Const16 { dst, value }
            }
            0x14 => {
                let dst = buffer[1];
                let value = read_u32_le(buffer, 2) as i32;
                Instruction::Const { dst, value }
            }
            0x15 => {
                let dst = buffer[1];
                let value = read_u16_le(buffer, 2) as i16;
                Instruction::ConstHigh16 { dst, value }
            }
            0x16 => {
                let dst = buffer[1];
                let value = read_u16_le(buffer, 2) as i16;
                Instruction::ConstWide16 { dst, value }
            }
            0x17 => {
                let dst = buffer[1];
                let value = read_u32_le(buffer, 2) as i32;
                Instruction::ConstWide32 { dst, value }
            }
            0x18 => {
                let dst = buffer[1];
                let value = read_u64_le(buffer, 2) as i64;
                Instruction::ConstWide { dst, value }
            }
            0x19 => {
                let dst = buffer[1];
                let value = read_u16_le(buffer, 2) as i16;
                Instruction::ConstWideHigh16 { dst, value }
            }
            0x1A => {
                let dst = buffer[1];
                let string_idx = read_u16_le(buffer, 2);
                Instruction::ConstString { dst, string_idx }
            }
            0x1B => {
                let dst = buffer[1];
                let string_idx = read_u32_le(buffer, 2);
                Instruction::ConstStringJumbo { dst, string_idx }
            }
            0x1C => {
                let dst = buffer[1];
                let type_idx = read_u16_le(buffer, 2);
                Instruction::ConstClass { dst, type_idx }
            }
            0x1D => Instruction::MonitorEnter {
                reference: buffer[1],
            },
            0x1E => Instruction::MonitorExit {
                reference: buffer[1],
            },
            0x1F => {
                let reference = buffer[1];
                let type_idx = read_u16_le(buffer, 2);
                Instruction::CheckCast {
                    reference,
                    type_idx,
                }
            }
            0x20 => {
                let (dst, reference) = to_nibbles(buffer[1]);
                let type_idx = read_u16_le(buffer, 2);
                Instruction::InstanceOf {
                    dst,
                    reference,
                    type_idx,
                }
            }

            // 21-2A: Arrays and jumps
            0x21 => {
                let (dst, array) = to_nibbles(buffer[1]);
                Instruction::ArrayLength { dst, array }
            }
            0x22 => {
                let dst = buffer[1];
                let type_idx = read_u16_le(buffer, 2);
                Instruction::NewInstance { dst, type_idx }
            }
            0x23 => {
                let (dst, size) = to_nibbles(buffer[1]);
                let type_idx = read_u16_le(buffer, 2);
                Instruction::NewArray {
                    dst,
                    size,
                    type_idx,
                }
            }
            0x24 => {
                let (g, a) = to_nibbles(buffer[1]);
                let type_idx = read_u16_le(buffer, 2);
                let (e, f) = to_nibbles(buffer[4]);
                let (c, d) = to_nibbles(buffer[5]);
                Instruction::FilledNewArray {
                    type_idx,
                    args: [c, d, e, f, g],
                    arg_cnt: a,
                }
            }
            0x25 => {
                let arg_cnt = buffer[1];
                let type_idx = read_u16_le(buffer, 2);
                let first_arg = read_u16_le(buffer, 4);
                Instruction::FilledNewArrayRange {
                    type_idx,
                    first_arg,
                    arg_cnt,
                }
            }
            0x26 => {
                let array = buffer[1];
                let offset = read_u32_le(buffer, 2) as i32;
                Instruction::FillArrayData { array, offset }
            }
            0x27 => Instruction::Throw {
                exception: buffer[1],
            },
            0x28 => Instruction::Goto {
                offset: buffer[1] as i8,
            },
            0x29 => {
                let offset = read_u16_le(buffer, 2) as i16;
                Instruction::Goto16 { offset }
            }
            0x2A => {
                let offset = read_u32_le(buffer, 2) as i32;
                Instruction::Goto32 { offset }
            }
            0x2B => {
                let value = buffer[1];
                let offset = read_u32_le(buffer, 2) as i32;
                Instruction::PackedSwitch { value, offset }
            }
            0x2C => {
                let value = buffer[1];
                let offset = read_u32_le(buffer, 2) as i32;
                Instruction::SparseSwitch { value, offset }
            }

            // 2D-37: Comparisons and branches
            0x2D..=0x31 => {
                let dst = buffer[1];
                let src_a = buffer[2];
                let src_b = buffer[3];
                match opcode {
                    0x2D => Instruction::CmplFloat { dst, src_a, src_b },
                    0x2E => Instruction::CmpgFloat { dst, src_a, src_b },
                    0x2F => Instruction::CmplDouble { dst, src_a, src_b },
                    0x30 => Instruction::CmpgDouble { dst, src_a, src_b },
                    0x31 => Instruction::CmpLong { dst, src_a, src_b },
                    _ => unreachable!(),
                }
            }
            // 0x32..=0x37: if-test
            0x32..=0x37 => {
                let (a, b) = to_nibbles(buffer[1]);
                let offset = read_u16_le(buffer, 2) as i16;

                match opcode {
                    0x32 => Instruction::IfEq { a, b, offset },
                    0x33 => Instruction::IfNe { a, b, offset },
                    0x34 => Instruction::IfLt { a, b, offset },
                    0x35 => Instruction::IfGe { a, b, offset },
                    0x36 => Instruction::IfGt { a, b, offset },
                    0x37 => Instruction::IfLe { a, b, offset },
                    _ => unreachable!(),
                }
            }
            0x38..=0x3D => {
                let value = buffer[1];
                let offset = read_u16_le(buffer, 2) as i16;

                match opcode {
                    0x38 => Instruction::IfEqz { value, offset },
                    0x39 => Instruction::IfNez { value, offset },
                    0x3A => Instruction::IfLtz { value, offset },
                    0x3B => Instruction::IfGez { value, offset },
                    0x3C => Instruction::IfGtz { value, offset },
                    0x3D => Instruction::IfLez { value, offset },
                    _ => unreachable!(),
                }
            }
            0x44..=0x51 => {
                let value = buffer[1];
                let array = buffer[2];
                let index = buffer[3];
                match opcode {
                    0x44 => Instruction::Aget {
                        src: value,
                        array,
                        index,
                    },
                    0x45 => Instruction::AgetWide {
                        src: value,
                        array,
                        index,
                    },
                    0x46 => Instruction::AgetObject {
                        src: value,
                        array,
                        index,
                    },
                    0x47 => Instruction::AgetBoolean {
                        src: value,
                        array,
                        index,
                    },
                    0x48 => Instruction::AgetByte {
                        src: value,
                        array,
                        index,
                    },
                    0x49 => Instruction::AgetChar {
                        src: value,
                        array,
                        index,
                    },
                    0x4A => Instruction::AgetShort {
                        src: value,
                        array,
                        index,
                    },
                    0x4B => Instruction::Aput {
                        dst: value,
                        array,
                        index,
                    },
                    0x4C => Instruction::AputWide {
                        dst: value,
                        array,
                        index,
                    },
                    0x4D => Instruction::AputObject {
                        dst: value,
                        array,
                        index,
                    },
                    0x4E => Instruction::AputBoolean {
                        dst: value,
                        array,
                        index,
                    },
                    0x4F => Instruction::AputByte {
                        dst: value,
                        array,
                        index,
                    },
                    0x50 => Instruction::AputChar {
                        dst: value,
                        array,
                        index,
                    },
                    0x51 => Instruction::AputShort {
                        dst: value,
                        array,
                        index,
                    },
                    _ => unreachable!(),
                }
            }
            0x52..=0x5F => {
                let (value, object) = to_nibbles(buffer[1]);
                let field_idx = read_u16_le(buffer, 2);
                match opcode {
                    0x52 => Instruction::Iget {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x53 => Instruction::IgetWide {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x54 => Instruction::IgetObject {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x55 => Instruction::IgetBoolean {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x56 => Instruction::IgetByte {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x57 => Instruction::IgetChar {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x58 => Instruction::IgetShort {
                        src: value,
                        object,
                        field_idx,
                    },
                    0x59 => Instruction::Iput {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5A => Instruction::IputWide {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5B => Instruction::IputObject {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5C => Instruction::IputBoolean {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5D => Instruction::IputByte {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5E => Instruction::IputChar {
                        dst: value,
                        object,
                        field_idx,
                    },
                    0x5F => Instruction::IputShort {
                        dst: value,
                        object,
                        field_idx,
                    },
                    _ => unreachable!(),
                }
            }
            0x60..=0x6D => {
                let value = buffer[1];
                let field_idx = read_u16_le(buffer, 2);
                match opcode {
                    0x60 => Instruction::Sget {
                        src: value,
                        field_idx,
                    },
                    0x61 => Instruction::SgetWide {
                        src: value,
                        field_idx,
                    },
                    0x62 => Instruction::SgetObject {
                        src: value,
                        field_idx,
                    },
                    0x63 => Instruction::SgetBoolean {
                        src: value,
                        field_idx,
                    },
                    0x64 => Instruction::SgetByte {
                        src: value,
                        field_idx,
                    },
                    0x65 => Instruction::SgetChar {
                        src: value,
                        field_idx,
                    },
                    0x66 => Instruction::SgetShort {
                        src: value,
                        field_idx,
                    },
                    0x67 => Instruction::Sput {
                        dst: value,
                        field_idx,
                    },
                    0x68 => Instruction::SputWide {
                        dst: value,
                        field_idx,
                    },
                    0x69 => Instruction::SputObject {
                        dst: value,
                        field_idx,
                    },
                    0x6A => Instruction::SputBoolean {
                        dst: value,
                        field_idx,
                    },
                    0x6B => Instruction::SputByte {
                        dst: value,
                        field_idx,
                    },
                    0x6C => Instruction::SputChar {
                        dst: value,
                        field_idx,
                    },
                    0x6D => Instruction::SputShort {
                        dst: value,
                        field_idx,
                    },
                    _ => unreachable!(),
                }
            }
            0x6E..=0x72 => {
                let (g, arg_cnt) = to_nibbles(buffer[1]);
                let method_idx = read_u16_le(buffer, 2);
                let (e, f) = to_nibbles(buffer[4]);
                let (c, d) = to_nibbles(buffer[5]);
                let args = [c, d, e, f, g];
                match opcode {
                    0x6E => Instruction::InvokeVirtual {
                        method_idx,
                        args,
                        arg_cnt,
                    },
                    0x6F => Instruction::InvokeSuper {
                        method_idx,
                        args,
                        arg_cnt,
                    },
                    0x70 => Instruction::InvokeDirect {
                        method_idx,
                        args,
                        arg_cnt,
                    },
                    0x71 => Instruction::InvokeStatic {
                        method_idx,
                        args,
                        arg_cnt,
                    },
                    0x72 => Instruction::InvokeInterface {
                        method_idx,
                        args,
                        arg_cnt,
                    },
                    _ => unreachable!(),
                }
            }
            0x74..=0x78 => {
                let arg_cnt = buffer[1];
                let method_idx = read_u16_le(buffer, 2);
                let first_arg = read_u16_le(buffer, 4);
                match opcode {
                    0x74 => Instruction::InvokeVirtualRange {
                        method_idx,
                        first_arg,
                        arg_cnt,
                    },
                    0x75 => Instruction::InvokeSuperRange {
                        method_idx,
                        first_arg,
                        arg_cnt,
                    },
                    0x76 => Instruction::InvokeDirectRange {
                        method_idx,
                        first_arg,
                        arg_cnt,
                    },
                    0x77 => Instruction::InvokeStaticRange {
                        method_idx,
                        first_arg,
                        arg_cnt,
                    },
                    0x78 => Instruction::InvokeInterfaceRange {
                        method_idx,
                        first_arg,
                        arg_cnt,
                    },
                    _ => unreachable!(),
                }
            }
            0x7B..=0x8F => {
                let (dst, src) = to_nibbles(buffer[1]);
                match opcode {
                    0x7b => Instruction::NegInt { dst, src },
                    0x7c => Instruction::NotInt { dst, src },
                    0x7d => Instruction::NegLong { dst, src },
                    0x7e => Instruction::NotLong { dst, src },
                    0x7f => Instruction::NegFloat { dst, src },
                    0x80 => Instruction::NegDouble { dst, src },
                    0x81 => Instruction::IntToLong { dst, src },
                    0x82 => Instruction::IntToFloat { dst, src },
                    0x83 => Instruction::IntToDouble { dst, src },
                    0x84 => Instruction::LongToInt { dst, src },
                    0x85 => Instruction::LongToFloat { dst, src },
                    0x86 => Instruction::LongToDouble { dst, src },
                    0x87 => Instruction::FloatToInt { dst, src },
                    0x88 => Instruction::FloatToLong { dst, src },
                    0x89 => Instruction::FloatToDouble { dst, src },
                    0x8A => Instruction::DoubleToInt { dst, src },
                    0x8B => Instruction::DoubleToLong { dst, src },
                    0x8C => Instruction::DoubleToFloat { dst, src },
                    0x8D => Instruction::IntToByte { dst, src },
                    0x8E => Instruction::IntToChar { dst, src },
                    0x8F => Instruction::IntToShort { dst, src },
                    _ => unreachable!(),
                }
            }
            0x90..=0xAF => {
                let dst = buffer[1];
                let src_a = buffer[2];
                let src_b = buffer[3];
                match opcode {
                    0x90 => Instruction::AddInt { dst, src_a, src_b },
                    0x91 => Instruction::SubInt { dst, src_a, src_b },
                    0x92 => Instruction::MulInt { dst, src_a, src_b },
                    0x93 => Instruction::DivInt { dst, src_a, src_b },
                    0x94 => Instruction::RemInt { dst, src_a, src_b },
                    0x95 => Instruction::AndInt { dst, src_a, src_b },
                    0x96 => Instruction::OrInt { dst, src_a, src_b },
                    0x97 => Instruction::XorInt { dst, src_a, src_b },
                    0x98 => Instruction::ShlInt { dst, src_a, src_b },
                    0x99 => Instruction::ShrInt { dst, src_a, src_b },
                    0x9A => Instruction::UshrInt { dst, src_a, src_b },
                    0x9B => Instruction::AddLong { dst, src_a, src_b },
                    0x9C => Instruction::SubLong { dst, src_a, src_b },
                    0x9D => Instruction::MulLong { dst, src_a, src_b },
                    0x9E => Instruction::DivLong { dst, src_a, src_b },
                    0x9F => Instruction::RemLong { dst, src_a, src_b },
                    0xA0 => Instruction::AndLong { dst, src_a, src_b },
                    0xA1 => Instruction::OrLong { dst, src_a, src_b },
                    0xA2 => Instruction::XorLong { dst, src_a, src_b },
                    0xA3 => Instruction::ShlLong { dst, src_a, src_b },
                    0xA4 => Instruction::ShrLong { dst, src_a, src_b },
                    0xA5 => Instruction::UshrLong { dst, src_a, src_b },
                    0xA6 => Instruction::AddFloat { dst, src_a, src_b },
                    0xA7 => Instruction::SubFloat { dst, src_a, src_b },
                    0xA8 => Instruction::MulFloat { dst, src_a, src_b },
                    0xA9 => Instruction::DivFloat { dst, src_a, src_b },
                    0xAA => Instruction::RemFloat { dst, src_a, src_b },
                    0xAB => Instruction::AddDouble { dst, src_a, src_b },
                    0xAC => Instruction::SubDouble { dst, src_a, src_b },
                    0xAD => Instruction::MulDouble { dst, src_a, src_b },
                    0xAE => Instruction::DivDouble { dst, src_a, src_b },
                    0xAF => Instruction::RemDouble { dst, src_a, src_b },
                    _ => unreachable!(),
                }
            }
            0xB0..=0xCF => {
                let (dst, src) = to_nibbles(buffer[1]);
                match opcode {
                    0xB0 => Instruction::AddInt2Addr { dst, src },
                    0xB1 => Instruction::SubInt2Addr { dst, src },
                    0xB2 => Instruction::MulInt2Addr { dst, src },
                    0xB3 => Instruction::DivInt2Addr { dst, src },
                    0xB4 => Instruction::RemInt2Addr { dst, src },
                    0xB5 => Instruction::AndInt2Addr { dst, src },
                    0xB6 => Instruction::OrInt2Addr { dst, src },
                    0xB7 => Instruction::XorInt2Addr { dst, src },
                    0xB8 => Instruction::ShlInt2Addr { dst, src },
                    0xB9 => Instruction::ShrInt2Addr { dst, src },
                    0xBA => Instruction::UshrInt2Addr { dst, src },
                    0xBB => Instruction::AddLong2Addr { dst, src },
                    0xBC => Instruction::SubLong2Addr { dst, src },
                    0xBD => Instruction::MulLong2Addr { dst, src },
                    0xBE => Instruction::DivLong2Addr { dst, src },
                    0xBF => Instruction::RemLong2Addr { dst, src },
                    0xC0 => Instruction::AndLong2Addr { dst, src },
                    0xC1 => Instruction::OrLong2Addr { dst, src },
                    0xC2 => Instruction::XorLong2Addr { dst, src },
                    0xC3 => Instruction::ShlLong2Addr { dst, src },
                    0xC4 => Instruction::ShrLong2Addr { dst, src },
                    0xC5 => Instruction::UshrLong2Addr { dst, src },
                    0xC6 => Instruction::AddFloat2Addr { dst, src },
                    0xC7 => Instruction::SubFloat2Addr { dst, src },
                    0xC8 => Instruction::MulFloat2Addr { dst, src },
                    0xC9 => Instruction::DivFloat2Addr { dst, src },
                    0xCA => Instruction::RemFloat2Addr { dst, src },
                    0xCB => Instruction::AddDouble2Addr { dst, src },
                    0xCC => Instruction::SubDouble2Addr { dst, src },
                    0xCD => Instruction::MulDouble2Addr { dst, src },
                    0xCE => Instruction::DivDouble2Addr { dst, src },
                    0xCF => Instruction::RemDouble2Addr { dst, src },
                    _ => unreachable!(),
                }
            }
            0xD0..=0xD7 => {
                let (dst, src) = to_nibbles(buffer[1]);
                let value = read_u16_le(buffer, 2) as i16;
                match opcode {
                    0xD0 => Instruction::AddIntLit16 { dst, src, value },
                    0xD1 => Instruction::RsubInt { dst, src, value },
                    0xD2 => Instruction::MulIntLit16 { dst, src, value },
                    0xD3 => Instruction::DivIntLit16 { dst, src, value },
                    0xD4 => Instruction::RemIntLit16 { dst, src, value },
                    0xD5 => Instruction::AndIntLit16 { dst, src, value },
                    0xD6 => Instruction::OrIntLit16 { dst, src, value },
                    0xD7 => Instruction::XorIntLit16 { dst, src, value },
                    _ => unreachable!(),
                }
            }
            0xD8..=0xE2 => {
                let dst = buffer[1];
                let src = buffer[2];
                let value = buffer[3] as i8;
                match opcode {
                    0xD8 => Instruction::AddIntLit8 { dst, src, value },
                    0xD9 => Instruction::RsubIntLit8 { dst, src, value },
                    0xDA => Instruction::MulIntLit8 { dst, src, value },
                    0xDB => Instruction::DivIntLit8 { dst, src, value },
                    0xDC => Instruction::RemIntLit8 { dst, src, value },
                    0xDD => Instruction::AndIntLit8 { dst, src, value },
                    0xDE => Instruction::OrIntLit8 { dst, src, value },
                    0xDF => Instruction::XorIntLit8 { dst, src, value },
                    0xE0 => Instruction::ShlIntLit8 { dst, src, value },
                    0xE1 => Instruction::ShrIntLit8 { dst, src, value },
                    0xE2 => Instruction::UshrIntLit8 { dst, src, value },
                    _ => unreachable!(),
                }
            }
            0xFA => {
                let (g, arg_cnt) = to_nibbles(buffer[1]);
                let method_idx = read_u16_le(buffer, 2);
                let (e, f) = to_nibbles(buffer[4]);
                let (c, d) = to_nibbles(buffer[5]);
                let proto_idx = read_u16_le(buffer, 6);
                let args = [c, d, e, f, g];
                Instruction::InvokePolymorphic {
                    method_idx,
                    proto_idx,
                    args,
                    arg_cnt,
                }
            }
            0xFB => {
                let arg_cnt = buffer[1];
                let method_idx = read_u16_le(buffer, 2);
                let first_arg = read_u16_le(buffer, 4);
                let proto_idx = read_u16_le(buffer, 6);
                Instruction::InvokePolymorphicRange {
                    method_idx,
                    proto_idx,
                    first_arg,
                    arg_cnt,
                }
            }
            0xFC => {
                let (g, arg_cnt) = to_nibbles(buffer[1]);
                let call_site_idx = read_u16_le(buffer, 2);
                let (e, f) = to_nibbles(buffer[4]);
                let (c, d) = to_nibbles(buffer[5]);
                let args = [c, d, e, f, g];
                Instruction::InvokeCustom {
                    call_site_idx,
                    args,
                    arg_cnt,
                }
            }
            0xFD => {
                let arg_cnt = buffer[1];
                let call_site_idx = read_u16_le(buffer, 2);
                let first_arg = read_u16_le(buffer, 4);
                Instruction::InvokeCustomRange {
                    call_site_idx,
                    first_arg,
                    arg_cnt,
                }
            }
            0xFE => {
                let dst = buffer[1];
                let method_handle_idx = read_u16_le(buffer, 2);
                Instruction::ConstMethodHandle {
                    dst,
                    method_handle_idx,
                }
            }
            0xFF => {
                let dst = buffer[1];
                let proto_idx = read_u16_le(buffer, 2);
                Instruction::ConstMethodType { dst, proto_idx }
            }
            unknown => {
                return Err(InstructionError::UnknownOpcode(unknown));
            }
        };
        Ok(inst)
    }
}
