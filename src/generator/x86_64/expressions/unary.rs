use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::unary::{Unary, UnaryOperator},
};

impl LocalGenerate for Unary {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        match self.op {
            UnaryOperator::Plus => {
                self.right.generate(vars);
            }
            UnaryOperator::Minus => {
                self.right.generate(vars);

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}
