use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::{
        expressions::{
            assignment::AssignExpr,
            postfix::PostfixExpr,
            primary::Primary,
            unary::{RefUnaryOperator, UnaryOperator},
        },
        VarAddr,
    },
};

impl LocalGenerate for AssignExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.src.generate(env);

        for ass in &self.dsts {
            match ass.dst.op {
                UnaryOperator::None => match ass.dst.refop {
                    RefUnaryOperator::Ref => {
                        panic!("Invalid Left Value");
                    }
                    RefUnaryOperator::Deref(count) => {
                        if count == 0 {
                            match &ass.dst.right {
                                PostfixExpr::Primary(prim) => match prim {
                                    Primary::Variable(var) => match var.addr {
                                        VarAddr::Local(offset) => {
                                            println!("pop rax");
                                            println!("mov [rbp - {offset}], rax");
                                            println!("push rax");
                                        }
                                    },
                                    _ => {
                                        panic!("Invalid Left Value");
                                    }
                                },
                                PostfixExpr::Unary(_) => {
                                    panic!("Invalid Left Value");
                                }
                            }
                        } else {
                            ass.dst.generate(env);

                            println!("pop rdi");
                            println!("pop rax");

                            println!("mov [rdi], rax");
                            println!("push rax");
                        }
                    }
                },
                UnaryOperator::Neg => {
                    panic!("Invalid Left Value");
                }
            }
        }
    }
}
