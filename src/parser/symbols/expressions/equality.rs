use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::relational::RelationalExpr;

// EqualityExpr = RelationalExpr ("==" RelationalExpr | "!=" RelationalExpr)*
#[derive(Debug)]
pub struct EqualityExpr {
    pub left: RelationalExpr,
    pub rights: Vec<EqualityExprNode>,
}

#[derive(Debug)]
pub struct EqualityExprNode {
    pub op: EqualityOperator,
    pub right: RelationalExpr,
}

#[derive(Debug)]
pub enum EqualityOperator {
    Equal, // ==
    NotEq, // !=
}

impl EqualityExpr {
    pub fn new(relat: RelationalExpr) -> Self {
        Self {
            left: relat,
            rights: vec![],
        }
    }

    fn push(&mut self, op: EqualityOperator, right: RelationalExpr) {
        self.rights.push(EqualityExprNode { op, right });
    }
}

impl Parse for EqualityExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut equal: Self;

        if let Ok(relat) = RelationalExpr::consume(tokens) {
            equal = Self::new(relat);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Equal => {
                        tokens.next();
                        if let Ok(right) = RelationalExpr::consume(tokens) {
                            equal.push(EqualityOperator::Equal, right);
                        }
                    }
                    Token::NotEq => {
                        tokens.next();
                        if let Ok(right) = RelationalExpr::consume(tokens) {
                            equal.push(EqualityOperator::NotEq, right);
                        }
                    }
                    _ => {
                        return Ok(equal);
                    }
                }
            }

            Ok(equal)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
