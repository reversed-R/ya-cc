use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, Parse, ParseError},
};

use super::Stmt;

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

impl Parse for BlockStmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let TokenKind::LBrace = matches(tokens.next(), vec![TokenKind::LBrace])? {
            let mut stmts: Vec<Stmt> = vec![];

            loop {
                if let Some(t) = tokens.peek() {
                    if let TokenKind::RBrace = t.kind {
                        tokens.next();
                        return Ok(Self { stmts });
                    }
                } else {
                    return Err(ParseError::InvalidEOF(vec![TokenKind::RBrace]));
                }

                let stmt = Stmt::consume(tokens)?;

                stmts.push(stmt);
            }
        }

        Err(ParseError::Unknown)
    }
}
