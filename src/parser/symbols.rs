pub mod expressions;
pub mod globals;
pub mod statements;

use crate::{
    lexer::token::Token,
    parser::{symbols::globals::Globals, Parse, ParseError},
};

#[derive(Debug)]
pub struct Program {
    pub globals: Vec<Globals>, // pub fns: Vec<FnDec>,
}

impl Parse for Program {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut prog = Self { globals: vec![] };

        while let Some(global) = Globals::consume(tokens)? {
            prog.globals.push(global);
        }

        Ok(prog)
    }
}
