use super::{Parser, Stmt, VariableDecl};
use apl_scanner::{Token, TokenType};

impl Parser {
    pub(crate) fn variable_declaration(&mut self) -> Result<Stmt, String> {
        self.advance(); // Consume let

        let name = match self.consume(
            TokenType::Identifier("".to_string()),
            "Expected variable name",
        ) {
            Ok(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            _ => return Err("Expected identifier".to_string()),
        };

        self.consume(TokenType::Equals, "Expected '=' after variable name")?;

        let initializer = self.expression()?;

        self.consume(
            TokenType::SemiColon,
            "Expected ';' after variable declaration",
        )?;

        Ok(Stmt::VariableDecl(VariableDecl::new(name, initializer)))
    }
}