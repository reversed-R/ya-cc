use crate::{
    parser::symbols::expressions,
    parser::symbols::expressions::{
        arithmetic, assignment, multiplication, postfix, primary, unary, relational, equality, 
    },
    validator::{
        expressions::{
            primary::Primary,
            unary::Unary ,
        },
        Env, ExprTypeValidate, Type, TypeError,
    },
};

#[derive(Debug)]
pub enum PostfixExpr {
    Primary(Primary),
    Unary(Box<Unary>), // NOTE: 
                       // array[index] converted *(array + index)
}

impl ExprTypeValidate for postfix::PostfixExpr {
    type ValidatedType = (Type, PostfixExpr);

    fn validate(&self, env: &Env) -> Result<Self::ValidatedType, TypeError> {
        match self {
            Self::Primary(prim) => {
                let (mut typ, prim) = prim.validate(env)?;
                
                if let Type::Array(atyp, _) = typ {
                    typ = Type::PtrTo(atyp);
                }

                Ok((typ, PostfixExpr::Primary(prim)))
            }
            Self::Index(postfix, expr) => {
                let u = unary::Unary {
                    op: unary::UnaryOperator::Plus,
                    right: unary::RefUnary {
                        ops: vec![unary::RefUnaryOperator::Deref],
                        right: postfix::PostfixExpr::Primary(
                            primary::Primary::Expr(
                                Box::new(
                                    expressions::Expr(
                                        assignment::AssignExpr{
                                            lefts: vec![],
                                            right: equality::EqualityExpr{
                                                left: relational::RelationalExpr{
                                                    left: arithmetic::ArithmExpr {
                                                        left: multiplication::MulExpr {
                                                            left: unary::Unary {
                                                                op: unary::UnaryOperator::Plus,
                                                                right: unary::RefUnary {
                                                                    ops: vec![],
                                                                    right: *postfix.clone(),
                                                                },
                                                            },
                                                            rights: vec![]
                                                        },
                                                        rights: vec![
                                                            arithmetic::ArithmExprNode{
                                                                op: arithmetic::ArithmOperator::Add,
                                                                right: multiplication::MulExpr {
                                                                    left: unary::Unary {
                                                                        op: unary::UnaryOperator::Plus,
                                                                        right: unary::RefUnary {
                                                                            ops: vec![],
                                                                            right: postfix::PostfixExpr::Primary(
                                                                                primary::Primary::Expr(
                                                                                    Box::new(
                                                                                        expressions::Expr(
                                                                                            assignment::AssignExpr{
                                                                                                lefts: vec![],
                                                                                                right: equality::EqualityExpr{
                                                                                                    left: relational::RelationalExpr{
                                                                                                        left: arithmetic::ArithmExpr {
                                                                                                            left: multiplication::MulExpr{
                                                                                                                left: unary::Unary { 
                                                                                                                    op: unary::UnaryOperator::Plus, 
                                                                                                                    right: unary::RefUnary{
                                                                                                                        ops: vec![],
                                                                                                                        right: postfix::PostfixExpr::Primary(
                                                                                                                            primary::Primary::Expr(expr.clone())
                                                                                                                        )
                                                                                                                    }
                                                                                                                },
                                                                                                                rights:vec![]
                                                                                                            },
                                                                                                            rights: vec![],
                                                                                                        },
                                                                                                        rights:vec![]
                                                                                                    },
                                                                                                    rights: vec![]
                                                                                                }
                                                                                            }
                                                                                        )
                                                                                    )
                                                                                )
                                                                            )
                                                                        },
                                                                    },
                                                                    rights: vec![]
                                                                }
                                                                    
                                                            }
                                                        ]
                                                    },
                                                    rights:vec![]
                                                },
                                                rights: vec![]
                                            }
                                        }
                                    )
                                )
                            )
                        )
                    },
                };

                println!("#...");

                let (typ, u) = u.validate(env)?;
                
                println!("# typ: {:?}", typ);

                Ok((typ, PostfixExpr::Unary(Box::new(u))))
            }
        }
    }
}
