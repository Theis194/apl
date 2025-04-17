use apl_derive::New;
use apl_scanner::TokenType;

use super::Expr;

#[derive(New, PartialEq, Debug)]
pub struct UnaryExpr {
    op: UnaryOp,
    right: Box<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Not,
    Negative,
}

impl UnaryOp {
    pub fn new(token_type: TokenType) -> Result<Self, String> {
        match token_type {
            TokenType::Bang => Ok(UnaryOp::Not),
            TokenType::Minus => Ok(UnaryOp::Negative),
            _ => Err("Invalid unary operator".to_string()),
        }
    }
}