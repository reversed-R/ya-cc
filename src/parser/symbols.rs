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
