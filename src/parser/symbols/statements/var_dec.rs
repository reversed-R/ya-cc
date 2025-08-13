use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
    validator::{PrimitiveType, Type},
};

#[derive(Debug)]
pub struct VarDec {
    pub typ: Type,
    pub name: String,
}

impl Parse for VarDec {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let primitive: PrimitiveType;

        if let Some(t) = tokens.next() {
            match t {
                Token::Int => {
                    primitive = PrimitiveType::Int;
                }
                Token::Char => {
                    primitive = PrimitiveType::Char;
                }
                _ => {
                    return Err(ParseError::InvalidToken);
                }
            }

            let typ = consume_scalar_type(primitive, tokens);

            if let Some(Token::String(id)) = tokens.next() {
                if let Some(Token::LBracket) = tokens.peek() {
                    tokens.next();

                    if let Some(Token::IntLiteral(i)) = tokens.next() {
                        if let Some(Token::RBracket) = tokens.next() {
                            if let Some(Token::SemiColon) = tokens.next() {
                                Ok(Self {
                                    typ: Type::Array(Box::new(typ), *i as usize),
                                    name: id.clone(),
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
                } else if let Some(Token::SemiColon) = tokens.next() {
                    Ok(Self {
                        typ,
                        name: id.clone(),
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
    }
}

pub fn consume_scalar_type(
    primitive: PrimitiveType,
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Type {
    let ptr_count = consume_ptr_dec(tokens);
    let mut typ = Type::Primitive(primitive);

    for _ in 0..ptr_count {
        typ = Type::PtrTo(Box::new(typ));
    }

    typ
}

fn consume_ptr_dec(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> usize {
    let mut count = 0;

    while let Some(Token::Asterisk) = tokens.peek() {
        count += 1;
        tokens.next();
    }

    count
}
