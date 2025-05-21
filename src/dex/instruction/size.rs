use crate::errors::InstructionError;

use super::Instruction;

impl Instruction {
    pub const fn size_bytes(&self) -> usize {
        match self {
            Instruction::Nop => 2,
            Instruction::Move { .. } => 2,
            Instruction::MoveFrom16 { .. } => 4,
            Instruction::Move16 { .. } => 4,
            Instruction::MoveWide { .. } => 2,
            Instruction::MoveWideFrom16 { .. } => 4,
            Instruction::MoveWide16 { .. } => 4,
            Instruction::MoveObject { .. } => 2,
            Instruction::MoveObjectFrom16 { .. } => 4,
            Instruction::MoveObject16 { .. } => 4,
            Instruction::MoveResult { .. } => 2,
            Instruction::MoveResultWide { .. } => 2,
            Instruction::MoveResultObject { .. } => 2,
            Instruction::MoveException { .. } => 2,
            Instruction::ReturnVoid => 2,
            Instruction::Return { .. } => 2,
            Instruction::ReturnWide { .. } => 2,
            Instruction::ReturnObject { .. } => 2,
            Instruction::Const4 { .. } => 2,
            Instruction::Const16 { .. } => 4,
            Instruction::Const { .. } => 6,
            Instruction::ConstHigh16 { .. } => 4,
            Instruction::ConstWide16 { .. } => 4,
            Instruction::ConstWide32 { .. } => 6,
            Instruction::ConstWide { .. } => 10,
            Instruction::ConstWideHigh16 { .. } => 4,
            Instruction::ConstString { .. } => 4,
            Instruction::ConstStringJumbo { .. } => 6,
            Instruction::ConstClass { .. } => 4,
            Instruction::MonitorEnter { .. } => 2,
            Instruction::MonitorExit { .. } => 2,
            Instruction::CheckCast { .. } => 4,
            Instruction::InstanceOf { .. } => 4,
            Instruction::ArrayLength { .. } => 2,
            Instruction::NewInstance { .. } => 4,
            Instruction::NewArray { .. } => 4,
            Instruction::FilledNewArray { .. } => 6,
            Instruction::FilledNewArrayRange { .. } => 6,
            Instruction::FillArrayData { .. } => 6,
            Instruction::Throw { .. } => 2,
            Instruction::Goto { .. } => 2,
            Instruction::Goto16 { .. } => 4,
            Instruction::Goto32 { .. } => 6,
            Instruction::PackedSwitch { .. } => 6,
            Instruction::SparseSwitch { .. } => 6,
            Instruction::CmplFloat { .. } => 4,
            Instruction::CmpgFloat { .. } => 4,
            Instruction::CmplDouble { .. } => 4,
            Instruction::CmpgDouble { .. } => 4,
            Instruction::CmpLong { .. } => 4,
            Instruction::IfEq { .. } => 4,
            Instruction::IfNe { .. } => 4,
            Instruction::IfLt { .. } => 4,
            Instruction::IfGe { .. } => 4,
            Instruction::IfGt { .. } => 4,
            Instruction::IfLe { .. } => 4,
            Instruction::IfEqz { .. } => 4,
            Instruction::IfNez { .. } => 4,
            Instruction::IfLtz { .. } => 4,
            Instruction::IfGez { .. } => 4,
            Instruction::IfGtz { .. } => 4,
            Instruction::IfLez { .. } => 4,
            Instruction::Aget { .. } => 4,
            Instruction::AgetWide { .. } => 4,
            Instruction::AgetObject { .. } => 4,
            Instruction::AgetBoolean { .. } => 4,
            Instruction::AgetByte { .. } => 4,
            Instruction::AgetChar { .. } => 4,
            Instruction::AgetShort { .. } => 4,
            Instruction::Aput { .. } => 4,
            Instruction::AputWide { .. } => 4,
            Instruction::AputObject { .. } => 4,
            Instruction::AputBoolean { .. } => 4,
            Instruction::AputByte { .. } => 4,
            Instruction::AputChar { .. } => 4,
            Instruction::AputShort { .. } => 4,
            Instruction::Iget { .. } => 4,
            Instruction::IgetWide { .. } => 4,
            Instruction::IgetObject { .. } => 4,
            Instruction::IgetBoolean { .. } => 4,
            Instruction::IgetByte { .. } => 4,
            Instruction::IgetChar { .. } => 4,
            Instruction::IgetShort { .. } => 4,
            Instruction::Iput { .. } => 4,
            Instruction::IputWide { .. } => 4,
            Instruction::IputObject { .. } => 4,
            Instruction::IputBoolean { .. } => 4,
            Instruction::IputByte { .. } => 4,
            Instruction::IputChar { .. } => 4,
            Instruction::IputShort { .. } => 4,
            Instruction::Sget { .. } => 4,
            Instruction::SgetWide { .. } => 4,
            Instruction::SgetObject { .. } => 4,
            Instruction::SgetBoolean { .. } => 4,
            Instruction::SgetByte { .. } => 4,
            Instruction::SgetChar { .. } => 4,
            Instruction::SgetShort { .. } => 4,
            Instruction::Sput { .. } => 4,
            Instruction::SputWide { .. } => 4,
            Instruction::SputObject { .. } => 4,
            Instruction::SputBoolean { .. } => 4,
            Instruction::SputByte { .. } => 4,
            Instruction::SputChar { .. } => 4,
            Instruction::SputShort { .. } => 4,
            Instruction::InvokeVirtual { .. } => 6,
            Instruction::InvokeSuper { .. } => 6,
            Instruction::InvokeDirect { .. } => 6,
            Instruction::InvokeStatic { .. } => 6,
            Instruction::InvokeInterface { .. } => 6,
            Instruction::InvokeVirtualRange { .. } => 6,
            Instruction::InvokeSuperRange { .. } => 6,
            Instruction::InvokeDirectRange { .. } => 6,
            Instruction::InvokeStaticRange { .. } => 6,
            Instruction::InvokeInterfaceRange { .. } => 6,
            Instruction::NegInt { .. } => 2,
            Instruction::NotInt { .. } => 2,
            Instruction::NegLong { .. } => 2,
            Instruction::NotLong { .. } => 2,
            Instruction::NegFloat { .. } => 2,
            Instruction::NegDouble { .. } => 2,
            Instruction::IntToLong { .. } => 2,
            Instruction::IntToFloat { .. } => 2,
            Instruction::IntToDouble { .. } => 2,
            Instruction::LongToInt { .. } => 2,
            Instruction::LongToFloat { .. } => 2,
            Instruction::LongToDouble { .. } => 2,
            Instruction::FloatToInt { .. } => 2,
            Instruction::FloatToLong { .. } => 2,
            Instruction::FloatToDouble { .. } => 2,
            Instruction::DoubleToInt { .. } => 2,
            Instruction::DoubleToLong { .. } => 2,
            Instruction::DoubleToFloat { .. } => 2,
            Instruction::IntToByte { .. } => 2,
            Instruction::IntToChar { .. } => 2,
            Instruction::IntToShort { .. } => 2,
            Instruction::AddInt { .. } => 4,
            Instruction::SubInt { .. } => 4,
            Instruction::MulInt { .. } => 4,
            Instruction::DivInt { .. } => 4,
            Instruction::RemInt { .. } => 4,
            Instruction::AndInt { .. } => 4,
            Instruction::OrInt { .. } => 4,
            Instruction::XorInt { .. } => 4,
            Instruction::ShlInt { .. } => 4,
            Instruction::ShrInt { .. } => 4,
            Instruction::UShrInt { .. } => 4,
            Instruction::AddLong { .. } => 4,
            Instruction::SubLong { .. } => 4,
            Instruction::MulLong { .. } => 4,
            Instruction::DivLong { .. } => 4,
            Instruction::RemLong { .. } => 4,
            Instruction::AndLong { .. } => 4,
            Instruction::OrLong { .. } => 4,
            Instruction::XorLong { .. } => 4,
            Instruction::ShlLong { .. } => 4,
            Instruction::ShrLong { .. } => 4,
            Instruction::UShrLong { .. } => 4,
            Instruction::AddFloat { .. } => 4,
            Instruction::SubFloat { .. } => 4,
            Instruction::MulFloat { .. } => 4,
            Instruction::DivFloat { .. } => 4,
            Instruction::RemFloat { .. } => 4,
            Instruction::AddDouble { .. } => 4,
            Instruction::SubDouble { .. } => 4,
            Instruction::MulDouble { .. } => 4,
            Instruction::DivDouble { .. } => 4,
            Instruction::RemDouble { .. } => 4,
            Instruction::AddInt2Addr { .. } => 2,
            Instruction::SubInt2Addr { .. } => 2,
            Instruction::MulInt2Addr { .. } => 2,
            Instruction::DivInt2Addr { .. } => 2,
            Instruction::RemInt2Addr { .. } => 2,
            Instruction::AndInt2Addr { .. } => 2,
            Instruction::OrInt2Addr { .. } => 2,
            Instruction::XorInt2Addr { .. } => 2,
            Instruction::ShlInt2Addr { .. } => 2,
            Instruction::ShrInt2Addr { .. } => 2,
            Instruction::UShrInt2Addr { .. } => 2,
            Instruction::AddLong2Addr { .. } => 2,
            Instruction::SubLong2Addr { .. } => 2,
            Instruction::MulLong2Addr { .. } => 2,
            Instruction::DivLong2Addr { .. } => 2,
            Instruction::RemLong2Addr { .. } => 2,
            Instruction::AndLong2Addr { .. } => 2,
            Instruction::OrLong2Addr { .. } => 2,
            Instruction::XorLong2Addr { .. } => 2,
            Instruction::ShlLong2Addr { .. } => 2,
            Instruction::ShrLong2Addr { .. } => 2,
            Instruction::UShrLong2Addr { .. } => 2,
            Instruction::AddFloat2Addr { .. } => 2,
            Instruction::SubFloat2Addr { .. } => 2,
            Instruction::MulFloat2Addr { .. } => 2,
            Instruction::DivFloat2Addr { .. } => 2,
            Instruction::RemFloat2Addr { .. } => 2,
            Instruction::AddDouble2Addr { .. } => 2,
            Instruction::SubDouble2Addr { .. } => 2,
            Instruction::MulDouble2Addr { .. } => 2,
            Instruction::DivDouble2Addr { .. } => 2,
            Instruction::RemDouble2Addr { .. } => 2,
            Instruction::AddIntLit16 { .. } => 4,
            Instruction::RsubInt { .. } => 4,
            Instruction::MulIntLit16 { .. } => 4,
            Instruction::DivIntLit16 { .. } => 4,
            Instruction::RemIntLit16 { .. } => 4,
            Instruction::AndIntLit16 { .. } => 4,
            Instruction::OrIntLit16 { .. } => 4,
            Instruction::XorIntLit16 { .. } => 4,
            Instruction::AddIntLit8 { .. } => 4,
            Instruction::RsubIntLit8 { .. } => 4,
            Instruction::MulIntLit8 { .. } => 4,
            Instruction::DivIntLit8 { .. } => 4,
            Instruction::RemIntLit8 { .. } => 4,
            Instruction::AndIntLit8 { .. } => 4,
            Instruction::OrIntLit8 { .. } => 4,
            Instruction::XorIntLit8 { .. } => 4,
            Instruction::ShlIntLit8 { .. } => 4,
            Instruction::ShrIntLit8 { .. } => 4,
            Instruction::UShrIntLit8 { .. } => 4,
            Instruction::InvokePolymorphic { .. } => 8,
            Instruction::InvokePolymorphicRange { .. } => 8,
            Instruction::InvokeCustom { .. } => 6,
            Instruction::InvokeCustomRange { .. } => 6,
            Instruction::ConstMethodHandle { .. } => 4,
            Instruction::ConstMethodType { .. } => 4,
        }
    }
}

