use crate::{
    parser::symbols::expressions::arithmetic,
    validator::{
        expressions::{BinOperator, Binary, Exprs},
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

impl ExprTypeValidate for crate::parser::symbols::expressions::arithmetic::ArithmExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        let (mut typ, left) = self.left.validate(env)?;

        if self.rights.is_empty() {
            return Ok((typ, left));
        }

        let mut expr = left;

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

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
