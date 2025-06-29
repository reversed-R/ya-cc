mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize("3- 47+ \n0xF8 *34");
    let tokens2 = lexer::tokenize(" \n 0b1101*41  * 22\t - 51*0x14");

    println!("{:?}", tokens);
    println!("{:#?}", parser::parse(tokens));

    println!("{:?}", tokens2);
    println!("{:#?}", parser::parse(tokens2));
}
