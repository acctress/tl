use tl_parser::*;
use crate::inst::Instr;
use crate::opcodes::Opcode;

pub struct Compiler {
    bytecode: Vec<Instr>,
    nx_reg: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Self { bytecode: vec![], nx_reg: 0 }
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.nx_reg;
        self.nx_reg += 1;
        r
    }

    fn emit(&mut self, op: Opcode, dst: u8, src1: u8, src2: u8) {
        self.bytecode.push(Instr::encode(op as u8, dst, src1, src2));
    }

    pub fn compile_expr(&mut self, node: &Expr) -> u8 {
        todo!()
    }

    pub fn compile_stmt(&mut self, node: &Stmt) {
        todo!()
    }

    pub fn done(mut self) -> Vec<Instr> {
        self.emit(Opcode::Halt, 0, 0, 0);
        self.bytecode
    }
}