use crate::{
    parser::symbols::expressions::primary,
    validator::{
        expressions::Expr, Env, ExprTypeValidate, PrimitiveType, Type, TypeError, Variable,
    },
};

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Variable(Variable),
    FnCall(FnCall),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(u8),
    String(usize),
}

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

impl ExprTypeValidate for primary::Primary {
    type ValidatedType = (Type, Primary);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
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
                    // TODO:
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
                // } else {
                //     Err(TypeError::FunctionNotFound(fcalling.name.clone()))
                // }
            }
        }
    }
}
