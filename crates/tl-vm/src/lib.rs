use crate::inst::Instr;
use crate::vm::{VM, VMOptions};

mod inst;
mod opcodes;
mod vm;

pub fn run(bytecode: Vec<Instr>, opts: VMOptions) {
    let mut vm = VM::new(bytecode, opts);
    vm.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::Opcode;

    fn run_get_reg(bytecode: Vec<Instr>, reg: usize) -> u64 {
        let mut vm = VM::new(bytecode, VMOptions { debug: true });
        vm.run();
        vm.get_reg(reg)
    }

    #[test]
    fn test_ldi() {
        let bytecode = vec![
            Instr::encode(Opcode::Ldi as u8, 0, 23, 0),
            Instr::encode(Opcode::Halt as u8, 0, 0, 0),
        ];

        assert_eq!(run_get_reg(bytecode, 0), 23);
    }

    #[test]
    fn test_add() {
        let bytecode = vec![
            Instr::encode(Opcode::Ldi as u8, 0, 10, 0),
            Instr::encode(Opcode::Ldi as u8, 1, 20, 0),
            Instr::encode(Opcode::Add as u8, 2, 0, 1),
            Instr::encode(Opcode::Halt as u8, 0, 0, 0),
        ];
        assert_eq!(run_get_reg(bytecode, 2), 30);
    }
}
