use crate::{generator::x86_64::LocalGenerate, parser::symbols::statements::while_stmt::WhileStmt};

impl LocalGenerate for WhileStmt {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {}
}
