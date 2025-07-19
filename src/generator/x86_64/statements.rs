pub mod block;
pub mod if_stmt;
pub mod while_stmt;

use crate::{generator::x86_64::globals::LocalGenerate, parser::symbols::statements::Stmt};

impl LocalGenerate for Stmt {
    fn generate(&self, vars: &mut super::globals::Vars) {
        match self {
            Self::Block(stmts) => {
                for stmt in stmts {
                    stmt.generate(vars);
                }
            }
            Self::Expr(expr) => {
                expr.generate(vars);

                println!("pop rax");
            }
            Self::Return(expr) => {
                expr.generate(vars);

                println!("pop rax");
                println!("leave");
                println!("ret");
            }
            Self::If(if_stmt) => {
                if_stmt.generate(vars);
            }
            Self::While(while_stmt) => {
                while_stmt.generate(vars);
            }
        }
    }
}
