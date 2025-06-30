pub mod expr;

use expr::ExprStmt;

use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::expressions::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
}

impl Parse for Stmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(t) = tokens.peek() {
            match t {
                Token::Return => {
                    tokens.next();
                    // same process as expr stmt
                    if let Ok(expr) = ExprStmt::consume(tokens) {
                        Ok(Self::Return(expr.expr))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => {
                    if let Ok(expr) = ExprStmt::consume(tokens) {
                        Ok(Self::Expr(expr.expr))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
