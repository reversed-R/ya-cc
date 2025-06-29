use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::arithmetic::ArithmExpr;

// Primary = Literal | "(" ArithmExpr ")"
#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Expr(Box<ArithmExpr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

impl Parse for Primary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        // Primary = Literal | "(" Expr ")"
        if let Some(t) = tokens.next() {
            match t {
                Token::IntLiteral(i) => Ok(Self::Literal(Literal::Int(*i))),
                Token::LPare => {
                    if let Ok(expr) = ArithmExpr::consume(tokens) {
                        if let Some(Token::RPare) = tokens.next() {
                            Ok(Self::Expr(Box::new(expr)))
                        } else {
                            Err(ParseError::InvalidToken)
                        }
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => Err(ParseError::InvalidToken),
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
