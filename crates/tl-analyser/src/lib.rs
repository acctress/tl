use crate::context::Context;
use crate::error::AnalysisErr;
use tl_parser::Program;

pub mod checker;
mod context;
pub mod error;
mod resolver;

pub fn analyse(program: &Program) -> Vec<AnalysisErr> {
    let mut ctx = Context::new();
    for stmt in &program.stmts {
        checker::hoist(stmt, &mut ctx);
    }
    for stmt in &program.stmts {
        checker::check_stmt(stmt, &mut ctx);
    }
    ctx.errors
}
