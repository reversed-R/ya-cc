mod expressions;
mod globals;
mod statements;

use crate::parser::symbols::Program;

pub const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn generate(prog: Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");

    let mut label_count: usize = 0;

    for fn_dec in prog.fns {
        label_count = fn_dec.generate(label_count);
    }
}
