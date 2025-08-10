use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::expressions::multiplication::{MulExpr, MulOperator},
};

impl LocalGenerate for MulExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.left.generate(env);

        for mul in &self.rights {
            match mul.op {
                MulOperator::Mul => {
                    mul.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("imul rax, rdi");
                    println!("push rax");
                }
                MulOperator::Div => {
                    mul.right.generate(env);

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
