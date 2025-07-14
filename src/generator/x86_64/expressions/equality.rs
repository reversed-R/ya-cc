use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::equality::{EqualityExpr, EqualityOperator},
};

impl LocalGenerate for EqualityExpr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        self.left.generate(locals);

        for equal in &self.rights {
            match equal.op {
                EqualityOperator::Equal => {
                    equal.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("sete al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                EqualityOperator::NotEq => {
                    equal.right.generate(locals);

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
