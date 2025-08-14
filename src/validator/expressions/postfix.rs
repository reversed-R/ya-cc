use crate::{
    parser::symbols::expressions::{self, arithmetic, assignment, equality, multiplication, postfix, primary, relational, unary},
    validator::{
        expressions::{
             Exprs, Primary ,
        },
        Env, ExprTypeValidate, Type, TypeError,
    },
};


impl ExprTypeValidate for postfix::PostfixExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), TypeError> {
        match self {
            Self::Primary(prim) => {
                let (mut typ, prim) = prim.validate(env)?;
                
                if let Type::Array(atyp, _) = typ {
                    typ = Type::PtrTo(atyp);
                }

                Ok((typ, Exprs::Primary(Primary::Expr(Box::new(prim)))))
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
                
                println!("# typ: {typ:?}");

                Ok((typ, Exprs::Primary(Primary::Expr(Box::new(u)))))
            }
        }
    }
}
