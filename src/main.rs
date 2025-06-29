mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize("3+ \n0xF8");
    let tokens2 = lexer::tokenize(" \n 0b1101\t - 51");

    println!("{:?}", tokens);
    println!("{:#?}", parser::parse(tokens));

    println!("{:?}", tokens2);
    println!("{:#?}", parser::parse(tokens2));
}
