use crate::validator::{DefinedTypeContent, Env};

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
                    } else if matches!(**pointed, Type::Primitive(PrimitiveType::Void)) {
                        TypeComarison::ImplicitlyConvertableFrom
                    } else if matches!(**other_pointed, Type::Primitive(PrimitiveType::Void)) {
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
