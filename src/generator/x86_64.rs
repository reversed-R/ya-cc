mod expressions;
mod globals;
mod statements;

use crate::parser::symbols::Program;

pub fn generate(prog: Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");

    for fn_dec in prog.fns {
        fn_dec.generate();
    }
}
