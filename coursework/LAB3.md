# Lab 03 - Dalvik Bytecode and Instruction Formats
Now that we've built a parser for the DEX file format and can extract class definitions, methods, and strings, it's time to dive into the actual bytecode instructions that make up Android applications. In this lab, we'll:
1. Understand Dalvik bytecode instruction formats
2. Implement a disassembler for Dalvik instructions
3. Analyze method implementations in DEX files

## Understanding Dalvik Bytecode
Dalvik bytecode is a register-based instruction set (unlike Java bytecode which is stack-based). Each instruction consists of:
1. An 8-bit opcode
2. Optional operands (registers, literals, method/field references, etc.)
3. Various size operands (16-bit, 32-bit, etc.)

#### Instruction Format Notation
Dalvik instructions follow a consistent naming pattern that encodes information about the instruction:
```
op vA, vB, kind@C
```
Where:
* `op` is a human-readable opcode name
* `vA`, `vB` are register operands
* `kind@C` is a literal or reference operand (type depends on instruction)

Instruction example:
```
const-string[op] vA[v], string@0000[kind,index]
// Puts reference to string@0000 (entry #0 in the string table) into vA.
```
The code for this instruction would be `1A08 0000` (in hex). Where:
* `1A` is the opcode for `const-string`
* `08` is the register number (vA)
* `0000` is the index of the string in the string table

You can also [look up](https://source.android.com/docs/core/runtime/instruction-formats) the instruction format for each individual opcode. The instruction format is a 3-4 character string: `12x`, `21c`, etc. that describes the instruction's structure:
* `12x` - **`op`** vA, vB
    * `1` - instruction length is 1 byte
    * `2` - number of registers used
    * `x` - no additional data
* `22s` - **`op`** vA, vB, #+CCCC
    * `2` - instruction length is 2 bytes
    * `2` - number of registers used
    * `s` - immediate signed short
* `35c` - **`[A=5]` `op`** {vC, vD, vE, vF, vG} ... **`[A=1]` `op`** {vC} `kind`@BBBB; **`[A=0]` `op`** {}, `kind`@BBBB
    * `3` - instruction length is 3 bytes
    * `5` - (max) number of registers used
    * `c` - constant pool index

#### Implementing the Disassembler
We'll extend our DEX parser to include bytecode disassembly. Here's the structure we'll add:
```
dex2smali
├── src
│   ├── dex
│   │   ├── code_item.rs        # New - contains method bytecode
│   │   ├── instruction.rs      # New - instruction parsing
│   │   ├── mod.rs
│   │   └── ... (existing files)
```

1. Parsing Code Items
First, let's implement the `CodeItem` struct that contains the actual bytecode for a method:
```rust
// src/dex/code_item.rs
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
            let (insn, length) = Instruction::decode(&buffer[offset..])?;
            insns.push(insn);
            total_size += length;
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
```

2. Instruction Representation
Next, we'll create an Instruction enum that can represent all Dalvik instructions:
```rust
// src/dex/instruction.rs
use crate::utils::{read_u16_le, to_nibbles};

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Move { dst: u8, src: u8 },
    MoveResult { dst: u8 },
    Const4 { dst: u8, value: i8 },
    Const16 { dst: u8, value: i16 },
    ConstString { dst: u8, string_idx: u16 },
    InvokeVirtual { method_idx: u16, args: Vec<u8> },
    ReturnVoid,
    Return { reg: u8 },
    // ... more instructions
}

impl Instruction {
    pub fn decode(buffer: &[u8], offset: usize) -> std::io::Result<(Self, usize)> {
        if buffer.len() < offset + 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer too small for instruction",
            ));
        }
        let buffer = &buffer[offset..];
        let opcode = buffer[0];
        let (inst, length) = match opcode {
            0x00 => (Instruction::Nop, 1),
            0x01 => {
                let (dst, src) = to_nibbles(buffer[1]);
                (Instruction::Move { dst, src }, 1)
            }
            0x0A => (Instruction::MoveResult { dst: buffer[1] }, 1),
            0x1A => {
                if buffer.len() < 4 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Buffer too small for `const-string` instruction",
                    ));
                }
                let dst = buffer[1];
                let string_idx = read_u16_le(buffer, 2);
                (Instruction::ConstString { dst, string_idx }, 2)
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
```

### Assignment
1. Implement Bytecode Disassembly

Extend the Instruction enum and parsing logic to support at least the following instruction formats:
* 10x (no operands) - e.g., `return-void`
* 12x (2 registers) - e.g., `move vA, vB`
* 11n (1 register, 4-bit literal) - e.g., `const/4 vA, #+B`
* 21c (1 register, 16-bit constant pool index) - e.g., `const-string vAA, string@BBBB`
* 35c (3 registers, 16-bit constant pool index) - e.g., `invoke-virtual {vC, vD, vE}, method@BBBB`

For each instruction, print:
1. The instruction name (from opcode)
2. Register operands
3. Any literal or reference operands (with resolved strings/methods where applicable)

Example output:
```
0000: const-string v0, "Hello, World!"
0002: invoke-virtual {v0}, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
0005: return-void
```

Combine this output with the one from the previous lab to create a better class/method overview.

### Resources
[Dalvik Bytecode Reference](https://source.android.com/docs/core/runtime/dalvik-bytecode)

[Instruction Formats](https://source.android.com/docs/core/runtime/instruction-formats)

[Smali Documentation](https://github.com/JesusFreke/smali/wiki) (useful for understanding disassembled output)

[Dalvik Opcodes](http://pallergabor.uw.hu/androidblog/dalvik_opcodes.html)