pub mod binary;
pub mod primary;
pub mod unary;

use crate::{generator::x86_64::globals::LocalGenerate, validator::expressions::Exprs};

impl LocalGenerate for Exprs {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self {
            Exprs::Primary(prim) => {
                primary::generate(prim, env);
            }
            Exprs::Unary(unary) => {
                unary::generate(unary, env);
            }
            Exprs::Binary(bin) => {
                binary::generate(bin, env);
            }
        }
    }
}
