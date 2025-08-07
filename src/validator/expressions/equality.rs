use crate::{
    parser::symbols::expressions::equality,
    validator::{expressions::relational::RelationalExpr, Env, ExprTypeValidate, Type, TypeError},
};

#[derive(Debug)]
pub struct EqualityExpr {
    pub left: RelationalExpr,
    pub rights: Vec<EqualityExprNode>,
}

#[derive(Debug)]
pub struct EqualityExprNode {
    pub op: EqualityOperator,
    pub right: RelationalExpr,
}

#[derive(Debug)]
pub enum EqualityOperator {
    Equal,
    NotEq,
}

impl From<&equality::EqualityOperator> for EqualityOperator {
    fn from(value: &equality::EqualityOperator) -> Self {
        match value {
            equality::EqualityOperator::Equal => Self::Equal,
            equality::EqualityOperator::NotEq => Self::NotEq,
        }
    }
}

impl ExprTypeValidate for equality::EqualityExpr {
    type ValidatedType = (Type, EqualityExpr);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        let (typ, left) = self.left.validate(env)?;
        let mut rights = vec![];

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }

            rights.push(EqualityExprNode {
                op: EqualityOperator::from(&r.op),
                right,
            });
        }

        Ok((typ, EqualityExpr { left, rights }))
    }
}
