use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("Unknown opcode: {0:x}")]
    UnknownOpcode(u8),
    #[error("Instruction buffer is empty")]
    EmptyBuffer,
    #[error("Instruction too short for opcode {opcode:x}")]
    Size {
        opcode: u8,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Error)]
pub enum DexParseError {
    #[error("Invalid size for field `{field}`, expected {expected}, got {actual}")]
    InvalidElementSize {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
}
