use crate::{
    parser::symbols::expressions::arithmetic,
    validator::{
        expressions::{BinOperator, Binary, Exprs, Literal, Primary},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeComarison, TypeError,
    },
};

impl BinOperator {
    fn from(op: &arithmetic::ArithmOperator, typ: &Type) -> Self {
        match op {
            arithmetic::ArithmOperator::Add => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Self::Iadd,
                    // PrimitiveType::Float => Self::Fadd,
                    PrimitiveType::Char => Self::Iadd,
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Self::Padd,
                Type::Array(_, _) => Self::Padd, // WARN: is it true?
            },
            arithmetic::ArithmOperator::Sub => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Self::Isub,
                    // PrimitiveType::Float => Self::Fsub,
                    PrimitiveType::Char => Self::Isub,
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Self::Psub,
                Type::Array(_, _) => Self::Psub, // WARN: is it true?
            },
        }
    }
}

fn get_left_and_right_if_one_is_ptr_and_the_other_is_int(
    left: &Exprs,
    left_typ: &Type,
    right: &Exprs,
    right_typ: &Type,
) -> Option<(Exprs, Exprs)> {
    if matches!(left_typ, Type::Primitive(PrimitiveType::Int)) {
        if let Type::PtrTo(pointed) = right_typ {
            Some((
                right.clone(),
                Exprs::Binary(Binary {
                    op: BinOperator::Imul,
                    left: Box::new(left.clone()),
                    right: Box::new(Exprs::Primary(Primary::Literal(Literal::Int(
                        pointed.size() as i64,
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
                        pointed.size() as i64,
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
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        let (mut typ, left) = self.left.validate(env)?;

        if self.rights.is_empty() {
            return Ok((typ, left));
        }

        let mut expr = left;

        for r in &self.rights {
            let (right_typ, mut right) = r.right.validate(env)?;

            if let Some((l, r)) = get_left_and_right_if_one_is_ptr_and_the_other_is_int(
                &expr, &typ, &right, &right_typ,
            ) {
                expr = l;
                right = r;
            }

            match typ.compare(&right_typ) {
                TypeComarison::Equal => {
                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ),
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyConvertableTo => {
                    typ = right_typ;

                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ),
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyConvertableFrom => {
                    expr = Exprs::Binary(Binary {
                        op: BinOperator::from(&r.op, &typ),
                        left: Box::new(expr),
                        right: Box::new(right),
                    });
                }
                TypeComarison::ImplicitlyUnconvertable => {
                    return Err(TypeError::Mismatch(typ, right_typ));
                }
            }
        }

        Ok((typ, expr))
    }
}
