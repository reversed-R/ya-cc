use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, Parse, ParseError},
    validator::{PrimitiveType, Type},
};

#[derive(Debug, Clone)]
pub struct VarDec {
    pub typ: Type,
    pub name: String,
}

impl Parse for VarDec {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let base: Type;

        if let Some(t) = tokens.next() {
            match t.kind {
                TokenKind::Int => {
                    base = Type::Primitive(PrimitiveType::Int);
                }
                TokenKind::Char => {
                    base = Type::Primitive(PrimitiveType::Char);
                }
                _ => {
                    return Err(ParseError::InvalidToken(
                        vec![TokenKind::Int, TokenKind::Char],
                        t.clone(),
                    ));
                }
            }

            let typ = consume_scalar_type(base, tokens);

            if let TokenKind::Identifier(id) =
                matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
            {
                let kind = matches(
                    tokens.peek().copied(),
                    vec![TokenKind::LBracket, TokenKind::SemiColon],
                )?;
                if let TokenKind::LBracket = kind {
                    tokens.next();

                    if let TokenKind::IntLiteral(i) =
                        matches(tokens.next(), vec![TokenKind::IntLiteral(0)])?
                    {
                        if let TokenKind::RBracket =
                            matches(tokens.next(), vec![TokenKind::RBracket])?
                        {
                            if let TokenKind::SemiColon =
                                matches(tokens.next(), vec![TokenKind::SemiColon])?
                            {
                                return Ok(Self {
                                    typ: Type::Array(Box::new(typ), i as usize),
                                    name: id.clone(),
                                });
                            }
                        }
                    }
                } else if let TokenKind::SemiColon = kind {
                    tokens.next();

                    return Ok(Self {
                        typ,
                        name: id.clone(),
                    });
                }
            }
        }

        Err(ParseError::InvalidEOF(vec![
            TokenKind::Int,
            TokenKind::Char,
        ]))
    }
}

pub fn consume_scalar_type(
    base: Type,
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Type {
    let ptr_count = consume_ptr_dec(tokens);
    let mut typ = base;

    for _ in 0..ptr_count {
        typ = Type::PtrTo(Box::new(typ));
    }

    typ
}

fn consume_ptr_dec(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> usize {
    let mut count = 0;

    while matches!(
        tokens.peek(),
        Some(Token {
            kind: TokenKind::Asterisk,
            range: _
        })
    ) {
        count += 1;
        tokens.next();
    }

    count
}
