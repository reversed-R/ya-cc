use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::expressions::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
}

impl Parse for Stmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Ok(expr) = Expr::consume(tokens) {
            if let Some(Token::SemiColon) = tokens.peek() {
                tokens.next();
                Ok(Self::Expr(expr))
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
