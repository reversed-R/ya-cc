use crate::{
    parser::symbols::statements::while_stmt::WhileStmt,
    validator::{Env, ExprTypeValidate, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for WhileStmt {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        self.cond.validate_type(env)?;

        self.stmt.validate_type(env)?;

        Ok(())
    }
}
