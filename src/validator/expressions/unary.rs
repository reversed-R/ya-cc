use crate::{
    parser::symbols::expressions::unary,
    validator::{
        expressions::{
            postfix::PostfixExpr,
            primary::{Literal, Primary},
        },
        Env, ExprTypeValidate, PrimitiveType, Type, TypeError,
    },
};

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOperator,
    pub refop: RefUnaryOperator,
    pub right: PostfixExpr,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    None,
    Neg,
}

impl From<&unary::UnaryOperator> for UnaryOperator {
    fn from(value: &unary::UnaryOperator) -> Self {
        match value {
            unary::UnaryOperator::SizeOf => Self::None,
            unary::UnaryOperator::Plus => Self::None,
            unary::UnaryOperator::Minus => Self::Neg,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RefUnaryOperator {
    Ref,          // &
    Deref(usize), // *
}

impl ExprTypeValidate for unary::Unary {
    type ValidatedType = (Type, Unary);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        let (mut typ, mut right) = self.right.right.validate(env)?;
        let mut ref_count: isize = 0;

        for op in self.right.ops.iter().rev() {
            match op {
                unary::RefUnaryOperator::Ref => {
                    typ = Type::ptr_to(typ);
                    ref_count -= 1;
                }
                unary::RefUnaryOperator::Deref => {
                    if let Some(deref) = Type::deref_of(&typ) {
                        typ = deref;
                        ref_count += 1;
                    } else {
                        return Err(TypeError::DerefNotAllowed(typ));
                    }
                }
            }
        }

        let op = UnaryOperator::from(&self.op);

        let refop;

        if ref_count >= 0 {
            refop = RefUnaryOperator::Deref(ref_count as usize);
        } else if ref_count == -1 {
            refop = RefUnaryOperator::Ref;
        } else {
            return Err(TypeError::DerefNotAllowed(typ));
        }

        if let unary::UnaryOperator::SizeOf = &self.op {
            typ = Type::Primitive(PrimitiveType::Int);
            right = PostfixExpr::Primary(Primary::Literal(Literal::Int(typ.aligned_size() as i64)));
        }

        Ok((typ, Unary { op, refop, right }))
    }
}
