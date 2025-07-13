use crate::{
    generator::x86_64::Generate,
    parser::symbols::expressions::relational::{RelationalExpr, RelationalOperator},
};

impl Generate for RelationalExpr {
    fn generate(&self) {
        self.left.generate();

        for relat in &self.rights {
            match relat.op {
                RelationalOperator::Lesser => {
                    relat.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setl al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::Greater => {
                    relat.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setg al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::LesEq => {
                    relat.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setle al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::GrtEq => {
                    relat.right.generate();

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setge al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
            }
        }
    }
}
