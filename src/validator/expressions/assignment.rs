use crate::{
    parser::symbols::expressions::assignment,
    validator::{
        expressions::{BinOperator, Binary, Exprs, Literal, Primary, UnOperator},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeError,
    },
};

impl From<&assignment::AssignOperator> for BinOperator {
    fn from(value: &assignment::AssignOperator) -> Self {
        match value {
            assignment::AssignOperator::Assign => BinOperator::Assign,
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
            } else if typ == Type::Primitive(PrimitiveType::Int)
                && is_numeric_zero(&src)
                && matches!(dst_typ, Type::PtrTo(_))
            {
                typ = dst_typ;
            } else {
                return Err(TypeError::Mismatch(dst_typ, typ));
            }

            src = Exprs::Binary(Binary {
                op: BinOperator::from(&left.op),
                left: Box::new(dst),
                right: Box::new(src),
            });
        }

        Ok((typ, src))
    }
}
