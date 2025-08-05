pub mod expressions;
pub mod globals;
pub mod statements;
use std::collections::HashMap;

use crate::parser::symbols::{globals::FnDec, statements::var_dec::VarDec, Program, Type};

pub fn validate(prog: Program) -> Result<(), TypeError> {
    let mut env = Env::new(&prog.fns);

    for f in &prog.fns {
        f.validate_type(&mut env)?;
    }

    Ok(())
}

pub enum TypeError {
    VariableNotFound(String),
    FunctionNotFound(String),
    VariableConflict(String),
    OutOfScopes,
    Mismatch(Type, Type), // outer type, inner type
}

pub trait StmtTypeValidate {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError>;
}

pub trait ExprTypeValidate {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError>;
}

#[derive(Debug)]
pub struct Env<'parsed> {
    fns: HashMap<String, FnSignature<'parsed>>,
    vars: NestedScope,
    rtype: Option<Type>,
}

impl<'parsed> Env<'parsed> {
    pub fn new(fns: &'parsed [FnDec]) -> Self {
        let mut fns_map = HashMap::<String, FnSignature>::new();

        for f in fns {
            if !fns_map.contains_key(&f.name) {
                fns_map.insert(f.name.clone(), FnSignature::from(f));
            }
        }

        Self {
            fns: fns_map,
            vars: NestedScope::new(),
            rtype: None,
        }
    }

    pub fn begin_local(&mut self, args: &[VarDec], rtype: &Type) {
        self.vars.push_scope();

        for arg in args {
            if let Err(e) = self.vars.insert(arg.name.clone(), arg.typ.clone()) {
                match e {
                    TypeError::OutOfScopes => {
                        panic!("Compiler Error, Out of Scopes");
                    }
                    TypeError::VariableConflict(var) => {
                        panic!("Function Arg Name Conflicting: {var}");
                    }
                    _ => {
                        panic!("Compiler Error, Unknown");
                    }
                }
            }
        }

        self.rtype = Some(rtype.clone());
    }

    pub fn end_local(&mut self) {
        self.vars.pop_scope();

        self.rtype = None;
    }

    pub fn begin_scope(&mut self) {
        self.vars.push_scope();
    }

    pub fn end_scope(&mut self) {
        self.vars.pop_scope();
    }
}

#[derive(Debug)]
pub struct NestedScope {
    scopes: Vec<HashMap<String, Type>>,
}

impl NestedScope {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, var: String, typ: Type) -> Result<(), TypeError> {
        if let Some(last) = self.scopes.last_mut() {
            if !last.contains_key(&var) {
                last.insert(var.clone(), typ);

                Ok(())
            } else {
                Err(TypeError::VariableConflict(var))
            }
        } else {
            Err(TypeError::OutOfScopes)
        }
    }

    pub fn get(&self, var: &String) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(var) {
                return Some(typ);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct FnSignature<'fndec> {
    pub args: &'fndec Vec<VarDec>,
    pub rtype: &'fndec Type,
}

impl<'fndec> From<&'fndec FnDec> for FnSignature<'fndec> {
    fn from(value: &'fndec FnDec) -> Self {
        Self {
            args: &value.args,
            rtype: &value.rtype,
        }
    }
}
