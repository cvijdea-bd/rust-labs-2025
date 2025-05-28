use super::Instruction;

impl Instruction {
    pub fn offset(&self) -> Option<i32> {
        let off = match self {
            Self::Goto { offset } => *offset as i32,
            Self::Goto16 { offset }
            | Self::IfEq { offset, .. }
            | Self::IfNe { offset, .. }
            | Self::IfLt { offset, .. }
            | Self::IfGe { offset, .. }
            | Self::IfGt { offset, .. }
            | Self::IfLe { offset, .. }
            | Self::IfEqz { offset, .. }
            | Self::IfNez { offset, .. }
            | Self::IfLtz { offset, .. }
            | Self::IfGez { offset, .. }
            | Self::IfGtz { offset, .. }
            | Self::IfLez { offset, .. } => *offset as i32,
            Self::Goto32 { offset }
            | Self::PackedSwitch { offset, .. }
            | Self::SparseSwitch { offset, .. }
            | Self::FillArrayData { offset, .. } => *offset,
            _ => return None,
        };
        Some(off)
    }
}
