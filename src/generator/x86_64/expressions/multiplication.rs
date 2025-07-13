use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::multiplication::{MulExpr, MulOperator},
};

impl Generate for MulExpr {
    fn generate(&self) {
        self.left.generate();

        for mul in &self.rights {
            match mul.op {
                MulOperator::Mul => {
                    mul.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("imul rax, rdi");
                    println!("push rax");
                }
                MulOperator::Div => {
                    mul.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cqo");
                    println!("idiv rdi");
                    println!("push rax");
                }
            }
        }
    }
}
