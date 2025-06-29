pub mod symbols;

use std::{iter::Peekable, slice::Iter};

use crate::lexer::token::Token;
use symbols::expr::Expr;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken,
}

trait Parse {
    type SelfType;

    fn consume(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Self::SelfType, ParseError>;
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    Expr::consume(&mut tokens.iter().peekable())
}
