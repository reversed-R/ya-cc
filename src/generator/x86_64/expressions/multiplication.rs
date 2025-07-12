use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::multiplication::{MulExpr, MulOperator},
};

impl Generate for MulExpr {
    fn generate(&self) {
        let mut i = 0;
        for mul in &self.nodes {
            if i == 0 {
                mul.right.generate();
            } else {
                match mul.op {
                    MulOperator::Mul => {
                        mul.right.generate();

                        println!("pop rdi");
                        println!("pop rax");
                        println!("mul rdi");
                        println!("push rax");
                    }
                    MulOperator::Div => {
                        mul.right.generate();
                        // TODO:
                    }
                }
            }

            i += 1;
        }
    }
}
