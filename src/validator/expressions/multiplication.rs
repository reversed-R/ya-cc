use crate::{
    parser::symbols::expressions::multiplication,
    validator::{
        expressions::{BinOperator, Binary, Exprs},
        Env, ExprTypeValidate, Type, TypeError,
    },
};

impl From<&multiplication::MulOperator> for BinOperator {
    fn from(value: &multiplication::MulOperator) -> Self {
        match value {
            multiplication::MulOperator::Mul => BinOperator::Imul,
            multiplication::MulOperator::Div => BinOperator::Idiv,
            multiplication::MulOperator::Mod => BinOperator::Mod,
        }
    }
}

impl ExprTypeValidate for crate::parser::symbols::expressions::multiplication::MulExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        let (typ, left) = self.left.validate(env)?;

        if self.rights.is_empty() {
            return Ok((typ, left));
        }

        let mut expr = left;

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(Box::new(typ), Box::new(right_typ)));
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
