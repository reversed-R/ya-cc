use std::collections::HashMap;

use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::{globals::FnDec, statements::Stmt},
};

impl FnDec {
    pub fn generate(&self) {
        let locals = self.list_local_variables();

        println!("{}:", self.name);
        println!("push rbp");
        println!("mov rbp, rsp");

        for stmt in &self.stmts {
            stmt.generate(&locals);
        }

        println!("pop rbp");
        println!("ret");
    }

    fn list_local_variables(&self) -> HashMap<String, usize> {
        let mut locals: HashMap<String, usize> = HashMap::new();
        const SIZE_OF_VARIABLE: usize = 4;

        for stmt in &self.stmts {
            if let Stmt::Expr(expr) = stmt {
                let ass = &expr.0;

                if let Some(id) = ass.assignable_variable() {
                    if !locals.contains_key(id) {
                        locals.insert(id.clone(), (locals.len() + 1) * SIZE_OF_VARIABLE);
                    }
                }
            }
        }

        locals
    }
}
