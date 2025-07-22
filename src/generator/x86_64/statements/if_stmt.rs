use crate::{
    generator::x86_64::globals::LocalGenerate, parser::symbols::statements::if_stmt::IfStmt,
};

impl LocalGenerate for IfStmt {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        self.cond.generate(vars);

        println!("pop rax");
        println!("cmp rax, 0");

        match &self.els {
            Some(els) => {
                println!("je .Lelse000");

                self.then.generate(vars);
                println!("jmp .Lend000");

                println!(".Lelse000:");
                els.generate(vars);
            }
            None => {
                println!("je .Lend000");

                self.then.generate(vars);
            }
        }

        println!(".Lend000:");
    }
}
