use super::{Position, Token, TokenType};
use apl_error::{LexError, lexerror::LexErrorType};
use std::{iter::Peekable, str::Chars};

enum ScannerMode {
    Normal,
    StringLiteral,
    CharLiteral,
    BlockComment,
    LineComment,
}

pub struct Scanner<'a> {
    // Character source
    chars: Peekable<Chars<'a>>,

    // Lexeme construction
    current_lexeme: String,

    // Position tracking
    position: Position,
    current_char: Option<char>,
    start_line: usize,
    start_column: usize,

    // Error handling
    pub errors: Vec<LexError>,

    // State flag
    mode: ScannerMode,
    // Original source for error context
    pub source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        assert!(!source.is_empty(), "Scanner source cannot be empty");

        Self {
            chars: source.chars().peekable(),
            current_lexeme: String::new(),
            position: Position::new(),
            current_char: None,
            start_line: 1,
            start_column: 1,
            errors: Vec::new(),
            mode: ScannerMode::Normal,
            source,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            match self.mode {
                ScannerMode::Normal => {
                    self.skip_whitespace();
                    if self.is_at_end() { break; }

                    self.start_token();
                    let c = self.advance().unwrap();

                    match c {
                        c if c.is_alphabetic() => {
                            self.scan_identifier();
                            let token_type = self.identify_keyword();
                            tokens.push(self.end_token(token_type));
                        },

                        c if c.is_digit(10) => {
                            self.scan_number();
                            tokens.push(self.end_token(TokenType::Number))
                        },

                        '"' => {
                            self.mode = ScannerMode::StringLiteral;
                        },

                        '+' => tokens.push(self.end_token(TokenType::Plus)),
                        '-' => tokens.push(self.end_token(TokenType::Minus)),

                        '=' => {
                            if let Some('=') = self.peek() {
                                self.advance();
                                tokens.push(self.end_token(TokenType::EqualsEquals));
                            } else {
                                tokens.push(self.end_token(TokenType::Equals));
                            }
                        },

                        '!' => {
                            if let Some('=') = self.peek() {
                                self.advance();
                                tokens.push(self.end_token(TokenType::BangEquals));
                            } else {
                                tokens.push(self.end_token(TokenType::Bang));
                            }
                        }

                        _ => self.record_error(LexErrorType::UnexpectedCharacter(c)),
                    }
                },
                ScannerMode::StringLiteral => {
                    tokens.push(self.scan_string_literal());
                    self.mode = ScannerMode::Normal;
                },
                _ => {}
            }
        }

        tokens
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.current_char = Some(c);
        self.current_lexeme.push(c);

        if c == '\n' {
            self.position.increment_line();
            self.position.increment_column();
        } else {
            self.position.increment_column();
        }

        Some(c)
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Some(c) => false,
            None => true,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek_n(&mut self, n: usize) -> Option<char> {
        let mut clone = self.chars.clone();
        for _ in 0..n - 1 {
            clone.next();
        }
        clone.next()
    }

    fn start_token(&mut self) {
        self.start_line = self.position.line;
        self.start_column = self.position.column;
        self.current_lexeme.clear();
    }

    fn end_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.current_lexeme.clone(),
            line: self.position.line,
            column: self.position.column,
        }
    }

    fn record_error(&mut self, error_type: LexErrorType) {
        let line = self.position.line;
        let column = self.position.column;
        self.errors.push(LexError {
            error_type,
            line: line,
            column: column,
            snippet: self.source
                .lines()
                .nth(line - 1)
                .unwrap_or_default()
                .to_string(),
        });
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_string_literal(&mut self) -> Token {
        let mut value = String::new();
        let mut terminated = false;

        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    terminated = true;
                    break;
                },
                '\\' => {
                    if let Some(escaped) = self.advance() {
                        value.push(match escaped {
                            'n' => '\n',
                            't' => '\t',
                            '"' => '"',
                            '\\' => '\\',
                            _ => {
                                self.record_error(LexErrorType::InvalidEscape(escaped));
                                escaped
                            },
                        })
                    } else {
                        self.record_error(LexErrorType::UnterminatedString);
                        break;
                    }
                },
                _ => value.push(c),
            }
        }

        if !terminated {
            self.record_error(LexErrorType::UnterminatedString);
        }

        self.end_token(TokenType::String(value))
    }

    fn scan_identifier(&mut self) {
        self.scan_type(|c| c.is_alphanumeric() || c == '_');
    }

    fn scan_number(&mut self) {
        self.scan_type(|c| c.is_digit(10) || c == '.');

        let lexeme = &self.current_lexeme;
        if lexeme.matches('.').count() > 1 {
            self.record_error(LexErrorType::TooManyDecimalPoints);
        } else if lexeme.ends_with('.') {
            self.record_error(LexErrorType::TrailingDecimalPoint);
        }
    }

    fn scan_type<F>(&mut self, f: F)
    where 
    F: Fn(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if f(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn identify_keyword(&self) -> TokenType {
        match self.current_lexeme.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            _ => TokenType::Identifier,
        }
    }
}
