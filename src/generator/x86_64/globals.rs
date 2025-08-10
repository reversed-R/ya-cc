use crate::{generator::x86_64::ARG_REGS, validator::globals::Function};

pub trait LocalGenerate {
    fn generate(&self, env: &mut Env);
}

#[derive(Debug)]
pub struct Env {
    pub fname: String,
    pub label_count: usize,
}

impl Env {
    pub fn new(fname: String) -> Self {
        Self {
            fname,
            label_count: 0,
        }
    }

    pub fn increment_label(&mut self) -> usize {
        self.label_count += 1;

        self.label_count
    }
}

impl Function {
    pub fn generate(&self, name: &str) {
        let mut env = Env::new(name.to_string());

        println!("{}:", name);
        println!("push rbp");
        println!("mov rbp, rsp");
        println!("sub rsp, {}", self.local_max_offset);

        for i in 0..self.arg_count {
            if let Some(reg) = ARG_REGS.get(i) {
                println!("mov [rbp - {}], {}", i * 8, reg);
            } else {
                panic!("Too Many Args for Function Call");
            }
        }

        for stmt in &self.stmts {
            stmt.generate(&mut env);
        }

        if name == "main" {
            println!("leave");
            println!("ret");
        } else {
            println!("mov rsp, rbp");
            println!("pop rbp");
            println!("ret");
        }
    }
}
