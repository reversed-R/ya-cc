use crate::{
    parser::symbols::expressions::assignment,
    validator::{
        expressions::{BinOperator, Binary, Exprs, Literal, Primary, UnOperator},
        DefinedType, Env, ExprTypeValidate, PrimitiveType, Type, TypeError,
    },
};

impl BinOperator {
    fn from_assignop(value: &assignment::AssignOperator, typ: &Type) -> Result<Self, TypeError> {
        match value {
            assignment::AssignOperator::Assign => match typ {
                Type::Primitive(p) => match p {
                    PrimitiveType::Int => Ok(Self::IAssign),
                    PrimitiveType::Char => Ok(Self::CAssign),
                    PrimitiveType::Void => Ok(Self::IAssign), // WARN: is it true?
                },
                Type::PtrTo(_) => Ok(Self::PAssign),
                Type::Array(_, _) => Ok(Self::PAssign), // WARN: is it true?
                Type::Defined(d) => Err(TypeError::StructNotAssignable(match d {
                    DefinedType::Struct(s) => s.clone(),
                })),
                // WARN: if i implement enum, i fix it
            },
        }
    }
}

fn is_numeric_zero(src: &Exprs) -> bool {
    matches!(src, Exprs::Primary(Primary::Literal(Literal::Int(0))))
        || if let Exprs::Unary(unary) = src {
            matches!(unary.op, UnOperator::IDeref(0))
                && matches!(
                    *unary.expr,
                    Exprs::Primary(Primary::Literal(Literal::Int(0)))
                )
        } else {
            false
        }
        || if let Exprs::Unary(unary) = src {
            // WARN: is it necessary?
            matches!(unary.op, UnOperator::CDeref(0))
                && matches!(
                    *unary.expr,
                    Exprs::Primary(Primary::Literal(Literal::Int(0)))
                )
        } else {
            false
        }
        || if let Exprs::Primary(Primary::Expr(expr)) = src {
            is_numeric_zero(expr)
        } else {
            false
        }
}

impl ExprTypeValidate for assignment::AssignExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        let (src_typ, mut src) = self.right.validate(env)?;

        if self.lefts.is_empty() {
            return Ok((src_typ, src));
        }

        let mut typ = src_typ.clone();

        for left in self.lefts.iter().rev() {
            let (dst_typ, dst) = left.left.validate(env)?;

            if typ.equals(&dst_typ) {
                // nothing to do
            } else if matches!(typ, Type::Primitive(PrimitiveType::Int))
                && is_numeric_zero(&src)
                && matches!(dst_typ, Type::PtrTo(_))
            {
                typ = dst_typ;
            } else {
                return Err(TypeError::Mismatch(Box::new(dst_typ), Box::new(typ)));
            }

            src = Exprs::Binary(Binary {
                op: BinOperator::from_assignop(&left.op, &typ)?,
                left: Box::new(dst),
                right: Box::new(src),
            });
        }

        Ok((typ, src))
    }
}
