pub mod block;
pub mod expr;
pub mod if_stmt;
pub mod while_stmt;

use crate::{generator::x86_64::Generate, parser::symbols::statements::Stmt};

impl Generate for Stmt {
    fn generate(&self) {
        match self {
            Self::Block(stmts) => {
                for stmt in stmts {
                    stmt.generate();
                }
            }
            Self::Expr(expr) => {
                expr.generate();
            }
            Self::Return(ret) => {
                ret.generate();
            }
            Self::If(if_stmt) => {
                if_stmt.generate();
            }
            Self::While(while_stmt) => {
                while_stmt.generate();
            }
        }
    }
}
