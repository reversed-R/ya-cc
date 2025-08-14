use crate::validator::{
    expressions::Exprs, statements::Stmt, Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub struct BranchStmt {
    pub cond: Exprs,
    pub then: Stmt,
    pub els: Option<Stmt>,
}

impl StmtTypeValidate for crate::parser::symbols::statements::if_stmt::IfStmt {
    type ValidatedType = BranchStmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        Ok(BranchStmt {
            cond: self.cond.validate(env)?.1,
            then: self.then.validate(env)?,
            els: self.els.as_ref().map(|els| els.validate(env)).transpose()?,
        })
    }
}
