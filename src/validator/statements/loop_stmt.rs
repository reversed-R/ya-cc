use crate::validator::{
    expressions::Exprs, statements::Stmt, Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub struct LoopStmt {
    pub cond: Exprs,
    pub stmt: Stmt,
}

impl StmtTypeValidate for crate::parser::symbols::statements::while_stmt::WhileStmt {
    type ValidatedType = LoopStmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        Ok(LoopStmt {
            cond: self.cond.validate(env)?.1,
            stmt: self.stmt.validate(env)?,
        })
    }
}
