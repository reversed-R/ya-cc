use crate::{
    generator::x86_64::{expressions::unary, globals::LocalGenerate},
    validator::expressions::{BinOperator, Binary, Exprs},
};

pub fn generate(bin: &Binary, env: &mut crate::generator::x86_64::globals::Env) {
    match bin.op {
        BinOperator::Iadd => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("add rax, rdi");
        }
        BinOperator::Isub => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("sub rax, rdi");
        }
        BinOperator::Padd => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("add rax, rdi");
        }
        BinOperator::Psub => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("sub rax, rdi");
        }
        BinOperator::Imul => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("imul rax, rdi");
        }
        BinOperator::Idiv => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cqo");
            println!("idiv rdi");
        }
        BinOperator::Mod => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cqo");
            println!("idiv rdi");
            println!("mov rax, rdx");
        }
        BinOperator::Greater => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("setg al");
            println!("movzb rax, al");
        }
        BinOperator::Lesser => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("setl al");
            println!("movzb rax, al");
        }
        BinOperator::GrtEq => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("setge al");
            println!("movzb rax, al");
        }
        BinOperator::LesEq => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("setle al");
            println!("movzb rax, al");
        }
        BinOperator::Equal => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("sete al");
            println!("movzb rax, al");
        }
        BinOperator::NotEq => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");

            println!("cmp rax, rdi");
            println!("setne al");
            println!("movzb rax, al");
        }
        BinOperator::IAssign => {
            if let Exprs::Unary(left) = &*bin.left {
                // WARN: is it true?

                unary::generate_as_left(left, env);
                println!("push rax");

                bin.right.generate(env);

                println!("pop rdi");

                // WARN: if int size become 4 bytes, fix it
                println!("mov [rdi], rax");
            } else {
                panic!("Invalid Left Value");
            }
        }
        BinOperator::PAssign => {
            if let Exprs::Unary(left) = &*bin.left {
                // WARN: is it true?

                unary::generate_as_left(left, env);
                println!("push rax");

                bin.right.generate(env);

                println!("pop rdi");

                println!("mov [rdi], rax");
            } else {
                panic!("Invalid Left Value");
            }
        }
        BinOperator::CAssign => {
            if let Exprs::Unary(left) = &*bin.left {
                // WARN: is it true?

                unary::generate_as_left(left, env);
                println!("push rax");

                bin.right.generate(env);

                println!("pop rdi");

                println!("mov [rdi], al");
            } else {
                panic!("Invalid Left Value");
            }
        }
    }
}

pub fn generate_as_left(bin: &Binary, env: &mut crate::generator::x86_64::globals::Env) -> usize {
    match bin.op {
        BinOperator::Padd => {
            bin.right.generate(env);
            println!("push rax");

            bin.left.generate(env);

            println!("pop rdi");
            println!("add rax, rdi");

            1
        }
        BinOperator::Psub => {
            bin.left.generate(env);
            println!("push rax");

            bin.right.generate(env);

            println!("pop rdi");
            println!("sub rax, rdi");

            1
        }
        _ => {
            panic!("Invalid Left Value");
        }
    }
}
