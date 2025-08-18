use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, symbols::expressions::Expr, Parse, ParseError},
};

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl Parse for ExprStmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let expr = Expr::consume(tokens)?;

        if let TokenKind::SemiColon = matches(tokens.next(), vec![TokenKind::SemiColon])? {
            Ok(Self { expr })
        } else {
            Err(ParseError::Unknown)
        }
    }
}
