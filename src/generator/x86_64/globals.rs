use std::collections::HashMap;

use crate::{
    generator::x86_64::ARG_REGS,
    parser::symbols::{globals::FnDec, statements::Stmt},
};

pub trait LocalGenerate {
    fn generate(&self, env: &mut Env);
}

const SIZE_OF_VARIABLE: usize = 8;

#[derive(Debug)]
pub struct Env {
    locals: HashMap<String, usize>,
    label_count: usize,
}

impl Env {
    pub fn new(args: &[String], stmts: &[Stmt]) -> Self {
        let mut locals = HashMap::<String, usize>::new();

        for arg in args {
            if !locals.contains_key(arg) {
                locals.insert(arg.clone(), (locals.len() + 1) * SIZE_OF_VARIABLE);
            }
        }

        for stmt in stmts {
            if let Stmt::Expr(expr) = stmt {
                let ass = &expr.0;

                if let Some(id) = ass.assignable_variable() {
                    if !locals.contains_key(id) {
                        locals.insert(id.clone(), (locals.len() + 1) * SIZE_OF_VARIABLE);
                    }
                }
            }
        }

        Self {
            locals,
            label_count: 0,
        }
    }

    pub fn offset(&self, id: &String) -> Option<usize> {
        self.locals.get(id).cloned()
    }

    pub fn increment_label(&mut self) -> usize {
        self.label_count += 1;

        self.label_count
    }
}

impl FnDec {
    pub fn generate(&self) {
        let mut env = Env::new(&self.args, &self.stmts);

        println!("{}:", self.name);
        println!("push rbp");
        println!("mov rbp, rsp");
        println!("sub rsp, {}", env.locals.len() * SIZE_OF_VARIABLE);

        for (i, arg) in self.args.iter().enumerate() {
            if let Some(reg) = ARG_REGS.get(i) {
                println!(
                    "mov [rbp - {}], {}",
                    env.offset(arg).expect("Arg Not Found"),
                    reg
                );
            } else {
                panic!("Too Many Args for Function Call");
            }
        }

        for stmt in &self.stmts {
            stmt.generate(&mut env);
        }

        if self.name == "main" {
            println!("leave");
            println!("ret");
        } else {
            println!("mov rsp, rbp");
            println!("pop rbp");
            println!("ret");
        }
    }
}
