pub mod expressions;
pub mod globals;
pub mod statements;
use std::collections::HashMap;

use crate::{
    parser::symbols::{globals::FnDec, statements::var_dec::VarDec},
    validator::globals::Function,
};

pub fn validate(prog: &crate::parser::symbols::Program) -> Result<(), TypeError> {
    let mut env = Env::new(&prog.fns);

    for f in &prog.fns {
        f.validate(&mut env)?;
    }

    Ok(())
}

#[derive(Debug)]
pub enum TypeError {
    VariableNotFound(String),
    FunctionNotFound(String),
    VariableConflict(String),
    OutOfScopes,
    ArgumentMismatch(Option<Type>, Option<Type>), // callee type, calling type
    Mismatch(Type, Type),                         // outer type, inner type
    DerefNotAllowed(Type),
}

// pub trait StmtTypeValidate {
//     fn validate_type(&self, env: &mut Env) -> Result<(), TypeError>;
// }

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Int,
    Float,
}

impl PrimitiveType {
    pub fn aligned_size(&self) -> usize {
        match self {
            Self::Int => 8,
            Self::Float => 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    PtrTo(Box<Type>),
}

impl Type {
    pub fn aligned_size(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.aligned_size(),
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match self {
            Self::Primitive(p) => match other {
                Self::Primitive(other_p) => p == other_p,
                _ => false,
            },
            Self::PtrTo(ptr) => match other {
                Self::PtrTo(other_ptr) => ptr.equals(other_ptr),
                _ => false,
            },
        }
    }

    pub fn ptr_to(typ: Self) -> Self {
        Self::PtrTo(Box::new(typ))
    }

    pub fn deref_of(typ: &Self) -> Option<Self> {
        if let Self::PtrTo(deref) = typ {
            Some(*deref.clone())
        } else {
            None
        }
    }
}
pub struct Program {
    fns: HashMap<String, Function>,
}

pub trait StmtTypeValidate {
    type ValidatedType;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError>;
}

pub trait ExprTypeValidate {
    type ValidatedType;

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError>;
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
