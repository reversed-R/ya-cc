pub mod symbols;

use std::{iter::Peekable, slice::Iter};

use crate::lexer::token::{Token, TokenKind};
use symbols::Program;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(Vec<TokenKind>, Token), // expected TokenKind, ... or TokenKind, but found Token in Token.range
    InvalidEOF(Vec<TokenKind>),          // expected TokenKind, ... or TokenKind, but found EOF
    Unknown,                             //
}

trait Parse {
    type SelfType;

    fn consume(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Self::SelfType, ParseError>;
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    Program::consume(&mut tokens.iter().peekable())
}

pub fn matches(opt_t: Option<&Token>, kinds: Vec<TokenKind>) -> Result<TokenKind, ParseError> {
    let t: &Token = opt_t.ok_or(ParseError::InvalidEOF(kinds.clone()))?;

    for kind in &kinds {
        if std::mem::discriminant(&t.kind) == std::mem::discriminant(kind) {
            return Ok(t.kind.clone());
        }
    }

    Err(ParseError::InvalidToken(kinds, t.clone()))
}
