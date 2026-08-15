use tl_parser::Program;
use crate::compile::Compiler;
pub use crate::vm::{VM, VMOptions};

mod inst;
mod opcodes;
mod vm;
mod compile;
mod value;

pub fn run(prog: &Program, opts: VMOptions) {
    let mut compiler = Compiler::new();
    for stmt in &prog.stmts {
        if let Err(e) = compiler.compile_stmt(stmt) {
            eprintln!("compile error: {e}");
            return;
        }
    }

    let (bytecode, constants) = compiler.done();
    let mut vm = VM::new(bytecode, constants, opts);

    vm.run();
}