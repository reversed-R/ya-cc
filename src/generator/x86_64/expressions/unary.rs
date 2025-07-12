use crate::{generator::x86_64::Generate, parser::symbols::expressions::unary::Unary};

impl Generate for Unary {
    fn generate(&self) {}
}
