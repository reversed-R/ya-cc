use crate::{generator::x86_64::Generate, parser::symbols::expressions::multiplication::MulExpr};

impl Generate for MulExpr {
    fn generate(&self) {}
}
