use crate::{
    lexer::token::Token,
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

impl RelationalExpr {
    pub fn new(arithm: ArithmExpr) -> Self {
        Self {
            left: arithm,
            rights: vec![],
        }
    }

    fn push(&mut self, op: RelationalOperator, right: ArithmExpr) {
        self.rights.push(RelationalExprNode { op, right });
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
