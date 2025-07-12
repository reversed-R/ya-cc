use crate::{generator::x86_64::Generate, parser::symbols::expressions::primary::Primary};

impl Generate for Primary {
    fn generate(&self) {}
}
