use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::equality::{EqualityExpr, EqualityOperator},
};

impl Generate for EqualityExpr {
    fn generate(&self) {
        let mut i = 0;
        for equal in &self.nodes {
            if i == 0 {
                equal.right.generate();
            } else {
                match equal.op {
                    EqualityOperator::Equal => {
                        equal.right.generate();

                        println!("pop rdi");
                        println!("pop rax");
                        println!("cmp rax, rdi");
                        println!("sete al");
                        println!("movzb rax, al");
                        println!("push rax");
                    }
                    EqualityOperator::NotEq => {
                        equal.right.generate();

                        println!("pop rdi");
                        println!("pop rax");
                        println!("cmp rax, rdi");
                        println!("setne al");
                        println!("movzb rax, al");
                        println!("push rax");
                    }
                }
            }

            i += 1;
        }
    }
}
