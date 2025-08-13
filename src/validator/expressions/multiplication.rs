use crate::{
    parser::symbols::expressions::multiplication,
    validator::{expressions::unary::Unary, Env, ExprTypeValidate, Type, TypeError},
};

#[derive(Debug)]
pub struct MulExpr {
    pub left: Unary,
    pub rights: Vec<MulExprNode>,
}

#[derive(Debug)]
pub struct MulExprNode {
    pub op: MulOperator,
    pub right: Unary,
}

#[derive(Debug)]
pub enum MulOperator {
    Mul,
    Div,
    Mod,
}

impl From<&multiplication::MulOperator> for MulOperator {
    fn from(value: &multiplication::MulOperator) -> Self {
        match value {
            multiplication::MulOperator::Mul => MulOperator::Mul,
            multiplication::MulOperator::Div => MulOperator::Div,
            multiplication::MulOperator::Mod => MulOperator::Mod,
        }
    }
}

impl ExprTypeValidate for crate::parser::symbols::expressions::multiplication::MulExpr {
    type ValidatedType = (Type, MulExpr);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let (typ, left) = self.left.validate(env)?;
        let mut rights = vec![];

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }

            rights.push(MulExprNode {
                op: MulOperator::from(&r.op),
                right,
            });
        }

        Ok((typ, MulExpr { left, rights }))
    }
}
