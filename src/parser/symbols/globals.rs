use crate::{
    lexer::token::Token,
    parser::{
        symbols::{statements::var_dec::VarDec, PrimitiveType, Type},
        Parse, ParseError,
    },
};

use super::{statements::block::BlockStmt, Stmt};

#[derive(Debug)]
pub struct FnDec {
    pub name: String,
    pub args: Vec<VarDec>,
    pub stmts: Vec<Stmt>,
}

impl Parse for FnDec {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(Token::String(name)) = tokens.next() {
            if let Some(Token::LPare) = tokens.peek() {
                if let Ok(args) = ArgsDec::consume(tokens) {
                    if let Ok(block) = BlockStmt::consume(tokens) {
                        Ok(Self {
                            name: name.clone(),
                            args: args.args,
                            stmts: block.stmts,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                } else {
                    Err(ParseError::InvalidToken)
                }
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

#[derive(Debug)]
struct ArgsDec {
    pub args: Vec<VarDec>,
}

impl Parse for ArgsDec {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(Token::LPare) = tokens.next() {
            let mut args: Vec<VarDec> = vec![];

            while let Some(t) = tokens.peek() {
                if let Token::RPare = t {
                    tokens.next();
                    return Ok(Self { args });
                } else {
                    let primitive: PrimitiveType;

                    match t {
                        Token::Int => {
                            tokens.next();
                            primitive = PrimitiveType::Int;
                        }
                        _ => {
                            return Err(ParseError::InvalidToken);
                        }
                    }

                    if let Some(Token::String(arg)) = tokens.next() {
                        args.push(VarDec {
                            typ: Type::Primitive(primitive),
                            name: arg.clone(),
                        });
                    }

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
                }
            }

            Err(ParseError::InvalidToken)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
