use crate::{
    lexer::token::Token,
    parser::{symbols::expressions::postfix::PostfixExpr, Parse, ParseError},
};

// Unary = ("sizeof" | +" | "-")? RefUnary
// = ("sizeof" | "+" | "-")? ("&", "*")* PostfixExpr
#[derive(Debug, Clone)]
pub struct Unary {
    pub op: UnaryOperator,
    pub right: RefUnary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    SizeOf, // sizeof
    Plus,   // +
    Minus,  // -
}

#[derive(Debug, Clone)]
pub struct RefUnary {
    pub ops: Vec<RefUnaryOperator>,
    pub right: PostfixExpr,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RefUnaryOperator {
    Ref,   // &
    Deref, // *
}

impl Parse for Unary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        // Unary = ("sizeof" | +" | "-")? RefUnary
        if let Some(t) = tokens.peek() {
            match t {
                Token::SizeOf => {
                    tokens.next();
                    if let Ok(right) = RefUnary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::SizeOf,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::Plus => {
                    tokens.next();
                    if let Ok(right) = RefUnary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Plus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                Token::Minus => {
                    tokens.next();
                    if let Ok(right) = RefUnary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Minus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => {
                    if let Ok(right) = RefUnary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Plus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

impl Parse for RefUnary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut ops: Vec<RefUnaryOperator> = vec![];

        // RefUnary = ("&" | "*")* PostfixExpr
        while let Some(t) = tokens.peek() {
            match t {
                Token::Ampersand => {
                    tokens.next();

                    ops.push(RefUnaryOperator::Ref);
                }
                Token::Asterisk => {
                    tokens.next();

                    ops.push(RefUnaryOperator::Deref);
                }
                _ => {
                    break;
                }
            }
        }

        if let Ok(right) = PostfixExpr::consume(tokens) {
            Ok(Self { ops, right })
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
