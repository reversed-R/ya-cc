use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::arithmetic::ArithmExpr;

// RelationalExpr = ArithmExpr ("<" ArithmExpr | ">" ArithmExpr | "<=" ArithmExpr | ">=" ArithmExpr)*
#[derive(Debug)]
pub struct RelationalExpr {
    pub nodes: Vec<RelationalExprNode>,
}

#[derive(Debug)]
pub struct RelationalExprNode {
    pub op: RelationalOperator, // `op` of the head (index 0th) element does not have meaning, just
    // a placeholder
    pub right: ArithmExpr,
}

#[derive(Debug)]
pub enum RelationalOperator {
    Lesser,  // <
    Greater, // >
    LesEq,   // <=
    GrtEq,   // >=
}

impl RelationalExpr {
    pub fn new(arithm: ArithmExpr) -> Self {
        Self {
            nodes: vec![RelationalExprNode {
                op: RelationalOperator::Lesser,
                right: arithm,
            }],
        }
    }

    fn push(&mut self, op: RelationalOperator, right: ArithmExpr) {
        self.nodes.push(RelationalExprNode { op, right });
    }
}

impl Parse for RelationalExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut relat: Self;

        if let Ok(arithm) = ArithmExpr::consume(tokens) {
            relat = Self::new(arithm);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Lesser => {
                        tokens.next();
                        if let Ok(right) = ArithmExpr::consume(tokens) {
                            relat.push(RelationalOperator::Lesser, right);
                        }
                    }
                    Token::Greater => {
                        tokens.next();
                        if let Ok(right) = ArithmExpr::consume(tokens) {
                            relat.push(RelationalOperator::Greater, right);
                        }
                    }
                    Token::LesEq => {
                        tokens.next();
                        if let Ok(right) = ArithmExpr::consume(tokens) {
                            relat.push(RelationalOperator::LesEq, right);
                        }
                    }
                    Token::GrtEq => {
                        tokens.next();
                        if let Ok(right) = ArithmExpr::consume(tokens) {
                            relat.push(RelationalOperator::GrtEq, right);
                        }
                    }
                    _ => {
                        return Ok(relat);
                    }
                }
            }

            Ok(relat)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
