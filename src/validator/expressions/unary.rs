use crate::{
    parser::symbols::{expressions::unary::Unary, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Unary {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
