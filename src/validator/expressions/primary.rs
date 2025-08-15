use crate::{
    parser::symbols::expressions::primary,
    validator::{
        expressions::{FnCall, Literal, Primary},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeComarison, TypeError,
    },
};

impl primary::Primary {
    pub fn validate(&self, env: &mut Env) -> Result<(Type, Primary), TypeError> {
        match self {
            Self::Literal(lit) => match lit {
                primary::Literal::Int(i) => Ok((
                    Type::Primitive(PrimitiveType::Int),
                    Primary::Literal(Literal::Int(*i)),
                )),
                primary::Literal::Float(f) => Ok((
                    // Type::Primitive(PrimitiveType::Float),
                    Type::Primitive(PrimitiveType::Int),
                    Primary::Literal(Literal::Float(*f)),
                )),
                primary::Literal::Char(c) => Ok((
                    // Type::Primitive(PrimitiveType::Float),
                    Type::Primitive(PrimitiveType::Char),
                    Primary::Literal(Literal::Char(*c)),
                )),
                primary::Literal::StringLiteral(s) => {
                    if let Some(id) = env.string_literals.get(s) {
                        Ok((
                            Type::Array(Box::new(Type::Primitive(PrimitiveType::Char)), s.len()),
                            Primary::Literal(Literal::String(*id)),
                        ))
                    } else {
                        let id = env.string_literals.values().len();
                        env.string_literals.insert(s.clone(), id);
                        Ok((
                            Type::Array(Box::new(Type::Primitive(PrimitiveType::Char)), s.len()),
                            Primary::Literal(Literal::String(id)),
                        ))
                    }
                }
            },
            Self::Identifier(id) => {
                let var = env
                    .vars
                    .get(id)
                    .ok_or(TypeError::VariableNotFound(id.clone()))?;

                Ok((var.typ.clone(), Primary::Variable(var.clone())))
            }
            Self::Expr(expr) => {
                let (typ, expr) = expr.validate(env)?;

                Ok((typ, Primary::Expr(Box::new(expr))))
            }
            Self::FnCall(fcalling) => {
                env.fns
                    .get(&fcalling.name)
                    .ok_or(TypeError::FunctionNotFound(fcalling.name.clone()))?;

                let mut i = 0;
                let mut args = vec![];

                while let Some(acalling) = fcalling.args.get(i) {
                    let (acalling_typ, acalling) = acalling.validate(env)?;

                    let fcallee = env.fns.get(&fcalling.name).unwrap();
                    if let Some(acallee) = fcallee.args.get(i) {
                        match acallee.typ.compare(&acalling_typ) {
                            TypeComarison::Equal => {}
                            TypeComarison::ImplicitlyConvertableFrom => {}
                            _ => {
                                return Err(TypeError::ArgumentMismatch(
                                    Some(acallee.typ.clone()),
                                    Some(acalling_typ),
                                ));
                            }
                        }
                    } else {
                        return Err(TypeError::ArgumentMismatch(None, Some(acalling_typ)));
                    }

                    i += 1;
                    args.push(acalling);
                }

                let fcallee = env.fns.get(&fcalling.name).unwrap();
                if let Some(acallee) = fcallee.args.get(i) {
                    Err(TypeError::ArgumentMismatch(Some(acallee.typ.clone()), None))
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
