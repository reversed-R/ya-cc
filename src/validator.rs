pub mod expressions;
pub mod globals;
pub mod statements;

use std::{collections::HashMap, ops::Deref};

use crate::{
    parser::symbols::{self, globals::FnDef, statements::var_dec::VarDec},
    validator::{expressions::Exprs, globals::Function},
};

pub fn validate(prog: &crate::parser::symbols::Program) -> Result<Program, TypeError> {
    let mut env = Env::new(&prog.globals)?;
    let mut fns = HashMap::new();
    let mut global_vars = HashMap::new();

    for g in &prog.globals {
        match g {
            symbols::globals::Globals::FnDeclare(_) => {
                // nothing to do
            }
            symbols::globals::Globals::FnDef(f) => {
                env.begin_local(&f.args, &f.rtype);

                fns.insert(f.name.clone(), f.validate(&mut env)?);

                env.end_local();
            }
            symbols::globals::Globals::VarDec(v) => {
                global_vars.insert(
                    v.name.clone(),
                    Variable {
                        typ: v.typ.clone(),
                        addr: VarAddr::Global(v.name.clone()),
                    },
                );
            }
        }
    }

    Ok(Program {
        fns,
        global_vars,
        string_literals: env.global.string_literals,
    })
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
    // Float,
    Char,
    Void,
}

