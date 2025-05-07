use crate::utils::{read_u16_le, to_nibbles};

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Move { dst: u8, src: u8 },
    MoveResult { dst: u8 },
    ConstString { dst: u8, string_idx: u16 },
    // ... more instructions
}

impl Instruction {
    pub fn decode(buffer: &[u8]) -> std::io::Result<(Self, usize)> {
        if buffer.len() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small for instruction",
            ));
        }
        let opcode = buffer[0];
        let (inst, length) = match opcode {
            0x00 => (Instruction::Nop, 2),
            0x01 => {
                let (dst, src) = to_nibbles(buffer[1]);
                (Instruction::Move { dst, src }, 2)
            }
            0x0A => (Instruction::MoveResult { dst: buffer[1] }, 2),
            0x1A => {
                if buffer.len() < 4 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Buffer too small for `const-string` instruction",
                    ));
                }
                let dst = buffer[1];
                let string_idx = read_u16_le(buffer, 2);
                (Instruction::ConstString { dst, string_idx }, 4)
            }
            unknown => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Unknown opcode: {:#x}", unknown),
                ));
            }
        };
        Ok((inst, length))
    }
}
