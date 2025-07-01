pub mod expr;
pub mod if_stmt;
pub mod while_stmt;

use expr::ExprStmt;
use if_stmt::IfStmt;
use while_stmt::WhileStmt;

use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::expressions::Expr;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Return(Expr),
    If(Box<IfStmt>),
    While(Box<WhileStmt>),
}

impl Parse for Stmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(t) = tokens.peek() {
            match t {
                Token::If => {
                    if let Ok(if_stmt) = IfStmt::consume(tokens) {
                        Ok(Self::If(Box::new(if_stmt)))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::While => {
                    if let Ok(while_stmt) = WhileStmt::consume(tokens) {
                        Ok(Self::While(Box::new(while_stmt)))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::Return => {
                    tokens.next();
                    // same process as expr stmt
                    if let Ok(expr) = ExprStmt::consume(tokens) {
                        Ok(Self::Return(expr.expr))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::LBrace => {
                    tokens.next();
                    let mut stmts: Vec<Self> = vec![];

                    while let Some(t) = tokens.peek() {
                        if let Token::RBrace = t {
                            tokens.next();
                            return Ok(Self::Block(stmts));
                        } else {
                            if let Ok(stmt) = Stmt::consume(tokens) {
                                stmts.push(stmt);
                            } else {
                                return Err(ParseError::InvalidToken);
                            }
                        }
                    }

                    Err(ParseError::InvalidToken)
                }
                _ => {
                    if let Ok(expr) = ExprStmt::consume(tokens) {
                        Ok(Self::Expr(expr.expr))
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
