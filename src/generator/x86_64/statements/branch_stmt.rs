use crate::{
    generator::x86_64::globals::LocalGenerate, validator::statements::branch_stmt::BranchStmt,
};

impl LocalGenerate for BranchStmt {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        let label_count = env.increment_label();

        self.cond.generate(env);

        println!("cmp rax, 0");

        match &self.els {
            Some(els) => {
                println!("je .L{}$else{label_count}", env.fname);

                self.then.generate(env);
                println!("jmp .L{}$end{label_count}", env.fname);

                println!(".L{}$else{label_count}:", env.fname);
                els.generate(env);
            }
            None => {
                println!("je .L{}$end{label_count}", env.fname);

                self.then.generate(env);
            }
        }

        println!(".L{}$end{label_count}:", env.fname);
    }
}
