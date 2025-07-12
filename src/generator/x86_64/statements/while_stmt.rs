use crate::{generator::x86_64::Generate, parser::symbols::statements::while_stmt::WhileStmt};

impl Generate for WhileStmt {
    fn generate(&self) {}
}
