use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::{
        assignment::{AssignExpr, AssignOperator},
        primary::Primary,
        unary::{RefUnaryOperator, UnaryOperator},
    },
};

impl LocalGenerate for AssignExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.right.generate(env);

        for ass in self.lefts.iter().rev() {
            match ass.left.op {
                UnaryOperator::Plus => {
                    if ass.left.right.ops.is_empty() {
                        match &ass.left.right.right {
                            Primary::Identifier(id) => {
                                if let Some(offset) = env.offset(id) {
                                    println!("pop rdi");
                                    ass.op.generate(&format!("[rbp - {offset}]"), "rdi");
                                    println!("push rdi");
                                } else {
                                    panic!("Local Variable Not Found");
                                }
                            }
                            _ => {
                                panic!("Invalid Left Value");
                            }
                        }
                    } else {
                        for (i, op) in ass.left.right.ops[0..ass.left.right.ops.len() - 1]
                            .iter()
                            .enumerate()
                        {
                            match op {
                                RefUnaryOperator::Ref => {
                                    panic!("Invalid Left Value");
                                }
                                RefUnaryOperator::Deref => {
                                    if i == 0 {
                                        match &ass.left.right.right {
                                            Primary::Identifier(id) => {
                                                if let Some(offset) = env.offset(id) {
                                                    println!("mov rax, [rbp - {offset}]");
                                                } else {
                                                    panic!("Local Variable Not Found");
                                                }
                                            }
                                            _ => {
                                                panic!("Invalid Left Value");
                                            }
                                        }
                                    }
                                    println!("mov rax, [rax]");
                                }
                            }
                        }

                        if let Some(RefUnaryOperator::Ref) = ass.left.right.ops.last() {
                            panic!("Invalid Left Value");
                        }

                        println!("pop rdi");
                        println!("mov [rax], rdi");
                        println!("push rdi");
                    }
                }
                UnaryOperator::Minus => {
                    panic!("Invalid Left Value");
                }
            }
        }
    }
}

impl AssignOperator {
    fn generate(&self, dst: &str, src: &str) {
        match self {
            Self::Assign => {
                println!("mov {dst}, {src}");
            }
        }
    }
}
