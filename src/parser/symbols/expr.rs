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

// ArithmExpr = MulExpr ("+" MulExpr | "-" MulExpr)*
#[derive(Debug)]
pub struct ArithmExpr {
    nodes: Vec<ArithmExprNode>,
}

#[derive(Debug)]
pub struct ArithmExprNode {
    pub op: Operator, // `op` of the head (index 0th) element does not have meaning, just
    // a placeholder
    pub right: MulExpr,
}

// MulExpr = Primary ("*" Primary | "/" Primary)*
#[derive(Debug)]
pub struct MulExpr {
    nodes: Vec<MulExprNode>,
}

#[derive(Debug)]
pub struct MulExprNode {
    pub op: Operator, // `op` of the head (index 0th) element does not have meaning, just
    // a placeholder
    pub right: Primary,
}

// Primary = Literal | "(" ArithmExpr ")"
#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Expr(Box<ArithmExpr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Operator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

impl ArithmExpr {
    pub fn new(mul: MulExpr) -> Self {
        Self {
            nodes: vec![ArithmExprNode {
                op: Operator::Add,
                right: mul,
            }],
        }
    }

    fn push(&mut self, op: Operator, right: MulExpr) {
        self.nodes.push(ArithmExprNode { op, right });
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
                        if let Ok(m_right) = MulExpr::consume(tokens) {
                            arithm.push(Operator::Add, m_right);
                        }
                    }
                    Token::Minus => {
                        tokens.next();
                        if let Ok(m_right) = MulExpr::consume(tokens) {
                            arithm.push(Operator::Sub, m_right);
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

impl MulExpr {
    pub fn new(prim: Primary) -> Self {
        Self {
            nodes: vec![MulExprNode {
                op: Operator::Mul,
                right: prim,
            }],
        }
    }

    fn push(&mut self, op: Operator, right: Primary) {
        self.nodes.push(MulExprNode { op, right });
    }
}

impl Parse for MulExpr {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        let mut mul: Self;

        if let Ok(prim) = Primary::consume(tokens) {
            mul = Self::new(prim);

            while let Some(t) = tokens.peek() {
                match t {
                    Token::Asterisk => {
                        tokens.next();
                        if let Ok(p_right) = Primary::consume(tokens) {
                            mul.push(Operator::Mul, p_right);
                        }
                    }
                    Token::Slash => {
                        tokens.next();
                        if let Ok(p_right) = Primary::consume(tokens) {
                            mul.push(Operator::Div, p_right);
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

impl Parse for Primary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        // Primary = Literal | "(" Expr ")"
        if let Some(t) = tokens.next() {
            match t {
                Token::IntLiteral(i) => Ok(Self::Literal(Literal::Int(*i))),
                Token::LPare => {
                    if let Ok(expr) = ArithmExpr::consume(tokens) {
                        if let Some(Token::RPare) = tokens.next() {
                            Ok(Self::Expr(Box::new(expr)))
                        } else {
                            Err(ParseError::InvalidToken)
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
