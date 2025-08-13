use crate::{
    generator::x86_64::globals::LocalGenerate,
    validator::{
        expressions::arithmetic::{ArithmExpr, ArithmOperator},
        PrimitiveType, Type,
    },
};

impl LocalGenerate for ArithmExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.left.generate(env);

        for arithm in &self.rights {
            match arithm.op {
                ArithmOperator::Iadd => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Isub => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("sub rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Padd => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!(
                        "imul rdi, {}",
                        Type::PtrTo(Box::new(Type::Primitive(PrimitiveType::Int))).size()
                    );
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Psub => {
                    arithm.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!(
                        "imul rdi, {}",
                        Type::PtrTo(Box::new(Type::Primitive(PrimitiveType::Int))).size()
                    );
                    println!("sub rax, rdi");
                    println!("push rax");
                }
                _ => {
                    panic!("TODO: float calc")
                }
            }
        }
    }
}
