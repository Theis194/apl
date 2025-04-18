use crate::ast::Function;

use super::{Parser, Stmt};
use apl_scanner::{Token, TokenType};

impl Parser {
    pub(crate) fn statement(&mut self) -> Result<Stmt, String> {
        match &self.peek().token_type {
            TokenType::Let => self.variable_declaration(),
            TokenType::Function => self.parse_function_declaration(),
            _ => Err("Unexpected statement".to_string()),
        }
    }

    fn parse_function_declaration(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Function, "Expected function keyword")?;

        let ident = self.parse_identifier("Expected function name")?;

        let mut params = Vec::new();

        self.consume(TokenType::ParenthesesOpen, "Expected '('")?;

        while self.peek().token_type != TokenType::ParenthesesClose {
            let param = match self.consume(
                TokenType::Identifier("".to_string()),
                "Expected parameter name",
            ) {
                Ok(Token {
                    token_type: TokenType::Identifier(name),
                    ..
                }) => name,
                _ => return Err("Expected identifier".to_string()),
            };
            params.push(param.clone());

            if !self.check(&TokenType::Comma) && !self.check(&TokenType::ParenthesesClose) {
                return Err("Expected ',' or ')' after parameter".to_string());
            }
            if self.check(&TokenType::Comma) {
                self.advance(); // Consume comma
            }
        }
        self.consume(TokenType::ParenthesesClose, "Expected ')'")?;

        let mut body: Vec<Stmt> = Vec::new();

        self.consume(TokenType::CurlyOpen, "Expected '{'")?;
        while self.peek().token_type != TokenType::CurlyClose {
            let stmt = self.statement()?;
            body.push(stmt);
        }

        self.consume(TokenType::CurlyClose, "Expected '}'")?;

        Ok(Stmt::FunctionDecl(Function::new(ident, params, body)))
    }

    fn parse_assignment_or_expression(&mut self) -> Result<Stmt, String> {
        todo!()
    }
}
