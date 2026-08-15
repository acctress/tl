use tl_parser::*;
use crate::inst::Instr;
use crate::opcodes::Opcode;
use crate::value::Value;

#[derive(Debug)]
pub enum CompileError {
    UnsupportedBinOp(BinOps),
    UndefinedFunction(String),
    UnsupportedExpr,
    UnsupportedStmt,
}

pub struct Compiler {
    bytecode: Vec<Instr>,
    constants: Vec<Value>,
    nx_reg: u8,
}

impl std::error::Error for CompileError {}

type CompResult<T> = Result<T, CompileError>;

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

    // add a new Value to the constant pool, returning its index
    fn add_const(&mut self, val: Value) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }

    pub fn compile_expr(&mut self, node: &Expr) -> CompResult<u8> {
        match node {
            Expr::Number(n) => {
                // allocate a register for loading the index of the number
                // in the constant pool
                let dst = self.alloc_reg();
                let idx = self.add_const(Value::Num(*n));
                self.emit(Opcode::Ldi, dst, idx, 0);
                Ok(dst)
            },

            Expr::String(st) => {
                let dst = self.alloc_reg();
                let idx = self.add_const(Value::Str(st.clone()));
                self.emit(Opcode::Ldi, dst, idx, 0);
                Ok(dst)
            },

            Expr::BinOp { op, lhs, rhs } => {
                let lhs = self.compile_expr(lhs)?;
                let rhs = self.compile_expr(rhs)?;
                let dst = self.alloc_reg();
                let opcode = match op {
                    BinOps::Add => Opcode::Add,
                    BinOps::Sub => Opcode::Sub,
                    BinOps::Mul => Opcode::Mul,
                    BinOps::Div => Opcode::Div,
                    _ => return Err(CompileError::UnsupportedBinOp(op.clone()))
                };
                self.emit(opcode, dst, lhs, rhs);
                Ok(dst)
            },

            Expr::Call { callee, args } => {
                match callee.as_str() {
                    "print" => {
                        let src = self.compile_expr(&args[0])?;
                        self.emit(Opcode::Print, 0, src, 0);
                        Ok(0)
                    },

                    _ => Err(CompileError::UndefinedFunction(callee.clone()))
                }
            }

            _ => Err(CompileError::UnsupportedExpr)
        }
    }

    pub fn compile_stmt(&mut self, node: &Stmt) -> CompResult<()> {
        match node {
            Stmt::Expr(e) => { self.compile_expr(e)?; Ok(()) },
            _ => Err(CompileError::UnsupportedStmt)
        }
    }

    pub fn done(mut self) -> (Vec<Instr>, Vec<Value>) {
        self.emit(Opcode::Halt, 0, 0, 0);
        (self.bytecode, self.constants)
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedBinOp(op) => write!(f, "unsupported binary operator: {:?}", op),
            Self::UndefinedFunction(name) => write!(f, "undefined function: {name}"),
            Self::UnsupportedExpr => write!(f, "unsupported expression"),
            Self::UnsupportedStmt => write!(f, "unsupported statement"),
        }
    }
}