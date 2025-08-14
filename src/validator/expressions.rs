pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod postfix;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::validator::{Env, ExprTypeValidate, Type, TypeError, Variable};

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Variable(Variable),
    FnCall(FnCall),
    Expr(Box<Exprs>),
}

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Exprs>,
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(u8),
    String(usize),
}

#[derive(Debug)]
pub enum Exprs {
    Primary(Primary),
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug)]
pub struct Unary {
    pub op: UnOperator,
    pub expr: Box<Exprs>,
}

#[derive(Debug)]
pub struct Binary {
    pub op: BinOperator,
    pub left: Box<Exprs>,
    pub right: Box<Exprs>,
}

#[derive(Debug)]
pub enum BinOperator {
    Iadd,
    Isub,
    Padd,
    Psub,
    Imul,
    Idiv,
    Mod,
    Greater,
    Lesser,
    GrtEq,
    LesEq,
    Equal,
    NotEq,
    Assign,
}

#[derive(Debug)]
pub enum UnOperator {
    Neg,
    Ref,
    Deref(usize),
}

impl ExprTypeValidate for crate::parser::symbols::expressions::Expr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        self.0.validate(env)
    }
}
