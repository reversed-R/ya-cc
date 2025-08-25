pub mod expressions;
pub mod globals;
pub mod statements;
pub mod types;

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use crate::{
    parser::symbols::{
        self,
        globals::{FnDef, TypeDef},
        statements::var_dec::VarDec,
    },
    validator::{
        expressions::Exprs,
        globals::Function,
        types::{DefinedType, PrimitiveType, Type},
    },
};

pub fn validate(prog: &crate::parser::symbols::Program) -> Result<Program, ValidateError> {
    let mut env = Env::new();
    let mut fns = HashMap::new();
    let mut global_vars = HashMap::new();

    for g in &prog.globals {
        match g {
            symbols::globals::Globals::FnDeclare(f) => {
                if !env.global.fns.contains_key(&f.name) {
                    env.global.fns.insert(
                        f.name.clone(),
                        FnSignature {
                            args: &f.args,
                            rtype: &f.rtype,
                        },
                    );
                }
            }
            symbols::globals::Globals::FnDef(f) => {
                // insert f to env function signature list
                if !env.global.fns.contains_key(&f.name) {
                    env.global.fns.insert(
                        f.name.clone(),
                        FnSignature {
                            args: &f.args,
                            rtype: &f.rtype,
                        },
                    );
                }

                // for next codegen path, validate function body
                env.begin_local(&f.args, &f.rtype);

                fns.insert(f.name.clone(), f.validate(&mut env)?);

                env.end_local();
            }
            symbols::globals::Globals::VarDec(v) => {
                // insert variable to env global variable list
                env.global.vars.insert(
                    v.name.clone(),
                    Variable {
                        typ: v.typ.clone(),
                        addr: VarAddr::Global(v.name.clone(), v.typ.size(&env)),
                    },
                );

                // for next codegen path, insert var
                global_vars.insert(
                    v.name.clone(),
                    Variable {
                        typ: v.typ.clone(),
                        addr: VarAddr::Global(v.name.clone(), v.typ.size(&env)),
                    },
                );
            }
            symbols::globals::Globals::TypeDef(t) => match t {
                TypeDef::Struct(s) => {
                    let mut members = HashMap::new();
                    let mut offset = 0usize;
                    let mut max_align = 0usize;

                    for (mem_typ, mem_name) in &s.members {
                        if !members.contains_key::<String>(mem_name) {
                            offset = offset.next_multiple_of(mem_typ.align(&env));

                            members.insert(mem_name.clone(), (mem_typ.clone(), offset));

                            offset += mem_typ.size(&env);

                            if max_align < mem_typ.align(&env) {
                                max_align = mem_typ.align(&env);
                            }
                        } else {
                            return Err(ValidateError::StructMemberConflict(
                                s.name.clone(),
                                mem_name.clone(),
                            ));
                        }
                    }

                    if !env.global.types.contains_key(&s.name) {
                        env.global.types.insert(
                            s.name.clone(),
                            DefinedTypeContent::Struct(StructContent {
                                members,
                                size: offset.next_multiple_of(max_align),
                                align: max_align,
                            }),
                        );
                    } else {
                        return Err(ValidateError::TypeConflict(s.name.clone()));
                    }
                }
            },
        }
    }

    Ok(Program {
        fns,
        global_vars,
        string_literals: env.global.string_literals,
    })
}

#[derive(Debug)]
pub enum ValidateError {
    // something not found
    VariableNotFound(String),
    FunctionNotFound(String),
    TypeNotFound(String),
    StructMemberNotFound(String, String), // struct name, member string
    // something conflict
    VariableConflict(String),
    StructMemberConflict(String, String), // struct name, member string
    TypeConflict(String),
    // type mismatch
    ArgumentMismatch(Option<Box<Type>>, Option<Box<Type>>), // callee type, calling type
    Mismatch(Box<Type>, Box<Type>),                         // outer type, inner type
    // operation not allowed for the type
    DerefNotAllowed(Type),
    StructNotAssignable(String),                 // struct name
    TypeAndOperatorNotSupported(String, String), // type name, op string
    // validator bug
    OutOfScopes,
}

#[derive(Debug)]
pub struct Program {
    pub string_literals: HashMap<String, usize>,
    pub fns: HashMap<String, Function>,
    pub global_vars: HashMap<String, Variable>,
}

pub trait ExprTypeValidate {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), ValidateError>;
}

#[derive(Debug)]
pub struct Env<'parsed> {
    global: EnvGlobal<'parsed>,
    local: Option<EnvLocal>,
}

#[derive(Debug, Clone)]
pub struct StructContent {
    members: HashMap<String, (Type, usize)>,
    size: usize,
    align: usize,
}

#[derive(Debug)]
pub enum DefinedTypeContent {
    Struct(StructContent),
}

