use crate::ast::{UnaryExpr, UnaryOp};

use super::{BinaryExpr, BinaryOp, Expr, Literal, Parser, Variable};
use apl_scanner::TokenType;

impl Parser {
    pub(crate) fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;

        if self.check(&TokenType::Equals) {
            self.advance();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assignment {
                    name: var.name,
                    value: Box::new(value),
                });
            }

            return Err("Invalid assignment target".to_string());
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.check(&TokenType::EqualsEquals) || self.check(&TokenType::BangEquals) {
            let operator = self.advance().clone();
            let right = self.comparison()?;

            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ));
        }

        Ok(expr)
    }

    // Add comparison operators (>, >=, <, <=)
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.check(&TokenType::GreaterThan)
            || self.check(&TokenType::GreaterThanOrEqual)
            || self.check(&TokenType::LessThan)
            || self.check(&TokenType::LessThanOrEqual)
        {
            let operator = self.advance().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.check(&TokenType::Plus) || self.check(&TokenType::Minus) {
            let operator = self.advance().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.check(&TokenType::Multiply)
            || self.check(&TokenType::Divide)
            || self.check(&TokenType::Modulo)
        {
            let operator = self.advance().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.check(&TokenType::Bang) || self.check(&TokenType::Minus) {
            let operator = self.advance().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr::new(
                UnaryOp::new(operator.token_type)?,
                Box::new(right),
            )))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token_type {
            TokenType::Number(n) => {
                let num = n
                    .parse()
                    .map_err(|_| "Invalid number literal".to_string())?;
                self.advance();
                Ok(Expr::Literal(Literal::Integer(num)))
            }
            // Check if this is a function call
            TokenType::Identifier(name) => {
                let name_clone = name.clone();
                self.advance();

                if self.check(&TokenType::Dot) {
                    self.parse_method_access(Expr::Variable(Variable { name: name_clone }))
                } else if self.check(&TokenType::ParenthesesOpen){
                    self.parse_call_expression(name_clone)
                } else {
                    Ok(Expr::Variable(Variable { name: name_clone }))
                }
            }
            TokenType::ParenthesesOpen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::ParenthesesClose, "Expected ')' after expression")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err("Expected expression".to_string()),
        }
    }

    pub(crate) fn parse_call_expression(&mut self, callee: String) -> Result<Expr, String> {
        self.consume(TokenType::ParenthesesOpen, "Expected '(' after function name")?;

        let mut arguments = Vec::new();
        if !self.check(&TokenType::ParenthesesClose) {
            loop {
                arguments.push(self.expression()?);
                if !self.check(&TokenType::Comma) {
                    break;
                }
                self.advance();
            }
        }

        self.consume(TokenType::ParenthesesClose, "Expected ')' after arguments")?;

        Ok(Expr::Call {
            callee: Box::new(Expr::Variable(Variable { name: callee })),
            arguments,
        })
    }

    fn parse_method_access(&mut self, object: Expr) -> Result<Expr, String> {
        self.consume(TokenType::Dot, "Expected '.' after object")?;

        let method = match self.advance().token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => return Err("Expected method name after '.'".to_string()),
        };
        
        if self.check(&TokenType::ParenthesesOpen) {
            self.parse_call_expression(method).map(|call_expr| {
                Expr::MethodAccess { object: Box::new(object), method: Box::new(call_expr) }
            })
        } else {
            Ok(Expr::PropertyAccess { object: Box::new(object), propert: method })
        }
    }
}
