pub mod block;
pub mod if_stmt;
pub mod while_stmt;

use crate::{generator::x86_64::globals::LocalGenerate, parser::symbols::statements::Stmt};

impl LocalGenerate for Stmt {
    fn generate(&self, env: &mut super::globals::Env) {
        match self {
            Self::Block(stmts) => {
                for stmt in stmts {
                    stmt.generate(env);
                }
            }
            Self::Expr(expr) => {
                expr.generate(env);

                println!("pop rax");
            }
            Self::Return(expr) => {
                expr.generate(env);

                println!("pop rax");
                println!("mov rsp, rbp");
                println!("pop rbp");
                println!("ret");
            }
            Self::If(if_stmt) => {
                if_stmt.generate(env);
            }
            Self::While(while_stmt) => {
                while_stmt.generate(env);
            }
        }
    }
}
