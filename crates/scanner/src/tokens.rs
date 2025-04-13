#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Identifier,
    Number,
    String(String),
    Char(char),
    Equals,
    EqualsEquals,
    BangEquals,
    Bang,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    If,
    For,
    While,
    SemiColon,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}