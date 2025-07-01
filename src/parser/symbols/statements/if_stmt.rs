use crate::{
    lexer::token::Token,
    parser::{symbols::expressions::Expr, Parse, ParseError},
};

use super::Stmt;

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Stmt,
    pub els: Option<Stmt>,
}

impl Parse for IfStmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(Token::If) = tokens.next() {
            if let Some(Token::LPare) = tokens.next() {
                if let Ok(cond) = Expr::consume(tokens) {
                    if let Some(Token::RPare) = tokens.next() {
                        if let Ok(then) = Stmt::consume(tokens) {
                            if let Some(Token::Else) = tokens.peek() {
                                tokens.next();
                                if let Ok(els) = Stmt::consume(tokens) {
                                    Ok(Self {
                                        cond,
                                        then,
                                        els: Some(els),
                                    })
                                } else {
                                    Err(ParseError::InvalidToken)
                                }
                            } else {
                                Ok(Self {
                                    cond,
                                    then,
                                    els: None,
                                })
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
            } else {
                Err(ParseError::InvalidToken)
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
