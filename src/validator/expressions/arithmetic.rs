use crate::{
    parser::symbols::expressions::arithmetic,
    validator::{expressions::multiplication::MulExpr, Env, ExprTypeValidate, Type, TypeError},
};

#[derive(Debug)]
pub struct ArithmExpr {
    pub left: MulExpr,
    pub rights: Vec<ArithmExprNode>,
}

#[derive(Debug)]
pub struct ArithmExprNode {
    pub op: ArithmOperator,
    pub right: MulExpr,
}

// TODO:
// devide operations by type
// e.g. iadd, isub, fadd
#[derive(Debug)]
pub enum ArithmOperator {
    Add, // +
    Sub, // -
}

impl From<&arithmetic::ArithmOperator> for ArithmOperator {
    fn from(value: &arithmetic::ArithmOperator) -> Self {
        match value {
            arithmetic::ArithmOperator::Add => Self::Add,
            arithmetic::ArithmOperator::Sub => Self::Sub,
        }
    }
}

impl ExprTypeValidate for crate::parser::symbols::expressions::arithmetic::ArithmExpr {
    type ValidatedType = (Type, ArithmExpr);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        let (typ, left) = self.left.validate(env)?;
        let mut rights = vec![];

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }

            rights.push(ArithmExprNode {
                op: ArithmOperator::from(&r.op),
                right,
            });
        }

        Ok((typ, ArithmExpr { left, rights }))
    }
}
