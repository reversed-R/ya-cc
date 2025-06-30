use crate::{
    lexer::token::Token,
    parser::{symbols::expressions::Expr, Parse, ParseError},
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
        if let Ok(expr) = Expr::consume(tokens) {
            if let Some(Token::SemiColon) = tokens.peek() {
                tokens.next();
                Ok(Self { expr })
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
