use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::{
        expressions::{
            postfix::PostfixExpr,
            primary::Primary,
            unary::{RefUnaryOperator, Unary, UnaryOperator},
        },
        VarAddr,
    },
};

impl LocalGenerate for Unary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self.op {
            UnaryOperator::None => {
                generate_ref_unary(&self.refop, &self.right, env);
            }
            UnaryOperator::Neg => {
                generate_ref_unary(&self.refop, &self.right, env);

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}

fn generate_ref_unary(
    refop: &RefUnaryOperator,
    postfix: &PostfixExpr,
    env: &mut crate::generator::x86_64::globals::Env,
) {
    match refop {
        RefUnaryOperator::Ref => {
            if let PostfixExpr::Primary(Primary::Variable(var)) = postfix {
                match &var.addr {
                    VarAddr::Local(offset) => {
                        println!("mov rax, rbp");
                        println!("sub rax, {offset}");
                        println!("push rax");
                    }
                    VarAddr::Global(label) => {
                        println!("lea rax, {label}[rip]");
                        println!("push rax");
                    }
                }
            } else {
                panic!("Expected Identifier");
            }
        }
        RefUnaryOperator::Deref(count) => {
            postfix.generate(env);

            println!("pop rax");

            for _ in 0..*count {
                println!("mov rax, [rax]");
            }
            println!("push rax");
        }
    }
}

impl Unary {
    pub fn generate_as_left(&self, env: &mut crate::generator::x86_64::globals::Env) {
        // 左辺値として生成
        println!("# unary as left ----");
        match self.op {
            UnaryOperator::None => match self.refop {
                RefUnaryOperator::Ref => {
                    panic!("Invalid Left Value");
                }
                RefUnaryOperator::Deref(count) => {
                    match &self.right {
                        PostfixExpr::Primary(prim) => match prim {
                            Primary::Variable(var) => match &var.addr {
                                VarAddr::Local(offset) => {
                                    println!("mov rax, rbp");
                                    println!("sub rax, {offset}");
                                    println!("push rax");
                                }
                                VarAddr::Global(label) => {
                                    println!("lea rax, {label}[rip]");
                                    println!("push rax");
                                }
                            },
                            Primary::Expr(expr) => {
                                expr.generate(env);
                            }
                            Primary::FnCall(f) => {
                                // TODO:
                                panic!("TODO");
                            }
                            _ => {
                                panic!("Invalid Left Value");
                            }
                        },
                        PostfixExpr::Unary(unary) => {
                            unary.generate_as_left(env);
                        }
                    }

                    println!("pop rax");

                    for _ in 1..count {
                        println!("mov rax, [rax]");
                    }
                    println!("push rax");
                }
            },
            UnaryOperator::Neg => {
                panic!("Invalid Left Value");
            }
        }
        println!("# ---- unary as left");
    }
}