impl PrimitiveType {
    pub fn size(&self) -> usize {
        match self {
            Self::Int => 8,
            // Self::Float => 8,
            Self::Char => 1,
            Self::Void => 0,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Self::Int => 8,
            // Self::Float => 8,
            Self::Char => 1,
            Self::Void => 0,
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
    pub fn size(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.size(),
            Self::Array(typ, size) => typ.size() * size,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.align(),
            Self::Array(typ, _) => typ.align(),
        }
    }

    pub fn compare(&self, other: &Self) -> TypeComarison {
        // ただし、Arrayについてはその識別子単体が渡されたと考える
        match self {
            Self::Primitive(prim) => match other {
                Self::Primitive(other_prim) => match prim {
                    PrimitiveType::Int => match other_prim {
                        PrimitiveType::Int => TypeComarison::Equal,
                        // PrimitiveType::Float => TypeComarison::ImplicitlyConvertableTo,
                        PrimitiveType::Char => TypeComarison::ImplicitlyConvertableFrom,
                        PrimitiveType::Void => TypeComarison::ImplicitlyUnconvertable,
                    },
                    // PrimitiveType::Float => match other_prim {
                    //     PrimitiveType::Int => TypeComarison::ImplicitlyConvertableFrom,
                    //     PrimitiveType::Float => TypeComarison::Equal,
                    //     PrimitiveType::Char => TypeComarison::ImplicitlyUnconvertable, // ??
                    // },
                    PrimitiveType::Char => match other_prim {
                        PrimitiveType::Int => TypeComarison::ImplicitlyConvertableTo,
                        // PrimitiveType::Float => TypeComarison::ImplicitlyUnconvertable, // ??
                        PrimitiveType::Char => TypeComarison::Equal,
                        PrimitiveType::Void => TypeComarison::ImplicitlyUnconvertable,
                    },
                    PrimitiveType::Void => match other_prim {
                        PrimitiveType::Void => TypeComarison::Equal,
                        _ => TypeComarison::ImplicitlyUnconvertable,
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
                        _ => TypeComarison::ImplicitlyUnconvertable, // PrimitiveType::Float => TypeComarison::ImplicitlyUnconvertable,
                    },
                    // PrimitiveType::Float => TypeComarison::ImplicitlyUnconvertable,
                    PrimitiveType::Char => TypeComarison::ImplicitlyUnconvertable,
                    PrimitiveType::Void => TypeComarison::ImplicitlyUnconvertable,
                },
                Self::PtrTo(other_pointed) => {
                    if pointed.equals(other_pointed) {
                        TypeComarison::Equal
                    } else if pointed.deref() == &Type::Primitive(PrimitiveType::Void) {
                        TypeComarison::ImplicitlyConvertableFrom
                    } else if other_pointed.deref() == &Type::Primitive(PrimitiveType::Void) {
                        TypeComarison::ImplicitlyConvertableTo
                    } else {
                        pointed.compare(other_pointed) // WARN: is it true?
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
    pub string_literals: HashMap<String, usize>,
    pub fns: HashMap<String, Function>,
    pub global_vars: HashMap<String, Variable>,
}

pub trait StmtTypeValidate {
    type ValidatedType;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError>;
}

pub trait ExprTypeValidate {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError>;
}

#[derive(Debug)]
pub struct Env<'parsed> {
    global: EnvGlobal<'parsed>,
    local: Option<EnvLocal>,
}

#[derive(Debug)]
struct EnvGlobal<'parsed> {
    fns: HashMap<String, FnSignature<'parsed>>,
    vars: HashMap<String, Variable>,
    string_literals: HashMap<String, usize>,
}

#[derive(Debug)]
struct EnvLocal {
    rtype: Type,
    vars: Vec<HashMap<String, Variable>>,
    local_max_offset: usize,
}

impl<'parsed> Env<'parsed> {
    fn new(globals: &'parsed [symbols::globals::Globals]) -> Result<Self, TypeError> {
        let mut fns = HashMap::<String, FnSignature>::new();
        let mut vars = HashMap::new();

        for g in globals {
            match g {
                symbols::globals::Globals::FnDef(f) => {
                    if !fns.contains_key(&f.name) {
                        fns.insert(f.name.clone(), FnSignature::from(f));
                    }
                }
                symbols::globals::Globals::VarDec(v) => {
                    if !vars.contains_key(&v.name) {
                        vars.insert(
                            v.name.clone(),
                            Variable {
                                typ: v.typ.clone(),
                                addr: VarAddr::Global(v.name.clone()),
                            },
                        );
                    }
                }
                symbols::globals::Globals::FnDeclare(f) => {
                    if !fns.contains_key(&f.name) {
                        fns.insert(
                            f.name.clone(),
                            FnSignature {
                                args: &f.args,
                                rtype: &f.rtype,
                            },
                        );
                    }
                }
            }
        }

        Ok(Self {
            global: EnvGlobal {
                fns,
                vars,
                string_literals: HashMap::new(),
            },
            local: None,
        })
    }

    fn begin_local(&mut self, args: &[VarDec], rtype: &Type) {
        self.local = Some(EnvLocal {
            rtype: rtype.clone(),
            vars: vec![HashMap::new()],
            local_max_offset: 0,
        });

        for arg in args {
            if let Err(e) = self.insert_var(arg.name.clone(), arg.typ.clone()) {
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

        if let Some(local) = &mut self.local {
            local.rtype = rtype.clone();
        }
    }

    fn end_local(&mut self) {
        self.local = None;
    }

    pub fn begin_scope(&mut self) {
        if let Some(local) = &mut self.local {
            local.vars.push(HashMap::new());
        }
    }

    pub fn end_scope(&mut self) {
        if let Some(local) = &mut self.local {
            local.vars.pop();
        }
    }

    pub fn insert_var(&mut self, var: String, typ: Type) -> Result<(), TypeError> {
        let offset = (self.get_local_varsize_sum() + typ.size()).next_multiple_of(typ.align());

        if let Some(local) = &mut self.local {
            if let Some(last_scope) = local.vars.last_mut() {
                if !last_scope.contains_key(&var) {
                    last_scope.insert(
                        var,
                        Variable {
                            typ,
                            addr: VarAddr::Local(offset),
                        },
                    );

                    if local.local_max_offset < offset {
                        local.local_max_offset = offset;
                    }

                    Ok(())
                } else {
                    Err(TypeError::VariableConflict(var))
                }
            } else {
                Err(TypeError::OutOfScopes)
            }
        } else {
            Err(TypeError::OutOfScopes)
        }
    }

    pub fn get_var(&mut self, var: &String) -> Option<&Variable> {
        if let Some(local) = &mut self.local {
            for scope in local.vars.iter().rev() {
                if let Some(var) = scope.get(var) {
                    return Some(var);
                }
            }
        }

        self.global.vars.get(var)
    }

    fn get_local_varsize_sum(&self) -> usize {
        if let Some(local) = &self.local {
            local
                .vars
                .iter()
                .map(|scope| scope.values().map(|v| v.typ.size()).sum::<usize>())
                .sum::<usize>()
        } else {
            0
        }
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
pub struct FnSignature<'fndec> {
    pub args: &'fndec Vec<VarDec>,
    pub rtype: &'fndec Type,
}

impl<'fndec> From<&'fndec FnDef> for FnSignature<'fndec> {
    fn from(value: &'fndec FnDef) -> Self {
        Self {
            args: &value.args,
            rtype: &value.rtype,
        }
    }
}
