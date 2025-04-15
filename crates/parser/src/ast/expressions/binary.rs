use apl_scanner::TokenType;
use apl_derive::New;

use super::Expr;

#[derive(New)]
pub struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
}

pub enum BinaryOp {
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Boolean
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl BinaryOp {
    pub fn new(token_type: TokenType) -> Result<Self, String> {
        match token_type {
            // Math
            TokenType::Plus => Ok(BinaryOp::Add),
            TokenType::Minus => Ok(BinaryOp::Subtract),
            TokenType::Multiply => Ok(BinaryOp::Multiply),
            TokenType::Divide => Ok(BinaryOp::Divide),
            TokenType::Modulo => Ok(BinaryOp::Modulo),
            // Boolean
            TokenType::Equals => Ok(BinaryOp::Equal),
            TokenType::BangEquals => Ok(BinaryOp::NotEqual),
            TokenType::GreaterThan => Ok(BinaryOp::Greater),
            TokenType::GreaterThanOrEqual => Ok(BinaryOp::GreaterEqual),
            TokenType::LessThan => Ok(BinaryOp::Less),
            TokenType::LessThanOrEqual => Ok(BinaryOp::LessEqual),
            _ => Err("Unknown binary operator".to_string()),
        }
    }
}