use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        matches,
        symbols::statements::var_dec::{consume_scalar_type, VarDec},
        Parse, ParseError,
    },
    validator::{PrimitiveType, Type},
};

use super::{statements::block::BlockStmt, Stmt};

#[derive(Debug)]
pub enum Globals {
    FnDef(FnDef),
    FnDeclare(FnDeclare),
    VarDec(VarDec),
}

impl Globals {
    pub fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Option<Self>, ParseError> {
        let primitive: PrimitiveType;

        if let Some(t) = tokens.next() {
            match t.kind {
                TokenKind::Int => {
                    primitive = PrimitiveType::Int;
                }
                TokenKind::Char => {
                    primitive = PrimitiveType::Char;
                }
                TokenKind::Void => {
                    primitive = PrimitiveType::Void;
                }
                _ => {
                    return Err(ParseError::InvalidToken(
                        vec![TokenKind::Int, TokenKind::Char, TokenKind::Void],
                        t.clone(),
                    ));
                }
            }

            let typ = consume_scalar_type(primitive, tokens);

            if let TokenKind::String(name) =
                matches(tokens.next(), vec![TokenKind::String("".to_string())])?
            {
                let kind = matches(
                    tokens.peek().copied(),
                    vec![TokenKind::LPare, TokenKind::SemiColon],
                )?;

                if let TokenKind::LPare = kind {
                    let args = ArgsDec::consume(tokens)?;

                    let kind = matches(
                        tokens.peek().copied(),
                        vec![TokenKind::SemiColon, TokenKind::LBrace],
                    )?;

                    if let TokenKind::SemiColon = kind {
                        tokens.next();

                        return Ok(Some(Self::FnDeclare(FnDeclare {
                            name: name.clone(),
                            args: args.args,
                            rtype: typ,
                        })));
                    } else if let TokenKind::LBrace = kind {
                        return Ok(Some(Self::FnDef(FnDef {
                            name: name.clone(),
                            args: args.args,
                            stmts: BlockStmt::consume(tokens)?.stmts,
                            rtype: typ,
                        })));
                    }
                } else if let TokenKind::SemiColon = kind {
                    tokens.next();

                    return Ok(Some(Self::VarDec(VarDec {
                        name: name.clone(),
                        typ,
                    })));
                }
            }
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub struct FnDeclare {
    pub name: String,
    pub args: Vec<VarDec>,
    pub rtype: Type,
}

#[derive(Debug)]
pub struct FnDef {
    pub name: String,
    pub args: Vec<VarDec>,
    pub stmts: Vec<Stmt>,
    pub rtype: Type,
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
        matches(tokens.next(), vec![TokenKind::LPare])?;

        let mut args: Vec<VarDec> = vec![];

        loop {
            let t = tokens.peek().ok_or(ParseError::InvalidEOF(vec![
                TokenKind::RPare,
                TokenKind::Int,
                TokenKind::Char,
                TokenKind::Void,
            ]))?;

            if let TokenKind::RPare = t.kind {
                tokens.next();
                return Ok(Self { args });
            } else {
                let primitive: PrimitiveType;

                match t.kind {
                    TokenKind::Int => {
                        tokens.next();
                        primitive = PrimitiveType::Int;
                    }
                    TokenKind::Char => {
                        tokens.next();
                        primitive = PrimitiveType::Char;
                    }
                    TokenKind::Void => {
                        tokens.next();
                        primitive = PrimitiveType::Void;
                    }
                    _ => {
                        return Err(ParseError::InvalidToken(
                            vec![TokenKind::Int, TokenKind::Char, TokenKind::Void],
                            t.clone().clone(),
                        ));
                    }
                }

                let typ = consume_scalar_type(primitive, tokens);

                if let TokenKind::String(arg) =
                    matches(tokens.next(), vec![TokenKind::String("".to_string())])?
                {
                    args.push(VarDec {
                        typ,
                        name: arg.clone(),
                    });
                }

                let kind = matches(
                    tokens.peek().copied(),
                    vec![TokenKind::Comma, TokenKind::RPare],
                )?;
                if let TokenKind::Comma = kind {
                    tokens.next();
                } else if let TokenKind::RPare = kind {
                    continue;
                }
            }
        }
    }
}
