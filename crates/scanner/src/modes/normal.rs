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
                Some(self.end_token(TokenType::Number))
            }

            '"' => {
                self.mode = ScannerMode::StringLiteral;
                None
            }

            '\'' => {
                self.mode = ScannerMode::CharLiteral;
                None
            }

            '+' => Some(self.end_token(TokenType::Plus)),
            '-' => Some(self.end_token(TokenType::Minus)),
            '*' => Some(self.end_token(TokenType::Multiply)),
            '%' => Some(self.end_token(TokenType::Divide)),
            '/' => {
                if let Some('*') = self.peek() {
                    self.advance();
                    self.mode = ScannerMode::BlockComment;
                    None
                } else if let Some('/') = self.peek() {
                    self.advance();
                    self.mode = ScannerMode::LineComment;
                    None
                } else {
                    Some(self.end_token(TokenType::Divide))
                }
            }

            '=' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::EqualsEquals))
                } else {
                    Some(self.end_token(TokenType::Equals))
                }
            }

            '!' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::BangEquals))
                } else {
                    Some(self.end_token(TokenType::Bang))
                }
            }

            '>' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::GreaterThanOrEqual))
                } else {
                    Some(self.end_token(TokenType::GreaterThan))
                }
            }

            '<' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Some(self.end_token(TokenType::LessThanOrEqual))
                } else {
                    Some(self.end_token(TokenType::LessThan))
                }
            }

            _ => {
                self.record_error(LexErrorType::UnexpectedCharacter(c));
                None
            }
        }
    }
}
