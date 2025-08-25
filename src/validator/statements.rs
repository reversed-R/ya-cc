pub mod branch_stmt;
pub mod compound;
pub mod loop_stmt;

use crate::validator::{
    expressions::Exprs,
    statements::{branch_stmt::BranchStmt, loop_stmt::LoopStmt},
    types::TypeComarison,
    Env, ExprTypeValidate, StmtTypeValidate, TypeError,
};

#[derive(Debug)]
pub enum Stmt {
    Compound(Vec<Stmt>),
    Expr(Exprs),
    Return(Exprs),
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

                if let Some(local) = &env.local {
                    match local.rtype.compare(&expr_typ) {
                        TypeComarison::Equal => Ok(Stmt::Return(expr)),
                        TypeComarison::ImplicitlyConvertableFrom => Ok(Stmt::Return(expr)),
                        _ => Err(TypeError::Mismatch(
                            Box::new(local.rtype.clone()),
                            Box::new(expr_typ),
                        )),
                    }
                } else {
                    Err(TypeError::OutOfScopes)
                }
            }
            Self::If(if_stmt) => Ok(Stmt::Branch(Box::new(if_stmt.validate(env)?))),
            Self::While(while_stmt) => Ok(Stmt::Loop(Box::new(while_stmt.validate(env)?))),
            Self::VarDec(var) => {
                env.insert_var(var.name.clone(), var.typ.clone())?;

                Ok(Stmt::VarDec)

                // TODO:
                // when support initialization, must validate type
            }
        }
    }
}
