mod lexer;

fn main() {
    let tokens = lexer::tokenize(
        "3c==8jdu(if) ){ jiff=wea\ne+}r[int{ifjs+3  fhawp\teih===hh2$8'&\"a}while]*]w*fsa**finte=w+int",
    );

    println!("{:?}", tokens);
}
