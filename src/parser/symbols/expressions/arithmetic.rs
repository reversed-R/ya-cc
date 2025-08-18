use crate::{
    lexer::token::{Token, TokenKind},
    parser::{Parse, ParseError},
};

use super::multiplication::MulExpr;

// ArithmExpr = MulExpr ("+" MulExpr | "-" MulExpr)*
#[derive(Debug, Clone)]
pub struct ArithmExpr {
    pub left: MulExpr,
    pub rights: Vec<ArithmExprNode>,
}

#[derive(Debug, Clone)]
pub struct ArithmExprNode {
    pub op: ArithmOperator,
    pub right: MulExpr,
}

#[derive(Debug, Clone, Copy)]
pub enum ArithmOperator {
    Add, // +
    Sub, // -
}

impl Parse for ArithmExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let left = MulExpr::consume(tokens)?;
        let mut rights = vec![];

        while let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::Plus => {
                    tokens.next();

                    let right = MulExpr::consume(tokens)?;
                    rights.push(ArithmExprNode {
                        op: ArithmOperator::Add,
                        right,
                    });
                }
                TokenKind::Minus => {
                    tokens.next();

                    let right = MulExpr::consume(tokens)?;
                    rights.push(ArithmExprNode {
                        op: ArithmOperator::Sub,
                        right,
                    });
                }
                _ => {
                    return Ok(Self { left, rights });
                }
            }
        }

        Ok(Self { left, rights })
    }
}
