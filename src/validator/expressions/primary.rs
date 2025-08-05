use crate::{
    parser::symbols::{
        expressions::primary::{Literal, Primary},
        PrimitiveType, Type,
    },
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Primary {
    fn validate_type(&self, env: &mut Env) -> Result<Type, TypeError> {
        match self {
            Self::Literal(lit) => match lit {
                Literal::Int(_) => Ok(Type::Primitive(PrimitiveType::Int)),
                Literal::Float(_) => Ok(Type::Primitive(PrimitiveType::Float)),
            },
            Self::Identifier(id) => env
                .locals
                .get(id)
                .ok_or(TypeError::VariableNotFound(id.clone()))
                .cloned(),
            Self::Expr(expr) => expr.validate_type(env),
            Self::FnCall(f) => {
                if let Some(fsign) = env.fns.get(&f.name) {
                    // TODO:
                    Ok(fsign.rtype.clone())
                } else {
                    Err(TypeError::VariableNotFound(f.name.clone()))
                }
            }
        }
    }
}
