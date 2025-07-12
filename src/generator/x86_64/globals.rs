use crate::{generator::x86_64::Generate, parser::symbols::globals::FnDec};

impl Generate for FnDec {
    fn generate(&self) {
        println!("{}:", self.name);

        for stmt in &self.stmts {
            stmt.generate();
        }

        println!("ret");
    }
}
