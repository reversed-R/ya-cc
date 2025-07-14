use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::unary::{Unary, UnaryOperator},
};

impl LocalGenerate for Unary {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        match self.op {
            UnaryOperator::Plus => {
                self.right.generate(locals);
            }
            UnaryOperator::Minus => {
                self.right.generate(locals);

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}
