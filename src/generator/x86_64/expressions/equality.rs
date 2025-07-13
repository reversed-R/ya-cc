use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::equality::{EqualityExpr, EqualityOperator},
};

impl Generate for EqualityExpr {
    fn generate(&self) {
        self.left.generate();

        for equal in &self.rights {
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
    }
}
