use crate::{
    parser::symbols::globals::FnDec,
    validator::{statements::Stmt, Env, StmtTypeValidate, TypeError},
};

pub struct Function {
    stmts: Vec<Stmt>,
}
// 次のcodegenで、関数はただのラベルに続けてインストラクションを並べただけなのでいらない
// args: Vec<Type>,
// rtype: Type,

impl StmtTypeValidate for FnDec {
    type ValidatedType = Function;
    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        env.begin_local(&self.args, &self.rtype);

        let stmts = self
            .stmts
            .iter()
            .map(|stmt| stmt.validate(env))
            .collect::<Result<Vec<Stmt>, TypeError>>()?;

        env.end_local();

        Ok(Function { stmts })
    }
}
