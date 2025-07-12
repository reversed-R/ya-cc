use crate::{generator::x86_64::Generate, parser::symbols::expressions::arithmetic::ArithmExpr};

impl Generate for ArithmExpr {
    fn generate(&self) {}
}