pub const fn instruction_size_bytes(opcode: u8) -> Result<usize, InstructionError> {
    match opcode {
        0x00
        | 0x0E
        | 0x21
        | 0x28
        | 0x0A..=0x0D
        | 0x0F..=0x11
        | 0x1D..=0x1E
        | 0x27
        | 0x12
        | 0x01
        | 0x04
        | 0x07
        | (0x7B..=0x8F)
        | (0xB0..=0xCF) => Ok(2),

        0x22
        | 0x29
        | 0x1A
        | 0x1C
        | (0x1F..=0x1F)
        | (0x60..=0x6D)
        | 0xFE
        | 0xFF
        | 0x15
        | 0x19
        | 0x13
        | 0x16
        | 0x38..=0x3D
        | 0xD8..=0xE2
        | 0x20
        | 0x23
        | (0x52..=0x5F)
        | 0xD0..=0xD7
        | 0x32..=0x37
        | 0x02
        | 0x05
        | 0x08
        | 0x2D..=0x31
        | (0x44..=0x51)
        | (0x90..=0xAF) => Ok(4),

        0x2A
        | 0x1B
        | 0x14
        | 0x17
        | 0x26
        | 0x2B
        | 0x2C
        | 0x03
        | 0x06
        | 0x09
        | 0x24
        | (0x6E..=0x72)
        | 0xFC
        | 0x25
        | (0x74..=0x78)
        | 0xFD => Ok(6),

        0xFA | 0xFB => Ok(8),

        0x18 => Ok(10),

        _ => Err(InstructionError::UnknownOpcode(opcode)),
    }
}
