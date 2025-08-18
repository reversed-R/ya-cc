use crate::{
    lexer::token::{Token, TokenKind},
    parser::{Parse, ParseError},
};

use super::unary::Unary;

// MulExpr = Primary ("*" Primary | "/" Primary)*
#[derive(Debug, Clone)]
pub struct MulExpr {
    pub left: Unary,
    pub rights: Vec<MulExprNode>,
}

#[derive(Debug, Clone)]
pub struct MulExprNode {
    pub op: MulOperator,
    pub right: Unary,
}

#[derive(Debug, Clone, Copy)]
pub enum MulOperator {
    Mul, // *
    Div, // /
    Mod, // %
}

impl Parse for MulExpr {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let left = Unary::consume(tokens)?;
        let mut rights = vec![];

        while let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::Asterisk => {
                    tokens.next();

                    let right = Unary::consume(tokens)?;
                    rights.push(MulExprNode {
                        op: MulOperator::Mul,
                        right,
                    });
                }
                TokenKind::Slash => {
                    tokens.next();

                    let right = Unary::consume(tokens)?;
                    rights.push(MulExprNode {
                        op: MulOperator::Div,
                        right,
                    });
                }
                TokenKind::Percent => {
                    tokens.next();

                    let right = Unary::consume(tokens)?;
                    rights.push(MulExprNode {
                        op: MulOperator::Mod,
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
