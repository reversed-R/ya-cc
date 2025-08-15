use crate::{
    generator::x86_64::{expressions::unary, globals::LocalGenerate},
    validator::expressions::{BinOperator, Binary, Exprs},
};

pub fn generate(bin: &Binary, env: &mut crate::generator::x86_64::globals::Env) {
    match bin.op {
        BinOperator::Iadd => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("add rax, rdi");
            println!("push rax");
        }
        BinOperator::Isub => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("sub rax, rdi");
            println!("push rax");
        }
        BinOperator::Padd => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("add rax, rdi");
            println!("push rax");
        }
        BinOperator::Psub => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("sub rax, rdi");
            println!("push rax");
        }
        BinOperator::Imul => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("imul rax, rdi");
            println!("push rax");
        }
        BinOperator::Idiv => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cqo");
            println!("idiv rdi");
            println!("push rax");
        }
        BinOperator::Mod => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cqo");
            println!("idiv rdi");
            println!("push rdx");
        }
        BinOperator::Greater => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("setg al");
            println!("movzb rax, al");
            println!("push rax");
        }
        BinOperator::Lesser => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("setl al");
            println!("movzb rax, al");
            println!("push rax");
        }
        BinOperator::GrtEq => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("setge al");
            println!("movzb rax, al");
            println!("push rax");
        }
        BinOperator::LesEq => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("setle al");
            println!("movzb rax, al");
            println!("push rax");
        }
        BinOperator::Equal => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("sete al");
            println!("movzb rax, al");
            println!("push rax");
        }
        BinOperator::NotEq => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("cmp rax, rdi");
            println!("setne al");
            println!("movzb rax, al");
            println!("push rax");
        }

        BinOperator::Assign => {
            bin.right.generate(env);

            if let Exprs::Unary(left) = &*bin.left {
                println!("# calling generate_as_left...");
                unary::generate_as_left(left, env);

                println!("pop rdi");
                println!("pop rax");
                println!("mov [rdi], rax");
                println!("push rax");
            }
        }
    }
}

pub fn generate_as_left(bin: &Binary, env: &mut crate::generator::x86_64::globals::Env) -> usize {
    match bin.op {
        BinOperator::Padd => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("add rax, rdi");
            println!("push rax");

            1
        }
        BinOperator::Psub => {
            bin.left.generate(env);
            bin.right.generate(env);

            println!("pop rdi");
            println!("pop rax");
            println!("sub rax, rdi");
            println!("push rax");

            1
        }
        _ => {
            panic!("Invalid Left Value");
        }
    }
}
