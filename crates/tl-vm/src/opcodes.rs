#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Mov,
    Ldi,
    Print,
    Halt,
}
