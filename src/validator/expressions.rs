pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod postfix;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::validator::{
    expressions::assignment::AssignExpr, Env, ExprTypeValidate, Type, TypeError,
};

#[derive(Debug)]
pub struct Expr(pub AssignExpr);

impl ExprTypeValidate for crate::parser::symbols::expressions::Expr {
    type ValidatedType = (Type, Expr);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        let (typ, ass) = self.0.validate(env)?;

        Ok((typ, Expr(ass)))
    }
}
