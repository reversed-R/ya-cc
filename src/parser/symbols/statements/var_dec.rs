use crate::{
    lexer::token::Token,
    parser::{
        symbols::{PrimitiveType, Type},
        Parse, ParseError,
    },
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

                    if let Some(Token::String(id)) = tokens.next() {
                        if let Some(Token::SemiColon) = tokens.next() {
                            Ok(Self {
                                typ: Type::Primitive(primitive),
                                name: id.clone(),
                            })
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
