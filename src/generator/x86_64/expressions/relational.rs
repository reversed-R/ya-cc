use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::expressions::relational::{RelationalExpr, RelationalOperator},
};

impl LocalGenerate for RelationalExpr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        self.left.generate(locals);

        for relat in &self.rights {
            match relat.op {
                RelationalOperator::Lesser => {
                    relat.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setl al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::Greater => {
                    relat.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setg al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::LesEq => {
                    relat.right.generate(locals);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setle al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::GrtEq => {
                    relat.right.generate(locals);

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
