use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::unary::{Unary, UnaryOperator},
};

impl LocalGenerate for Unary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self.op {
            UnaryOperator::Plus => {
                self.right.generate(env);
            }
            UnaryOperator::Minus => {
                self.right.generate(env);

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}
