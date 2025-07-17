pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{generator::x86_64::globals::LocalGenerate, parser::symbols::expressions::Expr};

impl LocalGenerate for Expr {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        self.0.generate(vars);
    }
}
