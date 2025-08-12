pub mod arithmetic;
pub mod assignment;
pub mod equality;
pub mod multiplication;
pub mod postfix;
pub mod primary;
pub mod relational;
pub mod unary;

use crate::{generator::x86_64::globals::LocalGenerate, validator::expressions::Expr};

impl LocalGenerate for Expr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.0.generate(env);
    }
}
