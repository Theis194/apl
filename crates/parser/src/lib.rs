pub mod ast;
pub mod core;
mod declarations;
mod expressions;
mod statements;

use ast::{BinaryExpr, BinaryOp, UnaryExpr, UnaryOp};
use ast::{Expr, Literal, Stmt};
use ast::{Function, Variable, VariableDecl};
use core::Parser;

#[cfg(test)]
mod tests {
    use apl_scanner::Scanner;

    use super::*;

    fn parse_expr(input: &str) -> Result<Expr, String> {
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        parser.expression()
    }

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

    #[test]
    fn test_basic_arithmetic() {
        assert!(parse_expr("1 + 2 * 3").is_ok());
        assert!(parse_expr("(1 + 2) * 3").is_ok());
    }

    #[test]
    fn test_function_calls() {
        assert!(parse_expr("foo()").is_ok());
        assert!(parse_expr("bar(1, 2)").is_ok());
        assert!(parse_expr("foo(bar())").is_ok());
    }

    #[test]
    fn test_method_calls() {
        // Valid cases
        assert!(parse_expr("obj.method()").is_ok());
        assert!(parse_expr("container.getItem(1)").is_ok());
        assert!(parse_expr("builder.setX(1).setY(2).build()").is_ok());

        // Error cases
        assert!(parse_expr("obj.").is_err());
        assert!(parse_expr("obj.method(").is_err());
        assert!(parse_expr("obj..method").is_err());
    }

    #[test]
    fn test_property_access() {
        assert!(parse_expr("obj.property").is_ok());
        assert!(parse_expr("array.length").is_ok());
    }

    #[test]
    fn test_error_cases() {
        assert!(parse_expr("foo(").is_err());
        assert!(parse_expr("obj.").is_err());
        assert!(parse_expr("1 +").is_err());
    }

    #[test]
    fn test_complex_expressions() {
        assert!(parse_expr("(a + b) * c.method(d)").is_ok());
        assert!(parse_expr("Math.max(score1, score2) + bonus").is_ok());
    }
}
