pub mod ast;
pub mod core;
mod declarations;
mod expressions;
mod statements;

use ast::{BinaryExpr, BinaryOp};
use ast::{Expr, Literal, Stmt};
use ast::{Variable, VariableDecl};
use core::Parser;

#[cfg(test)]
mod tests {
    use apl_scanner::Scanner;

    use crate::ast::Function;

    use super::*;

    #[test]
    fn parse_function_declaration() {
        let tokens = Scanner::new("fn test(a,b) {}").scan_tokens();

        let mut parser = Parser::new(tokens);

        let result = parser.parse();

        assert_eq!(
            result,
            vec![Stmt::FunctionDecl(Function::new(
                "test".to_string(),
                vec!["a".to_string(), "b".to_string()],
                Vec::new()
            ))]
        );
    }

    #[test]
    fn parse_variable_declaration() {
        let tokens = Scanner::new("let a = 1 == 2;").scan_tokens();
        
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse();

        assert_eq!(
            result,
            vec![Stmt::VariableDecl(VariableDecl::new(
                "a".to_string(),
                Expr::Binary(BinaryExpr::new(
                    Box::new(Expr::Literal(Literal::Integer(1))),
                    BinaryOp::Equal,
                    Box::new(Expr::Literal(Literal::Integer(2)))
                ))
            ))]
        );
    }
}
