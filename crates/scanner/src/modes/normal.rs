use apl_error::lexerror::LexErrorType;

use crate::tokens::{Token, TokenType};

use super::super::{Scanner, ScannerMode};

impl<'a> Scanner<'a> {
    pub(crate) fn scan_normal_mode(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.is_at_end() {
            return None;
        }

        self.start_token();
        let c = self.advance().unwrap();

        match c {
            c if c.is_alphabetic() => {
                self.scan_identifier();
                let token_type = self.identify_keyword();
                Some(self.end_token(token_type))
            }

            c if c.is_digit(10) => {
                self.scan_number();
                Some(self.end_token(TokenType::Number(self.current_lexeme.clone())))
            }

            '"' => {
                self.set_scanner_mode(ScannerMode::StringLiteral);
                None
            }

            '\'' => {
                self.set_scanner_mode(ScannerMode::CharLiteral);
                None
            }

            '+' => Some(self.end_token(TokenType::Plus)),
            '-' => Some(self.end_token(TokenType::Minus)),
            '*' => Some(self.end_token(TokenType::Multiply)),
            '%' => Some(self.end_token(TokenType::Modulo)),
            '/' => {
                if let Some('*') = self.peek() {
                    self.advance();
                    self.set_scanner_mode(ScannerMode::BlockComment);
                    None
                } else if let Some('/') = self.peek() {
                    self.advance();
                    self.set_scanner_mode(ScannerMode::LineComment);
                    None
                } else {
                    Some(self.end_token(TokenType::Divide))
                }
            },

            '=' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::EqualsEquals))
                } else {
                    Some(self.end_token(TokenType::Equals))
                }
            },

            '!' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::BangEquals))
                } else {
                    Some(self.end_token(TokenType::Bang))
                }
            },

            '>' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::GreaterThanOrEqual))
                } else {
                    Some(self.end_token(TokenType::GreaterThan))
                }
            },

            '<' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::LessThanOrEqual))
                } else {
                    Some(self.end_token(TokenType::LessThan))
                }
            },

            ';' => {
                Some(self.end_token(TokenType::SemiColon))
            },

            _ => {
                self.record_error(LexErrorType::UnexpectedCharacter(c));
                None
            }
        }
    }
}
