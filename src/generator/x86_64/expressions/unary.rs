use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::{
        expressions::{
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
    prim: &Primary,
    env: &mut crate::generator::x86_64::globals::Env,
) {
    match refop {
        RefUnaryOperator::Ref => {
            if let Primary::Variable(var) = prim {
                match var.addr {
                    VarAddr::Local(offset) => {
                        println!("mov rax, rbp");
                        println!("sub rax, {}", offset);
                        println!("push rax");
                    }
                }
            } else {
                panic!("Expected Identifier");
            }
        }
        RefUnaryOperator::Deref(count) => {
            prim.generate(env);

            println!("pop rax");
            println!("mov rax, [rax]");
            println!("push rax");

            for _ in 0..*count {
                println!("pop rax");
                println!("mov rax, [rax]");
                println!("push rax");
            }
        }
    }
}
