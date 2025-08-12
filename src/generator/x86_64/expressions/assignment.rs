use crate::{
    generator::x86_64::globals::LocalGenerate, validator::expressions::assignment::AssignExpr,
};

impl LocalGenerate for AssignExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.src.generate(env);

        for ass in &self.dsts {
            ass.dst.generate_as_left(env);

            println!("pop rdi");
            println!("pop rax");

            println!("mov [rdi], rax");
            println!("push rax");
        }
    }
}
