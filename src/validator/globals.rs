use crate::{
    parser::symbols::globals::FnDec,
    validator::{Env, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for FnDec {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        env.begin_local(&self.args, &self.rtype);

        for stmt in &self.stmts {
            stmt.validate_type(env)?;
        }

        env.end_local();

        Ok(())
    }
}
