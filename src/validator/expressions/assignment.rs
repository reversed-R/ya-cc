use crate::{
    parser::symbols::{expressions::assignment::AssignExpr, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for AssignExpr {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
