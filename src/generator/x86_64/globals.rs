use std::collections::HashMap;

use crate::{
    generator::x86_64::ARG_REGS,
    parser::symbols::{
        globals::FnDec,
        statements::{var_dec::VarDec, Stmt},
    },
};

pub trait LocalGenerate {
    fn generate(&self, env: &mut Env);
}

#[derive(Debug)]
pub struct Env {
    locals: HashMap<String, usize>,
    label_count: usize,
}

impl Env {
    pub fn new_with_vars_size(
        args: &[VarDec],
        stmts: &[Stmt],
        label_count: usize,
    ) -> (Self, usize) {
        let mut locals = HashMap::<String, usize>::new();
        let mut offset: usize = 0;

        for arg in args {
            if !locals.contains_key(&arg.name) {
                offset += arg.typ.aligned_size();

                locals.insert(arg.name.clone(), offset);
            }
        }

        for stmt in stmts {
            if let Stmt::VarDec(vardec) = stmt {
                if !locals.contains_key(&vardec.name) {
                    offset += vardec.typ.aligned_size();

                    locals.insert(vardec.name.clone(), offset);
                }
            }
        }

        (
            Self {
                locals,
                label_count,
            },
            offset,
        )
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
    pub fn generate(&self, label_count: usize) -> usize {
        let (mut env, vars_size) = Env::new_with_vars_size(&self.args, &self.stmts, label_count);

        println!("{}:", self.name);
        println!("push rbp");
        println!("mov rbp, rsp");
        println!("sub rsp, {vars_size}");

        for (i, arg) in self.args.iter().enumerate() {
            if let Some(reg) = ARG_REGS.get(i) {
                println!(
                    "mov [rbp - {}], {}",
                    env.offset(&arg.name).expect("Arg Not Found"),
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

        env.label_count
    }
}
