use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::expressions::arithmetic::{ArithmExpr, ArithmOperator},
};

impl LocalGenerate for ArithmExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.left.generate(env);

        for arithm in &self.rights {
            match arithm.op {
                ArithmOperator::Add => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Sub => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("sub rax, rdi");
                    println!("push rax");
                }
            }
        }
    }
}
