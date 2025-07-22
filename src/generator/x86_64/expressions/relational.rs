use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::relational::{RelationalExpr, RelationalOperator},
};

impl LocalGenerate for RelationalExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.left.generate(env);

        for relat in &self.rights {
            match relat.op {
                RelationalOperator::Lesser => {
                    relat.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setl al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::Greater => {
                    relat.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setg al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::LesEq => {
                    relat.right.generate(env);

                    println!("pop rdi");
                    println!("pop rax");
                    println!("cmp rax, rdi");
                    println!("setle al");
                    println!("movzb rax, al");
                    println!("push rax");
                }
                RelationalOperator::GrtEq => {
                    relat.right.generate(env);

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
