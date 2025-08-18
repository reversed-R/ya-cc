use crate::{
    lexer::token::{Token, TokenKind},
    parser::{Parse, ParseError},
};

use super::relational::RelationalExpr;

// EqualityExpr = RelationalExpr ("==" RelationalExpr | "!=" RelationalExpr)*
#[derive(Debug, Clone)]
pub struct EqualityExpr {
    pub left: RelationalExpr,
    pub rights: Vec<EqualityExprNode>,
}

#[derive(Debug, Clone)]
pub struct EqualityExprNode {
    pub op: EqualityOperator,
    pub right: RelationalExpr,
}

#[derive(Debug, Clone, Copy)]
pub enum EqualityOperator {
    Equal, // ==
    NotEq, // !=
}

impl Parse for EqualityExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let left = RelationalExpr::consume(tokens)?;
        let mut rights = vec![];

        while let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::Equal => {
                    tokens.next();

                    let right = RelationalExpr::consume(tokens)?;
                    rights.push(EqualityExprNode {
                        op: EqualityOperator::Equal,
                        right,
                    });
                }
                TokenKind::NotEq => {
                    tokens.next();

                    let right = RelationalExpr::consume(tokens)?;
                    rights.push(EqualityExprNode {
                        op: EqualityOperator::NotEq,
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
