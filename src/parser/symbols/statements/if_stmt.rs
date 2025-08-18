use crate::{
    lexer::token::{Token, TokenKind},
    parser::{matches, symbols::expressions::Expr, Parse, ParseError},
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
        if let TokenKind::If = matches(tokens.next(), vec![TokenKind::If])? {
            if let TokenKind::LPare = matches(tokens.next(), vec![TokenKind::LPare])? {
                let cond = Expr::consume(tokens)?;

                if let TokenKind::RPare = matches(tokens.next(), vec![TokenKind::RPare])? {
                    let then = Stmt::consume(tokens)?;

                    if let Some(t) = tokens.peek() {
                        if let TokenKind::Else = t.kind {
                            tokens.next();

                            let els = Stmt::consume(tokens)?;

                            return Ok(Self {
                                cond,
                                then,
                                els: Some(els),
                            });
                        } else {
                            return Ok(Self {
                                cond,
                                then,
                                els: None,
                            });
                        }
                    } else {
                        return Ok(Self {
                            cond,
                            then,
                            els: None,
                        });
                    }
                }
            }
        }

        Err(ParseError::Unknown)
    }
}
