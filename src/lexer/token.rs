#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub begin: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),    // identifier
    IntLiteral(i64),       // int literal
    CharLiteral(u8),       // char literal
    StringLiteral(String), // string literal
    If,                    // if
    Else,                  // else
    While,                 // while
    Return,                // return
    SizeOf,                // sizeof
    Int,                   // int (reserved word of type)
    Char,                  // char (reserved word of type)
    Void,                  // void (reserved word of type)
    Struct,                // struct (reserved word of type)
    LPare,                 // (
    RPare,                 // )
    LBrace,                // {
    RBrace,                // }
    LBracket,              // [
    RBracket,              // ]
    Plus,                  // +
    Minus,                 // -
    Asterisk,              // *
    Slash,                 // /
    Percent,               // %
    Ampersand,             // &
    Lesser,                // <
    Greater,               // >
    LesEq,                 // <=
    GrtEq,                 // >=
    Equal,                 // ==
    NotEq,                 // !=
    Assign,                // =
    Comma,                 // ,
    Colon,                 // :
    SemiColon,             // ;
}

impl TokenKind {
    pub fn pattern(&self) -> String {
        match self {
            Self::Identifier(_) => "".to_string(),
            Self::IntLiteral(_) => "".to_string(),
            Self::CharLiteral(_) => "".to_string(),
            Self::StringLiteral(_) => "".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
            Self::While => "while".to_string(),
            Self::Return => "return".to_string(),
            Self::SizeOf => "sizeof".to_string(),
            Self::Int => "int".to_string(),
            Self::Char => "char".to_string(),
            Self::Void => "void".to_string(),
            Self::Struct => "struct".to_string(),
            Self::LPare => "(".to_string(),
            Self::RPare => ")".to_string(),
            Self::LBrace => "{".to_string(),
            Self::RBrace => "}".to_string(),
            Self::LBracket => "[".to_string(),
            Self::RBracket => "]".to_string(),
            Self::Plus => "+".to_string(),
            Self::Minus => "-".to_string(),
            Self::Asterisk => "*".to_string(),
            Self::Slash => "/".to_string(),
            Self::Percent => "%".to_string(),
            Self::Ampersand => "&".to_string(),
            Self::Lesser => "<".to_string(),
            Self::Greater => ">".to_string(),
            Self::LesEq => "<=".to_string(),
            Self::GrtEq => ">=".to_string(),
            Self::Equal => "==".to_string(),
            Self::NotEq => "!=".to_string(),
            Self::Assign => "=".to_string(),
            Self::Comma => ",".to_string(),
            Self::Colon => ":".to_string(),
            Self::SemiColon => ";".to_string(),
        }
    }
}
