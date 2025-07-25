pub mod expressions;
pub mod globals;
pub mod statements;

use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};
use globals::FnDec;
use statements::Stmt;

#[derive(Debug)]
pub struct Program {
    pub fns: Vec<FnDec>,
}

impl Parse for Program {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut prog = Self { fns: vec![] };

        while let Ok(fn_dec) = FnDec::consume(tokens) {
            prog.fns.push(fn_dec);
        }

        Ok(prog)
    }
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
}

impl PrimitiveType {
    pub fn aligned_size(&self) -> usize {
        match self {
            Self::Int => 8,
        }
    }
}

#[derive(Debug)]
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
}
