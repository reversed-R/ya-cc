pub mod branch_stmt;
pub mod loop_stmt;

use crate::{generator::x86_64::globals::LocalGenerate, validator::statements::Stmt};

impl LocalGenerate for Stmt {
    fn generate(&self, env: &mut super::globals::Env) {
        match self {
            Self::Compound(stmts) => {
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
            Self::Branch(branch_stmt) => {
                branch_stmt.generate(env);
            }
            Self::Loop(loop_stmt) => {
                loop_stmt.generate(env);
            }
            Self::VarDec => {
                //nothing to do
            }
        }
    }
}
