pub mod token;

use token::Token;

pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    // delimiter operators
    // ISSUE: must sort loger by operator chars length
    let delims: Vec<Token> = vec![
        Token::Equal,    // ==
        Token::LPare,    // (
        Token::RPare,    // )
        Token::LBrace,   // {
        Token::RBrace,   // }
        Token::LBracket, // [
        Token::RBracket, // ]
        Token::Plus,     // +
        Token::Asterisk, // *
        Token::Assign,   // =
    ];
    let words: Vec<&str> = str.split(&[' ', '\t', '\n'][..]).collect();

    for w in words {
        tokens.append(&mut to_tokens(w, &delims));
    }

    // check tokens judged as string but can be judged as a reserved word
    for i in 0..tokens.len() {
        match tokens.get(i) {
            Some(t) => match t {
                Token::String(s) => {
                    let replacers = vec![Token::If, Token::While, Token::Int];

                    for r in replacers {
                        if &r.pattern() == s {
                            tokens.remove(i);
                            tokens.insert(i, r.to_owned());
                            break;
                        }
                    }
                }
                _ => {
                    continue;
                }
            },
            None => {
                continue;
            }
        }
    }

    tokens
}

fn to_tokens(str: &str, delims: &Vec<Token>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut last_index = 0; // last index of char which is not pushed to tokens vector

    let mut i: usize = 0;
    while i < str.len() {
        for d in delims {
            if (|p: &str| {
                if i + p.len() - 1 < str.len() {
                    if p == &str[i..i + p.len()] {
                        if i - last_index > 0 {
                            tokens.push(Token::String(str[last_index..i].to_string()));
                        }
                        tokens.push(d.clone());

                        last_index = i + p.len();
                        i = i + p.len() - 1;

                        return true;
                    }
                }

                false
            })(&d.pattern())
            {
                break; // if delims sorted by length of pattern string, match with longest
            }
        }

        i = i + 1;
    }

    if last_index < str.len() {
        tokens.push(Token::String(str[last_index..].to_string()));
    }

    tokens
}
