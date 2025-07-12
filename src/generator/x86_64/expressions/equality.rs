use crate::{generator::x86_64::Generate, parser::symbols::expressions::equality::EqualityExpr};

impl Generate for EqualityExpr {
    fn generate(&self) {}
}
