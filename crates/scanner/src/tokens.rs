#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Identifier,
    Number,
    String(String),
    Equals,
    EqualsEquals,
    BangEquals,
    Bang,
    Plus,
    Minus,
    If,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
    }
}