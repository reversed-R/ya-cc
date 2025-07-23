use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::{
        primary::Primary,
        unary::{RefUnary, RefUnaryOperator, Unary, UnaryOperator},
    },
};

impl LocalGenerate for Unary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self.op {
            UnaryOperator::Plus => {
                self.right.generate(env);
            }
            UnaryOperator::Minus => {
                self.right.generate(env);

                println!("pop rax");
                println!("neg rax");
                println!("push rax");
            }
        }
    }
}

impl LocalGenerate for RefUnary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        // TODO:

        if self.ops.is_empty() {
            self.right.generate(env);
        } else {
            for (i, op) in self.ops.iter().rev().enumerate() {
                if i == 0 {
                    match op {
                        RefUnaryOperator::Ref => {
                            if let Primary::Identifier(id) = &self.right {
                                println!("mov rax, rbp");
                                println!(
                                    "sub rax, {}",
                                    env.offset(id).expect("Variable Not Found")
                                );
                                println!("push rax");
                            } else {
                                panic!("Expected Identifier");
                            }
                        }
                        RefUnaryOperator::Deref => {
                            self.right.generate(env);

                            println!("pop rax");
                            println!("mov rax, [rax]");
                            println!("push rax");
                        }
                    }
                } else {
                    match op {
                        RefUnaryOperator::Ref => {
                            panic!("Expected Identifier");
                        }
                        RefUnaryOperator::Deref => {
                            println!("pop rax");
                            println!("mov rax, [rax]");
                            println!("push rax");
                        }
                    }
                }
            }
        }
    }
}
