use crate::{generator::x86_64::LocalGenerate, parser::symbols::statements::block::BlockStmt};

impl LocalGenerate for BlockStmt {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {}
}
