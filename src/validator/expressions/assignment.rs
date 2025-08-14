use crate::{
    parser::symbols::expressions::assignment,
    validator::{
        expressions::{
            equality::EqualityExpr,
            postfix::PostfixExpr,
            primary::{Literal, Primary},
            unary::{RefUnaryOperator, Unary, UnaryOperator},
        },
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

fn is_numeric_zero(src: &EqualityExpr) -> bool {
    if src.rights.is_empty() {
        if src.left.rights.is_empty() {
            let arithm = &src.left.left;
            if arithm.rights.is_empty() {
                let mul = &arithm.left;

                if mul.rights.is_empty() {
                    let unary = &mul.left;

                    if let UnaryOperator::None = unary.op {
                        if let RefUnaryOperator::Deref(0) = &unary.refop {
                            let postfix = &unary.right;

                            if let PostfixExpr::Primary(Primary::Literal(Literal::Int(0))) = postfix
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

impl ExprTypeValidate for assignment::AssignExpr {
    type ValidatedType = (Type, AssignExpr);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let (src_typ, src) = self.right.validate(env)?;
        let mut typ = src_typ.clone();
        let mut dsts = vec![];

        for left in self.lefts.iter().rev() {
            let (dst_typ, dst) = left.left.validate(env)?;

            if typ.equals(&dst_typ) {
                // typ = Type::Primitive(PrimitiveType::Int);
            } else if src_typ == Type::Primitive(PrimitiveType::Int)
                && is_numeric_zero(&src)
                && matches!(dst_typ, Type::PtrTo((_)))
            {
                typ = dst_typ;
            } else {
                return Err(TypeError::Mismatch(dst_typ, typ));
            }

            dsts.push(AssignDst {
                dst,
                op: AssignOperator::from(&left.op),
            });
        }

        Ok((typ, AssignExpr { src, dsts }))
    }
}
