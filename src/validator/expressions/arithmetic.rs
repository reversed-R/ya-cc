use crate::{
    parser::symbols::expressions::arithmetic,
    validator::{
        expressions::{BinOperator, Binary, Exprs, Literal, Primary},
        types::TypeComarison,
        DefinedType, Env, ExprTypeValidate, PrimitiveType, Type, ValidateError,
    },
};

impl BinOperator {
    fn from(op: &arithmetic::ArithmOperator, typ: &Type) -> Result<Self, ValidateError> {
        match op {
            arithmetic::ArithmOperator::Add => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Ok(Self::Iadd),
                    // PrimitiveType::Float => Ok(Self::Fadd),
                    PrimitiveType::Char => Ok(Self::Iadd),
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Ok(Self::Padd),
                Type::Array(_, _) => Ok(Self::Padd), // WARN: is it true?
                Type::Defined(d) => Err(ValidateError::TypeAndOperatorNotSupported(
                    match d {
                        DefinedType::Struct(s) => s.clone(),
                    },
                    "+".to_string(),
                )),
                // WARN: if i implement enum, i fix it
            },
            arithmetic::ArithmOperator::Sub => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Ok(Self::Isub),
                    // PrimitiveType::Float => Ok(Self::Fsub),
                    PrimitiveType::Char => Ok(Self::Isub),
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Ok(Self::Psub),
                Type::Array(_, _) => Ok(Self::Psub), // WARN: is it true?
                Type::Defined(d) => Err(ValidateError::TypeAndOperatorNotSupported(
                    match d {
                        DefinedType::Struct(s) => s.clone(),
                    },
                    "-".to_string(),
                )),
                // WARN: if i implement enum, i fix it
            },
        }
    }
}

fn get_left_and_right_if_one_is_ptr_and_the_other_is_int(
    left: &Exprs,
    left_typ: &Type,
    right: &Exprs,
    right_typ: &Type,
    env: &Env,
) -> Option<(Exprs, Exprs)> {
    if matches!(left_typ, Type::Primitive(PrimitiveType::Int)) {
        if let Type::PtrTo(pointed) = right_typ {
            Some((
                right.clone(),
                Exprs::Binary(Binary {
                    op: BinOperator::Imul,
                    left: Box::new(left.clone()),
                    right: Box::new(Exprs::Primary(Primary::Literal(Literal::Int(
                        pointed.size(env) as i64,
                    )))),
                }),
            ))
        } else {
            None
        }
    } else if matches!(right_typ, Type::Primitive(PrimitiveType::Int)) {
        if let Type::PtrTo(pointed) = left_typ {
            Some((
                left.clone(),
                Exprs::Binary(Binary {
                    op: BinOperator::Imul,
                    left: Box::new(right.clone()),
                    right: Box::new(Exprs::Primary(Primary::Literal(Literal::Int(
                        pointed.size(env) as i64,
                    )))),
                }),
            ))
        } else {
            None
        }
    } else {
        None
    }
}

impl ExprTypeValidate for crate::parser::symbols::expressions::arithmetic::ArithmExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), ValidateError> {
        let (mut typ, left) = self.left.validate(env)?;

        if self.rights.is_empty() {
            return Ok((typ, left));
        }

        let mut expr = left;

        for r in &self.rights {
            let (right_typ, mut right) = r.right.validate(env)?;

            if let Some((l, r)) = get_left_and_right_if_one_is_ptr_and_the_other_is_int(
                &expr, &typ, &right, &right_typ, env,
            ) {
                expr = l;
                right = r;
            }

            match typ.compare(&right_typ) {
                TypeComarison::Equal => {
                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ)?,
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyConvertableTo => {
                    typ = right_typ;

                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ)?,
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyConvertableFrom => {
                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ)?,
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyUnconvertable => {
                    return Err(ValidateError::Mismatch(Box::new(typ), Box::new(right_typ)));
                }
            }
        }

        Ok((typ, expr))
    }
}
