pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::validator::{Env, ExprTypeValidate, PrimitiveType, Type, TypeError};

#[derive(Debug)]
pub struct Expr {
    pub typ: Type,
    // pub ass: AssignExpr
}

impl ExprTypeValidate for crate::parser::symbols::expressions::Expr {
    type ValidatedType = Expr;

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        // self.0.validate(env)

        Ok(Expr {
            typ: Type::Primitive(PrimitiveType::Int),
        })
    }
}
