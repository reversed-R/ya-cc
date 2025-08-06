use crate::{
    parser::symbols::{expressions::assignment::AssignExpr, Type},
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for AssignExpr {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        let typ = self.right.validate_type(env)?;

        for left in self.lefts.iter().rev() {
            let left_typ = left.left.validate_type(env)?;

            if !typ.equals(&left_typ) {
                return Err(TypeError::Mismatch(left_typ, typ));
            }
        }

        Ok(typ)
    }
}
