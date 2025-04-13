use super::{Position, Token, TokenType};
use apl_error::{LexError, lexerror::LexErrorType};
use std::{iter::Peekable, str::Chars};

pub(crate) enum ScannerMode {
    Normal,
    StringLiteral,
    CharLiteral,
    BlockComment,
    LineComment,
}

pub struct Scanner<'a> {
    // Character source
    pub chars: Peekable<Chars<'a>>,

    // Lexeme construction
    pub current_lexeme: String,

    // Position tracking
    pub position: Position,
    pub current_char: Option<char>,
    pub start_line: usize,
    pub start_column: usize,

    // Error handling
    pub errors: Vec<LexError>,

    // State flag
    pub mode: ScannerMode,
    // Original source for error context
    pub source: &'a str,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
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

    pub(crate) fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let token = match self.mode {
                ScannerMode::Normal => self.scan_normal_mode(),
                ScannerMode::StringLiteral => self.scan_string_literal(),
                ScannerMode::CharLiteral => self.scan_char_literal(),
                ScannerMode::BlockComment => self.scan_block_comment(),
                ScannerMode::LineComment => self.scan_line_comment(),
            };

            match token {
                Some(token) => {
                    tokens.push(token);
                }
                None => {}
            }
        }

        tokens
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
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

    pub(crate) fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Some(c) => false,
            None => true,
        }
    }

    pub(crate) fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub(crate) fn peek_n(&mut self, n: usize) -> Option<char> {
        let mut clone = self.chars.clone();
        for _ in 0..n - 1 {
            clone.next();
        }
        clone.next()
    }

    pub(crate) fn start_token(&mut self) {
        self.start_line = self.position.line;
        self.start_column = self.position.column;
        self.current_lexeme.clear();
    }

    pub(crate) fn end_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.current_lexeme.clone(),
            line: self.position.line,
            column: self.position.column,
        }
    }

    pub(crate) fn record_error(&mut self, error_type: LexErrorType) {
        let line = self.position.line;
        let column = self.position.column;
        self.errors.push(LexError {
            error_type,
            line: line,
            column: column,
            snippet: self
                .source
                .lines()
                .nth(line - 1)
                .unwrap_or_default()
                .to_string(),
        });
    }

    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn scan_identifier(&mut self) {
        self.scan_type(|c| c.is_alphanumeric() || c == '_');
    }

    pub(crate) fn scan_number(&mut self) {
        self.scan_type(|c| c.is_digit(10) || c == '.');

        let lexeme = &self.current_lexeme;
        if lexeme.matches('.').count() > 1 {
            self.record_error(LexErrorType::TooManyDecimalPoints);
        } else if lexeme.ends_with('.') {
            self.record_error(LexErrorType::TrailingDecimalPoint);
        }
    }

    pub(crate) fn scan_type<F>(&mut self, f: F)
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

    pub(crate) fn identify_keyword(&self) -> TokenType {
        match self.current_lexeme.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "for" => TokenType::For,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}
