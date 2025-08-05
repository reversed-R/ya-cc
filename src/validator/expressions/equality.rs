use crate::{
    parser::symbols::{expressions::equality::EqualityExpr, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for EqualityExpr {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
