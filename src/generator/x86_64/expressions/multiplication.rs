use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::multiplication::{MulExpr, MulOperator},
};

impl LocalGenerate for MulExpr {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        self.left.generate(vars);

        for mul in &self.rights {
            match mul.op {
                MulOperator::Mul => {
                    mul.right.generate(vars);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("imul rax, rdi");
                    println!("push rax");
                }
                MulOperator::Div => {
                    mul.right.generate(vars);

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
