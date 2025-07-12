use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::arithmetic::{ArithmExpr, ArithmOperator},
};

impl Generate for ArithmExpr {
    fn generate(&self) {
        let mut i = 0;
        for arithm in &self.nodes {
            if i == 0 {
                arithm.right.generate();
            } else {
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

            i += 1;
        }
    }
}
