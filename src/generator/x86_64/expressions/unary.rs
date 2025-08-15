use std::ops::Deref;

use crate::{
    generator::x86_64::{expressions::primary, globals::LocalGenerate},
    validator::{
        expressions::{Exprs, Primary, UnOperator, Unary},
        VarAddr,
    },
};

pub fn generate(unary: &Unary, env: &mut crate::generator::x86_64::globals::Env) {
    match unary.op {
        UnOperator::Neg => {
            unary.expr.generate(env);

            println!("pop rax");
            println!("neg rax");
            println!("push rax");
        }
        UnOperator::Ref => {
            if let Exprs::Primary(Primary::Variable(var)) = &*unary.expr {
                match &var.addr {
                    VarAddr::Local(offset) => {
                        println!("mov rax, rbp");
                        println!("sub rax, {offset}");
                        println!("push rax");
                    }
                    VarAddr::Global(label) => {
                        println!("lea rax, {label}[rip]");
                        println!("push rax");
                    }
                }
            } else if let Exprs::Primary(Primary::Expr(expr)) = &*unary.expr {
                if let Exprs::Primary(Primary::Variable(var)) = expr.deref() {
                    match &var.addr {
                        VarAddr::Local(offset) => {
                            println!("mov rax, rbp");
                            println!("sub rax, {offset}");
                            println!("push rax");
                        }
                        VarAddr::Global(label) => {
                            println!("lea rax, {label}[rip]");
                            println!("push rax");
                        }
                    }
                } else {
                    panic!("Expected Identifier");
                }
            } else {
                panic!("Expected Identifier");
            }
        }
        UnOperator::Deref(count) => {
            unary.expr.generate(env);

            println!("pop rax");

            for _ in 0..count {
                println!("mov rax, [rax]");
            }
            println!("push rax");
        }
    }
}

pub fn generate_as_left(unary: &Unary, env: &mut crate::generator::x86_64::globals::Env) {
    // 左辺値として生成
    println!("# unary as left ----");
    match unary.op {
        UnOperator::Neg => {
            panic!("Invalid Left Value");
        }
        UnOperator::Ref => {
            panic!("Invalid Left Value");
        }
        UnOperator::Deref(count) => {
            match &*unary.expr {
                Exprs::Primary(prim) => {
                    println!("# deref primary as left ----");

                    primary::generate_as_left(prim, env);
                }
                Exprs::Unary(un) => {
                    println!("# deref unary as left ----");
                    generate_as_left(un, env);
                }
                Exprs::Binary(bin) => {
                    todo!();
                    // generate_as_left(&bin, env);
                }
            }

            println!("pop rax");

            for _ in 0..count {
                println!("mov rax, [rax]");
            }
            println!("push rax");
        }
    }
}
