use apl_error::lexerror::LexErrorType;

use crate::{match_operator, simple_token, tokens::{Token, TokenType}, transition_mode};

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
            // Scans keywords and identifiers
            c if c.is_alphabetic() => {
                self.scan_identifier();
                let token_type = self.identify_keyword();
                Some(self.end_token(token_type))
            },

            // Scans numbers floating point and integers
            c if c.is_digit(10) => {
                self.scan_number();
                Some(self.end_token(TokenType::Number(self.current_lexeme.clone())))
            },

            // Switches mode to StringLiteral or CharLiteral
            '"' => transition_mode!(self, StringLiteral),
            '\'' => transition_mode!(self, CharLiteral),

            // Handles operators and brackets
            '+' => simple_token!(self, Plus),
            '-' => simple_token!(self, Minus),
            '*' => simple_token!(self, Multiply),
            '%' => simple_token!(self, Modulo),
            ';' => simple_token!(self, SemiColon),
            '.' => match_operator!(self, '.', '.', Range, Dot),
            ',' => simple_token!(self, Comma),
            '{' => simple_token!(self, CurlyOpen),
            '}' => simple_token!(self, CurlyClose),
            '[' => simple_token!(self, BracketOpen),
            ']' => simple_token!(self, BracketClose),
            '(' => simple_token!(self, ParenthesesOpen),
            ')' => simple_token!(self, ParenthesesClose),

            // Handles equality/inequality 
            '=' => match_operator!(self, '=', '=', EqualsEquals, Equals),
            '!' => match_operator!(self, '!', '=', BangEquals, Bang),
            '>' => match_operator!(self, '>', '=', GreaterThanOrEqual, GreaterThan),
            '<' => match_operator!(self, '<', '=', LessThanOrEqual, LessThan),
            
            // Handles comments and divition
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

            // Handles unexpected characters
            _ => {
                self.record_error(LexErrorType::UnexpectedCharacter(c));
                None
            }
        }
    }
}
