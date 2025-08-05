use crate::{
    parser::symbols::globals::FnDec,
    validator::{Env, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for FnDec {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        let mut env = Env::new(&self.args, &self.rtype, &self.stmts);

        for stmt in &self.stmts {
            stmt.validate_type(&mut env)?;
        }

        Ok(())
    }
}
