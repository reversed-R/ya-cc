use crate::{
    parser::symbols::{expressions::multiplication::MulExpr, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for MulExpr {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
