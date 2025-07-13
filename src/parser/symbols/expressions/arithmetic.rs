use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::multiplication::MulExpr;

// ArithmExpr = MulExpr ("+" MulExpr | "-" MulExpr)*
#[derive(Debug)]
pub struct ArithmExpr {
    pub left: MulExpr,
    pub rights: Vec<ArithmExprNode>,
}

#[derive(Debug)]
pub struct ArithmExprNode {
    pub op: ArithmOperator,
    pub right: MulExpr,
}

#[derive(Debug)]
pub enum ArithmOperator {
    Add, // +
    Sub, // -
}

impl ArithmExpr {
    pub fn new(mul: MulExpr) -> Self {
        Self {
            left: mul,
            rights: vec![],
        }
    }

    fn push(&mut self, op: ArithmOperator, right: MulExpr) {
        self.rights.push(ArithmExprNode { op, right });
    }
}

impl Parse for ArithmExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut arithm: Self;

        if let Ok(mul) = MulExpr::consume(tokens) {
            arithm = Self::new(mul);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Plus => {
                        tokens.next();
                        if let Ok(right) = MulExpr::consume(tokens) {
                            arithm.push(ArithmOperator::Add, right);
                        }
                    }
                    Token::Minus => {
                        tokens.next();
                        if let Ok(right) = MulExpr::consume(tokens) {
                            arithm.push(ArithmOperator::Sub, right);
                        }
                    }
                    _ => {
                        return Ok(arithm);
                    }
                }
            }

            Ok(arithm)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
