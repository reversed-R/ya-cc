pub mod expressions;
pub mod globals;
pub mod statements;

use std::collections::HashMap;

use crate::{
    parser::{
        symbols,
        symbols::{globals::FnDec, statements::var_dec::VarDec},
    },
    validator::globals::Function,
};

pub fn validate(prog: &crate::parser::symbols::Program) -> Result<Program, TypeError> {
    let mut env = Env::new(&prog.globals)?;
    let mut globals = HashMap::new();

    for g in &prog.globals {
        match g {
            symbols::globals::Globals::FnDec(f) => {
                globals.insert(f.name.clone(), Globals::Function(f.validate(&mut env)?));
            }
            symbols::globals::Globals::VarDec(v) => {
                globals.insert(
                    v.name.clone(),
                    Globals::Variable(Variable {
                        typ: v.typ.clone(),
                        addr: VarAddr::Global(v.name.clone()),
                    }),
                );
            }
        }
    }

    Ok(Program { globals })
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
pub enum TypeComarison {
    Equal,
    ImplicitlyConvertableTo,
    ImplicitlyConvertableFrom,
    ImplicitlyUnconvertable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    PtrTo(Box<Type>),
    Array(Box<Type>, usize),
}

impl Type {
    pub fn aligned_size(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.aligned_size(),
            Self::Array(typ, size) => typ.aligned_size() * size,
        }
    }

    pub fn compare(&self, other: &Self) -> TypeComarison {
        // ただし、Arrayについてはその識別子単体が渡されたと考える
        match self {
            Self::Primitive(prim) => match other {
                Self::Primitive(other_prim) => match prim {
                    PrimitiveType::Int => match other_prim {
                        PrimitiveType::Int => TypeComarison::Equal,
                        PrimitiveType::Float => TypeComarison::ImplicitlyConvertableTo,
                    },
                    PrimitiveType::Float => match other_prim {
                        PrimitiveType::Int => TypeComarison::ImplicitlyConvertableFrom,
                        PrimitiveType::Float => TypeComarison::Equal,
                    },
                },
                Self::PtrTo(_) => {
                    if prim == &PrimitiveType::Int {
                        TypeComarison::ImplicitlyConvertableTo
                    } else {
                        TypeComarison::ImplicitlyUnconvertable
                    }
                }
                Self::Array(_, _) => TypeComarison::ImplicitlyUnconvertable,
            },
            Self::PtrTo(pointed) => match other {
                Self::Primitive(other_prim) => match other_prim {
                    PrimitiveType::Int => match other_prim {
                        PrimitiveType::Int => TypeComarison::ImplicitlyConvertableFrom,
                        PrimitiveType::Float => TypeComarison::ImplicitlyUnconvertable,
                    },
                    PrimitiveType::Float => TypeComarison::ImplicitlyUnconvertable,
                },
                Self::PtrTo(other_pointed) => {
                    if pointed.equals(other_pointed) {
                        TypeComarison::Equal
                    } else {
                        TypeComarison::ImplicitlyUnconvertable
                    }
                }
                Self::Array(atyp, _) => {
                    if pointed.equals(atyp) {
                        TypeComarison::ImplicitlyConvertableFrom
                    } else {
                        TypeComarison::ImplicitlyUnconvertable
                    }
                }
            },
            Self::Array(atyp, _) => Self::PtrTo(Box::new(*atyp.clone())).compare(other),
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
            Self::Array(_, _) => false,
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

#[derive(Debug)]
pub struct Program {
    pub globals: HashMap<String, Globals>,
}

#[derive(Debug)]
pub enum Globals {
    Function(Function),
    Variable(Variable),
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
    local_max_offset: usize,
}

impl<'parsed> Env<'parsed> {
    pub fn new(globals: &'parsed [symbols::globals::Globals]) -> Result<Self, TypeError> {
        let mut fns_map = HashMap::<String, FnSignature>::new();
        let mut vars = NestedScope::new();

        for g in globals {
            match g {
                symbols::globals::Globals::FnDec(f) => {
                    if !fns_map.contains_key(&f.name) {
                        fns_map.insert(f.name.clone(), FnSignature::from(f));
                    }
                }
                symbols::globals::Globals::VarDec(v) => {
                    if let Some(scope) = vars.scopes.last() {
                        if !scope.contains_key(&v.name) {
                            vars.insert(v.name.clone(), v.typ.clone())?;
                        }
                    }
                }
            }
        }

        Ok(Self {
            fns: fns_map,
            vars,
            rtype: None,
            local_max_offset: 0,
        })
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

    pub fn insert_var(&mut self, var: String, typ: Type) -> Result<(), TypeError> {
        self.vars.insert(var, typ)?;

        let cur = self.vars.get_current_local_max_offset();
        if self.local_max_offset < cur {
            self.local_max_offset = cur;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum VarAddr {
    Local(usize),   // basepointer + offset
    Global(String), //
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub typ: Type,
    pub addr: VarAddr,
}

#[derive(Debug)]
pub struct NestedScope {
    scopes: Vec<HashMap<String, Variable>>,
}

impl NestedScope {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // global scope
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn insert(&mut self, var: String, typ: Type) -> Result<(), TypeError> {
        let offset = self.get_current_local_max_offset() + typ.aligned_size();

        if let Some(last) = self.scopes.last_mut() {
            if !last.contains_key(&var) {
                last.insert(
                    var.clone(),
                    Variable {
                        typ,
                        addr: VarAddr::Local(offset),
                    },
                );

                Ok(())
            } else {
                Err(TypeError::VariableConflict(var))
            }
        } else {
            Err(TypeError::OutOfScopes)
        }
    }

    pub fn get(&self, var: &String) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(var) {
                return Some(var);
            }
        }

        None
    }

    fn get_current_local_max_offset(&self) -> usize {
        self.scopes
            .iter()
            .map(|scope| {
                scope
                    .values()
                    .map(|v| match v.addr {
                        VarAddr::Local(offset) => offset,
                        VarAddr::Global(_) => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
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
