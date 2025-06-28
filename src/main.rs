mod lexer;

fn main() {
    let tokens = lexer::tokenize(
        "3c==8jdu(if)0x1a ){ jiff=wea\ne+}r[int{ifjs+3  fhawp\t0xAB 0xAb90r eih===hh2$8'&\"a}while]*]w*fsa**finte=w+int",
    );

    println!("{:?}", tokens);
}
