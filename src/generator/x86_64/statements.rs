pub mod block;
pub mod expr;
pub mod if_stmt;
pub mod while_stmt;

use crate::{generator::x86_64::LocalGenerate, parser::symbols::statements::Stmt};

impl LocalGenerate for Stmt {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        match self {
            Self::Block(stmts) => {
                for stmt in stmts {
                    stmt.generate(locals);
                }
            }
            Self::Expr(expr) => {
                expr.generate(locals);
            }
            Self::Return(ret) => {
                ret.generate(locals);
            }
            Self::If(if_stmt) => {
                if_stmt.generate(locals);
            }
            Self::While(while_stmt) => {
                while_stmt.generate(locals);
            }
        }
    }
}
