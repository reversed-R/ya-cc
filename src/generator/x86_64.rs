mod expressions;
mod globals;
mod statements;

use crate::validator::{Globals, Program};

pub const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn generate(prog: &Program) {
    println!(".intel_syntax noprefix");

    let string_literal_kvs: Vec<(&String, &usize)> = prog.string_literals.iter().collect();

    println!(".text");
    println!(".section .rodata");
    for (str, id) in string_literal_kvs {
        println!(".L{id}:");
        println!(".string \"{str}\"");
    }

    println!(".text");
    println!(".globl main");

    for (gname, g) in &prog.globals {
        match g {
            Globals::Function(f) => {
                f.generate(gname);
            }
            _ => {
                // nothing to do
            }
        }
    }
}
