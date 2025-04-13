#[derive(Debug, Clone)]
pub struct LexError {
    pub error_type: LexErrorType,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexErrorType {
    UnexpectedCharacter(char),
    UnterminatedString,
    InvalidEscapeSequence(char),
    InvalidEscape(char),
    MalformedNumber,
    TooManyDecimalPoints,
    TrailingDecimalPoint,
    EmptyCharLiteral,
    UnterminatedChar,
    TooManyChars,
}