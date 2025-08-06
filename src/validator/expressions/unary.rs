use crate::{
    parser::symbols::{
        expressions::unary::{RefUnaryOperator, Unary},
        Type,
    },
    validator::{Env, ExprTypeValidate, TypeError},
};

impl ExprTypeValidate for Unary {
    fn validate_type(&self, env: &Env) -> Result<Type, TypeError> {
        let mut typ = self.right.right.validate_type(env)?;

        for op in self.right.ops.iter().rev() {
            match op {
                RefUnaryOperator::Ref => {
                    typ = Type::ptr_to(typ);
                }
                RefUnaryOperator::Deref => {
                    if let Some(deref) = Type::deref_of(&typ) {
                        typ = deref;
                    } else {
                        return Err(TypeError::DerefNotAllowed(typ));
                    }
                }
            }
        }

        Ok(typ)
    }
}
