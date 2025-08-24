use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        matches,
        symbols::expressions::{primary::Primary, Expr},
        Parse, ParseError,
    },
};

#[derive(Debug, Clone)]
pub enum PostfixExpr {
    Primary(Primary),
    Index(Box<PostfixExpr>, Box<Expr>),
    DotAccess(Primary, String),
    ArrowAccess(Primary, String),
}

impl Parse for PostfixExpr {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let prim = Primary::consume(tokens)?;

        if let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::LBracket => {
                    tokens.next();

                    let expr = Expr::consume(tokens)?;

                    if let TokenKind::RBracket = matches(tokens.next(), vec![TokenKind::RBracket])?
                    {
                        Ok(Self::Index(
                            Box::new(PostfixExpr::Primary(prim)),
                            Box::new(expr),
                        ))
                    } else {
                        Err(ParseError::Unknown)
                    }
                }
                TokenKind::Dot => {
                    tokens.next();
                    if let TokenKind::Identifier(member) =
                        matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
                    {
                        Ok(Self::DotAccess(prim, member))
                    } else {
                        Err(ParseError::Unknown)
                    }
                }
                TokenKind::Arrow => {
                    tokens.next();
                    if let TokenKind::Identifier(member) =
                        matches(tokens.next(), vec![TokenKind::Identifier("".to_string())])?
                    {
                        Ok(Self::ArrowAccess(prim, member))
                    } else {
                        Err(ParseError::Unknown)
                    }
                }

                _ => Ok(Self::Primary(prim)),
            }
        } else {
            Ok(Self::Primary(prim))
        }
    }
}
