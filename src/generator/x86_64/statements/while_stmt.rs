use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::while_stmt::WhileStmt,
};

impl LocalGenerate for WhileStmt {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        let label_count = env.increment_label();

        println!(".Lbegin{label_count}:");

        self.cond.generate(env);
        println!("pop rax");
        println!("cmp rax, 0");
        println!("je .Lend{label_count}");

        self.stmt.generate(env);
        println!("jmp .Lbegin{label_count}");

        println!(".Lend{label_count}:");
    }
}
