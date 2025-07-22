use crate::{
    generator::x86_64::{globals::LocalGenerate, ARG_REGS},
    parser::symbols::expressions::primary::{Literal, Primary},
};

impl LocalGenerate for Primary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self {
            Self::Literal(lit) => match lit {
                Literal::Int(i) => {
                    println!("push {i}");
                }
                _ => {

                    // TODO:
                }
            },
            Self::FnCall(f) => {
                for (i, arg) in f.args.iter().enumerate() {
                    arg.generate(env);

                    if let Some(reg) = ARG_REGS.get(i) {
                        println!("pop {reg}");
                    } else {
                        panic!("Too Many Args for Function Call");
                    }
                }

                println!("call {}", f.name);
                println!("push rax");
            }
            _ => {
                // TODO:
            }
        }
    }
}
