pub mod token;

use token::Token;

#[derive(Debug)]
pub enum TokenizeError {
    SingleQuoteCloseNotFound,
    DoubleQuoteCloseNotFound,
}

pub fn tokenize(str: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut pretokens: Vec<PreToken> = vec![];

    #[derive(Debug)]
    enum PreToken<'src> {
        Raw(&'src str),
        CharLiteral(u8),
        StringLiteral(&'src str),
    }

    let mut i = 0;
    let mut raw_head = 0;
    while i < str.len() {
        if &str[i..i + 1] == "\'" {
            if i - raw_head > 0 {
                pretokens.push(PreToken::Raw(&str[raw_head..i]));
            }
            i += 1;

            let mut iquote = i;
            while iquote < str.len() {
                // TODO:
                // if backslach appear, start escape
                if &str[iquote..iquote + 1] == "\'" {
                    raw_head = iquote + 1;
                    if iquote == i + 1 {
                        // ISSUE: more good code ...
                        let c: u8 = str[i..i + 1].as_bytes().first().unwrap().clone();
                        pretokens.push(PreToken::CharLiteral(c));
                        i = iquote + 1;
                        break;
                    } else {
                        panic!("TODO: Too Many Characters in Single Quote `'`");
                    }
                }

                iquote += 1;
            }
        } else if &str[i..i + 1] == "\"" {
            if i - raw_head > 0 {
                pretokens.push(PreToken::Raw(&str[raw_head..i]));
            }
            i += 1;

            let mut iquote = i;
            while iquote < str.len() {
                // TODO:
                // if backslach appear, start escape
                if &str[iquote..iquote + 1] == "\"" {
                    raw_head = iquote + 1;
                    pretokens.push(PreToken::StringLiteral(&str[i..iquote]));
                    i = iquote + 1;
                    break;
                }

                iquote += 1;
            }
        } else {
            i += 1;
        }
    }
    if i - raw_head > 0 {
        pretokens.push(PreToken::Raw(&str[raw_head..i]));
    }

    // delimiter operators
    // ISSUE: must sort longer by operator chars length
    let delims: Vec<Token> = vec![
        Token::LesEq,     // <=
        Token::GrtEq,     // >=
        Token::Equal,     // ==
        Token::NotEq,     // !=
        Token::LPare,     // (
        Token::RPare,     // )
        Token::LBrace,    // {
        Token::RBrace,    // }
        Token::LBracket,  // [
        Token::RBracket,  // ]
        Token::Plus,      // +
        Token::Minus,     // -
        Token::Asterisk,  // *
        Token::Slash,     // /
        Token::Percent,   // %
        Token::Ampersand, // &
        Token::Lesser,    // <
        Token::Greater,   // >
        Token::Assign,    // =
        Token::Comma,     // ,
        Token::Colon,     // :
        Token::SemiColon, // ;
    ];

    let mut token_vecs: Vec<Vec<Token>> = vec![];
    for pretoken in &pretokens {
        match pretoken {
            PreToken::CharLiteral(c) => {
                token_vecs.push(vec![Token::CharLiteral(*c)]);
            }
            PreToken::StringLiteral(s) => {
                token_vecs.push(vec![Token::StringLiteral(s.to_string())]);
            }
            PreToken::Raw(r) => {
                let words: Vec<&str> = r.split(&[' ', '\t', '\n'][..]).collect();
                let mut tmp_tokens = vec![];

                for w in words {
                    tmp_tokens.append(&mut to_tokens(w, &delims));
                }

                token_vecs.push(tmp_tokens);
            }
        }
    }

    // check tokens judged as string but can be judged as a reserved word
    let tokens: Vec<Token> = token_vecs
        .into_iter()
        .flatten()
        .map(|t| {
            match t {
                Token::String(s) => {
                    // check literal expressions
                    if let Some(t) = try_get_dec_int(&s) {
                        t
                    } else if let Some(t) = try_get_prefixed_int(&s) {
                        t
                    } else {
                        let replacers = vec![
                            Token::If,
                            Token::Else,
                            Token::While,
                            Token::Return,
                            Token::SizeOf,
                            Token::Int,
                            Token::Char,
                        ];

                        for r in replacers {
                            if r.pattern() == s {
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

    Ok(tokens)
}

fn to_tokens(str: &str, delims: &Vec<Token>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut last_index = 0; // last index of char which is not pushed to tokens vector

    let mut i: usize = 0;
    while i < str.len() {
        for d in delims {
            if (|p: &str| {
                if i + p.len() - 1 < str.len() && p == &str[i..i + p.len()] {
                    if i - last_index > 0 {
                        tokens.push(Token::String(str[last_index..i].to_string()));
                    }
                    tokens.push(d.clone());

                    last_index = i + p.len();
                    i = i + p.len() - 1;

                    return true;
                }

                false
            })(&d.pattern())
            {
                break; // if delims sorted by length of pattern string, match with longest
            }
        }

        i += 1;
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
    if str.len() > 2 && str.starts_with('0') {
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
