use std::collections::HashMap;

use crate::{
    generator::x86_64::ARG_REGS,
    parser::symbols::{globals::FnDec, statements::Stmt},
};

pub trait LocalGenerate {
    fn generate(&self, vars: &mut Vars);
}

const SIZE_OF_VARIABLE: usize = 4;

pub struct Vars {
    locals: HashMap<String, usize>,
}

impl Vars {
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

        Self { locals }
    }

    pub fn offset(&self, id: &String) -> Option<usize> {
        self.locals.get(id).cloned()
    }
}

impl FnDec {
    pub fn generate(&self) {
        let mut vars = Vars::new(&self.args, &self.stmts);

        println!("{}:", self.name);
        println!("push rbp");
        if self.name == "main" {
            println!("mov rbp, rsp");
        }
        println!("and rsp, -16");
        println!("sub rsp, {}", vars.locals.len() * SIZE_OF_VARIABLE);

        for (i, arg) in self.args.iter().enumerate() {
            if let Some(reg) = ARG_REGS.get(i) {
                println!(
                    "mov [rbp - {}], {}",
                    vars.offset(arg).expect("Arg Not Found"),
                    reg
                );
            } else {
                panic!("Too Many Args for Function Call");
            }
        }

        for stmt in &self.stmts {
            stmt.generate(&mut vars);
        }

        println!("leave");
        println!("ret");
    }
}
