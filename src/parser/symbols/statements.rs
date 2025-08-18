pub mod block;
pub mod expr;
pub mod if_stmt;
pub mod var_dec;
pub mod while_stmt;

use block::BlockStmt;
use expr::ExprStmt;
use if_stmt::IfStmt;
use while_stmt::WhileStmt;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{symbols::statements::var_dec::VarDec, Parse, ParseError},
};

use super::expressions::Expr;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Return(Expr),
    If(Box<IfStmt>),
    While(Box<WhileStmt>),
    VarDec(VarDec),
}

impl Parse for Stmt {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::If => Ok(Self::If(Box::new(IfStmt::consume(tokens)?))),
                TokenKind::While => Ok(Self::While(Box::new(WhileStmt::consume(tokens)?))),
                TokenKind::Return => {
                    tokens.next();
                    // same process as expr stmt
                    Ok(Self::Return(ExprStmt::consume(tokens)?.expr))
                }
                TokenKind::LBrace => Ok(Self::Block(BlockStmt::consume(tokens)?.stmts)),
                TokenKind::Int => Ok(Self::VarDec(VarDec::consume(tokens)?)),
                TokenKind::Char => Ok(Self::VarDec(VarDec::consume(tokens)?)),
                _ => Ok(Self::Expr(ExprStmt::consume(tokens)?.expr)),
            }
        } else {
            Err(ParseError::InvalidEOF(vec![
                TokenKind::If,
                TokenKind::While,
                TokenKind::Return,
                TokenKind::Int,
            ]))
        }
    }
}
