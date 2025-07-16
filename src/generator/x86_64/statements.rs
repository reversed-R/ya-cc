pub mod block;
pub mod expr;
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
            }
            Self::Return(ret) => {
                ret.generate(vars);
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
