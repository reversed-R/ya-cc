mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize(
        "-3- 47+ \n0xF8 *((34 + 0x3) /2) != 55;return 0xa= 3 != 0x42;\n 1 == 3 <4\t ;",
    );
    let tokens2 = lexer::tokenize(
        "13 == (3 + 22) < \n +0b1101/41  *( 22\t - 51)*0x14; return 21< 43 =2; 0b101= 3 = 33= 4 ==54;",
    );

    println!("{:?}", tokens);
    println!("{:#?}", parser::parse(tokens));

    println!("{:?}", tokens2);
    println!("{:#?}", parser::parse(tokens2));
}
