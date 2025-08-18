pub mod symbols;

use std::{fmt::Display, iter::Peekable, slice::Iter};

use crate::lexer::token::{Token, TokenKind};
use symbols::Program;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(Vec<TokenKind>, Token), // expected TokenKind, ... or TokenKind, but found Token in Token.range
    InvalidEOF(Vec<TokenKind>),          // expected TokenKind, ... or TokenKind, but found EOF
    Unknown,                             //
}

trait Parse {
    type SelfType;

    fn consume(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Self::SelfType, ParseError>;
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    Program::consume(&mut tokens.iter().peekable())
}

pub fn matches(opt_t: Option<&Token>, kinds: Vec<TokenKind>) -> Result<TokenKind, ParseError> {
    let t: &Token = opt_t.ok_or(ParseError::InvalidEOF(kinds.clone()))?;

    for kind in &kinds {
        if std::mem::discriminant(&t.kind) == std::mem::discriminant(kind) {
            return Ok(t.kind.clone());
        }
    }

    Err(ParseError::InvalidToken(kinds, t.clone()))
}

impl ParseError {
    pub fn panic_with_error_message(&self, src: &str) -> ! {
        eprint!("\x1b[1;38;2;255;20;0merror\x1b[m: ");
        match self {
            Self::InvalidToken(expecteds, found) => {
                eprintln!(
                    "\x1b[1mexpected {}, but found {} in the {}th character\x1b[m",
                    string_of(expecteds),
                    found.kind,
                    found.range.begin
                );
                eprintln!(
                    "... {}\x1b[4m{}\x1b[m{} ...",
                    &src[0.max(found.range.begin - 10)..found.range.begin],
                    &src[found.range.begin..found.range.end],
                    &src[found.range.end..src.len().min(found.range.end + 10)],
                );
            }
            Self::InvalidEOF(expecteds) => {
                eprintln!(
                    "\x1b[1mexpected {}, but found EOF\x1b[m",
                    string_of(expecteds),
                );
                eprintln!(
                    "... {}\x1b[4m{}\x1b[m",
                    &src[0.max(src.len() - 20)..0.max(src.len() - 5)],
                    &src[0.max(src.len() - 5)..],
                );
            }
            Self::Unknown => {
                eprintln!(
                    "\x1b[1munknown compilation error occured, when parsing source... sorry\x1b[m"
                );
            }
        }

        panic!("");
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                if s.is_empty() {
                    write!(f, "identifier")
                } else {
                    write!(f, "identifier `{s}`")
                }
            }
            Self::IntLiteral(i) => write!(f, "literal `{i}`"),
            Self::CharLiteral(c) => write!(f, "literal `'{c}'`"),
            Self::StringLiteral(s) => write!(f, "literal `\"{s}\"`"),
            _ => write!(f, "`{}`", self.pattern()),
        }
    }
}

fn string_of(kinds: &[TokenKind]) -> String {
    if kinds.is_empty() {
        "".to_string()
    } else if kinds.len() == 1 {
        kinds.first().unwrap().to_string()
    } else if kinds.len() == 2 {
        format!("{} or {}", kinds.first().unwrap(), kinds.last().unwrap())
    } else {
        format!(
            "{}{} or {}",
            &kinds[..kinds.len() - 2]
                .iter()
                .map(|kind| format!("{kind}, "))
                .collect::<String>(),
            &kinds[kinds.len() - 2..].first().unwrap(),
            &kinds[kinds.len() - 2..].last().unwrap()
        )
    }
}
