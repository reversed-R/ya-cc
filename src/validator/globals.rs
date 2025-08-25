use crate::{
    parser::symbols::globals::FnDef,
    validator::{statements::Stmt, Env, ValidateError},
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

impl FnDef {
    pub fn validate(&self, env: &mut Env) -> Result<Function, ValidateError> {
        let stmts = self
            .stmts
            .iter()
            .map(|stmt| stmt.validate(env))
            .collect::<Result<Vec<Option<Stmt>>, ValidateError>>()?
            .into_iter()
            .flatten()
            .collect();

        if let Some(local) = &env.local {
            Ok(Function {
                stmts,
                local_max_offset: local.local_max_offset,
                arg_count: self.args.len(),
            })
        } else {
            Err(ValidateError::OutOfScopes)
        }
    }
}
