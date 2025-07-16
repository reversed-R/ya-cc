use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::primary::{Literal, Primary},
};

impl LocalGenerate for Primary {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
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
                for arg in &f.args {
                    arg.generate(locals);
                    // calculated arg value will be pushed
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
