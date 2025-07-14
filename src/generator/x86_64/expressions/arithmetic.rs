use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::arithmetic::{ArithmExpr, ArithmOperator},
};

impl LocalGenerate for ArithmExpr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        self.left.generate(locals);

        for arithm in &self.rights {
            match arithm.op {
                ArithmOperator::Add => {
                    arithm.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Sub => {
                    arithm.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("sub rax, rdi");
                    println!("push rax");
                }
            }
        }
    }
}
