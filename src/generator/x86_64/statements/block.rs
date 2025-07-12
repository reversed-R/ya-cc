use crate::{generator::x86_64::Generate, parser::symbols::statements::block::BlockStmt};

impl Generate for BlockStmt {
    fn generate(&self) {}
}
