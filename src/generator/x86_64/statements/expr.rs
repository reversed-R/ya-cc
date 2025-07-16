use crate::{generator::x86_64::globals::LocalGenerate, parser::symbols::expressions::Expr};

impl LocalGenerate for Expr {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        self.0.generate(vars);

        println!("pop rax");
    }
}
