use crate::{
    generator::x86_64::{globals::LocalGenerate, ARG_REGS},
    validator::{
        expressions::primary::{Literal, Primary},
        Type, VarAddr,
    },
};

impl LocalGenerate for Primary {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        match self {
            Self::Literal(lit) => match lit {
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
            Self::Variable(var) => match &var.addr {
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
            Self::FnCall(f) => {
                for (i, arg) in f.args.iter().enumerate() {
                    arg.generate(env);

                    if let Some(reg) = ARG_REGS.get(i) {
                        println!("pop {reg}");
                    } else {
                        panic!("Too Many Args for Function Call");
                    }
                }

                println!("call {}", f.name);
                println!("push rax");
            }
            Self::Expr(expr) => {
                expr.generate(env);
            }
        }
    }
}
