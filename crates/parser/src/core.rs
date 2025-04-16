use apl_scanner::{Token, TokenType};

use super::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // Should return an AST
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.check(&TokenType::Let) {
            self.variable_declaration()
        } else {
            self.statement()
        };

        result.ok()
    }

    pub(crate) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub(crate) fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }

    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub(crate) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub(crate) fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match (&self.peek().token_type, &token_type) {
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (a, b) => a == *b,
        }
    }

    pub(crate) fn consume(
        &mut self,
        token_type: TokenType,
        message: &str,
    ) -> Result<&Token, String> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(format!(
                "{} at line {}. Expected {:?}, found {:?}",
                message,
                self.peek().line,
                token_type,
                self.peek().token_type
            ))
        }
    }

    pub(crate) fn check_sequence(&self, sequence: &[TokenType]) -> bool {
        sequence
            .iter()
            .enumerate()
            .all(|(i, tt)| self.peek_n(i).map(|t| &t.token_type) == Some(tt))
    }

    pub(crate) fn parse_identifier(&mut self, message: &str) -> Result<String, String> {
        Ok(self
            .consume(
                TokenType::Identifier("".to_string()),
                message,
            )?
            .lexeme
            .clone())
    }
}
