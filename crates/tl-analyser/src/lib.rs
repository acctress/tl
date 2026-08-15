pub use crate::context::{Context, Symbol, Type};
pub use crate::error::AnalysisErr;
use tl_parser::Program;

pub mod checker;
mod context;
pub mod error;
mod resolver;

pub struct Builtin {
    pub name: &'static str,
    pub params: Vec<Type>,
    pub ret: Type,
}

pub fn analyse(program: &Program, builtins: &[Builtin]) -> Vec<AnalysisErr> {
    let mut ctx = Context::new();

    for b in builtins {
        ctx.define(b.name, Symbol::Function {
            params: b.params.clone(),
            ret: b.ret.clone()
        });
    }

    for stmt in &program.stmts {
        checker::hoist(stmt, &mut ctx);
    }
    for stmt in &program.stmts {
        checker::check_stmt(stmt, &mut ctx);
    }
    ctx.errors
}
