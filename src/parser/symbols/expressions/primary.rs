use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::Expr;

// Primary = Literal | Identifier ( "(" ")" )? | "(" ArithmExpr ")"
#[derive(Debug, Clone)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
    FnCall(FnCall),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
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
                Token::String(s) => {
                    if let Some(Token::LPare) = tokens.peek() {
                        tokens.next();

                        let mut args: Vec<Expr> = vec![];

                        while let Some(t) = tokens.peek() {
                            if let Token::RPare = t {
                                tokens.next();
                                return Ok(Self::FnCall(FnCall {
                                    name: s.clone(),
                                    args,
                                }));
                            } else {
                                if let Ok(expr) = Expr::consume(tokens) {
                                    args.push(expr);
                                    if let Some(t) = tokens.peek() {
                                        match t {
                                            Token::Comma => {
                                                tokens.next();
                                            }
                                            Token::RPare => {
                                                continue;
                                            }
                                            _ => {
                                                return Err(ParseError::InvalidToken);
                                            }
                                        }
                                    } else {
                                        return Err(ParseError::InvalidToken);
                                    }
                                } else {
                                    return Err(ParseError::InvalidToken);
                                }
                            }
                        }

                        Err(ParseError::InvalidToken)
                    } else {
                        Ok(Self::Identifier(s.clone()))
                    }
                }
                Token::LPare => {
                    if let Ok(expr) = Expr::consume(tokens) {
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
