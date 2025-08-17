use std::ops::Deref;

use crate::{
    generator::x86_64::{
        expressions::{binary, primary},
        globals::LocalGenerate,
    },
    validator::{
        expressions::{Exprs, Primary, UnOperator, Unary},
        VarAddr,
    },
};

pub fn generate(unary: &Unary, env: &mut crate::generator::x86_64::globals::Env) {
    match unary.op {
        UnOperator::Neg => {
            unary.expr.generate(env);

            println!("neg rax");
        }
        UnOperator::Ref => {
            if let Exprs::Primary(Primary::Variable(var)) = &*unary.expr {
                match &var.addr {
                    VarAddr::Local(offset) => {
                        println!("mov rax, rbp");
                        println!("sub rax, {offset}");
                    }
                    VarAddr::Global(label) => {
                        println!("lea rax, {label}[rip]");
                    }
                }
            } else if let Exprs::Primary(Primary::Expr(expr)) = &*unary.expr {
                if let Exprs::Primary(Primary::Variable(var)) = expr.deref() {
                    match &var.addr {
                        VarAddr::Local(offset) => {
                            println!("mov rax, rbp");
                            println!("sub rax, {offset}");
                        }
                        VarAddr::Global(label) => {
                            println!("lea rax, {label}[rip]");
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

            for _ in 0..count {
                println!("mov rax, [rax]");
            }
        }
    }
}

pub fn generate_as_left(unary: &Unary, env: &mut crate::generator::x86_64::globals::Env) -> usize {
    // 左辺値として生成
    match unary.op {
        UnOperator::Neg => {
            panic!("Invalid Left Value");
        }
        UnOperator::Ref => {
            panic!("Invalid Left Value");
        }
        UnOperator::Deref(count) => {
            let derefed_count = match &*unary.expr {
                Exprs::Primary(prim) => primary::generate_as_left(prim, env),
                Exprs::Unary(un) => generate_as_left(un, env),
                Exprs::Binary(bin) => binary::generate_as_left(bin, env),
            };

            for _ in derefed_count..count {
                println!("mov rax, [rax]");
            }

            count
        }
    }
}
