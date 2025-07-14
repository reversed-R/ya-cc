use crate::{generator::x86_64::LocalGenerate, parser::symbols::expressions::Expr};

impl LocalGenerate for Expr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        self.0.generate(locals);

        println!("pop rax");
    }
}
