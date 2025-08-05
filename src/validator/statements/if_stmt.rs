use crate::{
    parser::symbols::statements::if_stmt::IfStmt,
    validator::{Env, ExprTypeValidate, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for IfStmt {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        self.cond.validate_type(env)?;

        self.then.validate_type(env)?;

        if let Some(els) = &self.els {
            els.validate_type(env)?;
        }

        Ok(())
    }
}
