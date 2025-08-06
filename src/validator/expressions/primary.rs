use crate::{
    parser::symbols::{
        expressions::primary::{Literal, Primary},
        PrimitiveType, Type,
    },
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Primary {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        match self {
            Self::Literal(lit) => match lit {
                Literal::Int(_) => Ok(Type::Primitive(PrimitiveType::Int)),
                Literal::Float(_) => Ok(Type::Primitive(PrimitiveType::Float)),
            },
            Self::Identifier(id) => env
                .vars
                .get(id)
                .ok_or(TypeError::VariableNotFound(id.clone()))
                .cloned(),
            Self::Expr(expr) => expr.validate_type(env),
            Self::FnCall(fcalling) => {
                if let Some(fcallee) = env.fns.get(&fcalling.name) {
                    let mut i = 0;
                    while let Some(acalling) = fcalling.args.get(i) {
                        let acalling_typ = acalling.validate_type(env)?;

                        if let Some(acallee) = fcallee.args.get(i) {
                            if !acalling_typ.equals(&acallee.typ) {
                                return Err(TypeError::ArgumentMismatch(
                                    Some(acallee.typ.clone()),
                                    Some(acalling_typ),
                                ));
                            }
                        } else {
                            return Err(TypeError::ArgumentMismatch(None, Some(acalling_typ)));
                        }

                        i += 1;
                    }

                    if let Some(acallee) = fcallee.args.get(i) {
                        Err(TypeError::ArgumentMismatch(Some(acallee.typ.clone()), None))
                    } else {
                        Ok(fcallee.rtype.clone())
                    }
                } else {
                    Err(TypeError::FunctionNotFound(fcalling.name.clone()))
                }
            }
        }
    }
}
