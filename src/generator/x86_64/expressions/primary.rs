use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::primary::{Literal, Primary},
};

impl LocalGenerate for Primary {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
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
                    arg.generate(vars);
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
