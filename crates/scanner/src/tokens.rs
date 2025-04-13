#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Identifier,
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
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}