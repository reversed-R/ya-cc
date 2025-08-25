use crate::{
    parser::symbols::expressions::{
        self, arithmetic, assignment, equality, multiplication, postfix, primary, relational, unary,
    },
    validator::{
        expressions::{BinOperator, Binary, Exprs, Literal, Primary, UnOperator, Unary},
        DefinedType, DefinedTypeContent, Env, ExprTypeValidate, Type, ValidateError,
    },
};

impl ExprTypeValidate for postfix::PostfixExpr {
    fn validate(&self, env: &mut Env) -> Result<(Type, Exprs), ValidateError> {
        match self {
            Self::Primary(prim) => {
                let (mut typ, prim) = prim.validate(env)?;

                if let Type::Array(atyp, _) = typ {
                    typ = Type::PtrTo(atyp);
                }

                Ok((typ, Exprs::Primary(prim)))
            }
            Self::DotAccess(left, member) => {
                match left {
                    primary::Primary::Identifier(id) => {
                        let var = env
                            .get_var(id)
                            .ok_or(ValidateError::VariableNotFound(id.clone()))?;

                        match &var.typ {
                            Type::Defined(defed_typ) => {
                                match defed_typ {
                                    DefinedType::Struct(struct_name) => {
                                        if let Some(defed_typ) = env.global.types.get(struct_name) {
                                            match defed_typ {
                                                DefinedTypeContent::Struct(s) => {
                                                    if let Some((mem_typ, mem_offset)) =
                                                        s.members.get(member)
                                                    {
                                                        let op = if mem_typ.size(env) == 1 {
                                                            UnOperator::CDeref(1)
                                                        } else {
                                                            UnOperator::IDeref(1)
                                                        };
                                                        // WARN: if install size 1, 2, 4, and 8, i fix it

                                                        Ok((
                                                    mem_typ.clone(),
                                                    Exprs::Unary(Unary {
                                                        op,
                                                        expr: Box::new(Exprs::Binary(Binary {
                                                            op: BinOperator::Padd,
                                                            left: Box::new(Exprs::Unary(Unary {
                                                                op: UnOperator::Ref,
                                                                expr: Box::new(Exprs::Primary(
                                                                    Primary::Variable(var.clone()),
                                                                )),
                                                            })),
                                                            right: Box::new(Exprs::Primary(
                                                                Primary::Literal(Literal::Int(
                                                                    *mem_offset as i64,
                                                                )),
                                                            )),
                                                        })),
                                                    }),
                                                ))
                                                    } else {
                                                        Err(ValidateError::StructMemberNotFound(
                                                            struct_name.clone(),
                                                            member.clone(),
                                                        ))
                                                    }
                                                }
                                            }
                                        } else {
                                            Err(ValidateError::TypeNotFound(struct_name.clone()))
                                        }
                                    }
                                }
                            }
                            _ => Err(ValidateError::TypeAndOperatorNotSupported(
                                var.typ.to_string(),
                                ".".to_string(),
                            )),
                        }
                    }
                    primary::Primary::Expr(expr) => {
                        let (typ, expr) = expr.validate(env)?;

                        if let Type::Defined(defed_typ) = typ {
                            match defed_typ {
                                DefinedType::Struct(struct_name) => {
                                    if let Some(defed_typ_cont) = env.global.types.get(&struct_name)
                                    {
                                        match defed_typ_cont {
                                            DefinedTypeContent::Struct(s) => {
                                                if let Some((mem_typ, mem_offset)) =
                                                    s.members.get(member)
                                                {
                                                    let op = if mem_typ.size(env) == 1 {
                                                        UnOperator::CDeref(1)
                                                    } else {
                                                        UnOperator::IDeref(1)
                                                    };
                                                    // WARN: if install size 1, 2, 4, and 8, i fix it

                                                    Ok((
                                                        mem_typ.clone(),
                                                        Exprs::Unary(Unary {
                                                            op,
                                                            expr: Box::new(Exprs::Binary(Binary {
                                                                op: BinOperator::Padd,
                                                                left: Box::new(Exprs::Unary(
                                                                    Unary {
                                                                        op: UnOperator::Ref,
                                                                        expr: Box::new(expr),
                                                                    },
                                                                )),
                                                                right: Box::new(Exprs::Primary(
                                                                    Primary::Literal(Literal::Int(
                                                                        *mem_offset as i64,
                                                                    )),
                                                                )),
                                                            })),
                                                        }),
                                                    ))
                                                } else {
                                                    Err(ValidateError::StructMemberNotFound(
                                                        struct_name,
                                                        member.clone(),
                                                    ))
                                                }
                                            }
                                        }
                                    } else {
                                        Err(ValidateError::TypeNotFound(struct_name))
                                    }
                                }
                            }
                        } else {
                            Err(ValidateError::TypeAndOperatorNotSupported(
                                typ.to_string(),
                                ".".to_string(),
                            ))
                        }
                    }
                    primary::Primary::FnCall(f) => {
                        let (typ, prim) = primary::Primary::FnCall(f.clone()).validate(env)?;

                        if let Type::Defined(defed_typ) = typ {
                            match defed_typ {
                                DefinedType::Struct(struct_name) => {
                                    if let Some(defed_typ_cont) = env.global.types.get(&struct_name)
                                    {
                                        match defed_typ_cont {
                                            DefinedTypeContent::Struct(s) => {
                                                if let Some((mem_typ, mem_offset)) =
                                                    s.members.get(member)
                                                {
                                                    let op = if mem_typ.size(env) == 1 {
                                                        UnOperator::CDeref(1)
                                                    } else {
                                                        UnOperator::IDeref(1)
                                                    };
                                                    // WARN: if install size 1, 2, 4, and 8, i fix it

                                                    Ok((
                                                        mem_typ.clone(),
                                                        Exprs::Unary(Unary {
                                                            op,
                                                            expr: Box::new(Exprs::Binary(Binary {
                                                                op: BinOperator::Padd,
                                                                left: Box::new(Exprs::Unary(
                                                                    Unary {
                                                                        op: UnOperator::Ref,
                                                                        expr: Box::new(
                                                                            Exprs::Primary(prim),
                                                                        ),
                                                                    },
                                                                )),
                                                                right: Box::new(Exprs::Primary(
                                                                    Primary::Literal(Literal::Int(
                                                                        *mem_offset as i64,
                                                                    )),
                                                                )),
                                                            })),
                                                        }),
                                                    ))
                                                } else {
                                                    Err(ValidateError::StructMemberNotFound(
                                                        struct_name,
                                                        member.clone(),
                                                    ))
                                                }
                                            }
                                        }
                                    } else {
                                        Err(ValidateError::TypeNotFound(struct_name))
                                    }
                                }
                            }
                        } else {
                            Err(ValidateError::TypeAndOperatorNotSupported(
                                typ.to_string(),
                                ".".to_string(),
                            ))
                        }
                    }
                    primary::Primary::Literal(_) => {
                        Err(ValidateError::TypeAndOperatorNotSupported(
                            "literal".to_string(),
                            ".".to_string(),
                        ))
                    }
                }
            }
            Self::ArrowAccess(left, member) => {
                match left {
                    primary::Primary::Identifier(id) => {
                        let var = env
                            .get_var(id)
                            .ok_or(ValidateError::VariableNotFound(id.clone()))?;

                        if let Type::PtrTo(pointed) = &var.typ {
                            match &**pointed {
                                Type::Defined(defed_typ) => {
                                    match defed_typ {
                                        DefinedType::Struct(struct_name) => {
                                            if let Some(defed_typ) =
                                                env.global.types.get(struct_name)
                                            {
                                                match defed_typ {
                                                    DefinedTypeContent::Struct(s) => {
                                                        if let Some((mem_typ, mem_offset)) =
                                                            s.members.get(member)
                                                        {
                                                            let op = if mem_typ.size(env) == 1 {
                                                                UnOperator::CDeref(1)
                                                            } else {
                                                                UnOperator::IDeref(1)
                                                            };
                                                            // WARN: if install size 1, 2, 4, and 8, i fix it

                                                            Ok((mem_typ.clone(),
                                                                Exprs::Unary(Unary {
                                                                    op,
                                                                    expr: Box::new(Exprs::Binary(Binary {
                                                                        op: BinOperator::Padd,
                                                                        left: Box::new(Exprs::Primary(
                                                                                Primary::Variable(var.clone()),
                                                                            )),
                                                                        right: Box::new(Exprs::Primary(
                                                                            Primary::Literal(Literal::Int(
                                                                                *mem_offset as i64,
                                                                            )),
                                                                        )),
                                                                    })),
                                                                }),
                                                            ))
                                                        } else {
                                                            Err(
                                                                ValidateError::StructMemberNotFound(
                                                                    struct_name.clone(),
                                                                    member.clone(),
                                                                ),
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                Err(ValidateError::TypeNotFound(
                                                    struct_name.clone(),
                                                ))
                                            }
                                        }
                                    }
                                }
                                _ => Err(ValidateError::TypeAndOperatorNotSupported(
                                    var.typ.to_string(),
                                    ".".to_string(),
                                )),
                            }
                        } else {
                            Err(ValidateError::TypeAndOperatorNotSupported(
                                var.typ.to_string(),
                                ".".to_string(),
                            ))
                        }
                    }
                    primary::Primary::Expr(expr) => {
                        let (typ, expr) = expr.validate(env)?;

                        if let Type::PtrTo(pointed) = &typ {
                            if let Type::Defined(defed_typ) = &**pointed {
                                match defed_typ {
                                    DefinedType::Struct(struct_name) => {
                                        if let Some(defed_typ_cont) =
                                            env.global.types.get(struct_name)
                                        {
                                            match defed_typ_cont {
                                                DefinedTypeContent::Struct(s) => {
                                                    if let Some((mem_typ, mem_offset)) =
                                                        s.members.get(member)
                                                    {
                                                        let op = if mem_typ.size(env) == 1 {
                                                            UnOperator::CDeref(1)
                                                        } else {
                                                            UnOperator::IDeref(1)
                                                        };
                                                        // WARN: if install size 1, 2, 4, and 8, i fix it

                                                        Ok((
                                                            mem_typ.clone(),
                                                            Exprs::Unary(Unary {
                                                                op,
                                                                expr: Box::new(Exprs::Binary(
                                                                    Binary {
                                                                        op: BinOperator::Padd,
                                                                        left: Box::new(expr),
                                                                        right: Box::new(
                                                                            Exprs::Primary(
                                                                                Primary::Literal(
                                                                                    Literal::Int(
                                                                                        *mem_offset
                                                                                            as i64,
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                )),
                                                            }),
                                                        ))
                                                    } else {
                                                        Err(ValidateError::StructMemberNotFound(
                                                            struct_name.clone(),
                                                            member.clone(),
                                                        ))
                                                    }
                                                }
                                            }
                                        } else {
                                            Err(ValidateError::TypeNotFound(struct_name.clone()))
                                        }
                                    }
                                }
                            } else {
                                Err(ValidateError::TypeAndOperatorNotSupported(
                                    typ.to_string(),
                                    ".".to_string(),
                                ))
                            }
                        } else {
                            Err(ValidateError::TypeAndOperatorNotSupported(
                                typ.to_string(),
                                ".".to_string(),
                            ))
                        }
                    }
                    primary::Primary::FnCall(f) => {
                        let (typ, prim) = primary::Primary::FnCall(f.clone()).validate(env)?;

                        if let Type::PtrTo(pointed) = &typ {
                            if let Type::Defined(defed_typ) = &**pointed {
                                match defed_typ {
                                    DefinedType::Struct(struct_name) => {
                                        if let Some(defed_typ_cont) =
                                            env.global.types.get(struct_name)
                                        {
                                            match defed_typ_cont {
                                                DefinedTypeContent::Struct(s) => {
                                                    if let Some((mem_typ, mem_offset)) =
                                                        s.members.get(member)
                                                    {
                                                        let op = if mem_typ.size(env) == 1 {
                                                            UnOperator::CDeref(1)
                                                        } else {
                                                            UnOperator::IDeref(1)
                                                        };
                                                        // WARN: if install size 1, 2, 4, and 8, i fix it

                                                        Ok((
                                                            mem_typ.clone(),
                                                            Exprs::Unary(Unary {
                                                                op,
                                                                expr: Box::new(Exprs::Binary(
                                                                    Binary {
                                                                        op: BinOperator::Padd,
                                                                        left: Box::new(
                                                                            Exprs::Primary(prim),
                                                                        ),
                                                                        right: Box::new(
                                                                            Exprs::Primary(
                                                                                Primary::Literal(
                                                                                    Literal::Int(
                                                                                        *mem_offset
                                                                                            as i64,
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                )),
                                                            }),
                                                        ))
                                                    } else {
                                                        Err(ValidateError::StructMemberNotFound(
                                                            struct_name.clone(),
                                                            member.clone(),
                                                        ))
                                                    }
                                                }
                                            }
                                        } else {
                                            Err(ValidateError::TypeNotFound(struct_name.clone()))
                                        }
                                    }
                                }
                            } else {
                                Err(ValidateError::TypeAndOperatorNotSupported(
                                    typ.to_string(),
                                    ".".to_string(),
                                ))
                            }
                        } else {
                            Err(ValidateError::TypeAndOperatorNotSupported(
                                typ.to_string(),
                                ".".to_string(),
                            ))
                        }
                    }
                    primary::Primary::Literal(_) => {
                        Err(ValidateError::TypeAndOperatorNotSupported(
                            "literal".to_string(),
                            ".".to_string(),
                        ))
                    }
                }
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
                                                                                                                left: unary::Unary{
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

                let (typ, u) = u.validate(env)?;

                Ok((typ, u))
            }
        }
    }
}
