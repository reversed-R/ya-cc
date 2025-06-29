pub mod token;

use token::Token;

pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    // delimiter operators
    // ISSUE: must sort longer by operator chars length
    let delims: Vec<Token> = vec![
        Token::Equal,    // ==
        Token::LPare,    // (
        Token::RPare,    // )
        Token::LBrace,   // {
        Token::RBrace,   // }
        Token::LBracket, // [
        Token::RBracket, // ]
        Token::Plus,     // +
        Token::Minus,    // -
        Token::Asterisk, // *
        Token::Assign,   // =
    ];
    let words: Vec<&str> = str.split(&[' ', '\t', '\n'][..]).collect();

    for w in words {
        tokens.append(&mut to_tokens(w, &delims));
    }

    // check tokens judged as string but can be judged as a reserved word
    tokens = tokens
        .into_iter()
        .map(|t| {
            match t {
                Token::String(s) => {
                    // check literal expressions
                    if let Some(t) = try_get_dec_int(&s) {
                        t
                    } else if let Some(t) = try_get_prefixed_int(&s) {
                        t
                    } else {
                        let replacers = vec![Token::If, Token::While, Token::Int];

                        for r in replacers {
                            if &r.pattern() == &s {
                                return r.to_owned();
                            }
                        }

                        Token::String(s)
                    }
                }
                _ => t,
            }
        })
        .collect();

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

fn try_get_dec_int(str: &str) -> Option<Token> {
    if let Ok(dec) = usize::from_str_radix(str, 10) {
        Some(Token::IntLiteral(dec as i64))
    } else {
        None
    }
}

fn try_get_prefixed_int(str: &str) -> Option<Token> {
    if str.len() > 2 && str.chars().nth(0).unwrap() == '0' {
        let radix = match str.chars().nth(1).unwrap().to_ascii_lowercase() {
            'b' => 2,
            'o' => 8,
            'x' => 16,
            _ => 0,
        };

        if radix != 0 {
            if let Ok(u) = usize::from_str_radix(&str[2..], radix) {
                Some(Token::IntLiteral(u as i64))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
