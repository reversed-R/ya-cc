use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};
use statements::Stmt;

pub mod expressions;
pub mod statements;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Parse for Program {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut prog = Self { stmts: vec![] };

        while let Ok(stmt) = Stmt::consume(tokens) {
            prog.stmts.push(stmt);
        }

        Ok(prog)
    }
}
