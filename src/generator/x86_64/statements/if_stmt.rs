use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::if_stmt::IfStmt,
};

impl LocalGenerate for IfStmt {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {}
}
