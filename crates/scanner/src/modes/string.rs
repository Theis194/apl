use apl_error::lexerror::LexErrorType;

use crate::tokens::{Token, TokenType};

use super::super::{Scanner, ScannerMode};

impl<'a> Scanner<'a> {
    pub(crate) fn scan_string_literal(&mut self) -> Option<Token> {
        let mut value = String::new();
        let mut terminated = false;

        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    terminated = true;
                    break;
                }
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
                            }
                        })
                    } else {
                        self.record_error(LexErrorType::UnterminatedString);
                        break;
                    }
                }
                _ => value.push(c),
            }
        }

        if !terminated {
            self.record_error(LexErrorType::UnterminatedString);
        }

        self.mode = ScannerMode::Normal;

        Some(self.end_token(TokenType::String(value)))
    }

    pub(crate) fn scan_char_literal(&mut self) -> Option<Token> {
        self.advance(); // Skip opening quote
        let value = match self.advance() {
            Some('\'') => {
                // Empty char literal ''
                self.record_error(LexErrorType::EmptyCharLiteral);
                '\0' // Default value for error case
            }
            Some('\\') => {
                // Handle escape sequences
                match self.advance() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('\\') => '\\',
                    Some('\'') => '\'',
                    Some(escaped) => {
                        self.record_error(LexErrorType::InvalidEscape(escaped));
                        escaped // Still use the character despite the error
                    }
                    None => {
                        self.record_error(LexErrorType::UnterminatedChar);
                        '\0'
                    }
                }
            }
            Some(c) => c,
            None => {
                self.record_error(LexErrorType::UnterminatedChar);
                return None;
            }
        };

        // Expect closing quote
        match self.advance() {
            Some('\'') => (),
            Some(_) => {
                self.record_error(LexErrorType::TooManyChars);
                // Skip until closing quote or end
                while let Some(c) = self.advance() {
                    if c == '\'' {
                        break;
                    }
                }
            }
            None => {
                self.record_error(LexErrorType::UnterminatedChar);
            }
        }

        Some(self.end_token(TokenType::Char(value)))
    }
}
