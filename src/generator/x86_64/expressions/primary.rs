use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::primary::{Literal, Primary},
};

impl Generate for Primary {
    fn generate(&self) {
        match self {
            Self::Literal(lit) => match lit {
                Literal::Int(i) => {
                    println!("push {i}");
                }
                _ => {

                    // TODO:
                }
            },
            _ => {
                // TODO:
            }
        }
    }
}
