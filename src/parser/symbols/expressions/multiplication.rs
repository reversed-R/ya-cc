use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::unary::Unary;

// MulExpr = Primary ("*" Primary | "/" Primary)*
#[derive(Debug)]
pub struct MulExpr {
    nodes: Vec<MulExprNode>,
}

#[derive(Debug)]
pub struct MulExprNode {
    pub op: MulOperator, // `op` of the head (index 0th) element does not have meaning, just
    // a placeholder
    pub right: Unary,
}

#[derive(Debug)]
pub enum MulOperator {
    Mul, // *
    Div, // /
}

impl MulExpr {
    pub fn new(unary: Unary) -> Self {
        Self {
            nodes: vec![MulExprNode {
                op: MulOperator::Mul,
                right: unary,
            }],
        }
    }

    fn push(&mut self, op: MulOperator, right: Unary) {
        self.nodes.push(MulExprNode { op, right });
    }
}

impl Parse for MulExpr {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut mul: Self;

        if let Ok(unary) = Unary::consume(tokens) {
            mul = Self::new(unary);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Asterisk => {
                        tokens.next();
                        if let Ok(right) = Unary::consume(tokens) {
                            mul.push(MulOperator::Mul, right);
                        }
                    }
                    Token::Slash => {
                        tokens.next();
                        if let Ok(right) = Unary::consume(tokens) {
                            mul.push(MulOperator::Div, right);
                        }
                    }
                    _ => {
                        return Ok(mul);
                    }
                }
            }

            Ok(mul)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
