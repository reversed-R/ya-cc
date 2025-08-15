mod expressions;
mod globals;
mod statements;

use crate::validator::{Program, VarAddr};

pub const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn generate(prog: &Program) {
    println!(".intel_syntax noprefix");

    let string_literal_kvs: Vec<(&String, &usize)> = prog.string_literals.iter().collect();

    if !string_literal_kvs.is_empty() {
        println!(".text");
        println!(".section .rodata");
        for (str, id) in string_literal_kvs {
            println!(".L{id}:");
            println!(".string \"{str}\"");
        }
    }

    if !prog.global_vars.is_empty() {
        println!(".data");
        for var in prog.global_vars.values() {
            if let VarAddr::Global(g) = &var.addr {
                println!("{g}:");
                println!("  .zero {}", var.typ.size());
            } else {
                panic!("Invalid Global Variable");
            }
        }
    }

    println!(".text");
    for (fname, f) in &prog.fns {
        if fname == "main" {
            println!(".globl main");
        }
        f.generate(fname);
    }
}
