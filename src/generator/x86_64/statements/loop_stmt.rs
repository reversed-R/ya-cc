use crate::{
    generator::x86_64::globals::LocalGenerate, validator::statements::loop_stmt::LoopStmt,
};

impl LocalGenerate for LoopStmt {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        let label_count = env.increment_label();

        println!(".L{}$begin{label_count}:", env.fname);

        self.cond.generate(env);
        println!("pop rax");
        println!("cmp rax, 0");
        println!("je .L{}$end{label_count}", env.fname);

        self.stmt.generate(env);
        println!("jmp .L{}$begin{label_count}", env.fname);

        println!(".L{}$end{label_count}:", env.fname);
    }
}
