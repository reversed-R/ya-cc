pub mod arithmetic;
pub mod equality;
pub mod multiplication;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use equality::EqualityExpr;

#[derive(Debug)]
pub struct Expr(EqualityExpr);

impl Parse for Expr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Ok(equal) = EqualityExpr::consume(tokens) {
            Ok(Self(equal))
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
