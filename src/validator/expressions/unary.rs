use crate::{
    parser::symbols::expressions::unary,
    validator::{
        expressions::{Exprs, Literal, Primary, UnOperator, Unary},
        Env, ExprTypeValidate, PrimitiveType, Type, TypeError,
    },
};

impl From<&unary::UnaryOperator> for Option<UnOperator> {
    fn from(value: &unary::UnaryOperator) -> Self {
        match value {
            unary::UnaryOperator::SizeOf => None,
            unary::UnaryOperator::Plus => None,
            unary::UnaryOperator::Minus => Some(UnOperator::Neg),
        }
    }
}

impl ExprTypeValidate for unary::Unary {
    fn validate(&self, env: &mut Env) -> Result<(Type, super::Exprs), TypeError> {
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

        let refop;

        if ref_count >= 0 {
            if let Some(base) = typ.get_ptr_base() {
                match base {
                    Type::Primitive(prim) => {
                        match prim {
                            PrimitiveType::Int => {
                                refop = UnOperator::IDeref(ref_count as usize);
                            }
                            PrimitiveType::Char => {
                                refop = UnOperator::CDeref(ref_count as usize);
                            }
                            PrimitiveType::Void => {
                                // WARN: is it true?
                                refop = UnOperator::IDeref(ref_count as usize);
                            }
                        }
                    }
                    Type::Struct(_) => {
                        // WARN: PDeref
                        refop = UnOperator::IDeref(ref_count as usize);
                    }
                    Type::Incomplete(_) => {
                        // WARN: PDeref
                        refop = UnOperator::IDeref(ref_count as usize);
                    }
                    _ => {
                        return Err(TypeError::DerefNotAllowed(typ));
                    }
                }
            } else {
                return Err(TypeError::DerefNotAllowed(typ));
            }
        } else if ref_count == -1 {
            refop = UnOperator::Ref;
        } else {
            return Err(TypeError::DerefNotAllowed(typ));
        }

        let is_neg: bool;

        match &self.op {
            unary::UnaryOperator::SizeOf => {
                is_neg = false;
                typ = Type::Primitive(PrimitiveType::Int);
                right = Exprs::Primary(Primary::Literal(Literal::Int(typ.size() as i64)));
            }
            unary::UnaryOperator::Plus => {
                is_neg = false;
            }
            unary::UnaryOperator::Minus => {
                is_neg = true;
            }
        }

        if is_neg {
            Ok((
                typ,
                Exprs::Unary(Unary {
                    op: UnOperator::Neg,
                    expr: Box::new(Exprs::Unary(Unary {
                        op: refop,
                        expr: Box::new(right),
                    })),
                }),
            ))
        } else {
            Ok((
                typ,
                Exprs::Unary(Unary {
                    op: refop,
                    expr: Box::new(right),
                }),
            ))
        }
    }
}
