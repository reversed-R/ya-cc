use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

use super::equality::EqualityExpr;

// EqualityExpr = RelationalExpr ("==" RelationalExpr | "!=" RelationalExpr)*
#[derive(Debug)]
pub struct AssignExpr {
    pub left: EqualityExpr,
    pub rights: Vec<AssignExprNode>,
    // now, only = can be used, but +=, -=, and so on will be used in the future
    // so, node style data structure has a meaning
    // (if only one operator can be used, Vec<Equality> is ok)
}

#[derive(Debug)]
pub struct AssignExprNode {
    pub op: AssignOperator,
    pub right: EqualityExpr,
}

#[derive(Debug)]
pub enum AssignOperator {
    Assign, // =
}

impl AssignExpr {
    pub fn new(equal: EqualityExpr) -> Self {
        Self {
            left: equal,
            rights: vec![],
        }
    }

    fn push(&mut self, op: AssignOperator, right: EqualityExpr) {
        self.rights.push(AssignExprNode { op, right });
    }
}

impl Parse for AssignExpr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut assign: Self;

        if let Ok(equal) = EqualityExpr::consume(tokens) {
            assign = Self::new(equal);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Assign => {
                        tokens.next();
                        if let Ok(right) = EqualityExpr::consume(tokens) {
                            assign.push(AssignOperator::Assign, right);
                        }
                    }
                    _ => {
                        return Ok(assign);
                    }
                }
            }

            Ok(assign)
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
