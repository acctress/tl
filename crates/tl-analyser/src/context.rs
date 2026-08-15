use std::collections::HashMap;
use tl_parser::{TypeExpr, Stmt, Expr, Program};
use crate::error::AnalysisErr;

/// Semantic meaning of the types used in the parser
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Str,
    Bool,
    Unit,
    List(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Struct(String),
    Unknown,
}

/// When a struct is declared, the name -> type data will be stored here
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub fields: Vec<(String, Type)>,
}

/// A symbol can be a variable (typed), function (params, ret), or a type definition
#[derive(Clone)]
pub enum Symbol {
    Variable { ty: Type },
    Function { params: Vec<Type>, ret: Type },
    Type { def: TypeDef }
}


/// Where symbols exists in a scope, for symbol resolution.
pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self{  symbols: HashMap::new() }
    }
}

pub struct Context {
    pub errors: Vec<AnalysisErr>,
    scopes: Vec<Scope>,
}

impl Context {
    /// Define the context with a global scope initially
    pub fn new() -> Self {
        Self {
            errors: vec![],
            scopes: vec![Scope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// This function will define a new symbol with a name in the inner most scope
    pub fn define(&mut self, name: &str, sym: Symbol) {
        // get the last scope in the vector as mutable
        let scope = self.scopes.last_mut().unwrap();
        if scope.symbols.contains_key(name) {
            // push an AlreadyDefined error if the named symbol already exists
            self.errors.push(AnalysisErr::AlreadyDefined(name.to_string()))
        } else {
            scope.symbols.insert(name.to_string(), sym);
        }
    }

    /// This function will walk the scopes from top to bottom
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.scopes.iter().rev().find_map(|s| s.symbols.get(name))
    }

    pub fn error(&mut self, err: AnalysisErr) {
        self.errors.push(err);
    }
}
