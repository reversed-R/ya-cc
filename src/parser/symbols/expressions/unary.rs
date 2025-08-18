use crate::{
    lexer::token::{Token, TokenKind},
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
            match t.kind {
                TokenKind::SizeOf => {
                    tokens.next();

                    let right = RefUnary::consume(tokens)?;
                    Ok(Self {
                        op: UnaryOperator::SizeOf,
                        right,
                    })
                }
                TokenKind::Plus => {
                    tokens.next();

                    let right = RefUnary::consume(tokens)?;
                    Ok(Self {
                        op: UnaryOperator::Plus,
                        right,
                    })
                }
                TokenKind::Minus => {
                    tokens.next();

                    let right = RefUnary::consume(tokens)?;
                    Ok(Self {
                        op: UnaryOperator::Minus,
                        right,
                    })
                }
                _ => {
                    let right = RefUnary::consume(tokens)?;
                    Ok(Self {
                        op: UnaryOperator::Plus,
                        right,
                    })
                }
            }
        } else {
            Err(ParseError::InvalidEOF(vec![
                TokenKind::String("".to_string()),
                TokenKind::IntLiteral(0),
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Ampersand,
                TokenKind::Asterisk,
                TokenKind::SizeOf,
            ]))
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
            match t.kind {
                TokenKind::Ampersand => {
                    tokens.next();

                    ops.push(RefUnaryOperator::Ref);
                }
                TokenKind::Asterisk => {
                    tokens.next();

                    ops.push(RefUnaryOperator::Deref);
                }
                _ => {
                    break;
                }
            }
        }

        let right = PostfixExpr::consume(tokens)?;

        Ok(Self { ops, right })
    }
}
