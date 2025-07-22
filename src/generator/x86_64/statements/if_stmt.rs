use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::if_stmt::IfStmt,
};

impl LocalGenerate for IfStmt {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        let label_count = env.increment_label();

        self.cond.generate(env);

        println!("pop rax");
        println!("cmp rax, 0");

        match &self.els {
            Some(els) => {
                println!("je .Lelse{label_count}");

                self.then.generate(env);
                println!("jmp .Lend{label_count}");

                println!(".Lelse{label_count}:");
                els.generate(env);
            }
            None => {
                println!("je .Lend{label_count}");

                self.then.generate(env);
            }
        }

        println!(".Lend{label_count}:");
    }
}
