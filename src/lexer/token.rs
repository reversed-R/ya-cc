#[derive(Clone, Debug)]
pub enum Token {
    String(String), // string of remain characters
    If,             // if
    While,          // while
    Int,            // int (reserved word of type)
    LPare,          // (
    RPare,          // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Plus,           // +
    Asterisk,       // *
    Assign,         // =
    Equal,          // ==
}

impl Token {
    pub fn pattern(&self) -> String {
        match self {
            Self::String(_) => "".to_string(),
            Self::If => "if".to_string(),
            Self::While => "while".to_string(),
            Self::Int => "int".to_string(),
            Self::LPare => "(".to_string(),
            Self::RPare => ")".to_string(),
            Self::LBrace => "{".to_string(),
            Self::RBrace => "}".to_string(),
            Self::LBracket => "[".to_string(),
            Self::RBracket => "]".to_string(),
            Self::Plus => "+".to_string(),
            Self::Asterisk => "*".to_string(),
            Self::Assign => "=".to_string(),
            Self::Equal => "==".to_string(),
        }
    }
}
