use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::arithmetic::{ArithmExpr, ArithmOperator},
};

impl Generate for ArithmExpr {
    fn generate(&self) {
        self.left.generate();

        for arithm in &self.rights {
            match arithm.op {
                ArithmOperator::Add => {
                    arithm.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("add rax, rdi");
                    println!("push rax");
                }
                ArithmOperator::Sub => {
                    arithm.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("sub rax, rdi");
                    println!("push rax");
                }
            }
        }
    }
}
