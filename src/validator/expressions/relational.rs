use crate::{
    parser::symbols::expressions::relational,
    validator::{
        expressions::{BinOperator, Binary, Exprs},
        Env, ExprTypeValidate, Type, TypeError,
    },
};

impl From<&relational::RelationalOperator> for BinOperator {
    fn from(value: &relational::RelationalOperator) -> Self {
        match value {
            relational::RelationalOperator::Lesser => Self::Lesser,
            relational::RelationalOperator::Greater => Self::Greater,
            relational::RelationalOperator::LesEq => Self::LesEq,
            relational::RelationalOperator::GrtEq => Self::GrtEq,
        }
    }
}

impl ExprTypeValidate for relational::RelationalExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
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
