use crate::tokens::Token;

use super::super::Scanner;

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
                _ => {}
            }
        }
        None
    }
}
