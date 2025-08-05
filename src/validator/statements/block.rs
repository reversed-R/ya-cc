use crate::{
    parser::symbols::statements::block::BlockStmt,
    validator::{Env, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for BlockStmt {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        env.begin_scope();

        for stmt in &self.stmts {
            stmt.validate_type(env)?;
        }

        env.end_scope();

        Ok(())
    }
}
