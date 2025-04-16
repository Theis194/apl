mod binary;
mod variable;

pub use binary::{BinaryExpr, BinaryOp};
pub use variable::Variable;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpr),
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
    },
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i32),
    Float(f32),
}
