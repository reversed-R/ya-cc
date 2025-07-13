use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::unary::{Unary, UnaryOperator},
};

impl Generate for Unary {
    fn generate(&self) {
        match self.op {
            UnaryOperator::Plus => {
                self.right.generate();
            }
            UnaryOperator::Minus => {
                self.right.generate();

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}
