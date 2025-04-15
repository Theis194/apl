use super::{Parser, Stmt};
use apl_scanner::TokenType;

impl Parser {
    pub(crate) fn statement(&mut self) -> Result<Stmt, String> {
        let identifier = self.consume(TokenType::Identifier("".to_string()), "Expected an identifier");

        match &self.peek().token_type {
            TokenType::ParenthesesOpen => {
                // This is a function/function call
                todo!()
            },
            _ => {
                Err("Unexpected statement".to_string())
            }
        }
    }
}