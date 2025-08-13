use crate::{
    parser::symbols::expressions::assignment,
    validator::{
        expressions::{equality::EqualityExpr, unary::Unary},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeError,
    },
};

// NOTE:
// you must compile src -> dst[0] -> dst[1] -> ... -> dst[i]
#[derive(Debug)]
pub struct AssignExpr {
    pub src: EqualityExpr,
    pub dsts: Vec<AssignDst>,
}

#[derive(Debug)]
pub struct AssignDst {
    pub dst: Unary,
    pub op: AssignOperator,
}

#[derive(Debug)]
pub enum AssignOperator {
    Assign,
}

impl From<&assignment::AssignOperator> for AssignOperator {
    fn from(value: &assignment::AssignOperator) -> Self {
        match value {
            assignment::AssignOperator::Assign => AssignOperator::Assign,
        }
    }
}

impl ExprTypeValidate for assignment::AssignExpr {
    type ValidatedType = (Type, AssignExpr);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let (src_typ, src) = self.right.validate(env)?;
        let mut typ = src_typ;
        let mut dsts = vec![];

        for left in self.lefts.iter().rev() {
            let (dst_typ, dst) = left.left.validate(env)?;

            if !typ.equals(&dst_typ) {
                return Err(TypeError::Mismatch(dst_typ, typ));
            }

            typ = Type::Primitive(PrimitiveType::Int);
            dsts.push(AssignDst {
                dst,
                op: AssignOperator::from(&left.op),
            });
        }

        Ok((typ, AssignExpr { src, dsts }))
    }
}
