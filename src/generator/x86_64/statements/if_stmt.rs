use crate::{generator::x86_64::LocalGenerate, parser::symbols::statements::if_stmt::IfStmt};

impl LocalGenerate for IfStmt {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {}
}
