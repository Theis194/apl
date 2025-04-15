use std::clone;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Identifier(String),
    Number(String),
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
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    ParenthesesOpen,
    ParenthesesClose,
    SemiColon,
    Dot,
    Comma,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}