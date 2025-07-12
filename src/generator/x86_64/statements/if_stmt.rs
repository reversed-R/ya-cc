use crate::{generator::x86_64::Generate, parser::symbols::statements::if_stmt::IfStmt};

impl Generate for IfStmt {
    fn generate(&self) {}
}
