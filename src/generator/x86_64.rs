mod expressions;
mod globals;
mod statements;

use crate::validator::Program;

pub const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn generate(prog: &Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");

    let mut label_count: usize = 0;

    for (fname, f) in &prog.fns {
        label_count = f.generate(&fname, label_count);
    }
}
