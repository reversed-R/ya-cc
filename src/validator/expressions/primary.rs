use crate::{
    parser::symbols::expressions::primary,
    validator::{
        expressions::{arithmetic::ArithmExpr, Expr},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeError, Variable,
    },
};

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Variable(Variable),
    FnCall(FnCall),
    Expr(Box<ArithmExpr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

impl ExprTypeValidate for primary::Primary {
    type ValidatedType = (Type, Primary);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        match self {
            Self::Literal(lit) => match lit {
                primary::Literal::Int(i) => Ok((
                    Type::Primitive(PrimitiveType::Int),
                    Primary::Literal(Literal::Int(*i)),
                )),
                primary::Literal::Float(f) => Ok((
                    Type::Primitive(PrimitiveType::Float),
                    Primary::Literal(Literal::Float(*f)),
                )),
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
                if let Some(fcallee) = env.fns.get(&fcalling.name) {
                    let mut i = 0;
                    let mut args = vec![];

                    while let Some(acalling) = fcalling.args.get(i) {
                        let (acalling_typ, acalling) = acalling.validate(env)?;

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
                } else {
                    Err(TypeError::FunctionNotFound(fcalling.name.clone()))
                }
            }
        }
    }
}
