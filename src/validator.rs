pub mod expressions;
pub mod globals;
pub mod statements;

use std::collections::HashMap;

use crate::parser::symbols::{
    statements::{var_dec::VarDec, Stmt},
    Type,
};

pub enum TypeError {
    Mismatch(Type, Type), // outer type, inner type
}

pub trait StmtTypeValidate {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError>;
}

pub trait ExprTypeValidate {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError>;
}

#[derive(Debug)]
pub struct Env {
    locals: HashMap<String, Type>,
    rtype: Type,
}

impl Env {
    pub fn new(args: &[VarDec], rtype: &Type, stmts: &[Stmt]) -> Self {
        let mut locals = HashMap::<String, Type>::new();

        for arg in args {
            if !locals.contains_key(&arg.name) {
                locals.insert(arg.name.clone(), arg.typ.clone());
            }
        }

        for stmt in stmts {
            if let Stmt::VarDec(vardec) = stmt {
                if !locals.contains_key(&vardec.name) {
                    locals.insert(vardec.name.clone(), vardec.typ.clone());
                }
            }
        }

        Self {
            locals,
            rtype: rtype.clone(),
        }
    }
}
