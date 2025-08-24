use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, Parse, ParseError},
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
    Char(u8),
    // Float(f64),
    String(String),
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
            match &t.kind {
                TokenKind::IntLiteral(i) => Ok(Self::Literal(Literal::Int(*i))),
                TokenKind::CharLiteral(c) => Ok(Self::Literal(Literal::Char(*c))),
                TokenKind::StringLiteral(s) => Ok(Self::Literal(Literal::String(s.clone()))),
                TokenKind::Identifier(s) => {
                    if let Some(t) = tokens.peek() {
                        if let TokenKind::LPare = t.kind {
                            tokens.next();

                            let mut args: Vec<Expr> = vec![];

                            while let Some(t) = tokens.peek() {
                                if let TokenKind::RPare = t.kind {
                                    tokens.next();
                                    return Ok(Self::FnCall(FnCall {
                                        name: s.clone(),
                                        args,
                                    }));
                                } else {
                                    let expr = Expr::consume(tokens)?;
                                    args.push(expr);

                                    let kind = matches(
                                        tokens.peek().copied(),
                                        vec![TokenKind::RPare, TokenKind::Comma],
                                    )?;
                                    if let TokenKind::Comma = kind {
                                        tokens.next();
                                        continue;
                                    } else if let TokenKind::RPare = kind {
                                        continue;
                                    }
                                }
                            }

                            Err(ParseError::InvalidEOF(vec![TokenKind::RPare]))
                        } else {
                            Ok(Self::Identifier(s.clone()))
                        }
                    } else {
                        Ok(Self::Identifier(s.clone()))
                    }
                }
                TokenKind::LPare => {
                    let expr = Expr::consume(tokens)?;

                    if let TokenKind::RPare = matches(tokens.next(), vec![TokenKind::RPare])? {
                        Ok(Self::Expr(Box::new(expr)))
                    } else {
                        Err(ParseError::InvalidToken(
                            vec![
                                TokenKind::Identifier("".to_string()),
                                TokenKind::IntLiteral(0),
                            ],
                            tokens.peek().unwrap().to_owned().clone(),
                        ))
                    }
                }
                _ => Err(ParseError::InvalidEOF(vec![
                    TokenKind::Identifier("".to_string()),
                    TokenKind::IntLiteral(0),
                ])),
            }
        } else {
            Err(ParseError::InvalidEOF(vec![
                TokenKind::Identifier("".to_string()),
                TokenKind::IntLiteral(0),
            ]))
        }
    }
}
