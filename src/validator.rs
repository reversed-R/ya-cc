pub mod expressions;
pub mod globals;
pub mod statements;

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    ops::Deref,
};

use crate::{
    parser::symbols::{
        self,
        globals::{FnDef, TypeDef},
        statements::var_dec::VarDec,
    },
    validator::{expressions::Exprs, globals::Function},
};

pub fn validate(prog: &crate::parser::symbols::Program) -> Result<Program, TypeError> {
    // ISSUE:
    // 現在の実装ではEnvの初期化時にグローバルから見える変数、関数宣言、定義、型定義を総なめしている
    // 実際のCでは上から発見し次第存在するものとなるが、今の実装のほうが便利なので良いかあ〜、と思ったり...
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
                        if !members.contains_key(mem_name) {
                            offset = offset.next_multiple_of(mem_typ.align(&env));

                            members.insert(mem_name.clone(), (mem_typ.clone(), offset));

                            offset += mem_typ.size(&env);

                            if max_align < mem_typ.align(&env) {
                                max_align = mem_typ.align(&env);
                            }
                        } else {
                            return Err(TypeError::StructMemberConflict(
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
                        return Err(TypeError::TypeConflict(s.name.clone()));
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
pub enum TypeError {
    VariableNotFound(String),
    FunctionNotFound(String),
    TypeNotFound(String),
    StructMemberNotFound(String, String), // struct name, member string
    VariableConflict(String),
    OutOfScopes,
    ArgumentMismatch(Option<Box<Type>>, Option<Box<Type>>), // callee type, calling type
    Mismatch(Box<Type>, Box<Type>),                         // outer type, inner type
    DerefNotAllowed(Type),
    StructNotAssignable(String),                 // struct name
    TypeAndOperatorNotSupported(String, String), // type name, op string
    StructMemberConflict(String, String),        // struct name, member string
    TypeConflict(String),
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

#[derive(Debug, Clone)]
pub struct StructContent {
    members: HashMap<String, (Type, usize)>,
    size: usize,
    align: usize,
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    PtrTo(Box<Type>),
    Array(Box<Type>, usize),
    Defined(DefinedType),
}

#[derive(Debug, Clone)]
pub enum DefinedType {
    Struct(String),
    // TypeDefed(String),
}

impl Type {
    pub fn size(&self, env: &Env) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.size(),
            Self::Array(typ, size) => typ.size(env) * size,
            Self::Defined(defed_type_id) => match defed_type_id {
                DefinedType::Struct(s) => {
                    if let Some(defed_typ) = env.global.types.get(s) {
                        match defed_typ {
                            DefinedTypeContent::Struct(s) => s.size,
                        }
                    } else {
                        panic!("Type Not Found `struct {s}`");
                    }
                }
            },
        }
    }

    pub fn align(&self, env: &Env) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.align(),
            Self::Array(typ, _) => typ.align(env),
            Self::Defined(defed_type_id) => match defed_type_id {
                DefinedType::Struct(s) => {
                    if let Some(defed_typ) = env.global.types.get(s) {
                        match defed_typ {
                            DefinedTypeContent::Struct(s) => s.align,
                        }
                    } else {
                        panic!("Type Not Found `struct {s}`");
                    }
                }
            },
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
                Self::Defined(_) => TypeComarison::ImplicitlyUnconvertable,
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
                    } else if matches!(pointed.deref(), &Type::Primitive(PrimitiveType::Void)) {
                        TypeComarison::ImplicitlyConvertableFrom
                    } else if matches!(other_pointed.deref(), &Type::Primitive(PrimitiveType::Void))
                    {
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
                Self::Defined(_) => TypeComarison::ImplicitlyUnconvertable,
            },
            Self::Array(atyp, _) => Self::PtrTo(Box::new(*atyp.clone())).compare(other),
            Self::Defined(_) => TypeComarison::ImplicitlyUnconvertable,
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
            Self::Defined(defed_typ) => match other {
                Self::Defined(other_defed_typ) => match defed_typ {
                    DefinedType::Struct(s) => match other_defed_typ {
                        DefinedType::Struct(other_s) => s == other_s,
                    },
                },
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

    pub fn get_ptr_base(&self) -> Option<Self> {
        match self {
            Self::Primitive(p) => Some(Self::Primitive(p.clone())),
            Self::PtrTo(pointed) => {
                match self {
                    Self::Primitive(p) => Some(Self::Primitive(p.clone())),
                    Self::PtrTo(_) => pointed.get_ptr_base(),
                    Self::Array(_, _) => {
                        None
                        // NOTE:
                        // atyp..get_ptr_base()
                    }
                    Self::Defined(_) => None,
                }
            }
            Self::Array(_, _) => {
                None
                // NOTE:
                // atyp.get_ptr_base()
            }
            Self::Defined(d) => Some(Self::Defined(d.clone())),
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

                    Entry::Occupied(_) => Err(TypeError::VariableConflict(var)),
                }
            } else {
                Err(TypeError::OutOfScopes)
            }
        } else {
            Err(TypeError::OutOfScopes)
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

impl TypeError {
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
