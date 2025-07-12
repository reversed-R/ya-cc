mod generator;
mod lexer;
mod parser;

use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if let Some(file_path) = args.get(1) {
            let mut f = File::open(file_path).expect(&format!("File Not Found: `{}`", file_path));

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect(&format!("Internal Error, Reading File: `{}`", file_path));

            println!("File Content:\n```\n{}\n```", contents);

            let tokens = lexer::tokenize(&contents);
            println!("{:?}", tokens);
            println!("{:#?}", parser::parse(tokens));
        }
    } else {
        println!("1 Argument Required: <file-path>")
    }
}
