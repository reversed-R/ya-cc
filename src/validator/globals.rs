use crate::{
    parser::symbols::globals::FnDef,
    validator::{statements::Stmt, Env, StmtTypeValidate, TypeError},
};

#[derive(Debug)]
pub struct Function {
    pub stmts: Vec<Stmt>,
    pub local_max_offset: usize,
    pub arg_count: usize,
}
// 次のcodegenで、関数はただのラベルに続けてインストラクションを並べただけなのでいらない
// args: Vec<Type>,
// rtype: Type,

impl StmtTypeValidate for FnDef {
    type ValidatedType = Function;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let stmts = self
            .stmts
            .iter()
            .map(|stmt| stmt.validate(env))
            .collect::<Result<Vec<Stmt>, TypeError>>()?;

        if let Some(local) = &env.local {
            Ok(Function {
                stmts,
                local_max_offset: local.local_max_offset,
                arg_count: self.args.len(),
            })
        } else {
            Err(TypeError::OutOfScopes)
        }
    }
}
