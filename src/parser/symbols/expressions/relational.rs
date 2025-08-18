use crate::{
    lexer::token::{Token, TokenKind},
    parser::{Parse, ParseError},
};

use super::arithmetic::ArithmExpr;

// RelationalExpr = ArithmExpr ("<" ArithmExpr | ">" ArithmExpr | "<=" ArithmExpr | ">=" ArithmExpr)*
#[derive(Debug, Clone)]
pub struct RelationalExpr {
    pub left: ArithmExpr,
    pub rights: Vec<RelationalExprNode>,
}

#[derive(Debug, Clone)]
pub struct RelationalExprNode {
    pub op: RelationalOperator,
    pub right: ArithmExpr,
}

#[derive(Debug, Clone, Copy)]
pub enum RelationalOperator {
    Lesser,  // <
    Greater, // >
    LesEq,   // <=
    GrtEq,   // >=
}

impl Parse for RelationalExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let left = ArithmExpr::consume(tokens)?;
        let mut rights = vec![];

        while let Some(t) = tokens.peek() {
            match t.kind {
                TokenKind::Lesser => {
                    tokens.next();

                    let right = ArithmExpr::consume(tokens)?;
                    rights.push(RelationalExprNode {
                        op: RelationalOperator::Lesser,
                        right,
                    });
                }
                TokenKind::Greater => {
                    tokens.next();

                    let right = ArithmExpr::consume(tokens)?;
                    rights.push(RelationalExprNode {
                        op: RelationalOperator::Greater,
                        right,
                    });
                }
                TokenKind::LesEq => {
                    tokens.next();

                    let right = ArithmExpr::consume(tokens)?;
                    rights.push(RelationalExprNode {
                        op: RelationalOperator::LesEq,
                        right,
                    });
                }
                TokenKind::GrtEq => {
                    tokens.next();

                    let right = ArithmExpr::consume(tokens)?;
                    rights.push(RelationalExprNode {
                        op: RelationalOperator::GrtEq,
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
