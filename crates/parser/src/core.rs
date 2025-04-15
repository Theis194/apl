use std::result;

use apl_scanner::{Token, TokenType};

use crate::ast::{BinaryExpr, BinaryOp};

use super::{Expr, Literal, Stmt};
use super::{Variable, VariableDecl};

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
        let result = if self.check(TokenType::Let) {
            self.variable_declaration()
        } else {
            self.statement()
        };

        result.ok()
    }

    fn variable_declaration(&mut self) -> Result<Stmt, String> {
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

    fn statement(&mut self) -> Result<Stmt, String> {
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

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        while self.check(TokenType::EqualsEquals) || self.check(TokenType::BangEquals) {
            let operator = self.advance().clone();
            let right = self.primary()?;
            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ))
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token_type {
            TokenType::Number(n) => {
                let num = n.parse().unwrap();
                self.advance();
                Ok(Expr::Literal(Literal::Integer(num)))
            }
            TokenType::Identifier(name) => {
                let name_clone = name.clone();
                self.advance();
                Ok(Expr::Variable(Variable { name: name_clone }))
            }
            _ => Err("Expected expression".to_string()),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }
}
