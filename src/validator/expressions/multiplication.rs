use crate::{
    parser::symbols::{expressions::multiplication::MulExpr, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for MulExpr {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        let typ = self.left.validate_type(env)?;

        for right in &self.rights {
            let right_typ = right.right.validate_type(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }
        }

        Ok(typ)
    }
}
