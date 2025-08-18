pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod postfix;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use assignment::AssignExpr;

#[derive(Debug, Clone)]
pub struct Expr(pub AssignExpr);

impl Parse for Expr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let assign = AssignExpr::consume(tokens)?;

        Ok(Self(assign))
    }
}
