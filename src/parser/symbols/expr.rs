use crate::{
    lexer::token::Token,
    parser::{Parse, ParseError},
};

#[derive(Debug)]
pub struct Expr(EqualityExpr);

// EqualityExpr = RelationalExpr ("==" RelationalExpr | "!=" RelationalExpr)*
#[derive(Debug)]
pub struct EqualityExpr {
    nodes: Vec<EqualityExprNode>,
}

#[derive(Debug)]
pub struct EqualityExprNode {
    pub op: EqualityOperator, // `op` of the head (index 0th) element does not have meaning, just
    // a placeholder
    pub right: RelationalExpr,
}

#[derive(Debug)]
pub enum EqualityOperator {
    Equal, // ==
    NotEq, // !=
}

// RelationalExpr = ArithmExpr ("<" ArithmExpr | ">" ArithmExpr | "<=" ArithmExpr | ">=" ArithmExpr)*
#[derive(Debug)]
pub struct RelationalExpr {
    nodes: Vec<RelationalExprNode>,
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
    pub right: Unary,
}

// Unary = ("+" | "-")? Primary
#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOperator,
    pub right: Primary,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,  // +
    Minus, // -
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

impl Parse for Expr {
    type SelfType = Self;

    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        if let Ok(equal) = EqualityExpr::consume(tokens) {
            Ok(Self(equal))
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

impl EqualityExpr {
    pub fn new(relat: RelationalExpr) -> Self {
        Self {
            nodes: vec![EqualityExprNode {
                op: EqualityOperator::Equal,
                right: relat,
            }],
        }
    }

    fn push(&mut self, op: EqualityOperator, right: RelationalExpr) {
        self.nodes.push(EqualityExprNode { op, right });
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
                        if let Ok(right) = MulExpr::consume(tokens) {
                            arithm.push(Operator::Add, right);
                        }
                    }
                    Token::Minus => {
                        tokens.next();
                        if let Ok(right) = MulExpr::consume(tokens) {
                            arithm.push(Operator::Sub, right);
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
    pub fn new(unary: Unary) -> Self {
        Self {
            nodes: vec![MulExprNode {
                op: Operator::Mul,
                right: unary,
            }],
        }
    }

    fn push(&mut self, op: Operator, right: Unary) {
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
                            mul.push(Operator::Mul, right);
                        }
                    }
                    Token::Slash => {
                        tokens.next();
                        if let Ok(right) = Unary::consume(tokens) {
                            mul.push(Operator::Div, right);
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

impl Parse for Unary {
    type SelfType = Self;
    fn consume(
        tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Self::SelfType, ParseError> {
        // Unary = ("+" | "-")? Primary
        if let Some(t) = tokens.peek() {
            match t {
                Token::Plus => {
                    tokens.next();
                    if let Ok(right) = Primary::consume(tokens) {
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
                    if let Ok(right) = Primary::consume(tokens) {
                        Ok(Self {
                            op: UnaryOperator::Minus,
                            right,
                        })
                    } else {
                        Err(ParseError::InvalidToken)
                    }
                }
                _ => {
                    if let Ok(right) = Primary::consume(tokens) {
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
