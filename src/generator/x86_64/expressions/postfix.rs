use crate::{
    generator::x86_64::globals::LocalGenerate, validator::expressions::postfix::PostfixExpr,
};

impl LocalGenerate for PostfixExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self {
            Self::Primary(prim) => {
                prim.generate(env);
            }
            Self::Unary(unary) => {
                unary.generate(env);
            }
        }
    }
}
