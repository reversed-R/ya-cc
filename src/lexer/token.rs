#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),  // string of remain characters
    IntLiteral(i64), // int literal
    If,              // if
    Else,            // else
    While,           // while
    Return,          // return
    Int,             // int (reserved word of type)
    LPare,           // (
    RPare,           // )
    LBrace,          // {
    RBrace,          // }
    LBracket,        // [
    RBracket,        // ]
    Plus,            // +
    Minus,           // -
    Asterisk,        // *
    Slash,           // /
    Lesser,          // <
    Greater,         // >
    LesEq,           // <=
    GrtEq,           // >=
    Equal,           // ==
    NotEq,           // !=
    Assign,          // =
    Colon,           // :
    SemiColon,       // ;
}

impl Token {
    pub fn pattern(&self) -> String {
        match self {
            Self::String(_) => "".to_string(),
            Self::IntLiteral(_) => "".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
            Self::While => "while".to_string(),
            Self::Return => "return".to_string(),
            Self::Int => "int".to_string(),
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
            Self::Lesser => "<".to_string(),
            Self::Greater => ">".to_string(),
            Self::LesEq => "<=".to_string(),
            Self::GrtEq => ">=".to_string(),
            Self::Equal => "==".to_string(),
            Self::NotEq => "!=".to_string(),
            Self::Assign => "=".to_string(),
            Self::Colon => ":".to_string(),
            Self::SemiColon => ";".to_string(),
        }
    }
}
