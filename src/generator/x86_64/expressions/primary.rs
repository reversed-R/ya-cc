use crate::{
    generator::x86_64::{globals::LocalGenerate, ARG_REGS},
    validator::{
        expressions::{Literal, Primary},
        Type, VarAddr,
    },
};

pub fn generate(prim: &Primary, env: &mut crate::generator::x86_64::globals::Env) {
    match prim {
        Primary::Literal(lit) => match lit {
            Literal::Int(i) => {
                println!("push {i}");
            }
            Literal::Char(c) => {
                println!("push {c}");
            }
            Literal::String(s) => {
                println!("lea rax, .L{s}[rip]");
                println!("push rax");
            }
            _ => {
                // TODO:
                panic!("TODO");
            }
        },
        Primary::Variable(var) => match &var.addr {
            VarAddr::Local(offset) => {
                if let Type::Array(_, _) = &var.typ {
                    println!("mov rax, rbp");
                    println!("sub rax, {offset}");
                    println!("push rax");
                } else {
                    println!("push [rbp - {offset}]");
                }
            }
            VarAddr::Global(label) => {
                println!("push QWORD PTR {label}[rip]");
            }
        },
        Primary::FnCall(f) => {
            let id = env.increment_label();

            for arg in f.args.iter().rev() {
                arg.generate(env);

                // NOTE: push calculated value to stack,
                // because if pop to argument register as soon as calculate,
                // calculation of remaining argument may break argument registers
            }

            for (i, _) in f.args.iter().enumerate() {
                if let Some(reg) = ARG_REGS.get(i) {
                    println!("pop {reg}");
                } else {
                    panic!("Too Many Args for Function Call");
                }
            }

            println!("mov rax, rsp");
            println!("and rax, 0xf");
            println!("cmp rax, 0");
            println!("je .L{}$fncall{id}$aligned", env.fname);
            println!("sub rsp, 8");
            println!("mov al, 0");
            println!("call {}", f.name);
            println!("add rsp, 8");
            println!("jmp .L{}$fncall{id}$end", env.fname);
            println!(".L{}$fncall{id}$aligned:", env.fname);
            println!("mov al, 0");
            println!("call {}", f.name);
            println!(".L{}$fncall{id}$end:", env.fname);
            println!("push rax");
        }
        Primary::Expr(expr) => {
            expr.generate(env);
        }
    }
}
