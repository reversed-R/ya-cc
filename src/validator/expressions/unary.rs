use crate::{
    parser::symbols::expressions::unary,
    validator::{expressions::primary::Primary, Env, ExprTypeValidate, Type, TypeError},
};

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOperator,
    pub refop: RefUnaryOperator,
    pub right: Primary,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    None,
    Neg,
}

impl From<&unary::UnaryOperator> for UnaryOperator {
    fn from(value: &unary::UnaryOperator) -> Self {
        match value {
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
        let (mut typ, right) = self.right.right.validate(env)?;
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

        if ref_count >= 0 {
            Ok((
                typ,
                Unary {
                    op,
                    refop: RefUnaryOperator::Deref(ref_count as usize),
                    right,
                },
            ))
        } else if ref_count == -1 {
            Ok((
                typ,
                Unary {
                    op,
                    refop: RefUnaryOperator::Ref,
                    right,
                },
            ))
        } else {
            Err(TypeError::DerefNotAllowed(typ))
        }
    }
}
