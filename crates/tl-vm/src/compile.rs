use tl_parser::*;
use crate::inst::Instr;
use crate::opcodes::Opcode;

pub struct Compiler {
    bytecode: Vec<Instr>,
    constants: Vec<f64>,
    nx_reg: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Self { bytecode: vec![], constants: vec![], nx_reg: 0 }
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.nx_reg;
        self.nx_reg += 1;
        r
    }

    fn emit(&mut self, op: Opcode, dst: u8, src1: u8, src2: u8) {
        self.bytecode.push(Instr::encode(op as u8, dst, src1, src2));
    }

    // add a new f64 to the constant pool, returning its index
    fn add_const(&mut self, val: f64) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }

    pub fn compile_expr(&mut self, node: &Expr) -> u8 {
        match node {
            Expr::Number(n) => {
                // allocate a register for loading the index of the number
                // in the constant pool
                let dst = self.alloc_reg();
                let idx = self.add_const(*n);
                self.emit(Opcode::Ldi, dst, idx, 0);
                dst
            },

            _ => todo!()
        }
    }

    pub fn compile_stmt(&mut self, node: &Stmt) {
        match node {
            Stmt::Expr(e) => { self.compile_expr(e); },
            _ => todo!()
        }
    }

    pub fn done(mut self) -> (Vec<Instr>, Vec<f64>) {
        self.emit(Opcode::Halt, 0, 0, 0);
        (self.bytecode, self.constants)
    }
}