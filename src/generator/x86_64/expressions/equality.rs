use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::equality::{EqualityExpr, EqualityOperator},
};

impl LocalGenerate for EqualityExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.left.generate(env);

        for equal in &self.rights {
            match equal.op {
                EqualityOperator::Equal => {
                    equal.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("sete al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                EqualityOperator::NotEq => {
                    equal.right.generate(env);

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
