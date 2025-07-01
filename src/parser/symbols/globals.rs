use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::{statements::block::BlockStmt, Stmt};

#[derive(Debug)]
pub struct FnDec {
    pub name: String,
    pub args: Vec<String>,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
struct ArgsDec {
    pub args: Vec<String>,
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

impl Parse for ArgsDec {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(Token::LPare) = tokens.next() {
            let mut args: Vec<String> = vec![];

            while let Some(t) = tokens.peek() {
                if let Token::RPare = t {
                    tokens.next();
                    return Ok(Self { args });
                } else {
                    if let Some(Token::String(arg)) = tokens.peek() {
                        tokens.next();
                        args.push(arg.clone());
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
            Err(ParseError::InvalidToken)
        }
    }
}
