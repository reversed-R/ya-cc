use crate::validator::{Env, ExprTypeValidate, TypeError};

pub struct ArithmExpr;

impl ExprTypeValidate for crate::parser::symbols::expressions::arithmetic::ArithmExpr {
    type ValidatedType = ArithmExpr;

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        let typ = self.left.validate(env)?;

        for right in &self.rights {
            let right_typ = right.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }
        }

        Ok(typ)
    }
}
