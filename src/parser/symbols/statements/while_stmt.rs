use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, symbols::expressions::Expr, Parse, ParseError},
};

use super::Stmt;

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: Expr,
    pub stmt: Stmt,
}

impl Parse for WhileStmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let TokenKind::While = matches(tokens.next(), vec![TokenKind::While])? {
            if let TokenKind::LPare = matches(tokens.next(), vec![TokenKind::LPare])? {
                let cond = Expr::consume(tokens)?;

                if let TokenKind::RPare = matches(tokens.next(), vec![TokenKind::RPare])? {
                    let stmt = Stmt::consume(tokens)?;

                    return Ok(Self { cond, stmt });
                }
            }
        }

        Err(ParseError::Unknown)
    }
}
