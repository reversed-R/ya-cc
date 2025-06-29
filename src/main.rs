mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize("3+ \n0xF8");

    println!("{:?}", tokens);

    println!("{:#?}", parser::parse(tokens));
}
