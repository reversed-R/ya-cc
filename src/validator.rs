pub mod expressions;
pub mod globals;
pub mod statements;

use std::{collections::HashMap, fmt::Display, ops::Deref};

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
            symbols::globals::Globals::TypeDef(_) => {
                // nothing to do
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
}

#[derive(Debug, Clone)]
pub struct StructMember {
    pub typ: Type,
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub members: Vec<StructMember>,
    size: usize,
    align: usize,
}

impl StructType {
    pub fn new(name: String, mems: &[(Type, String)]) -> Self {
        let mut offset = 0usize;
        let mut members: Vec<StructMember> = vec![];
        let mut max_align = 0usize;

        for (typ, name) in mems {
            // TODO: detect member name confiliction
            offset = offset.next_multiple_of(typ.align());
            members.push(StructMember {
                typ: typ.clone(),
                name: name.clone(),
                offset,
            });

            offset += typ.size();

            if max_align < typ.align() {
                max_align = typ.align();
            }
        }

        Self {
            name,
            members,
            size: offset.next_multiple_of(max_align),
            align: max_align,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn align(&self) -> usize {
        self.align
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    PtrTo(Box<Type>),
    Array(Box<Type>, usize),
    Struct(StructType),
    Incomplete(String),
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.size(),
            Self::Array(typ, size) => typ.size() * size,
            Self::Struct(s) => s.size(),
            Self::Incomplete(_) => 0, // ISSUE: size unknown
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Self::PtrTo(_) => 8,
            Self::Primitive(p) => p.align(),
            Self::Array(typ, _) => typ.align(),
            Self::Struct(s) => s.align(),
            Self::Incomplete(_) => 0, // ISSUE: align unknown
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
                Self::Struct(_) => TypeComarison::ImplicitlyUnconvertable,
                Self::Incomplete(_) => TypeComarison::ImplicitlyUnconvertable,
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
                Self::Struct(_) => TypeComarison::ImplicitlyUnconvertable,
                Self::Incomplete(_) => TypeComarison::ImplicitlyUnconvertable,
            },
            Self::Array(atyp, _) => Self::PtrTo(Box::new(*atyp.clone())).compare(other),
            Self::Struct(_) => TypeComarison::ImplicitlyUnconvertable,
            Self::Incomplete(_) => TypeComarison::ImplicitlyUnconvertable,
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
            Self::Struct(s) => {
                if let Self::Struct(other_s) = other {
                    // ISSUE: struct names must guarantee not conflicting
                    s.name == other_s.name
                } else {
                    false
                }
            }
            Self::Incomplete(name) => match other {
                Self::Struct(s) => name == &s.name,
                Self::Incomplete(other_name) => name == other_name,
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

    pub fn get_ptr_base(&self) -> Option<PrimitiveType> {
        match self {
            Self::Primitive(p) => Some(p.clone()),
            Self::PtrTo(pointed) => {
                match self {
                    Self::Primitive(p) => Some(p.clone()),
                    Self::PtrTo(_) => pointed.get_ptr_base(),
                    Self::Array(_, _) => {
                        None
                        // NOTE:
                        // atyp..get_ptr_base()
                    }
                    Self::Struct(_) => {
                        None
                        // NOTE:
                        // Some(s.clone())
                    }
                    Self::Incomplete(_) => None,
                }
            }
            Self::Array(_, _) => {
                None
                // NOTE:
                // atyp.get_ptr_base()
            }
            Self::Struct(_) => {
                None
                // NOTE:
                // Some(s.clone())
            }
            Self::Incomplete(_) => None,
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
    fn new(globals: &'parsed [symbols::globals::Globals]) -> Result<Self, TypeError> {
        let mut fns = HashMap::<String, FnSignature>::new();
        let mut vars = HashMap::new();
        let mut types = HashMap::new();

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
                symbols::globals::Globals::TypeDef(t) => match t {
                    TypeDef::Struct(s) => {
                        let mut members = HashMap::new();

                        for mem in &s.members {
                            if !members.contains_key(&mem.name) {
                                members.insert(mem.name.clone(), (mem.typ.clone(), mem.offset));
                            } else {
                                return Err(TypeError::StructMemberConflict(
                                    s.name.clone(),
                                    mem.name.clone(),
                                ));
                            }
                        }

                        if !types.contains_key(&s.name) {
                            types.insert(
                                s.name.clone(),
                                DefinedTypeContent::Struct(StructContent { members }),
                            );
                        } else {
                            return Err(TypeError::TypeConflict(s.name.clone()));
                        }
                    }
                },
            }
        }

        Ok(Self {
            global: EnvGlobal {
                fns,
                vars,
                types,
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
            Self::Struct(s) => write!(f, "struct {}", s.name),
            Self::Incomplete(i) => write!(f, "{i}"),
        }
    }
}
