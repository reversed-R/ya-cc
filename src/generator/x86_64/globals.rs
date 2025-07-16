use std::collections::HashMap;

use crate::parser::symbols::{globals::FnDec, statements::Stmt};

pub trait LocalGenerate {
    fn generate(&self, vars: &mut Vars);
}

const SIZE_OF_VARIABLE: usize = 4;

pub struct Vars {
    args: HashMap<String, usize>,
    locals: HashMap<String, usize>,
}

impl Vars {
    pub fn new(args: &[String], locals: HashMap<String, usize>) -> Self {
        let mut args_map = HashMap::<String, usize>::new();

        for (i, arg) in args.iter().enumerate() {
            if !args_map.contains_key(arg) {
                args_map.insert(arg.clone(), (args_map.len() - i) * SIZE_OF_VARIABLE);
            }
        }

        Self {
            args: args_map,
            locals,
        }
    }

    pub fn offset(&self, id: &String) -> Option<usize> {
        if let Some(offset) = self.locals.get(id) {
            Some(*offset)
        } else {
            self.args.get(id).cloned()
        }
    }
}

impl FnDec {
    pub fn generate(&self) {
        let locals = self.list_local_variables();
        let mut vars = Vars::new(&self.args, locals);

        println!("{}:", self.name);
        println!("push rbp");
        println!("mov rbp, rsp");

        for stmt in &self.stmts {
            stmt.generate(&mut vars);
        }

        println!("pop rbp");
        println!("ret");
    }

    fn list_local_variables(&self) -> HashMap<String, usize> {
        let mut locals: HashMap<String, usize> = HashMap::new();

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