#[derive(Debug)]
struct EnvGlobal<'parsed> {
    fns: HashMap<String, FnSignature<'parsed>>,
    vars: HashMap<String, Variable>,
    types: HashMap<String, DefinedTypeContent>,
    string_literals: HashMap<String, usize>,
}

#[derive(Debug)]
struct EnvLocal {
    rtype: Type,
    vars: Vec<HashMap<String, Variable>>,
    local_max_offset: usize,
}

impl<'parsed> Env<'parsed> {
    fn new() -> Self {
        Self {
            global: EnvGlobal {
                fns: HashMap::new(),
                vars: HashMap::new(),
                types: HashMap::new(),
                string_literals: HashMap::new(),
            },
            local: None,
        }
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
                    ValidateError::OutOfScopes => {
                        panic!("Compiler Error, Out of Scopes");
                    }
                    ValidateError::VariableConflict(var) => {
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

    pub fn insert_var(&mut self, var: String, typ: Type) -> Result<(), ValidateError> {
        let offset =
            (self.get_local_varsize_sum() + typ.size(self)).next_multiple_of(typ.align(self));

        if let Some(local) = &mut self.local {
            if let Some(last_scope) = local.vars.last_mut() {
                match last_scope.entry(var.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(Variable {
                            typ,
                            addr: VarAddr::Local(offset),
                        });

                        if local.local_max_offset < offset {
                            local.local_max_offset = offset;
                        }

                        Ok(())
                    }

                    Entry::Occupied(_) => Err(ValidateError::VariableConflict(var)),
                }
            } else {
                Err(ValidateError::OutOfScopes)
            }
        } else {
            Err(ValidateError::OutOfScopes)
        }
    }

    pub fn get_var(&self, var: &String) -> Option<&Variable> {
        if let Some(local) = &self.local {
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
                .map(|scope| scope.values().map(|v| v.typ.size(self)).sum::<usize>())
                .sum::<usize>()
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub enum VarAddr {
    Local(usize),          // basepointer + offset
    Global(String, usize), // label name, byte size
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

impl ValidateError {
    pub fn panic_with_error_message(&self) -> ! {
        eprint!("\x1b[1;38;2;255;20;0merror\x1b[m: ");
        eprint!("\x1b[1m");
        match self {
            Self::VariableNotFound(var) => {
                eprint!("variable `{var}` not found");
            }
            Self::FunctionNotFound(f) => {
                eprint!("function `{f}` not found");
            }
            Self::TypeNotFound(typ) => {
                eprint!("type `{typ}` not found");
            }
            Self::StructMemberNotFound(s, mem) => {
                eprint!("member `{mem}` not found in struct `{s}`");
            }
            Self::VariableConflict(var) => {
                eprint!("variable `{var}` conflicting");
            }
            Self::ArgumentMismatch(callee_typ, calling_typ) => {
                if let Some(callee_typ) = callee_typ {
                    if let Some(calling_typ) = calling_typ {
                        eprint!("types mismatch in function call, expected `{callee_typ}`, but found `{calling_typ}`");
                    } else {
                        eprint!("types mismatch in function call, expected `{callee_typ}`, but found nothing");
                    }
                } else if let Some(calling_typ) = calling_typ {
                    eprint!("types mismatch in function call, expected nothing, but found `{calling_typ}`");
                } else {
                    eprint!("types mismatch in function call, expected nothing, but found nothing");
                    // ISSUE: ???
                }
            }
            Self::Mismatch(outer_typ, inner_typ) => {
                eprint!("types mismatch, expected `{outer_typ}`, but found `{inner_typ}`");
            }
            Self::DerefNotAllowed(typ) => {
                eprint!("dereference not allowed for `{typ}`");
            }
            Self::StructNotAssignable(s) => {
                eprint!("assignment not allowed for `struct {s}`");
            }
            Self::TypeAndOperatorNotSupported(typ, op) => {
                eprint!("operator `{op}` not allowed for `{typ}`");
            }
            Self::StructMemberConflict(s, mem) => {
                eprint!("member `{mem}` conflicting in `struct {s}`");
            }
            Self::TypeConflict(typ) => {
                eprint!("type `{typ}` conflicting");
            }
            Self::OutOfScopes => {
                eprint!("unknown compiler error occured, sorry");
            }
        }

        eprintln!("\x1b[m");
        panic!("");
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PtrTo(p) => write!(f, "{p}*"),
            Self::Primitive(p) => match p {
                PrimitiveType::Int => write!(f, "int"),
                PrimitiveType::Char => write!(f, "char"),
                PrimitiveType::Void => write!(f, "void"),
            },
            Self::Array(typ, size) => write!(f, "{typ}[{size}]"),
            Self::Defined(defed_type_id) => match defed_type_id {
                DefinedType::Struct(s) => write!(f, "struct {s}"),
            },
        }
    }
}
