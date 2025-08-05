use crate::{
    parser::symbols::{expressions::relational::RelationalExpr, PrimitiveType, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for RelationalExpr {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError> {
        // TODO:

        Ok(Type::Primitive(PrimitiveType::Int))
    }
}
