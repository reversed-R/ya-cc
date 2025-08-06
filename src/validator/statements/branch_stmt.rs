use crate::validator::{
    expressions::Expr, statements::Stmt, Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub struct BranchStmt {
    pub cond: Expr,
    pub then: Stmt,
    pub els: Option<Stmt>,
}

impl StmtTypeValidate for crate::parser::symbols::statements::if_stmt::IfStmt {
    type ValidatedType = BranchStmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        Ok(BranchStmt {
            cond: self.cond.validate(env)?,
            then: self.then.validate(env)?,
            els: self.els.as_ref().map(|els| els.validate(env)).transpose()?,
        })
    }
}
