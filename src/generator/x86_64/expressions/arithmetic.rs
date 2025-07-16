use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::arithmetic::{ArithmExpr, ArithmOperator},
};

impl LocalGenerate for ArithmExpr {
    fn generate(&self, vars: &mut crate::generator::x86_64::globals::Vars) {
        self.left.generate(vars);

        for arithm in &self.rights {
            match arithm.op {
                ArithmOperator::Add => {
                    arithm.right.generate(vars);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Sub => {
                    arithm.right.generate(vars);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("sub rax, rdi");
                    println!("push rax");
                }
            }
        }
    }
}
