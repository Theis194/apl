use super::{BinaryExpr, BinaryOp, Expr, Literal, Parser, Variable};
use apl_scanner::TokenType;

impl Parser {
    pub(crate) fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    pub(crate) fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        
        while self.check(&TokenType::EqualsEquals) || self.check(&TokenType::BangEquals) {
            let operator = self.advance().clone();
            let right = self.primary()?;

            expr = Expr::Binary(BinaryExpr::new(
                Box::new(expr),
                BinaryOp::new(operator.token_type)?,
                Box::new(right),
            ));
        }

        Ok(expr)
    }

    pub(crate) fn primary(&mut self) -> Result<Expr, String> {
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

    pub(crate) fn parse_call_expression(&mut self) -> Result<Expr, String> {
        if self.check_sequence(&[
            TokenType::Identifier("".to_string()),
            TokenType::ParenthesesOpen,
        ]) {
            self.parse_function_call()
        } else if self.check_sequence(&[TokenType::Identifier("".to_string()), TokenType::Dot]) {
            self.parse_method_call()
        } else {
            Err("Unknown token in call expression".to_string())
        }
    }

    fn parse_function_call(&mut self) -> Result<Expr, String> {
        let function_name = self.parse_identifier("Expected function name");
        todo!()
    }

    fn parse_method_call(&mut self) -> Result<Expr, String> {
        let object_name = self.parse_identifier("Expected method name");
        todo!()
    }
}
