use tl_parser::TypeExpr;
use crate::context::{Context, Symbol, Type};
use crate::error::AnalysisErr;

/// This function will map out syntactic types (TypeExpr from tl-parser) to its semantic type
pub fn resolve_type(te: &TypeExpr, ctx: &mut Context) -> Type {
    match te {
        TypeExpr::Named(n) => match n.as_str() {
            "number"    => Type::Number,
            "str"       => Type::Str,
            "bool"      => Type::Bool,
            "unit"      => Type::Unit,
            _ => {
                // if the type isn't a primitive, check if it's user defined?
                if let Some(Symbol::Type { .. }) = ctx.lookup(n) {
                    Type::Struct(n.clone())
                } else {
                    // else push an undefined error
                    ctx.error(AnalysisErr::Undefined(n.clone()));
                    Type::Unknown
                }
            }
        },
        TypeExpr::Function(params, ret) => {
            // use rust goodies to resolve each type in params cleanly
            let params = params.iter().map(|p| resolve_type(p, ctx)).collect();
            let ret = Box::new(resolve_type(ret, ctx));
            Type::Function(params, ret)
        }
    }
}