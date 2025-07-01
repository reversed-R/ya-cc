use crate::{
    lexer::token::Token,
    parser::{symbols::expressions::Expr, Parse, ParseError},
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
        if let Some(Token::While) = tokens.next() {
            if let Some(Token::LPare) = tokens.next() {
                if let Ok(cond) = Expr::consume(tokens) {
                    if let Some(Token::RPare) = tokens.next() {
                        if let Ok(stmt) = Stmt::consume(tokens) {
                            Ok(Self { cond, stmt })
                        } else {
                            Err(ParseError::InvalidToken)
                        }
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                } else {
                    Err(ParseError::InvalidToken)
                }
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
