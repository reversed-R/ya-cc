use crate::{
    parser::symbols::expressions::primary,
    validator::{
        expressions::{FnCall, Literal, Primary},
        types::TypeComarison,
        Env, ExprTypeValidate, PrimitiveType, Type, ValidateError,
    },
};

impl primary::Primary {
    pub fn validate(&self, env: &mut Env) -> Result<(Type, Primary), ValidateError> {
        match self {
            Self::Literal(lit) => match lit {
                primary::Literal::Int(i) => Ok((
                    Type::Primitive(PrimitiveType::Int),
                    Primary::Literal(Literal::Int(*i)),
                )),
                // primary::Literal::Float(f) => Ok((
                //     // Type::Primitive(PrimitiveType::Float),
                //     Type::Primitive(PrimitiveType::Int),
                //     Primary::Literal(Literal::Float(*f)),
                // )),
                primary::Literal::Char(c) => Ok((
                    // Type::Primitive(PrimitiveType::Float),
                    Type::Primitive(PrimitiveType::Char),
                    Primary::Literal(Literal::Char(*c)),
                )),
                primary::Literal::String(s) => {
                    if let Some(id) = env.global.string_literals.get(s) {
                        Ok((
                            Type::Array(Box::new(Type::Primitive(PrimitiveType::Char)), s.len()),
                            Primary::Literal(Literal::String(*id)),
                        ))
                    } else {
                        let id = env.global.string_literals.values().len();
                        env.global.string_literals.insert(s.clone(), id);
                        Ok((
                            Type::Array(Box::new(Type::Primitive(PrimitiveType::Char)), s.len()),
                            Primary::Literal(Literal::String(id)),
                        ))
                    }
                }
            },
            Self::Identifier(id) => {
                let var = env
                    .get_var(id)
                    .ok_or(ValidateError::VariableNotFound(id.clone()))?;

                Ok((var.typ.clone(), Primary::Variable(var.clone())))
            }
            Self::Expr(expr) => {
                let (typ, expr) = expr.validate(env)?;

                Ok((typ, Primary::Expr(Box::new(expr))))
            }
            Self::FnCall(fcalling) => {
                env.global
                    .fns
                    .get(&fcalling.name)
                    .ok_or(ValidateError::FunctionNotFound(fcalling.name.clone()))?;

                let mut i = 0;
                let mut args = vec![];

                while let Some(acalling) = fcalling.args.get(i) {
                    let (acalling_typ, acalling) = acalling.validate(env)?;

                    let fcallee = env.global.fns.get(&fcalling.name).unwrap();
                    if let Some(acallee) = fcallee.args.get(i) {
                        match acallee.typ.compare(&acalling_typ) {
                            TypeComarison::Equal => {}
                            TypeComarison::ImplicitlyConvertableFrom => {}
                            _ => {
                                return Err(ValidateError::ArgumentMismatch(
                                    Some(Box::new(acallee.typ.clone())),
                                    Some(Box::new(acalling_typ)),
                                ));
                            }
                        }
                    } else {
                        return Err(ValidateError::ArgumentMismatch(
                            None,
                            Some(Box::new(acalling_typ)),
                        ));
                    }

                    i += 1;
                    args.push(acalling);
                }

                let fcallee = env.global.fns.get(&fcalling.name).unwrap();
                if let Some(acallee) = fcallee.args.get(i) {
                    Err(ValidateError::ArgumentMismatch(
                        Some(Box::new(acallee.typ.clone())),
                        None,
                    ))
                } else {
                    Ok((
                        fcallee.rtype.clone(),
                        Primary::FnCall(FnCall {
                            name: fcalling.name.clone(),
                            args,
                        }),
                    ))
                }
            }
        }
    }
}
