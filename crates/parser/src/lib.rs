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
        println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);

        let result = parser.parse();

        assert_eq!(result, vec![Stmt::FunctionDecl(Function::new("test".to_string(), vec!["a".to_string(), "b".to_string()], Vec::new()))]);
    }
}