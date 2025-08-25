use std::ops::Deref;

use crate::{
    generator::x86_64::{
        expressions::{binary, unary},
        globals::LocalGenerate,
        ARG_REGS,
    },
    validator::{
        expressions::{Exprs, Literal, Primary},
        types::Type,
        VarAddr,
    },
};

pub fn generate(prim: &Primary, env: &mut crate::generator::x86_64::globals::Env) {
    match prim {
        Primary::Literal(lit) => match lit {
            Literal::Int(i) => {
                println!("mov rax, {i}");
            }
            Literal::Char(c) => {
                println!("mov rax, {c}");
            }
            Literal::String(s) => {
                println!("lea rax, .L{s}[rip]");
            }
        },
        Primary::Variable(var) => match &var.addr {
            VarAddr::Local(offset) => {
                if let Type::Array(_, _) = &var.typ {
                    println!("mov rax, rbp");
                    println!("sub rax, {offset}");
                } else {
                    println!("mov rax, [rbp - {offset}]");
                }
            }
            VarAddr::Global(label, _) => {
                println!("mov rax, QWORD PTR {label}[rip]");
            }
        },
        Primary::FnCall(f) => {
            let id = env.increment_label();

            for arg in f.args.iter().rev() {
                arg.generate(env);
                println!("push rax");

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
        }
        Primary::Expr(expr) => {
            expr.generate(env);
        }
    }
}

pub fn generate_as_left(prim: &Primary, env: &mut crate::generator::x86_64::globals::Env) -> usize {
    // 行ったderef演算子の数
    match prim {
        Primary::Variable(var) => match &var.addr {
            VarAddr::Local(offset) => {
                println!("mov rax, rbp");
                println!("sub rax, {offset}");
                0
            }
            VarAddr::Global(label, _) => {
                println!("lea rax, {label}[rip]");
                0
            }
        },
        Primary::Expr(expr) => match &expr.deref() {
            Exprs::Primary(prim) => generate_as_left(prim, env),
            Exprs::Unary(unary) => unary::generate_as_left(unary, env),
            Exprs::Binary(bin) => binary::generate_as_left(bin, env),
        },
        Primary::FnCall(_) => {
            todo!();
        }
        _ => {
            panic!("Invalid Left Value");
        }
    }
}
