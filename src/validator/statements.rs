use crate::validator::{
    expressions::Exprs, types::TypeComarison, Env, ExprTypeValidate, ValidateError,
};

#[derive(Debug)]
pub enum Stmt {
    Compound(Vec<Stmt>),
    Expr(Exprs),
    Return(Exprs),
    Branch(Box<BranchStmt>),
    Loop(Box<LoopStmt>),
}

#[derive(Debug)]
pub struct BranchStmt {
    pub cond: Exprs,
    pub then: Stmt,
    pub els: Option<Stmt>,
}

#[derive(Debug)]
pub struct LoopStmt {
    pub cond: Exprs,
    pub stmt: Stmt,
}

impl crate::parser::symbols::statements::Stmt {
    pub fn validate(&self, env: &mut Env) -> Result<Option<Stmt>, ValidateError> {
        match self {
            Self::Block(stmts) => {
                env.begin_scope();

                let stmts = stmts
                    .iter()
                    .map(|stmt| stmt.validate(env))
                    .collect::<Result<Vec<Option<Stmt>>, ValidateError>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                env.end_scope();

                Ok(Some(Stmt::Compound(stmts)))
            }
            Self::Expr(expr) => Ok(Some(Stmt::Expr(expr.validate(env)?.1))),
            Self::Return(expr) => {
                let (expr_typ, expr) = expr.validate(env)?;

                if let Some(local) = &env.local {
                    match local.rtype.compare(&expr_typ) {
                        TypeComarison::Equal => Ok(Some(Stmt::Return(expr))),
                        TypeComarison::ImplicitlyConvertableFrom => Ok(Some(Stmt::Return(expr))),
                        _ => Err(ValidateError::Mismatch(
                            Box::new(local.rtype.clone()),
                            Box::new(expr_typ),
                        )),
                    }
                } else {
                    Err(ValidateError::OutOfScopes)
                }
            }
            Self::If(if_stmt) => Ok(Some(Stmt::Branch(Box::new(BranchStmt {
                cond: if_stmt.cond.validate(env)?.1,
                then: if_stmt.then.validate(env)?.expect("error"),
                els: if_stmt
                    .els
                    .as_ref()
                    .map(|els| els.validate(env))
                    .transpose()?
                    .flatten(),
            })))),
            Self::While(while_stmt) => Ok(Some(Stmt::Loop(Box::new(LoopStmt {
                cond: while_stmt.cond.validate(env)?.1,
                stmt: while_stmt.stmt.validate(env)?.expect("error"),
            })))),
            Self::VarDec(var) => {
                env.insert_var(var.name.clone(), var.typ.clone())?;

                Ok(None)

                // TODO:
                // when support initialization, must validate type
            }
        }
    }
}
