use crate::inst::Instr;
use crate::opcodes::Opcode;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Str(String),
}

pub struct VMOptions {
    pub debug: bool,
}

impl Default for VMOptions {
    fn default() -> Self {
        Self { debug: false }
    }
}

pub struct VM {
    regs: [Value; 256],
    pc: usize,
    bytecode: Vec<Instr>,
    constants: Vec<Value>,
    opts: VMOptions,
}

impl VM {
    pub fn new(bytecode: Vec<Instr>, constants: Vec<Value>, opts: VMOptions) -> Self {
        Self {
            regs: std::array::from_fn(|_| Value::Num(0.0)),
            pc: 0,
            constants,
            bytecode,
            opts,
        }
    }

    pub fn get_reg(self, reg: usize) -> Value {
        self.regs[reg].clone()
    }

    pub fn run(&mut self) {
        if self.opts.debug {
            println!("[constants] {:?}", self.constants);
        }

        loop {
            let inst = self.bytecode[self.pc];
            self.pc += 1;

            if self.opts.debug {
                println!(
                    "[{:04}] {:?}, op: {:?}, dst: {}, src1: {}, src2: {}",
                    self.pc - 1,
                    inst,
                    inst.op(),
                    inst.dst(),
                    inst.src1(),
                    inst.src2()
                );
            }

            let dst = inst.dst() as usize;
            let src1 = inst.src1() as usize;
            let src2 = inst.src2() as usize;

            match inst.op() {
                Opcode::Add => self.regs[dst] = &self.regs[src1] + &self.regs[src2],
                Opcode::Sub => self.regs[dst] = &self.regs[src1] - &self.regs[src2],
                Opcode::Mul => self.regs[dst] = &self.regs[src1] * &self.regs[src2],
                Opcode::Div => self.regs[dst] = &self.regs[src1] / &self.regs[src2],
                Opcode::Mov => self.regs[dst] = self.regs[src1].clone(),
                Opcode::Ldi => self.regs[dst] = self.constants[src1].clone(),
                Opcode::Halt => break,
            }
        }
    }
}

impl Add for &Value {
    type Output = Value;
    fn add(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
            _ => panic!("type error"),
        }
    }
}

impl Sub for &Value {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
            _ => panic!("type error"),
        }
    }
}

impl Mul for &Value {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
            _ => panic!("type error"),
        }
    }
}

impl Div for &Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a / b),
            _ => panic!("type error"),
        }
    }
}