use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::while_stmt::WhileStmt,
};

impl LocalGenerate for WhileStmt {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {}
}
