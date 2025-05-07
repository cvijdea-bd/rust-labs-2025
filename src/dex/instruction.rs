use crate::utils::{read_u16_le, to_nibbles};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop,
    Move {
        dst: u8,
        src: u8,
    },
    MoveResult {
        dst: u8,
    },
    ConstString {
        dst: u8,
        string_idx: u16,
    },
    SgetObject {
        dst: u8,
        field_idx: u16,
    },
    InvokeDirect {
        arg_count: u8,   // A
        method_idx: u16, // B
        arg0: u8,        // C
        arg1: u8,        // D
        arg2: u8,        // E
        arg3: u8,        // F
        arg4: u8,        // G
    },
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
            0x62 => {
                if buffer.len() < 4 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Buffer too small for",
                    ));
                }

                let dst = buffer[1];
                let field_idx = read_u16_le(buffer, 2);
                (Instruction::SgetObject { dst, field_idx }, 4)
            }
            0x70 => {
                let ag = buffer[1];
                let bbbb = &buffer[2..=3];
                let dc = buffer[4];
                let fe = buffer[5];

                let (g, a) = to_nibbles(ag); // A|G
                let (c, d) = to_nibbles(dc);
                let (e, f) = to_nibbles(fe);
                let b = read_u16_le(bbbb, 0);

                (
                    Instruction::InvokeDirect {
                        arg_count: a,
                        method_idx: b,
                        arg0: c,
                        arg1: d,
                        arg2: e,
                        arg3: f,
                        arg4: g,
                    },
                    6,
                )
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode_nop() {
        let buffer = [0x00, 0x00];
        let (inst, length) = super::Instruction::decode(&buffer).unwrap();
        assert_eq!(inst, super::Instruction::Nop);
        assert_eq!(length, 2);
    }

    #[test]
    fn test_decode_move() {
        let buffer = [0x01, 0x56];
        let (inst, length) = super::Instruction::decode(&buffer).unwrap();
        assert_eq!(inst, super::Instruction::Move { dst: 6, src: 5 });
        assert_eq!(length, 2);
    }

    #[test]
    fn test_decode_move2() {
        let buffer = [0x01, 0x10];
        let (inst, length) = super::Instruction::decode(&buffer).unwrap();
        assert_eq!(inst, super::Instruction::Move { dst: 0, src: 1 });
        assert_eq!(length, 2);
    }

    #[test]
    fn test_sget_object() {
        let buffer = [0x62, 0x01, 0x0c, 0x00];
        let (inst, length) = super::Instruction::decode(&buffer).unwrap();
        assert_eq!(
            inst,
            super::Instruction::SgetObject {
                dst: 1,
                field_idx: 0xc,
            }
        );
        assert_eq!(length, 4);
    }

    #[test]
    fn test_invoke_direct() {
        let buffer = [0x70, 0x10, 0x08, 0x00, 0x01, 0x00];
        let (inst, length) = super::Instruction::decode(&buffer).unwrap();
        assert_eq!(
            inst,
            super::Instruction::InvokeDirect {
                arg_count: 1,
                method_idx: 8,
                arg0: 1,
                arg1: 0,
                arg2: 0,
                arg3: 0,
                arg4: 0,
            }
        );
        assert_eq!(length, 6);
    }
}
