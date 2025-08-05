pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{
    parser::symbols::{expressions::Expr, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Expr {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
