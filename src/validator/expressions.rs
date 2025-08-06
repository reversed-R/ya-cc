pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{
    parser::symbols::{expressions::Expr, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Expr {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        self.0.validate_type(env)
    }
}
