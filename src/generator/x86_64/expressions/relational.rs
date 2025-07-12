use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::relational::{RelationalExpr, RelationalOperator},
};

impl Generate for RelationalExpr {
    fn generate(&self) {
        let mut i = 0;
        for relat in &self.nodes {
            if i == 0 {
                relat.right.generate();
            } else {
                match relat.op {
                    RelationalOperator::Lesser => {
                        relat.right.generate();

                        println!("pop rdi");
                        println!("pop rax");
                        println!("cmp rax, rdi");
                        println!("setg rax");
                        println!("pop rax");
                    }
                    _ => {
                        // TODO:
                    }
                }
            }

            i += 1;
        }
    }
}
