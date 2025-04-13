use apl_scanner::{Scanner, TokenType};

fn main() {
    let mut scanner = Scanner::new("let a = 10;");
    let tokens: Vec<TokenType> = scanner.scan_tokens().iter().map(|t| t.token_type.clone()).collect();
    println!("{:?}", tokens);
}