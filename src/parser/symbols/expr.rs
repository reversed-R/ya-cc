use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

#[derive(Debug)]
pub enum Expr {
    Arithm(Box<ArithmExpr>),
    Identifier(String),
    Literal(Literal),
    // Assign,
    // FnCall,
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct ArithmExpr {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Operator {
    Add, // +
    Mul, // *
}

impl Parse for ArithmExpr {
    type SelfType = Self;

    fn consume(tokens: &mut std::slice::Iter<'_, Token>) -> Result<Self::SelfType, ParseError> {
        let left: i64;
        let right: i64;
        let op: Operator;
        if let Some(t) = tokens.next() {
            match t {
                Token::IntLiteral(i) => {
                    left = *i;

                    if let Some(t) = tokens.next() {
                        match t {
                            Token::Plus => {
                                op = Operator::Add;
                                if let Some(t) = tokens.next() {
                                    match t {
                                        Token::IntLiteral(i) => {
                                            right = *i;
                                            Ok(Self {
                                                op,
                                                left: Box::new(Expr::Literal(Literal::Int(left))),
                                                right: Box::new(Expr::Literal(Literal::Int(right))),
                                            })
                                        }
                                        _ => Err(ParseError::InvalidToken),
                                    }
                                } else {
                                    Err(ParseError::InvalidToken)
                                }
                            }
                            _ => Err(ParseError::InvalidToken),
                        }
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => Err(ParseError::InvalidToken),
            }
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
