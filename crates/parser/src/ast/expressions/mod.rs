mod binary;
mod unary;
mod variable;

pub use binary::{BinaryExpr, BinaryOp};
pub use unary::{UnaryExpr, UnaryOp};
pub use variable::Variable;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(Box<Expr>),
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MethodAccess {
        object: Box<Expr>,
        method: Box<Expr>,
    },
    PropertyAccess {
        object: Box<Expr>,
        propert: String,
    },
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i32),
    Float(f32),
}
