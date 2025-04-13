use apl_error::lexerror::LexErrorType;

use crate::tokens::{Token, TokenType};

use super::super::{Scanner, ScannerMode};

impl<'a> Scanner<'a> {
    pub(crate) fn scan_block_comment(&mut self) -> Option<Token> {
        while let Some(c) = self.advance() {
            match c {
                '*' => {
                    if let Some('/') = self.peek() {
                        self.advance();
                        break;
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub(crate) fn scan_line_comment(&mut self) -> Option<Token> {
        while let Some(c) = self.advance() {
            match c {
                '\n' => break,
                _ => {},
            }
        }
        None
    }
}