use tl_parser::{Expr, Stmt};
use crate::context::{Context, Symbol, Type, TypeDef};
use crate::error::AnalysisErr;
use crate::resolver::resolve_type;

/// This function will only check Struct and FnDecl signatures, this is so forward refs work
pub fn hoist(stmt: &Stmt, ctx: &mut Context) {
    match stmt {
        Stmt::FnDecl { name, params, ret, .. } => {
            let param_types: Vec<Type> = params.iter().map(|p| {
                match &p.ty {
                    Some(t) => resolve_type(t, ctx),
                    None => {
                        ctx.error(AnalysisErr::MissingAnnotation(p.name.clone()));
                        Type::Unknown
                    }
                }
            }).collect();

            let ret_ty = ret.as_ref()
                .map(|t| resolve_type(t, ctx))
                .unwrap_or(Type::Unit);

            ctx.define(name, Symbol::Function {
                params: param_types,
                ret: ret_ty,
            });
        }

        Stmt::Struct { name, fields } => {
            let resolved: Vec<(String, Type)> = fields.iter().map(|f| {
                (f.name.clone(), resolve_type(&f.ty, ctx))
            }).collect();

            ctx.define(name, Symbol::Type {
                def: TypeDef { fields: resolved },
            });
        }

        _ => {}
    }
}

/// This function will check a statement, such as:
/// Let: check its name, value and type
/// FnDecl: check its name, params, ret and body
/// Struct: checks its name and fields
/// Expr: calls check_expr
pub fn check_stmt(stmt: &Stmt, ctx: &mut Context) {
    match stmt {
        Stmt::Let { name, value, ty } => {
            // now get an inferred value type to be compared to annotation
            let inferred = check_expr(value, ctx);
            let declared = ty.as_ref().map(|t| resolve_type(t, ctx));

            // now compare the inferred and declared types
            let compared_ty = match declared {
                Some(decl) => {
                    if decl != inferred {
                        ctx.error(AnalysisErr::TypeMismatch {
                            expected: decl.clone(),
                            found: inferred,
                        });
                    }

                    decl
                }
                None => inferred,   // just use the inferred type if there is no annotation
            };

            ctx.define(name, Symbol::Variable { ty: compared_ty });
        }

        Stmt::FnDecl { name, params, ret, body } => {
            // resolve all param types
            let param_types: Vec<Type> = params.iter().map(|p| {
                match &p.ty {
                    Some(t) => resolve_type(t, ctx),
                    None => {
                        ctx.error(AnalysisErr::MissingAnnotation(p.name.clone()));
                        Type::Unknown
                    }
                }
            }).collect();

            let ret_ty = ret.as_ref()
                .map(|t| resolve_type(t, ctx))
                .unwrap_or(Type::Unit);

            ctx.define(name, Symbol::Function {
                params: param_types.clone(),
                ret: ret_ty.clone()
            });

            // now check the body, we'll need to create a new scope for it
            ctx.push_scope();
            // essentially define each param as a variable in the scope
            for (param, ty) in params.iter().zip(param_types) {
                ctx.define(&param.name, Symbol::Variable { ty });
            }

            for s in body {
                check_stmt(s, ctx);
            }

            ctx.pop_scope();
        }

        Stmt::Struct { name, fields } => {
            let resolved: Vec<(String, Type)> = fields.iter().map(|f| {
                (f.name.clone(), resolve_type(&f.ty, ctx))
            }).collect();

            ctx.define(name, Symbol::Type {
                def: TypeDef { fields: resolved }
            });
        }

        Stmt::Expr(e) => { check_expr(e, ctx); }
    }
}

// Infer each expression to a defined type
pub fn check_expr(expr: &Expr, ctx: &mut Context) -> Type {
    match expr {
        Expr::Number(_) => Type::Number,
        Expr::String(_) => Type::Str,

        Expr::Ident(name) => {
            match ctx.lookup(name) {
                Some(Symbol::Variable { ty }) => ty.clone(),
                Some(_) => {
                    ctx.error(AnalysisErr::Undefined(name.clone()));
                    Type::Unknown
                }
                None => {
                    ctx.error(AnalysisErr::Undefined(name.clone()));
                    Type::Unknown
                }
            }
        }

        Expr::BinOp { op, lhs, rhs } => {
            let lhs = check_expr(lhs, ctx);
            let rhs = check_expr(rhs, ctx);
            if lhs != rhs {
                ctx.error(AnalysisErr::TypeMismatch { expected: lhs, found: rhs });
            }

            Type::Number
        }

        Expr::Call { callee, args } => {
            match ctx.lookup(callee).cloned() {
                Some(Symbol::Function { params, ret }) => {
                    // type check each param and arg
                    for (arg, expected) in args.iter().zip(&params) {
                        let found = check_expr(arg, ctx);
                        if found != *expected {
                            ctx.error(AnalysisErr::TypeMismatch {
                                expected: expected.clone(),
                                found,
                            });
                        }
                    }

                    // return the return type of the function call
                    ret
                }

                _ => {
                    ctx.error(AnalysisErr::Undefined(callee.clone()));
                    Type::Unknown
                }
            }
        }

        Expr::If { cond, then, else_ } => {
            check_expr(cond, ctx);

            ctx.push_scope();
            for s in then { check_stmt(s, ctx); }
            ctx.pop_scope();

            if let Some(el) = else_ {
                ctx.push_scope();
                for s in el { check_stmt(s, ctx); }
                ctx.pop_scope();
            }

            Type::Unit // if is not used as a value rn so just return a unit
        }

        _ => Type::Unknown
    }
}