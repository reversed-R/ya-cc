use crate::{generator::x86_64::Generate, parser::symbols::expressions::Expr};

impl Generate for Expr {
    fn generate(&self) {}
}
