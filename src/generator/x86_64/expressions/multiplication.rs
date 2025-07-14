use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::multiplication::{MulExpr, MulOperator},
};

impl LocalGenerate for MulExpr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        self.left.generate(locals);

        for mul in &self.rights {
            match mul.op {
                MulOperator::Mul => {
                    mul.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("imul rax, rdi");
                    println!("push rax");
                }
                MulOperator::Div => {
                    mul.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cqo");
                    println!("idiv rdi");
                    println!("push rax");
                }
            }
        }
    }
}
