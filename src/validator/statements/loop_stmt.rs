use crate::validator::{
    expressions::Expr, statements::Stmt, Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub struct LoopStmt {
    pub cond: Expr,
    pub stmt: Stmt,
}

impl StmtTypeValidate for crate::parser::symbols::statements::while_stmt::WhileStmt {
    type ValidatedType = LoopStmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        Ok(LoopStmt {
            cond: self.cond.validate(env)?,
            stmt: self.stmt.validate(env)?,
        })
    }
}
