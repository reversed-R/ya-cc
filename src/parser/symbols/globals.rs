use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        matches,
        symbols::statements::{
            block::BlockStmt,
            var_dec::{consume_scalar_type, VarDec},
            Stmt,
        },
        Parse, ParseError,
    },
    validator::{DefinedType, PrimitiveType, Type},
};

#[derive(Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub members: Vec<(Type, String)>,
}

impl StructType {
    pub fn new(name: String, members: Vec<(Type, String)>) -> Self {
        // TODO: detect member name confliction

        Self { name, members }
    }
}

#[derive(Debug)]
pub enum Globals {
    FnDef(FnDef),
    FnDeclare(FnDeclare),
    VarDec(VarDec),
    TypeDef(TypeDef),
}

impl Globals {
    pub fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Option<Self>, ParseError> {
        let base: Type;

        if let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::Int => {
                    tokens.next();
                    base = Type::Primitive(PrimitiveType::Int);
                }
                TokenKind::Char => {
                    tokens.next();
                    base = Type::Primitive(PrimitiveType::Char);
                }
                TokenKind::Void => {
                    tokens.next();
                    base = Type::Primitive(PrimitiveType::Void);
                }
                TokenKind::Struct => {
                    let name = consume_struct_and_name(tokens)?;

                    let t = tokens
                        .peek()
                        .ok_or(ParseError::InvalidEOF(vec![TokenKind::LBrace]))?;

                    if let TokenKind::LBrace = t.kind {
                        tokens.next();
                        return Ok(Some(Self::TypeDef(TypeDef::Struct(consume_struct_definition_body_from_lbrace_already_appeard_until_semicolon_appears(name, tokens)?))));
                    } else {
                        base = Type::Defined(DefinedType::Struct(name));
                    }
                }
                _ => {
                    return Err(ParseError::InvalidToken(
                        vec![
                            TokenKind::Int,
                            TokenKind::Char,
                            TokenKind::Void,
                            TokenKind::Struct,
                        ],
                        t.to_owned().clone(),
                    ));
                }
            }

            let typ = consume_scalar_type(base, tokens);

            if let TokenKind::Identifier(name) =
                matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
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
pub enum TypeDef {
    Struct(StructType),
    // Enum(EnumType),
    // Typedef(Box<Self>),
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
                let base: Type = match t.kind {
                    TokenKind::Int => {
                        tokens.next();
                        Type::Primitive(PrimitiveType::Int)
                    }
                    TokenKind::Char => {
                        tokens.next();
                        Type::Primitive(PrimitiveType::Char)
                    }
                    TokenKind::Void => {
                        tokens.next();
                        Type::Primitive(PrimitiveType::Void)
                    }
                    _ => {
                        return Err(ParseError::InvalidToken(
                            vec![TokenKind::Int, TokenKind::Char, TokenKind::Void],
                            t.to_owned().clone(),
                        ));
                    }
                };

                let typ = consume_scalar_type(base, tokens);

                if let TokenKind::Identifier(arg) =
                    matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
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

pub fn consume_struct_and_name(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<String, ParseError> {
    let mut name = "".to_string();

    if let TokenKind::Struct = matches(tokens.next(), vec![TokenKind::Struct])? {
        if let TokenKind::Identifier(id) =
            matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
        {
            name = id;
        }
    }

    Ok(name)
}

fn consume_struct_definition_body_from_lbrace_already_appeard_until_semicolon_appears(
    name: String,
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<StructType, ParseError> {
    let mut members = vec![];

    loop {
        let t = tokens.peek().ok_or(ParseError::InvalidEOF(vec![
            TokenKind::RBrace,
            TokenKind::Int,
            TokenKind::Char,
            TokenKind::Void,
            TokenKind::Struct,
        ]))?;

        if let TokenKind::RBrace = t.kind {
            tokens.next();

            matches(tokens.next(), vec![TokenKind::SemiColon])?;

            return Ok(StructType::new(name, members));
        } else {
            let base: Type = match t.kind {
                TokenKind::Int => {
                    tokens.next();
                    Type::Primitive(PrimitiveType::Int)
                }
                TokenKind::Char => {
                    tokens.next();
                    Type::Primitive(PrimitiveType::Char)
                }
                TokenKind::Void => {
                    tokens.next();
                    Type::Primitive(PrimitiveType::Void)
                }
                TokenKind::Struct => {
                    Type::Defined(DefinedType::Struct(consume_struct_and_name(tokens)?))
                }
                _ => {
                    return Err(ParseError::InvalidToken(
                        vec![
                            TokenKind::Int,
                            TokenKind::Char,
                            TokenKind::Void,
                            TokenKind::Struct,
                        ],
                        t.to_owned().clone(),
                    ));
                }
            };

            let typ = consume_scalar_type(base, tokens);

            if let TokenKind::Identifier(id) =
                matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
            {
                members.push((typ, id.clone()));
            }

            let kind = matches(
                tokens.peek().copied(),
                vec![TokenKind::SemiColon, TokenKind::RBrace],
            )?;
            if let TokenKind::SemiColon = kind {
                tokens.next();
            } else if let TokenKind::RBrace = kind {
                continue;
            }
        }
    }
}
