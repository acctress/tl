use crate::opcodes::Opcode;

/// A struct to represent an instruction which is packed into a 32 bit fixed width integer
/// this allows for small bytecode sizes
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Instr(u32);

impl Instr {
    /// Encode the opcode, destination, and source values into
    /// the final 32-bit integer
    pub fn encode(op: u8, dst: u8, src1: u8, src2: u8) -> Self {
        Self((op as u32) << 24 | (dst as u32) << 16 | (src1 as u32) << 8 | (src2 as u32))
    }

    /// I'm using an unsafe operation here as it's the most concise approach
    /// And in reality, this is not going to cause any issues
    pub fn op(self) -> Opcode {
        unsafe { std::mem::transmute((self.0 >> 24) as u8) }
    }

    /// Destination register
    pub fn dst(self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// First source register
    pub fn src1(self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Second source register
    pub fn src2(self) -> u8 {
        self.0 as u8
    }
}
