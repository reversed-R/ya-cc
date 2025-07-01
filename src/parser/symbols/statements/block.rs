use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
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
        if let Some(Token::LBrace) = tokens.next() {
            let mut stmts: Vec<Stmt> = vec![];

            while let Some(t) = tokens.peek() {
                if let Token::RBrace = t {
                    tokens.next();
                    return Ok(Self { stmts });
                } else {
                    if let Ok(stmt) = Stmt::consume(tokens) {
                        stmts.push(stmt);
                    } else {
                        return Err(ParseError::InvalidToken);
                    }
                }
            }

            Err(ParseError::InvalidToken)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
