use crate::validator::{statements::Stmt, Env, StmtTypeValidate, TypeError};

impl StmtTypeValidate for crate::parser::symbols::statements::block::BlockStmt {
    type ValidatedType = Stmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        env.begin_scope();

        let stmts = self
            .stmts
            .iter()
            .map(|stmt| stmt.validate(env))
            .collect::<Result<Vec<Stmt>, TypeError>>()?;

        env.end_scope();

        Ok(Stmt::Compound(stmts))
    }
}
