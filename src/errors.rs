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

#[derive(Debug, Error)]
pub enum TableIdxError {
    #[error("Invalid strings idx: {0}")]
    String(usize),
    #[error("Invalid types idx: {0}")]
    Type(usize),
    #[error("Invalid field id idx: {0}")]
    FieldId(usize),
    #[error("Invalid method id idx: {0}")]
    MethodId(usize),
    #[error("Invalid proto id idx: {0}")]
    ProtoId(usize),
    #[error("Invalid call site item idx: {0}")]
    CallSite(usize),
    #[error("Invalid method handle idx: {0}")]
    MethodHandle(usize),
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("table index error: {0}")]
    TableIdxError(#[from] TableIdxError),
    #[error("missing label error: {0}")]
    MissingLabel(usize),
}
