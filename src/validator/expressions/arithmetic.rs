use crate::{
    parser::symbols::expressions::arithmetic,
    validator::{
        expressions::multiplication::MulExpr, Env, ExprTypeValidate, PrimitiveType, Type,
        TypeComarison, TypeError,
    },
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
// e.g. iadd, isub, fadd, padd
#[derive(Debug)]
pub enum ArithmOperator {
    Iadd,
    Isub,
    Fadd,
    Fsub,
    Padd,
    Psub,
    Cadd,
    Csub,
}

impl ArithmOperator {
    fn new(op: &arithmetic::ArithmOperator, typ: &Type) -> Self {
        match op {
            arithmetic::ArithmOperator::Add => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Self::Iadd,
                    // PrimitiveType::Float => Self::Fadd,
                    PrimitiveType::Char => Self::Cadd,
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Self::Padd,
                Type::Array(_, _) => Self::Padd, // WARN: is it true?
            },
            arithmetic::ArithmOperator::Sub => match typ {
                Type::Primitive(prim) => match prim {
                    PrimitiveType::Int => Self::Isub,
                    // PrimitiveType::Float => Self::Fsub,
                    PrimitiveType::Char => Self::Csub,
                    PrimitiveType::Void => {
                        panic!("Cannot add or sub void");
                    }
                },
                Type::PtrTo(_) => Self::Psub,
                Type::Array(_, _) => Self::Psub, // WARN: is it true?
            },
        }
    }
}

impl ExprTypeValidate for crate::parser::symbols::expressions::arithmetic::ArithmExpr {
    type ValidatedType = (Type, ArithmExpr);

    fn validate(&self, env: &mut Env) -> Result<Self::ValidatedType, TypeError> {
        let (mut typ, left) = self.left.validate(env)?;
        let mut rights = vec![];

        for r in &self.rights {
            let (right_typ, right) = r.right.validate(env)?;

            match typ.compare(&right_typ) {
                TypeComarison::Equal => {
                    rights.push(ArithmExprNode {
                        op: ArithmOperator::new(&r.op, &typ),
                        right,
                    });
                }
                TypeComarison::ImplicitlyConvertableTo => {
                    typ = right_typ;

                    rights.push(ArithmExprNode {
                        op: ArithmOperator::new(&r.op, &typ),
                        right,
                    });
                }
                TypeComarison::ImplicitlyConvertableFrom => {
                    rights.push(ArithmExprNode {
                        op: ArithmOperator::new(&r.op, &typ),
                        right,
                    });
                }
                TypeComarison::ImplicitlyUnconvertable => {
                    return Err(TypeError::Mismatch(typ, right_typ));
                }
            }
        }

        Ok((typ, ArithmExpr { left, rights }))
    }
}
