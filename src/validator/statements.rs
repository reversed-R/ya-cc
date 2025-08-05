pub mod block;
pub mod if_stmt;
pub mod while_stmt;

use crate::{
    parser::symbols::statements::Stmt,
    validator::{Env, ExprTypeValidate, StmtTypeValidate, TypeError},
};

impl StmtTypeValidate for Stmt {
    fn validate_type(&self, env: &mut Env) -> Result<(), TypeError> {
        match self {
            Self::Block(stmts) => {
                for stmt in stmts {
                    stmt.validate_type(env)?;
                }

                Ok(())
            }
            Self::Expr(expr) => {
                expr.validate_type(env)?;

                Ok(())
            }
            Self::Return(expr) => {
                let expr_typ = expr.validate_type(env)?;
                if expr_typ.equals(&env.rtype) {
                    Ok(())
                } else {
                    Err(TypeError::Mismatch(env.rtype.clone(), expr_typ))
                }
            }
            Self::If(if_stmt) => if_stmt.validate_type(env),
            Self::While(while_stmt) => while_stmt.validate_type(env),
            Self::VarDec(_) => {
                // TODO:
                // when support initialization, must validate type
                Ok(())
            }
        }
    }
}
