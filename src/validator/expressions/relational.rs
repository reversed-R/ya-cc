use crate::{
    parser::symbols::expressions::relational,
    validator::{expressions::arithmetic::ArithmExpr, Env, ExprTypeValidate, Type, TypeError},
};

#[derive(Debug)]
pub struct RelationalExpr {
    pub left: ArithmExpr,
    pub rights: Vec<RelationalExprNode>,
}

#[derive(Debug)]
pub struct RelationalExprNode {
    pub op: RelationalOperator,
    pub right: ArithmExpr,
}

#[derive(Debug)]
pub enum RelationalOperator {
    Lesser,  // <
    Greater, // >
    LesEq,   // <=
    GrtEq,   // >=
}

impl From<&relational::RelationalOperator> for RelationalOperator {
    fn from(value: &relational::RelationalOperator) -> Self {
        match value {
            relational::RelationalOperator::Lesser => Self::Lesser,
            relational::RelationalOperator::Greater => Self::Greater,
            relational::RelationalOperator::LesEq => Self::LesEq,
            relational::RelationalOperator::GrtEq => Self::GrtEq,
        }
    }
}

impl ExprTypeValidate for relational::RelationalExpr {
    type ValidatedType = (Type, RelationalExpr);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let (typ, left) = self.left.validate(env)?;
        let mut rights = vec![];

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            if !typ.equals(&right_typ) {
                return Err(TypeError::Mismatch(typ, right_typ));
            }

            rights.push(RelationalExprNode {
                op: RelationalOperator::from(&r.op),
                right,
            });
        }

        Ok((typ, RelationalExpr { left, rights }))
    }
}
