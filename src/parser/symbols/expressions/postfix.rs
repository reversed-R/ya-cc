use crate::{
    lexer::token::Token,
    parser::{
        symbols::expressions::{primary::Primary, Expr},
        Parse, ParseError,
    },
};

#[derive(Debug, Clone)]
pub enum PostfixExpr {
    Primary(Primary),
    Index(Box<PostfixExpr>, Box<Expr>),
}

impl Parse for PostfixExpr {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Ok(prim) = Primary::consume(tokens) {
            if let Some(t) = tokens.peek() {
                match t {
                    Token::LBracket => {
                        tokens.next();

                        if let Ok(expr) = Expr::consume(tokens) {
                            if let Some(Token::RBracket) = tokens.next() {
                                Ok(Self::Index(
                                    Box::new(PostfixExpr::Primary(prim)),
                                    Box::new(expr),
                                ))
                            } else {
                                Err(ParseError::InvalidToken)
                            }
                        } else {
                            Err(ParseError::InvalidToken)
                        }
                    }
                    _ => Ok(Self::Primary(prim)),
                }
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
