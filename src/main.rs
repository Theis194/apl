use apl_parser::{ast::Stmt, core::Parser};
use apl_scanner::{Scanner, Token};

fn main() {
    let mut scanner = Scanner::new("let a = 10;");
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let result: Vec<Stmt> = parser.parse();
    println!("{:?}", result);
}