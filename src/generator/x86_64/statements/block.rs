use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::block::BlockStmt,
};

impl LocalGenerate for BlockStmt {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {}
}
