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
                env.begin_scope();

                for stmt in stmts {
                    stmt.validate_type(env)?;
                }

                env.end_scope();

                Ok(())
            }
            Self::Expr(expr) => {
                expr.validate_type(env)?;

                Ok(())
            }
            Self::Return(expr) => {
                let expr_typ = expr.validate_type(env)?;

                if let Some(rtype) = &env.rtype {
                    if expr_typ.equals(rtype) {
                        Ok(())
                    } else {
                        Err(TypeError::Mismatch(rtype.clone(), expr_typ))
                    }
                } else {
                    Err(TypeError::OutOfScopes)
                }
            }
            Self::If(if_stmt) => if_stmt.validate_type(env),
            Self::While(while_stmt) => while_stmt.validate_type(env),
            Self::VarDec(var) => {
                env.vars.insert(var.name.clone(), var.typ.clone())

                // TODO:
                // when support initialization, must validate type
            }
        }
    }
}
