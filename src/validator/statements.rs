pub mod branch_stmt;
pub mod compound;
pub mod loop_stmt;
pub mod vardec_stmt;

use crate::validator::{
    expressions::Expr,
    statements::{branch_stmt::BranchStmt, loop_stmt::LoopStmt},
    Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub enum Stmt {
    Compound(Vec<Stmt>),
    Expr(Expr),
    Return(Expr),
    Branch(Box<BranchStmt>),
    Loop(Box<LoopStmt>),
    VarDec, // in codegen, variable declaration statement is nothing to do
}

impl StmtTypeValidate for crate::parser::symbols::statements::Stmt {
    type ValidatedType = Stmt;

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        match self {
            Self::Block(stmts) => {
                env.begin_scope();

                let stmts = stmts
                    .iter()
                    .map(|stmt| stmt.validate(env))
                    .collect::<Result<Vec<Stmt>, TypeError>>()?;

                env.end_scope();

                Ok(Stmt::Compound(stmts))
            }
            Self::Expr(expr) => Ok(Stmt::Expr(expr.validate(env)?.1)),
            Self::Return(expr) => {
                let (expr_typ, expr) = expr.validate(env)?;

                if let Some(rtype) = &env.rtype {
                    if expr_typ.equals(rtype) {
                        Ok(Stmt::Return(expr))
                    } else {
                        Err(TypeError::Mismatch(rtype.clone(), expr_typ))
                    }
                } else {
                    Err(TypeError::OutOfScopes)
                }
            }
            Self::If(if_stmt) => Ok(Stmt::Branch(Box::new(if_stmt.validate(env)?))),
            Self::While(while_stmt) => Ok(Stmt::Loop(Box::new(while_stmt.validate(env)?))),
            Self::VarDec(var) => {
                env.vars.insert(var.name.clone(), var.typ.clone())?;

                Ok(Stmt::VarDec)

                // TODO:
                // when support initialization, must validate type
            }
        }
    }
}
