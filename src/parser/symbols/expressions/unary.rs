use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::primary::Primary;

// Unary = ("+" | "-")? Primary
#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOperator,
    pub right: Primary,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,  // +
    Minus, // -
}

impl Parse for Unary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        // Unary = ("+" | "-")? Primary
        if let Some(t) = tokens.peek() {
            match t {
                Token::Plus => {
                    tokens.next();
                    if let Ok(right) = Primary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Plus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::Minus => {
                    tokens.next();
                    if let Ok(right) = Primary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Minus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => {
                    if let Ok(right) = Primary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Plus,
                            right,
                        })
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
