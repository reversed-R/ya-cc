use crate::{
    parser::symbols::expressions::equality,
    validator::{
        expressions::{BinOperator, Binary, Exprs},
        Env, ExprTypeValidate, Type, TypeError,
    },
};

impl From<&equality::EqualityOperator> for BinOperator {
    fn from(value: &equality::EqualityOperator) -> Self {
        match value {
            equality::EqualityOperator::Equal => Self::Equal,
            equality::EqualityOperator::NotEq => Self::NotEq,
        }
    }
}

impl ExprTypeValidate for equality::EqualityExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, super::Exprs), TypeError> {
        let (typ, left) = self.left.validate(env)?;

        if self.rights.is_empty() {
            return Ok((typ, left));
        }

        let mut expr = left;

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }

            expr = Exprs::Binary(Binary {
                op: BinOperator::from(&r.op),
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        Ok((typ, expr))
    }
}
